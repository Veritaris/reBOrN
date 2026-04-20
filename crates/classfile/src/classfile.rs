use std::fmt::{Debug, Display, Formatter};

use crate::access_flags::{AccessFlagContext, AccessFlags};
use crate::attributes::Attribute;
use crate::constant_pool_tag::ConstantPoolTags;
use crate::field::Field;
use crate::method::Method;
use crate::type_alias;
use indoc::indoc;

pub const CLASS_HEADER: u32 = 0xCAFEBABE;


// ClassFile {
//     type_alias::u4             magic;
//     type_alias::u2             minor_version;
//     type_alias::u2             major_version;
//     type_alias::u2             constant_pool_count;
//     cp_info        constant_pool[constant_pool_count-1];
//     type_alias::u2             access_flags;
//     type_alias::u2             this_class;
//     type_alias::u2             super_class;
//     type_alias::u2             interfaces_count;
//     type_alias::u2             interfaces[interfaces_count];
//     type_alias::u2             fields_count;
//     field_info     fields[fields_count];
//     type_alias::u2             methods_count;
//     method_info    methods[methods_count];
//     type_alias::u2             attributes_count;
//     attribute_info attributes[attributes_count];
// }
pub struct ClassFile {
    pub magic: type_alias::u4,
    pub minor_version: type_alias::u2,
    pub major_version: type_alias::u2,
    pub constant_pool_count: type_alias::u2,
    pub constant_pool: Vec<ConstantPoolTags>,
    pub access_flags: AccessFlags,
    pub this_class: type_alias::u2,
    pub super_class: type_alias::u2,
    pub interfaces_count: type_alias::u2,
    pub interfaces: Vec<type_alias::u2>,
    pub fields_count: type_alias::u2,
    pub fields: Vec<Field>,
    pub methods_count: type_alias::u2,
    pub methods: Vec<Method>,
    pub attributes_count: type_alias::u2,
    pub attributes: Vec<Attribute>,

    pub _len: u64,
}

impl ClassFile {
    pub fn get_string_from_cpool(&self, index: u16) -> String {
        match self.constant_pool.get(index as usize).unwrap() {
            ConstantPoolTags::Utf8 { _value, .. } => _value.clone(),
            ConstantPoolTags::String { string_index, .. } => self.get_string_from_cpool(*string_index),
            _ => String::new()
        }
    }
}

impl Debug for ClassFile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let constant_pool: String = self.constant_pool
            .iter()
            .enumerate()
            .skip(1)
            .map(|(index, entry)| {
                format!("{} {}", index, self.tag_to_display(entry))
            })
            .fold(
                String::from("        "),
                |acc, e| {
                    acc + "\n        " + e.as_str()
                },
            );

        let interfaces = self.interfaces
            .iter()
            .map(|class_index| {
                match self.constant_pool.get(*class_index as usize) {
                    Some(class_info) => match class_info {
                        ConstantPoolTags::Class { name_index, .. } => {
                            self.get_string_from_cpool(*name_index)
                        }
                        _ => panic!("expected Class_info found at index {} in constant pool: got {}", class_index, class_info)
                    },
                    None => panic!("nothing found at index {} in constant pool", class_index)
                }
            })
            .fold(
                String::from("        "),
                |acc, e| {
                    acc + "\n        " + e.as_str()
                },
            );

        let fields = self.fields
            .iter()
            .map(|f| {
                format!("{} {}",
                        AccessFlags::from((AccessFlagContext::Field, f.access_flags)).as_string(),
                        self.get_string_from_cpool(f.name_index)
                )
            })
            .fold(
                String::from("        "),
                |acc, f| {
                    acc + "\n        " + f.as_str()
                },
            );
        let methods = self.methods
            .iter()
            .map(|m| {
                format!("{} {} {}",
                        AccessFlags::from((AccessFlagContext::Field, m.access_flags)).as_string(),
                        self.get_string_from_cpool(m.name_index),
                        self.get_string_from_cpool(m.descriptor_index),
                )
            })
            .fold(
                String::from("        "),
                |acc, f| {
                    acc + "\n        " + f.as_str()
                },
            );
        let attributes = self.attributes
            .iter()
            .map(|attr| {
                format!("{:?}", attr)
            })
            .fold(
                String::from("        "),
                |acc, f| {
                    acc + "\n        " + f.as_str()
                },
            );

        f.write_str(
            format!(
                indoc!("
                Classfile version {}.{} of class {}
                    constant_pool_count: {}
                    constant_pool: {}
                    access_flags: {:?}
                    this_class: {}
                    super_class: {}
                    interfaces_count: {}{}
                    fields_count: {}{}
                    methods_count: {}{}
                    attributes_count: {}{}
                    "),
                self.major_version,
                self.minor_version,
                self.class_name_from_cp(),
                self.constant_pool_count,
                constant_pool,
                self.access_flags,
                self.this_class,
                self.super_class,
                self.interfaces_count,
                match self.interfaces_count {
                    0 => String::from("<empty>"),
                    _ => interfaces,
                },
                self.fields_count,
                match self.fields_count {
                    0 => String::from("<empty>"),
                    _ => fields,
                },
                self.methods_count,
                match self.methods_count {
                    0 => String::from("<empty>"),
                    _ => methods,
                },
                self.attributes_count,
                match self.attributes_count {
                    0 => String::from("<empty>"),
                    _ => attributes,
                },
            ).as_str()
        )
    }
}

