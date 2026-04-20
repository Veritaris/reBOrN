#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use classfile_parser;

use std::fs;

use std::io::{BufReader, Read};

use std::path::{Path};
use cafebabe::attributes::AttributeData;


fn get_field_name(class: &classy::ClassFile, field_name_index: u16) -> Option<String> {
    return match class.constant_pool[field_name_index as usize] {
        classy::Constant::Utf8(ref res) => Some(res.clone()),
        _ => None
    };
}

// fn main() -> eframe::Result<()> {
fn main() {
    let path = Path::new(".").join("jars");
    let file = fs::File::open(path.join("Thaumcraft-1.7.10-4.2.3.5.jar")).unwrap();
    let reader = BufReader::new(file);
    let mut zip = zip::ZipArchive::new(reader).unwrap();
    let mut buff = Vec::new();

    for i in 0..zip.len() {
        let mut file = zip.by_index(i).unwrap();
        if file.name().ends_with(".class") {
            println!("{}: ", file.name());
            let _res = file.read_to_end(&mut buff);

            let class = cafebabe::parse_class(&*buff).unwrap();
            for method in &class.methods {
                println!("\t{:?}", method.name);

                for attr in &method.attributes {
                    match attr.data {
                        AttributeData::Code(ref code) => {
                            for attr in &code.attributes {
                                match attr.data {
                                    AttributeData::LocalVariableTable(ref lvt) => {
                                        print!("\t\t");
                                        for lv in lvt {
                                            print!("{}, ", lv.name);
                                        }
                                        print!("\n");
                                    }
                                    _ => {}
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
            buff.clear();
        }
    }
}
