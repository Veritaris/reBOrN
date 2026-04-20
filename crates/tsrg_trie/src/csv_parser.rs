use std::collections::HashMap;
use utils::split::split_thrice_maybe;

#[derive(Debug, Clone)]
pub struct MappingEntry {
    pub srg_name: String,
    pub mcp_name: Option<String>,
    pub side: Option<CodeMemberSide>,
    pub comment: Option<String>,
}

#[derive(Debug, Clone, Copy)]
pub enum CodeMemberSide {
    Client,
    Server,
    Common,
    Unknown,
}

impl From<&str> for CodeMemberSide {
    fn from(value: &str) -> Self {
        match value {
            "0" => CodeMemberSide::Client,
            "1" => CodeMemberSide::Server,
            "2" => CodeMemberSide::Common,
            _ => CodeMemberSide::Unknown,
        }
    }
}

pub fn parse_mappings_csv<S: Into<String>>(content: S) -> HashMap<String, MappingEntry> {
    let mut csv_mapping: HashMap<String, MappingEntry> = HashMap::new();
    for line in content.into().lines() {
        let (srg, mcp, side, comment) = split_thrice_maybe(line, ",");
        let mapping_entry = MappingEntry {
            srg_name: srg.unwrap_or(line).to_string(),
            mcp_name: mcp.map(String::from),
            side: side.map(From::<&str>::from),
            comment: comment.map(String::from),
        };
        csv_mapping.insert(mapping_entry.srg_name.clone(), mapping_entry);
    }
    csv_mapping
}
