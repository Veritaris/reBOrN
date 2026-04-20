use std::fmt::{format, Debug, Display, Formatter};
use std::string::String;

use crate::access_flags::{AccessFlagContext, AccessFlags};
use crate::attributes::Attribute;
use crate::constant_pool_tag::ConstantPoolTag;
use crate::field::Field;
use crate::method::Method;
use crate::mutf8::read_modified_utf8;
use crate::signature_parser::{parse_jvm_descriptor, parse_object_or_array_descriptor};
use crate::type_alias;
use indoc::indoc;

pub const CLASS_HEADER: u32 = 0xCAFEBABE;

/// ```javadoc
/// ClassFile {
///     type_alias::u4              magic;
///     type_alias::u2              minor_version;
///     type_alias::u2              major_version;
///     type_alias::u2              constant_pool_count;
///     cp_info                     constant_pool[constant_pool_count-1];
///     type_alias::u2              access_flags;
///     type_alias::u2              this_class;
///     type_alias::u2              super_class;
///     type_alias::u2              interfaces_count;
///     type_alias::u2              interfaces[interfaces_count];
///     type_alias::u2              fields_count;
///     field_info                  fields[fields_count];
///     type_alias::u2              methods_count;
///     method_info                 methods[methods_count];
///     type_alias::u2              attributes_count;
///     attribute_info              attributes[attributes_count];
/// }
/// ```
#[derive(Clone)]
pub struct ClassFile {
    pub magic: type_alias::u4,
    pub minor_version: type_alias::u2,
    pub major_version: type_alias::u2,
    pub constant_pool_count: type_alias::u2,
    pub constant_pool: Vec<ConstantPoolTag>,
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

const EMPTY_STRING: String = String::new();

impl ClassFile {
    pub fn get_string_from_cpool(&self, index: u16) -> String {
        match self.constant_pool.get(index as usize).unwrap() {
            ConstantPoolTag::Utf8 { _value, .. } => _value.clone(),
            ConstantPoolTag::String { string_index, .. } => self.get_string_from_cpool(*string_index),
            _ => EMPTY_STRING,
        }
    }

    #[inline]
    pub fn get_from_cpool(&self, index: type_alias::u2) -> ConstantPoolTag {
        self.constant_pool[index as usize].clone()
    }

