use std::collections::HashMap;
use std::io::BufRead;
use std::io::Error;
use std::path::Path;
use tsrg_trie::csv_parser::parse_mappings_csv;
use tsrg_trie::tsrg_parser::{Class, ClassMember, TsrgTrie, parse_class_line, parse_class_member_line};

fn main() -> Result<(), Error> {
    let fields_mapping = {
        let mappings_file_path = Path::new("./resources/mappings/1.7.10/channels/stable/12/fields.csv");
        let file_content = std::fs::read_to_string(mappings_file_path)?;
        parse_mappings_csv(&file_content)
    };
    let methods_mapping = {
        let mappings_file_path = Path::new("./resources/mappings/1.7.10/channels/stable/12/methods.csv");
        let file_content = std::fs::read_to_string(mappings_file_path)?;
        parse_mappings_csv(&file_content)
    };
    let params_mapping = {
        let mappings_file_path = Path::new("./resources/mappings/1.7.10/channels/stable/12/params.csv");
        let file_content = std::fs::read_to_string(mappings_file_path)?;
        parse_mappings_csv(&file_content)
    };

    let tsrg_file_path = Path::new("./resources/mappings/1.7.10/joined.tsrg");
    let file = std::fs::read_to_string(tsrg_file_path)?;
    let mut tsrg_classes: HashMap<&str, Class> = HashMap::new();
    let mut last_visited_class_name: Option<&str> = None;
    let mut tsrg_trie = TsrgTrie::new("", false, None);

    for line in file.lines() {
        if line.starts_with(char::is_whitespace) {
            if let Some(class_member) = parse_class_member_line(
                line,
                Option::from(&fields_mapping),
                Option::from(&methods_mapping),
                Option::from(&params_mapping),
            ) {
                // println!("found class member {:?}", class_member);
                if let Some(last_visited_class_name) = last_visited_class_name {
                    match class_member {
                        ClassMember::Method {
                            notch_name, srg_name, ..
                        } => {
                            let class = tsrg_classes.get_mut(last_visited_class_name).unwrap();

                            class.methods.push(class_member);
                            class.methods_notch.insert(notch_name, class_member);
                            class.methods_srg.insert(srg_name, class_member);
                        }
                        ClassMember::Field {
                            notch_name, srg_name, ..
                        } => {
                            let class = tsrg_classes.get_mut(last_visited_class_name).unwrap();
                            class.fields.push(class_member);
                            class.fields_notch.insert(notch_name, class_member);
                            class.fields_srg.insert(srg_name, class_member);
                        }
                    }
                }
            }
        } else if let Some(class) = parse_class_line(line) {
            println!("found class {:?}", class);
            last_visited_class_name = Some(class.notch_class);
            tsrg_classes.insert(class.notch_class, class);
        }
    }

    Ok(())
}
