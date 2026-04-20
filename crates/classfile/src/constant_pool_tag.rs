use crate::type_alias;
use byteorder::{BigEndian, WriteBytesExt};
use std::fmt::{Display, Formatter};
use std::io::{Error, Write};

pub const CONTINUATION_TAG: ConstantPoolTag = ConstantPoolTag::ContinuationTag { tag: 0 };

///```javadoc
/// Utf8                        1   45.3    1.0.2
/// Integer                     3   45.3    1.0.2
/// Float                       4   45.3    1.0.2
/// Long                        5   45.3    1.0.2
/// Double                      6   45.3    1.0.2
/// Class                       7   45.3    1.0.2
/// String                      8   45.3    1.0.2
/// Fieldref                    9   45.3    1.0.2
/// Methodref                   10  45.3    1.0.2
/// InterfaceMethodref          11  45.3    1.0.2
/// NameAndType                 12  45.3    1.0.2
/// MethodHandle                15  51.0    7
/// MethodType                  16  51.0    7
/// Dynamic                     17  55.0    11
/// InvokeDynamic               18  51.0    7
/// Module                      19  53.0    9
/// Package                     20  53.0    9
/// ```
///
#[derive(Clone, Debug)]
pub enum ConstantPoolTag {
    ContinuationTag {
        tag: type_alias::u1,
    },
    Utf8 {
        tag: type_alias::u1,
        length: type_alias::u2,
        bytes: Vec<type_alias::u1>,
        _value: String,
    },
    Integer {
        tag: type_alias::u1,
        bytes: type_alias::u4,
        _value: i32,
    },
    Float {
        tag: type_alias::u1,
        bytes: type_alias::u4,
        _value: f32,
    },
    Long {
        tag: type_alias::u1,
        high_bytes: type_alias::u4,
        low_bytes: type_alias::u4,
        _value: i64,
    },
    Double {
        tag: type_alias::u1,
        high_bytes: type_alias::u4,
        low_bytes: type_alias::u4,
        _value: f64,
    },
    Class {
        tag: type_alias::u1,
        name_index: type_alias::u2,
    },
    String {
        tag: type_alias::u1,
        string_index: type_alias::u2,
    },
    Fieldref {
        tag: type_alias::u1,
        class_index: type_alias::u2,
        name_and_type_index: type_alias::u2,
    },
    Methodref {
        tag: type_alias::u1,
        class_index: type_alias::u2,
        name_and_type_index: type_alias::u2,
    },
    InterfaceMethodref {
        tag: type_alias::u1,
        class_index: type_alias::u2,
        name_and_type_index: type_alias::u2,
    },
    NameAndType {
        tag: type_alias::u1,
        name_index: type_alias::u2,
        descriptor_index: type_alias::u2,
    },
    MethodHandle {
        tag: type_alias::u1,
        reference_kind: type_alias::u1,
        reference_index: type_alias::u2,
    },
    MethodType {
        tag: type_alias::u1,
        descriptor_index: type_alias::u2,
    },
    Dynamic {
        tag: type_alias::u1,
        bootstrap_method_attr_index: type_alias::u2,
        name_and_type_index: type_alias::u2,
    },
    InvokeDynamic {
        tag: type_alias::u1,
        bootstrap_method_attr_index: type_alias::u2,
        name_and_type_index: type_alias::u2,
    },
    Module {
        tag: type_alias::u1,
        name_index: type_alias::u2,
    },
    Package {
        tag: type_alias::u1,
        name_index: type_alias::u2,
    },
}

impl ConstantPoolTag {
    pub fn jvm_tag(&self) -> u8 {
        match self {
            ConstantPoolTag::ContinuationTag { .. } => 0,
            ConstantPoolTag::Utf8 { .. } => 1,
            ConstantPoolTag::Integer { .. } => 3,
            ConstantPoolTag::Float { .. } => 4,
            ConstantPoolTag::Long { .. } => 5,
            ConstantPoolTag::Double { .. } => 6,
            ConstantPoolTag::Class { .. } => 7,
            ConstantPoolTag::String { .. } => 8,
            ConstantPoolTag::Fieldref { .. } => 9,
            ConstantPoolTag::Methodref { .. } => 10,
            ConstantPoolTag::InterfaceMethodref { .. } => 11,
            ConstantPoolTag::NameAndType { .. } => 12,
            ConstantPoolTag::MethodHandle { .. } => 15,
            ConstantPoolTag::MethodType { .. } => 16,
            ConstantPoolTag::Dynamic { .. } => 17,
            ConstantPoolTag::InvokeDynamic { .. } => 18,
            ConstantPoolTag::Module { .. } => 19,
            ConstantPoolTag::Package { .. } => 20,
        }
    }

