use std::fmt::{Debug, Display, Formatter};
use std::io::{Error, Write};
use byteorder::{BigEndian, WriteBytesExt};

use indoc::indoc;
use crate::access_flags::{AccessFlagContext, AccessFlags};
use crate::attributes::Attribute;
// use crate::classfile::attribu

pub const CLASS_HEADER: u32 = 0xCAFEBABE;
pub const CONTINUATION_TAG: ConstantPoolTags = ConstantPoolTags::ContinuationTag { tag: 0 };

#[allow(non_camel_case_types)]
pub type u1 = u8;
#[allow(non_camel_case_types)]
pub type u2 = u16;
#[allow(non_camel_case_types)]
pub type u4 = u32;

// Utf8	                    1	45.3	1.0.2
// Integer	                3	45.3	1.0.2
// Float	                4	45.3	1.0.2
// Long	                    5	45.3	1.0.2
// Double	                6	45.3	1.0.2
// Class	                7	45.3	1.0.2
// String	                8	45.3	1.0.2
// Fieldref	                9	45.3	1.0.2
// Methodref	            10	45.3	1.0.2
// InterfaceMethodref	    11	45.3	1.0.2
// NameAndType	            12	45.3	1.0.2
// MethodHandle	            15	51.0	7
// MethodType	            16	51.0	7
// Dynamic	                17	55.0	11
// InvokeDynamic	        18	51.0	7
// Module	                19	53.0	9
// Package	                20	53.0	9
#[derive(Debug, Clone)]
pub enum ConstantPoolTags {
    ContinuationTag { tag: u1 },
    Utf8 { tag: u1, length: u2, bytes: Vec<u1>, _value: String },
    Integer { tag: u1, bytes: u4, _value: i32 },
    Float { tag: u1, bytes: u4, _value: f32 },
    Long { tag: u1, high_bytes: u4, low_bytes: u4, _value: i64 },
    Double { tag: u1, high_bytes: u4, low_bytes: u4, _value: f64 },
    Class { tag: u1, name_index: u2 },
    String { tag: u1, string_index: u2 },
    Fieldref { tag: u1, class_index: u2, name_and_type_index: u2 },
    Methodref { tag: u1, class_index: u2, name_and_type_index: u2 },
    InterfaceMethodref { tag: u1, class_index: u2, name_and_type_index: u2 },
    NameAndType { tag: u1, name_index: u2, descriptor_index: u2 },
    MethodHandle { tag: u1, reference_kind: u1, reference_index: u2 },
    MethodType { tag: u1, descriptor_index: u2 },
    Dynamic { tag: u1, bootstrap_method_attr_index: u2, name_and_type_index: u2 },
    InvokeDynamic { tag: u1, bootstrap_method_attr_index: u2, name_and_type_index: u2 },
    Module { tag: u1, name_index: u2 },
    Package { tag: u1, name_index: u2 },
}

