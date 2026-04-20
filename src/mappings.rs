use std::fmt::{Debug, Display, Formatter};
use std::fs;
use std::path::{Path, PathBuf};
use clap::ValueEnum;
use linked_hash_map::LinkedHashMap;
use serde::de::Error;
use serde::Deserialize;

pub const MAPPINGS_DIR: &str = "resources/mappings/1.7.10/stable/12"; // TODO(Veritaris): remove after cache && mappings downloading implemented successfully

#[derive(Debug, Copy, Clone, ValueEnum)]
pub enum ModLoader {
    FORGE,
    NEOFORGE,
    FABRIC,
}

#[allow(unused)]
#[derive(Deserialize)]
pub struct MappingRecord {
    searge: String,
    name: String,
    side: u8,
    desc: String,
}

#[allow(unused)]
#[derive(Deserialize)]
pub struct ParamMappingRecord {
    param: String,
    name: String,
    side: u8,
}

#[derive(Debug)]
pub enum MappingKind {
    Fields,
    Methods,
    Params,
}


#[derive(Debug)]
pub enum MappingsSourceType {
    LocalFile,
    WebFile,
    Inline,
}

pub struct MappingsSource {
    pub source_type: MappingsSourceType,
    pub source: String,
    pub kind: MappingKind,
}

pub struct Mappings {
    pub fields: LinkedHashMap<String, String>,
    pub methods: LinkedHashMap<String, String>,
    pub params: LinkedHashMap<String, String>,
}

impl Debug for MappingsSource {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            format!(
                "MappingsSource {{ source_type: {:?}, source: \"{:?}\", kind: {:?} }}",
                self.source_type,
                self.source,
                self.kind
            ).as_str()
        )
    }
}

impl Display for MappingsSource {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            format!(
                "MappingsSource {{ source_type: {:?}, source: \"{}\", kind: {:?} }}",
                self.source_type,
                match self.source_type {
                    MappingsSourceType::LocalFile | MappingsSourceType::WebFile => &self.source,
                    MappingsSourceType::Inline => "<passed inline>"
                },
                self.kind
            ).as_str()
        )
    }
}

#[derive(Debug, Clone)]
pub struct MappingSourceParseError;


impl Display for MappingSourceParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "error occurred while parsing mappings source")
    }
}

pub fn merge_mappings(
    extra_mappings: &Vec<String>,
    mappings: &mut Mappings,
) {
    for mapping_source in extra_mappings {
        match parse_mappings_source(&mapping_source) {
            Ok(source) => {
                match source.source_type {
                    MappingsSourceType::Inline => {
                        let values: Vec<(&str, &str)> = source.source.split(";")
                            .into_iter()
                            .filter_map(|e| {
                                match utils::utils::split_once(e, "=") {
                                    None => None,
                                    Some((k, v)) => Some((k, v))
                                }
                            })
                            .collect();
                        match source.kind {
                            MappingKind::Fields => {
                                for (k, v) in values {
                                    mappings.fields.insert(String::from(k), String::from(v));
                                };
                            }
                            MappingKind::Methods => {
                                for (k, v) in values {
                                    mappings.methods.insert(String::from(k), String::from(v));
                                };
                            }
                            MappingKind::Params => {
                                for (k, v) in values {
                                    mappings.params.insert(String::from(k), String::from(v));
                                };
                            }
                        };
                    }
                    _ => {}
                }
                println!("    {:?}", source);
            }
            Err(err) => {
                println!("    {}", err);
            }
        }
    }
}

pub fn parse_mappings_source(mappings: &String) -> Result<MappingsSource, MappingSourceParseError> {
    let (m_kind, f_source) = match utils::utils::split_once(mappings, ":") {
        None => { return Err(MappingSourceParseError); }
        Some(res) => { res }
    };

    let kind = match m_kind {
        "fields" => { MappingKind::Fields }
        "methods" => { MappingKind::Methods }
        "params" => { MappingKind::Params }
        _ => { return Err(MappingSourceParseError); }
    };

    let source_type = if f_source.starts_with("http://") || f_source.starts_with("https://") {
        MappingsSourceType::WebFile
    } else if f_source.starts_with("file://") || Path::new(f_source).exists() {
        MappingsSourceType::LocalFile
    } else {
        MappingsSourceType::Inline
    };

    return Ok(MappingsSource {
        source_type,
        source: f_source.to_string(),
        kind,
    });
}


pub fn get_mappings_file_path<P: AsRef<Path>>(mappings_dir: P, m_type: &MappingKind) -> PathBuf {
    return mappings_dir.as_ref().join(
        match m_type {
            MappingKind::Fields => "fields.csv",
            MappingKind::Methods => "methods.csv",
            MappingKind::Params => "params.csv"
        }
    );
}

fn load_mappings_with_default<P: AsRef<Path>>(mappings_dir: P, kind: MappingKind) -> LinkedHashMap<String, String> {
    match load_mappings(
        &mappings_dir,
        &kind,
    ) {
        Ok(it) => { it }
        Err(err) => {
            println!("unable to load mappings from {:?}, reason: {}. {:?} won't be remapped", mappings_dir.as_ref(), err, &kind);
            LinkedHashMap::new()
        }
    }
}

pub fn load_all_mappings<P: AsRef<Path>>(mappings_dir: P) -> Result<Mappings, std::io::Error> {
    let fields_mappings = load_mappings_with_default(&mappings_dir, MappingKind::Fields);
    let method_mappings = load_mappings_with_default(&mappings_dir, MappingKind::Methods);
    let params_mappings = load_mappings_with_default(&mappings_dir, MappingKind::Params);

    Ok(Mappings {
        fields: fields_mappings,
        methods: method_mappings,
        params: params_mappings,
    })
}

pub fn load_mappings<P: AsRef<Path>>(mappings_dir: &P, mapping_type: &MappingKind) -> Result<LinkedHashMap<String, String>, std::io::Error> {
    let m_path = get_mappings_file_path(mappings_dir, &mapping_type);
    let file = fs::File::open(m_path)?;
    let buffered_reader = std::io::BufReader::new(file);
    let mut csv_reader = csv::Reader::from_reader(buffered_reader);

    let mut sorted: Vec<(String, String)> =
        match mapping_type {
            MappingKind::Fields | MappingKind::Methods => {
                csv_reader.deserialize::<MappingRecord>().into_iter()
                    .map(|r| r.unwrap())
                    .map(|r| {
                        (r.searge, r.name.clone())
                    }
                    ).collect()
            }
            MappingKind::Params => {
                csv_reader.deserialize::<ParamMappingRecord>().into_iter()
                    .map(|r| r.unwrap())
                    .map(|r| {
                        (r.param, r.name.clone())
                    }
                    ).collect()
            }
        };

    sorted.sort_by_key(|e| e.0.clone());
    let mappings = LinkedHashMap::from_iter(sorted);
    return Ok(mappings);
}
