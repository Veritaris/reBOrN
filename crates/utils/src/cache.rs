use directories_next::ProjectDirs;
use linked_hash_map::LinkedHashMap;
use std::collections::HashMap;
use std::io::{BufReader, Error, ErrorKind, Write};
use std::path::{Path, PathBuf};
use std::process::exit;

use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

use crate::download::download_file;

const VERSIONS_URL: &str = "https://mcp.thiakil.com/data/versions.json"; // TODO(Veritaris) migrate to self-hosted

#[derive(Eq, PartialEq, Clone, Deserialize, Serialize)]
pub struct RebornCache {
    pub versions_json_hash: String,
}

impl RebornCache {
    pub fn load() -> Self {
        Self::read_from_file()
    }

    pub fn read_from_file() -> Self {
        let cache_file_path = get_cache_file_path();

        if !cache_file_path.exists() {
            let defaule_cache = Self {
                versions_json_hash: "".to_string(),
            };
            Self::save_to_disk(&defaule_cache);
            return defaule_cache;
        }

        let cache_content = std::fs::read_to_string(&cache_file_path).unwrap();

        ron::from_str::<RebornCache>(cache_content.as_str()).unwrap()
    }

    pub fn save_to_disk(&self) {
        let cache_file_path = get_cache_file_path();
        let prefix = match cache_file_path.parent() {
            None => {
                println!("error while looking for parent dir: it has empty name (disallowed) or root dir");
                exit(-1);
            }
            Some(it) => it,
        };
        if !prefix.exists() {
            println!("cache dir not exists, creating one");
            match std::fs::create_dir_all(prefix) {
                Ok(_) => {}
                Err(err) => {
                    println!("unable to create cache directory: {err}");
                    exit(-1);
                }
            }
        }
        let mut file = std::fs::File::create(cache_file_path).expect("unable to open cache file");
        let cache_string =
            ron::ser::to_string_pretty(&self, ron::ser::PrettyConfig::default()).unwrap();
        file.write(cache_string.as_bytes())
            .expect("unable to save default cache");
    }
}

pub fn get_appdata_dir() -> Box<ProjectDirs> {
    Box::new(ProjectDirs::from("me", "veritaris", "jarremapper").unwrap())
}

pub fn get_cache_file_path() -> PathBuf {
    get_appdata_dir().cache_dir().join("cachefile.ron")
}

pub fn mappings_exists(mc_version: &str, channel: &str, mappings_version: &str) -> bool {
    let project_dirs = get_appdata_dir();
    let store_dir = project_dirs.cache_dir();
    let tsrg_save_path = store_dir.join("mappings").join(mc_version);
    let save_path = store_dir
        .join("mappings")
        .join(mc_version)
        .join(channel)
        .join(mappings_version);
    save_path.join("methods.csv").exists()
        && save_path.join("params.csv").exists()
        && save_path.join("fields.csv").exists()
        && tsrg_save_path.join("joined.tsrg").exists()
}

pub fn ensure_versions_json(file_hash: Option<String>) -> Result<String, Error> {
    let cache_dir = get_appdata_dir();
    let cache_dir_path = cache_dir.cache_dir();

    if !cache_dir_path.exists() {
        std::fs::create_dir_all(cache_dir_path)
            .expect("unable to create cache directory, aborting");
    }

    let versions_json_link = VERSIONS_URL.to_string();
    let versions_json_path = cache_dir_path.join(Path::new("versions.json"));
    let mut current_file_hash = String::new();

    let hash_matches = if let Some(hash) = file_hash.clone() {
        match sha256::try_digest(&versions_json_path) {
            Ok(it) => {
                let cache_file_hash_matched = it == hash;
                current_file_hash = it;
                cache_file_hash_matched
            }
            Err(err) => {
                println!(
                    "Unable to get hash of {:?}, reason: {}",
                    &versions_json_path, err
                );
                false
            }
        }
    } else {
        false
    };

    if !(&versions_json_path.exists() & hash_matches) {
        print!("need re-download versions.json, reason: ");
        println!(
            "{}",
            if !hash_matches {
                format!(
                    "file hash does not match cached hash: '{} != {}'",
                    file_hash.unwrap_or("".to_string()),
                    current_file_hash
                )
            } else {
                "files does not exist".to_string()
            }
        );

        return if let Err(err) = download_file(versions_json_path.as_path(), &versions_json_link) {
            match err.status() {
                Some(StatusCode::NOT_FOUND) => {
                    println!("unable to find resource {versions_json_link}")
                }
                Some(code) => println!("unhandled error while downloading: {code}"),
                None => println!("None error happened while downloading"),
            }
            Err(Error::new(ErrorKind::Other, err.to_string()))
        } else {
            Ok(sha256::try_digest(&versions_json_path)?)
        };
    }

    Ok(current_file_hash)
}

pub fn read_versions_json() -> LinkedHashMap<String, HashMap<String, Vec<String>>> {
    let cache_dir = get_appdata_dir();
    let cache_dir_path = cache_dir.cache_dir();
    let versions_json_path = cache_dir_path.join(Path::new("versions.json"));
    println!("{:?}", versions_json_path);
    let file = std::fs::File::open(versions_json_path).unwrap();
    let reader = BufReader::new(file);
    println!("{:?}", cache_dir);
    println!("{:?}", cache_dir_path);
    let hm = serde_json::from_reader::<
        BufReader<std::fs::File>,
        HashMap<String, HashMap<String, Vec<String>>>,
    >(reader)
    .unwrap();

    let mut sorted: Vec<(String, HashMap<String, Vec<String>>)> = hm
        .into_iter()
        .map(|(k, v)| {
            let new_key = k
                .split(".")
                .map(|e| format!("{:0>2}", e))
                .collect::<Vec<String>>()
                .join(".");
            (new_key, v)
        })
        .collect();

    sorted.sort_by_key(|(k, _)| k.clone());

    sorted = sorted
        .into_iter()
        .map(|(k, v)| {
            let new_key = k
                .split(".")
                .map(|e| e.trim_start_matches("0").to_string())
                .collect::<Vec<String>>()
                .join(".");
            (new_key, v)
        })
        .collect();

    LinkedHashMap::from_iter(sorted)
}