impl ConstantPoolTags {
    pub fn jvm_tag(&self) -> u8 {
        return match self {
            ConstantPoolTags::ContinuationTag { .. } => 0,
            ConstantPoolTags::Utf8 { .. } => 1,
            ConstantPoolTags::Integer { .. } => 3,
            ConstantPoolTags::Float { .. } => 4,
            ConstantPoolTags::Long { .. } => 5,
            ConstantPoolTags::Double { .. } => 6,
            ConstantPoolTags::Class { .. } => 7,
            ConstantPoolTags::String { .. } => 8,
            ConstantPoolTags::Fieldref { .. } => 9,
            ConstantPoolTags::Methodref { .. } => 10,
            ConstantPoolTags::InterfaceMethodref { .. } => 11,
            ConstantPoolTags::NameAndType { .. } => 12,
            ConstantPoolTags::MethodHandle { .. } => 15,
            ConstantPoolTags::MethodType { .. } => 16,
            ConstantPoolTags::Dynamic { .. } => 17,
            ConstantPoolTags::InvokeDynamic { .. } => 18,
            ConstantPoolTags::Module { .. } => 19,
            ConstantPoolTags::Package { .. } => 20,
        };
    }
    #[allow(unused)]
    pub fn write(self, buff: &mut Vec<u8>) -> Result<(), Error> {
        // skip writing anything on ContinuationTag met
        match self {
            ConstantPoolTags::ContinuationTag { .. } => return Ok(()),
            _ => buff.write_u8(self.jvm_tag())?,
        };

        match self {
            ConstantPoolTags::ContinuationTag { .. } => {}
            ConstantPoolTags::Utf8 { length, bytes, .. } => {
                buff.write_u16::<BigEndian>(length)?;
                buff.write(bytes.as_slice())?;
            }
            ConstantPoolTags::Integer { bytes, .. } => { buff.write_u32::<BigEndian>(bytes)? }
            ConstantPoolTags::Float { bytes, .. } => { buff.write_u32::<BigEndian>(bytes)? }
            ConstantPoolTags::Long { high_bytes, low_bytes, .. } => {
                buff.write_u32::<BigEndian>(high_bytes)?;
                buff.write_u32::<BigEndian>(low_bytes)?;
            }
            ConstantPoolTags::Double { high_bytes, low_bytes, .. } => {
                buff.write_u32::<BigEndian>(high_bytes)?;
                buff.write_u32::<BigEndian>(low_bytes)?;
            }
            ConstantPoolTags::Class { name_index, .. } => { buff.write_u16::<BigEndian>(name_index)? }
            ConstantPoolTags::String { string_index, .. } => { buff.write_u16::<BigEndian>(string_index)? }
            ConstantPoolTags::Fieldref { class_index, name_and_type_index, .. } => {
                buff.write_u16::<BigEndian>(class_index)?;
                buff.write_u16::<BigEndian>(name_and_type_index)?;
            }
            ConstantPoolTags::Methodref { class_index, name_and_type_index, .. } => {
                buff.write_u16::<BigEndian>(class_index)?;
                buff.write_u16::<BigEndian>(name_and_type_index)?;
            }
            ConstantPoolTags::InterfaceMethodref { class_index, name_and_type_index, .. } => {
                buff.write_u16::<BigEndian>(class_index)?;
                buff.write_u16::<BigEndian>(name_and_type_index)?;
            }
            ConstantPoolTags::NameAndType { name_index, descriptor_index, .. } => {
                buff.write_u16::<BigEndian>(name_index)?;
                buff.write_u16::<BigEndian>(descriptor_index)?;
            }
            ConstantPoolTags::MethodHandle { reference_kind, reference_index, .. } => {
                buff.write_u8(reference_kind)?;
                buff.write_u16::<BigEndian>(reference_index)?;
            }
            ConstantPoolTags::MethodType { descriptor_index, .. } => {
                buff.write_u16::<BigEndian>(descriptor_index)?;
            }
            ConstantPoolTags::Dynamic { bootstrap_method_attr_index, name_and_type_index, .. } => {
                buff.write_u16::<BigEndian>(bootstrap_method_attr_index)?;
                buff.write_u16::<BigEndian>(name_and_type_index)?;
            }
            ConstantPoolTags::InvokeDynamic { bootstrap_method_attr_index, name_and_type_index, .. } => {
                buff.write_u16::<BigEndian>(bootstrap_method_attr_index)?;
                buff.write_u16::<BigEndian>(name_and_type_index)?;
            }
            ConstantPoolTags::Module { name_index, .. } => { buff.write_u16::<BigEndian>(name_index)? }
            ConstantPoolTags::Package { name_index, .. } => { buff.write_u16::<BigEndian>(name_index)? }
        };
        Ok(())
    }
}

impl TryInto<Vec<u8>> for ConstantPoolTags {
    type Error = Error;