    pub fn tag_to_display(&self, tag: &ConstantPoolTag) -> String {
        match tag {
            ConstantPoolTag::Utf8 { bytes, length, .. } => {
                let bytes_stringified = match read_modified_utf8(bytes) {
                    Ok(res) => res,
                    Err(err) => {
                        eprintln!("[TagDisplayError]: err={}, raw data: {:?}", err, bytes);
                        String::from("<error>")
                    }
                };
                format!(
                    "Utf8<length={}, bytes='{:?}', value='{}'>",
                    length,
                    bytes.clone(),
                    bytes_stringified.clone()
                )
            }
            ConstantPoolTag::Integer { _value, .. } => {
                format!("Integer<value={}>", _value)
            }
            ConstantPoolTag::Float { _value, .. } => {
                format!("Float<value={}>", _value)
            }
            ConstantPoolTag::Long { _value, .. } => {
                format!("Long<value={}>", _value)
            }
            ConstantPoolTag::Double { _value, .. } => {
                format!("Double<value={}>", _value)
            }
            ConstantPoolTag::Class { name_index, .. } => {
                let val = self.constant_pool.get(*name_index as usize).unwrap();
                format!("Class<name_index={}, content={}>", name_index, self.tag_to_display(val))
            }
            ConstantPoolTag::String { string_index, .. } => {
                let val = self.constant_pool.get(*string_index as usize).unwrap();
                format!(
                    "String<string_index={}, content={}>",
                    string_index,
                    self.tag_to_display(val)
                )
            }
            ConstantPoolTag::Fieldref {
                class_index,
                name_and_type_index,
                ..
            } => {
                let class = self.constant_pool.get(*class_index as usize).unwrap();
                let name_and_type = self.constant_pool.get(*name_and_type_index as usize).unwrap();
                format!(
                    "FieldRef<class={}, name_and_type={}>",
                    self.tag_to_display(class),
                    self.tag_to_display(name_and_type)
                )
            }
            ConstantPoolTag::Methodref {
                class_index,
                name_and_type_index,
                ..
            } => {
                let class = self.constant_pool.get(*class_index as usize).unwrap();
                let name_and_type = self.constant_pool.get(*name_and_type_index as usize).unwrap();
                format!(
                    "MethodRef<class={}, name_and_type={}>",
                    self.tag_to_display(class),
                    self.tag_to_display(name_and_type)
                )
            }
            ConstantPoolTag::InterfaceMethodref {
                class_index,
                name_and_type_index,
                ..
            } => {
                let class = self.constant_pool.get(*class_index as usize).unwrap();
                let name_and_type = self.constant_pool.get(*name_and_type_index as usize).unwrap();
                format!(
                    "InterfaceMethodRef<class={}, name_and_type={}>",
                    self.tag_to_display(class),
                    self.tag_to_display(name_and_type)
                )
            }
            ConstantPoolTag::NameAndType {
                name_index,
                descriptor_index,
                ..
            } => {
                let name = self.constant_pool.get(*name_index as usize).unwrap();
                let descriptor = self.constant_pool.get(*descriptor_index as usize).unwrap();
                format!(
                    "NameAndType<name={}, descriptor={}>",
                    self.tag_to_display(name),
                    self.tag_to_display(descriptor)
                )
            }
            ConstantPoolTag::MethodHandle { .. } => String::from("MethodHandle<TODO>"),
            ConstantPoolTag::MethodType { .. } => String::from("MethodType<TODO>"),
            ConstantPoolTag::Dynamic { .. } => String::from("Dynamic<TODO>"),
            ConstantPoolTag::InvokeDynamic { .. } => String::from("InvokeDynamic<TODO>"),
            ConstantPoolTag::Module { name_index, .. } => {
                format!("Module<name={}>", self.get_string_from_cpool(*name_index))
            }
            ConstantPoolTag::Package { name_index, .. } => {
                format!("Module<name={}>", self.get_string_from_cpool(*name_index))
            }
            ConstantPoolTag::ContinuationTag { .. } => String::from("ContinuationTag"),
        }
    }

    pub fn class_name_from_cp(&self, is_super: bool) -> String {
        let class_name_index = if is_super { self.super_class } else { self.this_class } as usize;
        let class_cp_field_name = if is_super { "super_class" } else { "this_class" };

        match self.constant_pool.get(class_name_index) {
            Some(ConstantPoolTag::Class { name_index, .. }) => match self.constant_pool.get(*name_index as usize) {
                Some(ConstantPoolTag::Utf8 { bytes, .. }) => {
                    String::from_utf8_lossy(bytes).parse::<String>().unwrap_or(EMPTY_STRING)
                }
                Some(other) => format!(
                    "<Error while accessing {} - expected ConstantPoolTag::Utf8 at index {}, got {} instead>",
                    class_cp_field_name, name_index, other
                ),
                None => format!(
                    "<Error accessing {} - index {} is out of bounds (constant pool size is {})>",
                    class_cp_field_name,
                    name_index,
                    self.constant_pool.len()
                ),
            },
            Some(other) => format!(
                "<Error accessing {} - expected ConstantPoolTag::Class at index {}, got {} instead>",
                class_cp_field_name, class_name_index, other
            ),
            None => format!(
                "<Error accessing {} - index {} is out of bounds (constant pool size is {})>",
                class_cp_field_name,
                class_name_index,
                self.constant_pool.len()
            ),
        }
    }
}

impl Debug for ClassFile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let class_repr = format!("{}", self);

