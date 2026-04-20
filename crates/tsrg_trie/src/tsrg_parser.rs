use std::fs::read_to_string;
use std::path::Path;

// parse tsrg -> apply mappings to parsed trie

#[derive(Clone, Default, Debug)]
struct TsrgTrie<'a> {
    children: Vec<TsrgTrie<'a>>,
    key: &'a str,
}

impl<'a> TsrgTrie<'a> {
    #[allow(unused)]
    pub(crate) fn add_child(&mut self, child: TsrgTrie<'a>) {
        self.children.push(child);
    }

    pub(crate) fn ensure_child(&mut self, child_key: &'a str) {
        // self.children.push(child);
        if let Some(_) = self.children.iter().find(|st| st.key == child_key) {
        } else {
            self.create_child(child_key);
        }
    }

    pub(crate) fn create_child(&mut self, child_key: &'a str) {
        let node = TsrgTrie {
            children: Vec::new(),
            key: child_key,
        };
        self.children.push(node);
    }
}

pub fn read_tsrg<P: AsRef<Path>>(p: P) {
    let mut mapping_trie = TsrgTrie::default();

    let lines_iter = read_to_string(p).unwrap();
    for line in lines_iter.lines() {
        if !line.starts_with("\t") && line.contains("/") {
            let mut split = line.splitn(2, " ");
            let (_, deobfed_class_path) = (split.next().unwrap(), split.next().unwrap());
            let mut path_split = deobfed_class_path.split("/");

            while let Some(part) = path_split.next() {
                TsrgTrie::ensure_child(&mut mapping_trie, part);
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