    fn try_into(self) -> Result<Vec<u8>, Self::Error> {
        let mut output_bytes = Vec::new();

        // skip writing anything on ContinuationTag met
        match self {
            ConstantPoolTags::ContinuationTag { .. } => return Ok(output_bytes),
            _ => output_bytes.write_u8(self.jvm_tag())?,
        };

        match self {
            ConstantPoolTags::ContinuationTag { .. } => {}
            ConstantPoolTags::Utf8 { length, bytes, .. } => {
                output_bytes.write_u16::<BigEndian>(length)?;
                output_bytes.write(bytes.as_slice())?;
            }
            ConstantPoolTags::Integer { bytes, .. } => { output_bytes.write_u32::<BigEndian>(bytes)? }
            ConstantPoolTags::Float { bytes, .. } => { output_bytes.write_u32::<BigEndian>(bytes)? }
            ConstantPoolTags::Long { high_bytes, low_bytes, .. } => {
                output_bytes.write_u32::<BigEndian>(high_bytes)?;
                output_bytes.write_u32::<BigEndian>(low_bytes)?;
            }
            ConstantPoolTags::Double { high_bytes, low_bytes, .. } => {
                output_bytes.write_u32::<BigEndian>(high_bytes)?;
                output_bytes.write_u32::<BigEndian>(low_bytes)?;
            }
            ConstantPoolTags::Class { name_index, .. } => { output_bytes.write_u16::<BigEndian>(name_index)? }
            ConstantPoolTags::String { string_index, .. } => { output_bytes.write_u16::<BigEndian>(string_index)? }
            ConstantPoolTags::Fieldref { class_index, name_and_type_index, .. } => {
                output_bytes.write_u16::<BigEndian>(class_index)?;
                output_bytes.write_u16::<BigEndian>(name_and_type_index)?;
            }
            ConstantPoolTags::Methodref { class_index, name_and_type_index, .. } => {
                output_bytes.write_u16::<BigEndian>(class_index)?;
                output_bytes.write_u16::<BigEndian>(name_and_type_index)?;
            }
            ConstantPoolTags::InterfaceMethodref { class_index, name_and_type_index, .. } => {
                output_bytes.write_u16::<BigEndian>(class_index)?;
                output_bytes.write_u16::<BigEndian>(name_and_type_index)?;
            }
            ConstantPoolTags::NameAndType { name_index, descriptor_index, .. } => {
                output_bytes.write_u16::<BigEndian>(name_index)?;
                output_bytes.write_u16::<BigEndian>(descriptor_index)?;
            }
            ConstantPoolTags::MethodHandle { reference_kind, reference_index, .. } => {
                output_bytes.write_u8(reference_kind)?;
                output_bytes.write_u16::<BigEndian>(reference_index)?;
            }
            ConstantPoolTags::MethodType { descriptor_index, .. } => {
                output_bytes.write_u16::<BigEndian>(descriptor_index)?;
            }
            ConstantPoolTags::Dynamic { bootstrap_method_attr_index, name_and_type_index, .. } => {
                output_bytes.write_u16::<BigEndian>(bootstrap_method_attr_index)?;
                output_bytes.write_u16::<BigEndian>(name_and_type_index)?;
            }
            ConstantPoolTags::InvokeDynamic { bootstrap_method_attr_index, name_and_type_index, .. } => {
                output_bytes.write_u16::<BigEndian>(bootstrap_method_attr_index)?;
                output_bytes.write_u16::<BigEndian>(name_and_type_index)?;
            }
            ConstantPoolTags::Module { name_index, .. } => { output_bytes.write_u16::<BigEndian>(name_index)? }
            ConstantPoolTags::Package { name_index, .. } => { output_bytes.write_u16::<BigEndian>(name_index)? }
        };
        Ok(output_bytes)
    }
}

impl Display for ConstantPoolTags {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.jvm_tag().to_string().as_str())
    }
}

#[derive(Debug)]
pub enum ConstantPoolJvmTag {
    INVALID = 0,
    Utf8 = 1,
    Integer = 3,
    Float = 4,
    Long = 5,
    Double = 6,
    Class = 7,
    String = 8,
    Fieldref = 9,
    Methodref = 10,
    InterfaceMethodref = 11,
    NameAndType = 12,
    MethodHandle = 15,
    MethodType = 16,
    Dynamic = 17,
    InvokeDynamic = 18,
    Module = 19,
    Package = 20,
}

impl Into<u8> for ConstantPoolJvmTag {
    fn into(self) -> u8 {
        self as u8
    }
}

impl From<u8> for ConstantPoolJvmTag {
    fn from(value: u8) -> ConstantPoolJvmTag {
        return match value {
            1 => ConstantPoolJvmTag::Utf8,
            3 => ConstantPoolJvmTag::Integer,
            4 => ConstantPoolJvmTag::Float,
            5 => ConstantPoolJvmTag::Long,
            6 => ConstantPoolJvmTag::Double,
            7 => ConstantPoolJvmTag::Class,
            8 => ConstantPoolJvmTag::String,
            9 => ConstantPoolJvmTag::Fieldref,
            10 => ConstantPoolJvmTag::Methodref,
            11 => ConstantPoolJvmTag::InterfaceMethodref,
            12 => ConstantPoolJvmTag::NameAndType,
            15 => ConstantPoolJvmTag::MethodHandle,
            16 => ConstantPoolJvmTag::MethodType,
            17 => ConstantPoolJvmTag::Dynamic,
            18 => ConstantPoolJvmTag::InvokeDynamic,
            19 => ConstantPoolJvmTag::Module,
            20 => ConstantPoolJvmTag::Package,
            _ => ConstantPoolJvmTag::INVALID,
        };
    }
}