        let constant_pool: String = self
            .constant_pool
            .iter()
            .enumerate()
            .skip(1)
            .map(|(index, entry)| format!("{} {}", index, self.tag_to_display(entry)))
            .fold(String::from("        "), |acc, e| acc + "\n        " + e.as_str());

        f.write_str(
            format!(
                indoc!(
                    "
                    {}
                        constant_pool: {}
                    "
                ),
                class_repr, constant_pool
            )
            .as_str(),
        )
    }
}

impl Display for ClassFile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let class_name = self.class_name_from_cp(false);
        let super_class_name = self.class_name_from_cp(true);

        let interfaces = self
            .interfaces
            .iter()
            .map(|class_index| match self.constant_pool.get(*class_index as usize) {
                Some(class_info) => match class_info {
                    ConstantPoolTag::Class { name_index, .. } => self.get_string_from_cpool(*name_index),
                    _ => format!(
                        "Error accessing interfaces - expected Class_info found at index {} in the constant pool: got {}",
                        class_index, class_info
                    ),
                },
                None => format!("Error accessing interfaces - nothing found at index {} in the constant pool", class_index),
            })
            .fold(String::from("        "), |acc, e| acc + "\n        " + e.as_str());

        let fields = self
            .fields
            .iter()
            .map(|f| {
                let field_signature = parse_jvm_descriptor(self.get_string_from_cpool(f.descriptor_index).as_str())
                    .unwrap()
                    .to_string();
                format!(
                    "{} {} {}",
                    AccessFlags::from((AccessFlagContext::Field, f.access_flags)).as_string(),
                    field_signature.trim(),
                    self.get_string_from_cpool(f.name_index).trim()
                )
                .trim()
                .to_string()
            })
            .fold(String::from("        "), |acc, f| acc + "\n        " + f.as_str());
        let methods = self
            .methods
            .iter()
            .flat_map(|m| {
                let method_signature = parse_jvm_descriptor(self.get_string_from_cpool(m.descriptor_index).as_str())
                    .unwrap()
                    .to_string();
                let method_repr = format!(
                    "{} {} {}",
                    AccessFlags::from((AccessFlagContext::Field, m.access_flags)).as_string(),
                    self.get_string_from_cpool(m.name_index).trim(),
                    method_signature.trim(),
                )
                .trim()
                .to_string();
                let method_attributes = m
                    .attributes
                    .iter()
                    .map(|attr| format!("  {:#?}", attr))
                    .collect::<Vec<String>>();
                let mut method_info: Vec<String> = vec![method_repr];
                method_info.extend(method_attributes);
                method_info
            })
            .fold(String::from("        "), |acc, f| acc + "\n        " + f.as_str());
        let attributes = self
            .attributes
            .iter()
            .map(|attr| format!("{:?}", attr))
            .fold(String::from("        "), |acc, f| acc + "\n        " + f.as_str());

        f.write_str(
            format!(
                indoc!(
                    "
                Classfile version {}.{} of class {} extends {}
                    constant_pool_count: {}
                    access_flags: {}
                    this_class: {}
                    super_class: {}
                    interfaces_count: {}{}
                    fields_count: {}{}
                    methods_count: {}{}
                    attributes_count: {}{}
                    "
                ),
                self.major_version,
                self.minor_version,
                class_name,
                super_class_name,
                self.constant_pool_count,
                self.access_flags.as_string(),
                class_name,
                super_class_name,
                self.interfaces_count,
                match self.interfaces_count {
                    0 => String::from("[]"),
                    _ => interfaces,
                },
                self.fields_count,
                match self.fields_count {
                    0 => String::from("[]"),
                    _ => fields,
                },
                self.methods_count,
                match self.methods_count {
                    0 => String::from("[]"),
                    _ => methods,
                },
                self.attributes_count,
                match self.attributes_count {
                    0 => String::from("[]"),
                    _ => attributes,
                },
            )
            .as_str(),
        )
    }
}
