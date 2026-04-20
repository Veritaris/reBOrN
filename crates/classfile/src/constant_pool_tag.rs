use std::fmt::{Display, Formatter};
use std::io::{Error, Write};
use byteorder::{BigEndian, WriteBytesExt};
use crate::type_alias;

pub const CONTINUATION_TAG: ConstantPoolTags = ConstantPoolTags::ContinuationTag { tag: 0 };

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
    ContinuationTag { tag: type_alias::u1 },
    Utf8 { tag: type_alias::u1, length: type_alias::u2, bytes: Vec<type_alias::u1>, _value: String },
    Integer { tag: type_alias::u1, bytes: type_alias::u4, _value: i32 },
    Float { tag: type_alias::u1, bytes: type_alias::u4, _value: f32 },
    Long { tag: type_alias::u1, high_bytes: type_alias::u4, low_bytes: type_alias::u4, _value: i64 },
    Double { tag: type_alias::u1, high_bytes: type_alias::u4, low_bytes: type_alias::u4, _value: f64 },
    Class { tag: type_alias::u1, name_index: type_alias::u2 },
    String { tag: type_alias::u1, string_index: type_alias::u2 },
    Fieldref { tag: type_alias::u1, class_index: type_alias::u2, name_and_type_index: type_alias::u2 },
    Methodref { tag: type_alias::u1, class_index: type_alias::u2, name_and_type_index: type_alias::u2 },
    InterfaceMethodref { tag: type_alias::u1, class_index: type_alias::u2, name_and_type_index: type_alias::u2 },
    NameAndType { tag: type_alias::u1, name_index: type_alias::u2, descriptor_index: type_alias::u2 },
    MethodHandle { tag: type_alias::u1, reference_kind: type_alias::u1, reference_index: type_alias::u2 },
    MethodType { tag: type_alias::u1, descriptor_index: type_alias::u2 },
    Dynamic { tag: type_alias::u1, bootstrap_method_attr_index: type_alias::u2, name_and_type_index: type_alias::u2 },
    InvokeDynamic { tag: type_alias::u1, bootstrap_method_attr_index: type_alias::u2, name_and_type_index: type_alias::u2 },
    Module { tag: type_alias::u1, name_index: type_alias::u2 },
    Package { tag: type_alias::u1, name_index: type_alias::u2 },
}

impl ConstantPoolTags {
    pub fn jvm_tag(&self) -> u8 {
        match self {
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
        }
    }

    pub fn string_tag_to_string(constant_pool: &Vec<ConstantPoolTags>, string: ConstantPoolTags, visited_entries: Option<Vec<type_alias::u2>>) -> String {
        let visited_entries = visited_entries.unwrap_or(vec![]);
        match string {
            ConstantPoolTags::Utf8 { _value, .. } => _value,
            ConstantPoolTags::String { string_index, .. } => {
                if visited_entries.contains(&string_index) {
                    return "".to_string();
                }

                match constant_pool.get(string_index as usize) {
                    None => format!("<error: tag with index={string_index} does not exist>"),
                    Some(tag) => {
                        Self::string_tag_to_string(constant_pool, tag.clone(), Some(visited_entries))
                    }
                }
            }
            _ => "<error: not String or Utf8 tag>".to_string(),
        }
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
        match value {
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
        }
    }
}
