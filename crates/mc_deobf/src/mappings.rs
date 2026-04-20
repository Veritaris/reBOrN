use clap::ValueEnum;
use linked_hash_map::LinkedHashMap;
use serde::Deserialize;
use std::fmt::{Debug, Display, Formatter};
use std::fs;
use std::fs::read_to_string;
use std::path::{Path, PathBuf};

pub const MAPPINGS_DIR: &str = "resources/mappings/1.7.10/stable/12"; // TODO(Veritaris): remove after cache && mappings downloading implemented successfully

#[derive(PartialOrd, PartialEq, Debug, Copy, Clone, ValueEnum, serde::Deserialize, serde::Serialize)]
pub enum ModLoader {
    Forge,
    NeoForge,
    Fabric,
}

#[derive(PartialOrd, PartialEq, Debug, Copy, Clone, ValueEnum, serde::Deserialize, serde::Serialize)]
pub enum DeobfMappingsType {
    VersionsJSON,
    Custom,
}

impl Display for ModLoader {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ModLoader::Forge => f.write_str("Forge"),
            ModLoader::NeoForge => f.write_str("NeoForge"),
            ModLoader::Fabric => f.write_str("Fabric"),
        }
    }
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

#[derive(Default)]
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
                self.source_type, self.source, self.kind
            )
            .as_str(),
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
                    MappingsSourceType::Inline => "<passed inline>",
                },
                self.kind
            )
            .as_str(),
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

pub fn merge_mappings(extra_mappings: &Vec<String>, mappings: &mut Mappings) {
    for mapping_source in extra_mappings {
        match parse_mappings_source(mapping_source) {
            Ok(source) => {
                let table_to_populate = match source.kind {
                    MappingKind::Fields => &mut mappings.fields,
                    MappingKind::Methods => &mut mappings.methods,
                    MappingKind::Params => &mut mappings.params,
                };

                match source.source_type {
                    MappingsSourceType::LocalFile => {
                        let mappings_url = match url::Url::parse(&source.source) {
                            Ok(url) => url,
                            Err(err) => {
                                eprintln!(
                                    "Unable to parse mappings file location ({}): error: {}",
                                    source.source, err
                                );
                                continue;
                            }
                        };
                        if mappings_url.scheme() != "file" {
                            eprintln!(
                                "file:// scheme expected, {} provided ({})",
                                mappings_url.scheme(),
                                source.source
                            );
                            continue;
                        }
                        match read_to_string(mappings_url.path()) {
                            Ok(content) => {
                                let parsed_mappings = tsrg_trie::csv_parser::parse_mappings_csv(content);
                                for (k, v) in parsed_mappings {
                                    table_to_populate.insert(k, v.mcp_name.unwrap_or("".to_string()));
                                }
                            }
                            Err(err) => {
                                eprintln!("Unable to load mappings file {}, err={}", source.source, err);
                            }
                        };
                    }
                    MappingsSourceType::WebFile => {
                        todo!("Not yet implemented, sorry");
                    }
                    MappingsSourceType::Inline => {
                        let values: Vec<(&str, &str)> =
                            source.source.split(";").filter_map(|e| e.split_once("=")).collect();

                        for (k, v) in values {
                            table_to_populate.insert(String::from(k), String::from(v));
                        }
                    }
                }
                println!("    {:?}", source);
            }
            Err(err) => {
                println!("    {}", err);
            }
        }
    }
}

pub fn parse_mappings_source(mappings: &str) -> Result<MappingsSource, MappingSourceParseError> {
    let (m_kind, f_source) = match mappings.split_once(":") {
        None => {
            return Err(MappingSourceParseError);
        }
        Some(res) => res,
    };

    let kind = match m_kind {
        "fields" => MappingKind::Fields,
        "methods" => MappingKind::Methods,
        "params" => MappingKind::Params,
        _ => {
            return Err(MappingSourceParseError);
        }
    };

    let source_type = if f_source.starts_with("http://") || f_source.starts_with("https://") {
        MappingsSourceType::WebFile
    } else if f_source.starts_with("file://") || Path::new(f_source).exists() {
        MappingsSourceType::LocalFile
    } else {
        MappingsSourceType::Inline
    };

    Ok(MappingsSource {
        source_type,
        source: f_source.to_string(),
        kind,
    })
}

pub fn get_mappings_file_path<P: AsRef<Path>>(mappings_dir: P, m_type: &MappingKind) -> PathBuf {
    mappings_dir.as_ref().join(match m_type {
        MappingKind::Fields => "fields.csv",
        MappingKind::Methods => "methods.csv",
        MappingKind::Params => "params.csv",
    })
}

fn load_mappings_with_default<P: AsRef<Path>>(mappings_dir: P, kind: MappingKind) -> LinkedHashMap<String, String> {
    match load_mappings(&mappings_dir, &kind) {
        Ok(it) => it,
        Err(err) => {
            println!(
                "unable to load mappings from {:?}, reason: {}. {:?} won't be remapped",
                mappings_dir.as_ref(),
                err,
                &kind
            );
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

pub fn load_mappings<P: AsRef<Path>>(
    mappings_dir: &P,
    mapping_type: &MappingKind,
) -> Result<LinkedHashMap<String, String>, std::io::Error> {
    let m_path = get_mappings_file_path(mappings_dir, mapping_type);
    let file = fs::File::open(m_path)?;
    let buffered_reader = std::io::BufReader::new(file);
    let mut csv_reader = csv::Reader::from_reader(buffered_reader);

    let mut sorted: Vec<(String, String)> = match mapping_type {
        MappingKind::Fields | MappingKind::Methods => csv_reader
            .deserialize::<MappingRecord>()
            .map(|r| r.unwrap())
            .map(|r| (r.searge, r.name.clone()))
            .collect(),
        MappingKind::Params => csv_reader
            .deserialize::<ParamMappingRecord>()
            .map(|r| r.unwrap())
            .map(|r| (r.param, r.name.clone()))
            .collect(),
    };

    sorted.sort_by_key(|e| e.0.clone());
    let mappings = LinkedHashMap::from_iter(sorted);
    Ok(mappings)
}
