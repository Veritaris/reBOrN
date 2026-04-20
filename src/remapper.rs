use crate::cli::RebornCliArgs;
use crate::mappings;
use classfile::classfile::ClassFile;
use indoc::indoc;
use linked_hash_map::LinkedHashMap;
use std::io::{stdout, BufReader, BufWriter, Cursor, Error, Read, Write};
use std::ops::AddAssign;
use std::path::{Path, PathBuf};
use std::process::exit;
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;
use utils::cache;

const REMAPPED_SUFFIX: &'static str = "-remapped.jar";
const JAR_SUFFIX: &'static str = ".jar";

pub fn build_output_file_name(input_file_name: String, path: &Path) -> String {
    let canonical_path = match path.canonicalize() {
        Ok(it) => it,
        Err(err) => {
            eprintln!("{}", err);
            exit(-1);
        }
    };
    if path.is_dir() {
        canonical_path
            .join(input_file_name.strip_suffix(JAR_SUFFIX).unwrap().to_owned() + REMAPPED_SUFFIX)
            .to_str()
            .unwrap()
            .to_string()
    } else {
        canonical_path
            .to_str()
            .unwrap()
            .strip_suffix(JAR_SUFFIX)
            .unwrap()
            .to_owned()
            + REMAPPED_SUFFIX
    }
}

pub fn remap_files(args: &RebornCliArgs, files: &Vec<String>) {
    let files_amount = files.len();

    let cpus = num_cpus::get();
    let mut handlers: Vec<JoinHandle<()>> = Vec::new();
    let counter = Arc::new(Mutex::new(0));

    for files_in_group in files.chunks(cpus) {
        let group = Vec::from(files_in_group).clone();
        let thread_args = args.clone();
        let cnt = counter.clone();

        let handle = thread::spawn(move || {
            for (i, file_path) in group.iter().enumerate() {
                cnt.lock().unwrap().add_assign(1);
                println!("\rProcessing [{}/{files_amount}]", cnt.lock().unwrap());
                stdout().flush().unwrap();
                remap_file(&thread_args, i, file_path);
            }
        });

        handlers.push(handle);
    }

    for handler in handlers {
        handler.join().unwrap();
    }
}

pub fn remap_file(args: &RebornCliArgs, input_source_index: usize, input_file_full_path: &String) {
    let input_file_name = Path::new(input_file_full_path)
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

    let output_file_path = match args.output {
        None => input_file_name.strip_suffix(JAR_SUFFIX).unwrap().to_owned() + REMAPPED_SUFFIX,
        Some(ref res) => {
            if &res.len() != &args.input.len() {
                match &res.len() {
                    1 => build_output_file_name(input_file_name, Path::new(res.get(0).unwrap())),
                    _ => {
                        println!("output must contain N entries where N is number of entry files or only 1 entry");
                        input_file_name.strip_suffix(JAR_SUFFIX).unwrap().to_owned()
                            + REMAPPED_SUFFIX
                    }
                }
            } else {
                build_output_file_name(input_file_name, Path::new(res.get(0).unwrap()))
            }
        }
    };

    let input_file_path = Path::new(input_file_full_path);
    let input_file = std::fs::File::open(input_file_path).unwrap();
    let reader = BufReader::new(input_file);

    let mut target_jar = zip::ZipArchive::new(reader).unwrap();
    let appdata = cache::get_appdata_dir();
    let cache_dir = appdata.cache_dir();
    let mapping_dir = cache_dir
        .join("mappings")
        .join(args.game_version.as_str())
        .join(args.mappings_channel.as_str())
        .join(args.mappings_version.as_str());
    let mut mappings = mappings::load_all_mappings(mapping_dir).unwrap();

    if args.verbose > 0 {
        println!("debug: {:?}", args.debug);
        println!("verbosity: {:?}", args.verbose);
        println!("input file or dir to remap: {}", input_file_full_path);
        println!("output file: {}", output_file_path);
        println!("game version: {}", args.game_version);
        println!("using mappings: {}", args.mappings_channel);
    }
    if args.extra_mappings.len() > 0 {
        if args.verbose > 0 {
            println!("using extra mappings:");
        }
        mappings::merge_mappings(&args.extra_mappings, &mut mappings);
        if args.verbose > 0 {
            println!("merged {} mappings", &args.extra_mappings.len());
        }
    }
    let mut common_mappings: LinkedHashMap<String, String> = LinkedHashMap::new();
    common_mappings.extend(mappings.fields);
    common_mappings.extend(mappings.methods);
    common_mappings.extend(mappings.params);

    match remap_jar(&mut target_jar, output_file_path, &args, &common_mappings) {
        Ok(_) => {}
        Err(_) => {}
    };

    println!("Writing remapped file");
    println!(
        "remapping of {:?} finished. Happy coding!",
        input_file_path.file_name().unwrap()
    );
    if args.input.len() > 1 && input_source_index < args.input.len() - 1 {
        println!("{}", "=".repeat(80));
    }
}