// ClassFile {
//     u4             magic;
//     u2             minor_version;
//     u2             major_version;
//     u2             constant_pool_count;
//     cp_info        constant_pool[constant_pool_count-1];
//     u2             access_flags;
//     u2             this_class;
//     u2             super_class;
//     u2             interfaces_count;
//     u2             interfaces[interfaces_count];
//     u2             fields_count;
//     field_info     fields[fields_count];
//     u2             methods_count;
//     method_info    methods[methods_count];
//     u2             attributes_count;
//     attribute_info attributes[attributes_count];
// }
pub struct ClassFile {
    pub magic: u4,
    pub minor_version: u2,
    pub major_version: u2,
    pub constant_pool_count: u2,
    pub constant_pool: Vec<ConstantPoolTags>,
    pub access_flags: AccessFlags,
    pub this_class: u2,
    pub super_class: u2,
    pub interfaces_count: u2,
    pub interfaces: Vec<u2>,
    pub fields_count: u2,
    pub fields: Vec<Field>,
    pub methods_count: u2,
    pub methods: Vec<Method>,
    pub attributes_count: u2,
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
        let attributes = String::from("");

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
                    0 => String::from(""),
                    _ => interfaces,
                },
                self.fields_count,
                match self.fields_count {
                    0 => String::from(""),
                    _ => fields,
                },
                self.methods_count,
                match self.methods_count {
                    0 => String::from(""),
                    _ => methods,
                },
                self.attributes_count,
                match self.attributes_count {
                    0 => String::from(""),
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
        let attributes = String::from("");

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
                    0 => String::from(""),
                    _ => interfaces,
                },
                self.fields_count,
                match self.fields_count {
                    0 => String::from(""),
                    _ => fields,
                },
                self.methods_count,
                match self.methods_count {
                    0 => String::from(""),
                    _ => methods,
                },
                self.attributes_count,
                match self.attributes_count {
                    0 => String::from(""),
                    _ => attributes,
                },
            ).as_str()
        )
    }
}

#[repr(packed)]
pub struct Field {
    pub access_flags: u2,
    pub name_index: u2,
    pub descriptor_index: u2,
    pub attributes_count: u2,
    pub attributes: Vec<Attribute>,
}

impl Field {
    pub fn write(self, buff: &mut Vec<u8>) -> Result<(), Error> {
        buff.write_u16::<BigEndian>(self.access_flags)?;
        buff.write_u16::<BigEndian>(self.name_index)?;
        buff.write_u16::<BigEndian>(self.descriptor_index)?;
        buff.write_u16::<BigEndian>(self.attributes_count)?;

        for attr in self.attributes {
            let bytes: Vec<u8> = attr.try_into()?;
            buff.write(bytes.as_slice())?;
        }
        Ok(())
    }
}

impl TryInto<Vec<u8>> for Field {
    type Error = Error;

    fn try_into(self) -> Result<Vec<u8>, Self::Error> {
        let mut output_bytes = Vec::new();

        output_bytes.write_u16::<BigEndian>(self.access_flags)?;
        output_bytes.write_u16::<BigEndian>(self.name_index)?;
        output_bytes.write_u16::<BigEndian>(self.descriptor_index)?;
        output_bytes.write_u16::<BigEndian>(self.attributes_count)?;

        for attr in self.attributes {
            let bytes: Vec<u8> = attr.try_into()?;
            output_bytes.write(bytes.as_slice())?;
        }

        Ok(output_bytes)
    }
}


#[repr(packed)]
pub struct Method {
    pub access_flags: u2,
    pub name_index: u2,
    pub descriptor_index: u2,
    pub attributes_count: u2,
    pub attributes: Vec<Attribute>,
}

impl Method {
    pub fn write(self, buff: &mut Vec<u8>) -> Result<(), Error> {
        buff.write_u16::<BigEndian>(self.access_flags)?;
        buff.write_u16::<BigEndian>(self.name_index)?;
        buff.write_u16::<BigEndian>(self.descriptor_index)?;
        buff.write_u16::<BigEndian>(self.attributes_count)?;

        for attr in self.attributes {
            let bytes: Vec<u8> = attr.try_into()?;
            buff.write(bytes.as_slice())?;
        }
        Ok(())
    }
}

impl TryInto<Vec<u8>> for Method {
    type Error = Error;

    fn try_into(self) -> Result<Vec<u8>, Self::Error> {
        let mut output_bytes = Vec::new();

        output_bytes.write_u16::<BigEndian>(self.access_flags)?;
        output_bytes.write_u16::<BigEndian>(self.name_index)?;
        output_bytes.write_u16::<BigEndian>(self.descriptor_index)?;
        output_bytes.write_u16::<BigEndian>(self.attributes_count)?;

        for attr in self.attributes {
            let bytes: Vec<u8> = attr.try_into()?;
            output_bytes.write(bytes.as_slice())?;
        }

        Ok(output_bytes)
    }
}