    pub fn string_tag_to_string(
        constant_pool: &Vec<ConstantPoolTag>,
        string: ConstantPoolTag,
        visited_entries: Option<Vec<type_alias::u2>>,
    ) -> String {
        let visited_entries = visited_entries.unwrap_or_default();
        match string {
            ConstantPoolTag::Utf8 { _value, .. } => _value,
            ConstantPoolTag::String { string_index, .. } => {
                if visited_entries.contains(&string_index) {
                    return "".to_string();
                }

                match constant_pool.get(string_index as usize) {
                    None => format!("<error: tag with index={string_index} does not exist>"),
                    Some(tag) => Self::string_tag_to_string(constant_pool, tag.clone(), Some(visited_entries)),
                }
            }
            _ => "<error: not String or Utf8 tag>".to_string(),
        }
    }

    #[allow(unused)]
    pub fn write(self, buff: &mut Vec<u8>) -> Result<(), Error> {
        // skip writing anything on ContinuationTag met
        match self {
            ConstantPoolTag::ContinuationTag { .. } => return Ok(()),
            _ => buff.write_u8(self.jvm_tag())?,
        };

        match self {
            ConstantPoolTag::ContinuationTag { .. } => {}
            ConstantPoolTag::Utf8 { length, bytes, .. } => {
                buff.write_u16::<BigEndian>(length)?;
                let _ = buff.write(bytes.as_slice())?;
            }
            ConstantPoolTag::Integer { bytes, .. } => buff.write_u32::<BigEndian>(bytes)?,
            ConstantPoolTag::Float { bytes, .. } => buff.write_u32::<BigEndian>(bytes)?,
            ConstantPoolTag::Long {
                high_bytes, low_bytes, ..
            } => {
                buff.write_u32::<BigEndian>(high_bytes)?;
                buff.write_u32::<BigEndian>(low_bytes)?;
            }
            ConstantPoolTag::Double {
                high_bytes, low_bytes, ..
            } => {
                buff.write_u32::<BigEndian>(high_bytes)?;
                buff.write_u32::<BigEndian>(low_bytes)?;
            }
            ConstantPoolTag::Class { name_index, .. } => buff.write_u16::<BigEndian>(name_index)?,
            ConstantPoolTag::String { string_index, .. } => buff.write_u16::<BigEndian>(string_index)?,
            ConstantPoolTag::Fieldref {
                class_index,
                name_and_type_index,
                ..
            } => {
                buff.write_u16::<BigEndian>(class_index)?;
                buff.write_u16::<BigEndian>(name_and_type_index)?;
            }
            ConstantPoolTag::Methodref {
                class_index,
                name_and_type_index,
                ..
            } => {
                buff.write_u16::<BigEndian>(class_index)?;
                buff.write_u16::<BigEndian>(name_and_type_index)?;
            }
            ConstantPoolTag::InterfaceMethodref {
                class_index,
                name_and_type_index,
                ..
            } => {
                buff.write_u16::<BigEndian>(class_index)?;
                buff.write_u16::<BigEndian>(name_and_type_index)?;
            }
            ConstantPoolTag::NameAndType {
                name_index,
                descriptor_index,
                ..
            } => {
                buff.write_u16::<BigEndian>(name_index)?;
                buff.write_u16::<BigEndian>(descriptor_index)?;
            }
            ConstantPoolTag::MethodHandle {
                reference_kind,
                reference_index,
                ..
            } => {
                buff.write_u8(reference_kind)?;
                buff.write_u16::<BigEndian>(reference_index)?;
            }
            ConstantPoolTag::MethodType { descriptor_index, .. } => {
                buff.write_u16::<BigEndian>(descriptor_index)?;
            }
            ConstantPoolTag::Dynamic {
                bootstrap_method_attr_index,
                name_and_type_index,
                ..
            } => {
                buff.write_u16::<BigEndian>(bootstrap_method_attr_index)?;
                buff.write_u16::<BigEndian>(name_and_type_index)?;
            }
            ConstantPoolTag::InvokeDynamic {
                bootstrap_method_attr_index,
                name_and_type_index,
                ..
            } => {
                buff.write_u16::<BigEndian>(bootstrap_method_attr_index)?;
                buff.write_u16::<BigEndian>(name_and_type_index)?;
            }
            ConstantPoolTag::Module { name_index, .. } => buff.write_u16::<BigEndian>(name_index)?,
            ConstantPoolTag::Package { name_index, .. } => buff.write_u16::<BigEndian>(name_index)?,
        };
        Ok(())
    }
}

impl TryInto<Vec<u8>> for ConstantPoolTag {
    type Error = Error;

    fn try_into(self) -> Result<Vec<u8>, Self::Error> {
        let mut output_bytes = Vec::new();
        self.write(&mut output_bytes)?;
        Ok(output_bytes)
    }
}

impl Display for ConstantPoolTag {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.jvm_tag().to_string().as_str())
    }
}

#[derive(Copy, Clone, Debug)]
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
