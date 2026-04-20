use std::io::{BufWriter, Write};
use std::ops::Deref;
use std::path::Path;
use std::thread::JoinHandle;

const SOURCE_DATA_DEFAULT_URL: &str = "https://mcp.thiakil.com/data";
const MAPPINGS_KINDS: [&str; 3] = ["fields.csv", "methods.csv", "params.csv"];

pub fn download_file<P: AsRef<Path>>(save_path: P, url: &String) -> Result<(), reqwest::Error> {
    println!("downloading {url}");
    let response = reqwest::blocking::get(url)?;

    match response.error_for_status_ref() {
        Err(e) => Err(e),
        Ok(..) => {
            let data_file = std::fs::File::create(save_path).unwrap();

            let mut file_writer = BufWriter::new(data_file);
            let bytes = response.bytes()?;
            let _ = file_writer.write(bytes.deref()).unwrap();
            Ok(())
        }
    }
}

pub fn download_tsrg<P: AsRef<Path>>(store_dir: P, mc_version: &str) -> JoinHandle<()> {
    let file_url = format!("{SOURCE_DATA_DEFAULT_URL}/{mc_version}/joined.tsrg");

    let save_path = store_dir.as_ref().join("mappings").join(mc_version).join("joined.tsrg");
    std::fs::create_dir_all(save_path.parent().unwrap()).unwrap();
    std::thread::spawn(move || {
        match download_file(save_path.as_path(), &file_url) {
            Ok(_) => {
                println!("joined.tsrg downloaded");
            }
            Err(err) => {
                println!("error while downloading joined.tsrg: {err}");
            }
        };
    })
}

pub fn download_mappings<P: AsRef<Path>, F: FnOnce()>(
    store_dir: P,
    mc_version: &str,
    channel: &str,
    mappings_version: &str,
    on_finish: Option<F>,
) {
    let mappings_url_prepath = format!("{}/{}/{}", mc_version, channel, mappings_version);
    let mappings_url = format!("{SOURCE_DATA_DEFAULT_URL}/{mappings_url_prepath}");
    let mut v = Vec::<std::thread::JoinHandle<()>>::new();

    for mk in MAPPINGS_KINDS {
        let file_url = format!("{}/{}", mappings_url.clone(), mk);
        let save_path = store_dir
            .as_ref()
            .join("mappings")
            .join(mc_version)
            .join(channel)
            .join(mappings_version)
            .join(mk);
        std::fs::create_dir_all(save_path.parent().unwrap()).unwrap();

        v.push(std::thread::spawn(move || {
            match download_file(save_path.as_path(), &file_url) {
                Ok(_) => {
                    println!("{mk} downloaded");
                }
                Err(err) => {
                    println!("error while downloading {mk}: {err}");
                }
            };
        }));
    }
    v.push(download_tsrg(&store_dir, mc_version));
    for join_handle in v {
        join_handle.join();
    }
    if let Some(f) = on_finish {
        f()
    }
}