pub fn prepare_mappings(args: &RebornCliArgs) {
    let (game_version, mappings_channel, mappings_version) = (
        args.game_version.clone(),
        args.mappings_channel.clone(),
        args.mappings_version.clone(),
    );
    let download_thread = std::thread::spawn(move || {
        utils::download::download_mappings(
            cache::get_appdata_dir().cache_dir(),
            game_version.as_str(),
            mappings_channel.as_str(),
            mappings_version.as_str(),
        );
    });

    match download_thread.join() {
        Ok(_) => {}
        Err(_err) => {}
    }
}

pub fn gather_input_files(input_files: &mut Vec<String>, dir_files: &Vec<String>) {
    for file_path in dir_files {
        let path = Path::new(file_path);

        if path.is_file() {
            input_files.push(String::from(path.canonicalize().unwrap().to_str().unwrap()));
        } else if path.is_dir() {
            let nested_dir_files: Vec<String> = match std::fs::read_dir(file_path) {
                Ok(files) => files
                    .into_iter()
                    .filter_map(|e| match e {
                        Ok(file) => {
                            if !(file.metadata().unwrap().is_dir()
                                || file.path().to_str().unwrap().ends_with(JAR_SUFFIX))
                            {
                                None
                            } else {
                                match file.path().to_str() {
                                    Some(path) => Some(String::from(path)),
                                    None => None,
                                }
                            }
                        }
                        Err(_) => None,
                    })
                    .collect(),
                Err(err) => {
                    println!("cannot list directory {file_path}, err='{err}'");
                    continue;
                }
            };
            gather_input_files(input_files, &nested_dir_files);
        }
    }
}

pub fn remap_jar(
    jar: &mut zip::ZipArchive<BufReader<std::fs::File>>,
    output_file_path: String,
    args: &RebornCliArgs,
    mappings: &LinkedHashMap<String, String>,
) -> Result<(), Error> {
    let output_path = Path::new(output_file_path.as_str());
    let output_file = std::fs::File::create(output_path)?;
    let output_buffer = BufWriter::new(output_file);
    let mut output_jar = zip::ZipWriter::new(output_buffer);

    let files_amount = jar.len();

    for i in 0..files_amount {
        let file: zip::read::ZipFile<'_, BufReader<std::fs::File>> = jar.by_index(i)?;
        let mangled_name = PathBuf::clone(&file.mangled_name());
        let filename = mangled_name.to_str().unwrap();
        let file_size = file.size();

        if filename.ends_with(".class") {
            match args.verbose {
                0 | 1 | 2 => (),
                _ => {
                    let msg = format!(
                        indoc!(
                            r#"
                    found class file {}
                      compression: {}
                      size: {} bytes
                    "#
                        ),
                        filename,
                        file.compression(),
                        file_size
                    );
                    println!("{}", msg)
                }
            };
            let mut zip_reader = BufReader::new(file);
            let mut tmp_buf = vec![0u8; file_size as usize];
            zip_reader.read_exact(&mut tmp_buf)?;
            let reader = BufReader::new(Cursor::new(tmp_buf));

            let classfile_raw = if args.no_deobf {
                ClassFile::read(reader, None)
            } else {
                ClassFile::read(reader, Some(mappings))
            };

            let classfile = match classfile_raw {
                Ok(cf) => {
                    if args.debug {
                        println!("{:?}", cf)
                    } else if args.print_class {
                        println!("{}", cf)
                    };
                    cf
                }
                Err(err) => {
                    println!("error while reading class='{filename}', error={err}");
                    continue;
                }
            };
            classfile.write(&mut output_jar, filename, args.deflate_compress_level)?;
        } else if file.is_file()
            && (!args.strip_resources
                || filename.eq("MANIFEST.MF")
                || filename.ends_with("_at.cfg"))
        {
            match output_jar.raw_copy_file(file) {
                Ok(_) => {
                    continue;
                }
                Err(err) => {
                    println!("unable to write file {filename}, err={err}");
                    continue;
                }
            };
        }
    }
    output_jar.finish()?;
    Ok(())
}