impl Display for ClassFile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let class_name = match self.constant_pool.get(self.this_class as usize) {
            Some(ConstantPoolTags::Class { name_index, .. }) => {
                match self.constant_pool.get(*name_index as usize) {
                    Some(ConstantPoolTags::Utf8 { bytes, .. }) => String::from_utf8(bytes.clone()).unwrap(),
                    _ => panic!("pizdec")
                }
            }
            _ => panic!("pizdec x2")
        };

        let interfaces = self.interfaces
            .iter()
            .map(|class_index| {
                match self.constant_pool.get(*class_index as usize) {
                    Some(class_info) => match class_info {
                        ConstantPoolTags::Class { name_index, .. } => {
                            self.get_string_from_cpool(*name_index)
                        }
                        _ => panic!("expected Class_info found at index {} in constant pool: got {}", class_index, class_info)
                    },
                    None => panic!("nothing found at index {} in constant pool", class_index)
                }
            })
            .fold(
                String::from("        "),
                |acc, e| {
                    acc + "\n        " + e.as_str()
                },
            );

        let fields = self.fields
            .iter()
            .map(|f| {
                format!("{} {}",
                        AccessFlags::from((AccessFlagContext::Field, f.access_flags)).as_string(),
                        self.get_string_from_cpool(f.name_index)
                )
            })
            .fold(
                String::from("        "),
                |acc, f| {
                    acc + "\n        " + f.as_str()
                },
            );
        let methods = self.methods
            .iter()
            .map(|m| {
                format!("{} {} {}",
                        AccessFlags::from((AccessFlagContext::Field, m.access_flags)).as_string(),
                        self.get_string_from_cpool(m.name_index),
                        self.get_string_from_cpool(m.descriptor_index),
                )
            })
            .fold(
                String::from("        "),
                |acc, f| {
                    acc + "\n        " + f.as_str()
                },
            );
        let attributes = self.attributes
            .iter()
            .map(|attr| {
                format!("{:?}", attr)
            })
            .fold(
                String::from("        "),
                |acc, f| {
                    acc + "\n        " + f.as_str()
                },
            );

        f.write_str(
            format!(
                indoc!("
                Classfile version {}.{} of class {}
                    constant_pool_count: {}
                    access_flags: {}
                    this_class: {}
                    super_class: {}
                    interfaces_count: {}{}
                    fields_count: {}{}
                    methods_count: {}{}
                    attributes_count: {}{}
                    "),
                self.major_version,
                self.minor_version,
                self.class_name_from_cp(),
                self.constant_pool_count,
                self.access_flags.as_string(),
                class_name,
                match self.constant_pool.get(self.super_class as usize) {
                    Some(ConstantPoolTags::Class { name_index, .. }) => {
                        match self.constant_pool.get(*name_index as usize) {
                            Some(ConstantPoolTags::Utf8 { bytes, .. }) => String::from_utf8(bytes.clone()).unwrap(),
                            _ => panic!("pizdec")
                        }
                    }
                    _ => panic!("pizdec x2")
                },
                self.interfaces_count,
                match self.interfaces_count {
                    0 => String::from("<empty>"),
                    _ => interfaces,
                },
                self.fields_count,
                match self.fields_count {
                    0 => String::from("<empty>"),
                    _ => fields,
                },
                self.methods_count,
                match self.methods_count {
                    0 => String::from("<empty>"),
                    _ => methods,
                },
                self.attributes_count,
                match self.attributes_count {
                    0 => String::from("<empty>"),
                    _ => attributes,
                },
            ).as_str()
        )
    }
}



