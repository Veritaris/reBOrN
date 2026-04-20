use crate::csv_parser::MappingEntry;
use std::collections::HashMap;
use std::fs::read_to_string;
use std::path::Path;
// parse tsrg -> apply mappings to parsed trie

#[derive(Debug, Clone)]
pub struct Class<'a> {
    pub super_class: Option<&'a str>,
    pub notch_class: &'a str,
    pub mcp_class: &'a str,
    pub fields: Vec<ClassMember<'a>>,
    pub methods: Vec<ClassMember<'a>>,
    // maps for different names for faster lookup
    pub fields_notch: HashMap<&'a str, ClassMember<'a>>,
    pub fields_srg: HashMap<&'a str, ClassMember<'a>>,
    pub fields_mcp: HashMap<&'a str, ClassMember<'a>>,
    pub methods_notch: HashMap<&'a str, ClassMember<'a>>,
    pub methods_srg: HashMap<&'a str, ClassMember<'a>>,
    pub methods_mcp: HashMap<&'a str, ClassMember<'a>>,
}

#[derive(Debug, Copy, Clone)]
pub enum ClassMember<'a> {
    Field {
        notch_name: &'a str,
        srg_name: &'a str,
        mcp_name: Option<&'a str>,
    },
    Method {
        notch_name: &'a str,
        signature: &'a str,
        srg_name: &'a str,
        mcp_name: Option<&'a str>,
    },
}

pub fn parse_class_line(line: &'_ str) -> Option<Class<'_>> {
    match utils::split::split_once(line, " ") {
        Some((notch_name, mcp_name)) => Some(Class {
            super_class: None,
            notch_class: notch_name,
            mcp_class: mcp_name,
            fields: vec![],
            methods: vec![],
            fields_notch: Default::default(),
            fields_srg: Default::default(),
            fields_mcp: Default::default(),
            methods_notch: Default::default(),
            methods_srg: Default::default(),
            methods_mcp: Default::default(),
        }),
        None => {
            println!("cannot split class line {line}");
            None
        }
    }
}

pub fn parse_class_member_line<'a>(
    line: &'a str,
    _fields_mapping: Option<&HashMap<String, MappingEntry>>,
    _methods_mapping: Option<&HashMap<String, MappingEntry>>,
    _params_mapping: Option<&HashMap<String, MappingEntry>>,
) -> Option<ClassMember<'a>> {
    let line = line.trim_matches(char::is_whitespace);
    match line.chars().filter(|it| it.is_whitespace()).count() {
        1 => match utils::split::split_once(line, " ") {
            Some((notch_name, srg_name)) => Some(ClassMember::Field {
                notch_name,
                srg_name,
                mcp_name: None,
            }),
            None => {
                println!("cannot split class field line {line}");
                None
            }
        },
        2 => match utils::split::split_twice(line, " ") {
            Some((notch_name, signature, srg_name)) => Some(ClassMember::Method {
                notch_name,
                signature,
                srg_name,
                mcp_name: None,
            }),
            None => {
                println!("cannot split class method line {line}");
                None
            }
        },
        other => {
            println!("unable to determine class member: {other} whitespaces met in line '{line}'");
            None
        }
    }
}

#[derive(Clone, Default, Debug)]
pub struct TsrgTrie<'a> {
    pub children: Vec<TsrgTrie<'a>>,
    pub parent: Option<&'a TsrgTrie<'a>>,
    pub is_leaf: bool,
    pub key: &'a str,
}

impl<'a> TsrgTrie<'a> {
    pub fn new(key: &'a str, is_leaf: bool, parent: Option<&'a TsrgTrie>) -> Self {
        TsrgTrie {
            key,
            is_leaf,
            children: vec![],
            parent,
        }
    }

    #[allow(unused)]
    pub(crate) fn add_child(&'a mut self, child: TsrgTrie<'a>) {
        self.children.push(child);
    }

    pub(crate) fn ensure_child(&'a mut self, child_key: &'a str) {
        // self.children.push(child);
        if let Some(_) = self.children.iter().find(|st| st.key == child_key) {
        } else {
            self.create_child(child_key, false);
        }
    }

    pub(crate) fn create_child(&mut self, child_key: &'a str, is_leaf: bool) -> &'a mut TsrgTrie {
        let child = TsrgTrie::new(child_key, is_leaf, self.parent);
        self.children.push(child);
        self.children.last_mut().unwrap()
    }
}

pub fn read_tsrg<P: AsRef<Path>>(p: P) {
    let mut mapping_trie = TsrgTrie::new("", false, None);

    let lines_iter = read_to_string(p).unwrap();
    for line in lines_iter.lines() {
        if !line.starts_with("\t") && line.contains("/") {
            let mut split = line.splitn(2, " ");
            let (_, deobfed_class_path) = (split.next().unwrap(), split.next().unwrap());
            let mut path_split = deobfed_class_path.split("/");

            while let Some(part) = path_split.next() {
                // TsrgTrie::ensure_child(&mut mapping_trie, part);
                // mapping_trie.create_child(part, false);
                // if let Some(subtrie) = mapping_trie.children.iter().find(|st| { st.key == part }) {
                //
                // } else {
                //     mapping_trie.create_child(part);
                // }
            }
        }
    }
    // println!("{:?}", mapping_trie);
}
