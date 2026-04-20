use std::fmt::{Debug, Display, Formatter};
use std::io::{BufReader, Error, ErrorKind, Read, Write};

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

use crate::access_flags::AccessFlags;
use crate::opcodes::OPCODES_MAP;
use crate::type_alias;

///```javadoc
/// {   type_alias::u2 start_pc;
///     type_alias::u2 end_pc;
///     type_alias::u2 handler_pc;
///     type_alias::u2 catch_type;
/// } exception_table[exception_table_length];
///```
#[derive(Copy, Clone, Debug)]
pub struct ExceptionTableEntry {
    pub start_pc: type_alias::u2,
    pub end_pc: type_alias::u2,
    pub handler_pc: type_alias::u2,
    pub catch_type: type_alias::u2,
}

impl ExceptionTableEntry {
    pub fn write<B: Write>(self, buff: &mut B) -> Result<usize, Error> {
        buff.write_u16::<BigEndian>(self.start_pc)?;
        buff.write_u16::<BigEndian>(self.end_pc)?;
        buff.write_u16::<BigEndian>(self.handler_pc)?;
        buff.write_u16::<BigEndian>(self.catch_type)?;
        Ok(size_of::<u16>() + size_of::<u16>() + size_of::<u16>() + size_of::<u16>())
    }
}

///```javadoc
/// {   u2 start_pc;
///     u2 line_number;
/// } line_number_table[line_number_table_length];
///```
#[derive(Copy, Clone, Debug)]
pub struct LineNumberEntry {
    pub start_pc: type_alias::u2,
    pub line_number: type_alias::u2,
}

impl LineNumberEntry {
    pub fn write<B: Write>(self, buff: &mut B) -> Result<usize, Error> {
        buff.write_u16::<BigEndian>(self.start_pc)?;
        buff.write_u16::<BigEndian>(self.line_number)?;
        Ok(size_of::<u16>() + size_of::<u16>())
    }
}

///```javadoc
/// {   type_alias::u2 start_pc;
///     type_alias::u2 length;
///     type_alias::u2 name_index;
///     type_alias::u2 descriptor_index;
///     type_alias::u2 index;
/// } local_variable_table[local_variable_table_length];
///```
#[derive(Copy, Clone, Debug)]
pub struct LocalVariableTableEntry {
    pub start_pc: type_alias::u2,
    pub length: type_alias::u2,
    pub name_index: type_alias::u2,
    pub descriptor_index: type_alias::u2,
    pub index: type_alias::u2,
}

impl LocalVariableTableEntry {
    pub fn write<B: Write>(self, buff: &mut B) -> Result<usize, Error> {
        buff.write_u16::<BigEndian>(self.start_pc)?;
        buff.write_u16::<BigEndian>(self.length)?;
        buff.write_u16::<BigEndian>(self.name_index)?;
        buff.write_u16::<BigEndian>(self.descriptor_index)?;
        buff.write_u16::<BigEndian>(self.index)?;
        Ok(size_of::<u16>() + size_of::<u16>() + size_of::<u16>() + size_of::<u16>() + size_of::<u16>())
    }
}

///```javadoc
/// {   type_alias::u2 start_pc;
///     type_alias::u2 length;
///     type_alias::u2 name_index;
///     type_alias::u2 signature_index;
///     type_alias::u2 index;
/// } local_variable_type_table[local_variable_type_table_length];
/// ```
#[derive(Copy, Clone, Debug)]
pub struct LocalVariableTypeTableEntry {
    pub start_pc: type_alias::u2,
    pub length: type_alias::u2,
    pub name_index: type_alias::u2,
    pub signature_index: type_alias::u2,
    pub index: type_alias::u2,
}

impl LocalVariableTypeTableEntry {
    pub fn write<B: Write>(self, buff: &mut B) -> Result<usize, Error> {
        buff.write_u16::<BigEndian>(self.start_pc)?;
        buff.write_u16::<BigEndian>(self.length)?;
        buff.write_u16::<BigEndian>(self.name_index)?;
        buff.write_u16::<BigEndian>(self.signature_index)?;
        buff.write_u16::<BigEndian>(self.index)?;
        Ok(size_of::<u16>() + size_of::<u16>() + size_of::<u16>() + size_of::<u16>() + size_of::<u16>())
    }
}

///```javadoc
/// {   type_alias::u2 bootstrap_method_ref;
///     type_alias::u2 num_bootstrap_arguments;
///     type_alias::u2 bootstrap_arguments[num_bootstrap_arguments];
/// } bootstrap_methods[num_bootstrap_methods];
/// ```
#[derive(Clone, Debug)]
pub struct BootstrapMethodEntry {
    pub bootstrap_method_ref: type_alias::u2,
    pub num_bootstrap_arguments: type_alias::u2,
    pub bootstrap_arguments: Vec<type_alias::u2>,
}

impl BootstrapMethodEntry {
    pub fn write<B: Write>(self, buff: &mut B) -> Result<usize, Error> {
        buff.write_u16::<BigEndian>(self.bootstrap_method_ref)?;
        buff.write_u16::<BigEndian>(self.num_bootstrap_arguments)?;
        let mut bytes_written = size_of::<u16>() + size_of::<u16>();
        bytes_written += write_vec_u16_as_bytes(&self.bootstrap_arguments, buff)?;
        Ok(bytes_written)
    }
}

///```javadoc
/// {   type_alias::u2 inner_class_info_index;
///     type_alias::u2 outer_class_info_index;
///     type_alias::u2 inner_name_index;
///     type_alias::u2 inner_class_access_flags;
/// } classes[number_of_classes];
/// ```
#[derive(Copy, Clone, Debug)]
pub struct InnerClassEntry {
    pub inner_class_info_index: type_alias::u2,
    pub outer_class_info_index: type_alias::u2,
    pub inner_name_index: type_alias::u2,
    pub inner_class_access_flags: type_alias::u2,
}

impl InnerClassEntry {
    pub fn write<B: Write>(self, buff: &mut B) -> Result<usize, Error> {
        buff.write_u16::<BigEndian>(self.inner_class_info_index)?;
        buff.write_u16::<BigEndian>(self.outer_class_info_index)?;
        buff.write_u16::<BigEndian>(self.inner_name_index)?;
        buff.write_u16::<BigEndian>(self.inner_class_access_flags)?;
        Ok(size_of::<u16>() + size_of::<u16>() + size_of::<u16>() + size_of::<u16>())
    }
}

///```javadoc
/// record_component_info {
///     type_alias::u2             name_index;
///     type_alias::u2             descriptor_index;
///     type_alias::u2             attributes_count;
///     attribute_info attributes[attributes_count];
/// }
/// ```
#[derive(Clone, Debug)]
pub struct RecordComponentInfo {
    pub name_index: type_alias::u2,
    pub descriptor_index: type_alias::u2,
    pub attributes_count: type_alias::u2,
    pub attributes: Vec<Attribute>,
}

impl RecordComponentInfo {
    pub fn write<B: Write>(self, buff: &mut B) -> Result<usize, Error> {
        buff.write_u16::<BigEndian>(self.name_index)?;
        buff.write_u16::<BigEndian>(self.descriptor_index)?;
        buff.write_u16::<BigEndian>(self.attributes_count)?;
        let mut bytes_written = size_of::<u16>() + size_of::<u16>() + size_of::<u16>();
        for attr in self.attributes {
            bytes_written += attr.write(buff)?;
        }
        Ok(bytes_written)
    }
}

///```javadoc
/// annotation {
///     type_alias::u2 type_index;
///     type_alias::u2 num_element_value_pairs;
///     {   type_alias::u2            element_name_index;
///         element_value value;
///     } element_value_pairs[num_element_value_pairs];
/// }
///```
#[derive(Clone, Debug)]
pub struct Annotation {
    pub type_index: type_alias::u2,
    pub num_element_value_pairs: type_alias::u2,
    pub element_value_pairs: Vec<ElementValuePair>,
}

impl Annotation {
    pub fn write<B: Write>(self, buff: &mut B) -> Result<usize, Error> {
        buff.write_u16::<BigEndian>(self.type_index)?;
        buff.write_u16::<BigEndian>(self.num_element_value_pairs)?;
        let mut bytes_written = size_of::<u16>() + size_of::<u16>();
        for one_pair in self.element_value_pairs {
            bytes_written += one_pair.write(buff)?;
        }
        Ok(bytes_written)
    }
}

///```javadoc
/// {   type_alias::u2            element_name_index;
///     element_value value;
/// }
/// ```
#[derive(Clone, Debug)]
pub struct ElementValuePair {
    pub element_name_index: type_alias::u2,
    pub value: ElementValue,
}

impl ElementValuePair {
    pub fn write<B: Write>(self, buff: &mut B) -> Result<usize, Error> {
        let mut bytes_written = 0;
        buff.write_u16::<BigEndian>(self.element_name_index)?;
        bytes_written += size_of::<u16>();
        bytes_written += self.value.write(buff)?;
        Ok(bytes_written)
    }
}

///```javadoc
/// element_value {
///     type_alias::u1 tag;
///     union {
///         type_alias::u2 const_value_index;
///
///         {   type_alias::u2 type_name_index;
///             type_alias::u2 const_name_index;
///         } enum_const_value;
///
///         type_alias::u2 class_info_index;
///
///         annotation annotation_value;
///
///         {   type_alias::u2            num_values;
///             element_value values[num_values];
///         } array_value;
///     } value;
/// }
///```
#[derive(Clone, Debug)]
pub struct ElementValue {
    pub tag: type_alias::u1,
    pub value: Value,
}

impl ElementValue {
    pub fn write<B: Write>(self, buff: &mut B) -> Result<usize, Error> {
        let mut bytes_written = 0;
        buff.write_u8(self.tag)?;
        bytes_written += size_of::<u8>();
        bytes_written += self.value.write(buff)?;

        Ok(bytes_written)
    }
}

///```javadoc
/// union {
///     type_alias::u2 const_value_index;
///
///     {   type_alias::u2 type_name_index;
///         type_alias::u2 const_name_index;
///     } enum_const_value;
///
///     type_alias::u2 class_info_index;
///
///     annotation annotation_value;
///
///     {   type_alias::u2            num_values;
///         element_value values[num_values];
///     } array_value;
/// } value;
///```
/// | tag Item  | Type                  | value Item            | Constant Type     |
/// |-----------|-----------------------|-----------------------|-------------------|
/// | B         | byte                  | const_value_index     | CONSTANT_Integer  |
/// | C         | char                  | const_value_index     | CONSTANT_Integer  |
/// | D         | double                | const_value_index     | CONSTANT_Double   |
/// | F         | float                 | const_value_index     | CONSTANT_Float    |
/// | I         | int                   | const_value_index     | CONSTANT_Integer  |
/// | J         | long                  | const_value_index     | CONSTANT_Long     |
/// | S         | short                 | const_value_index     | CONSTANT_Integer  |
/// | Z         | boolean               | const_value_index     | CONSTANT_Integer  |
/// | s         | String                | const_value_index     | CONSTANT_Utf8     |
/// | e         | Enum class            | enum_const_value      | Not applicable    |
/// | c         | Class                 | class_info_index      | Not applicable    |
/// | @         | Annotation interface  | annotation_value      | Not applicable    |
/// | [         | Array type            | array_value           | Not applicable    |
#[derive(Clone, Debug)]
pub enum Value {
    ConstValueIndex {
        const_value_index: type_alias::u2,
    },
    EnumConstValue {
        type_name_index: type_alias::u2,
        const_name_index: type_alias::u2,
    },
    ClassInfoIndex {
        class_info_index: type_alias::u2,
    },
    AnnotationValue {
        annotation_value: Annotation,
    },
    ArrayValue {
        num_values: type_alias::u2,
        values: Vec<ElementValue>,
    },
}

impl Value {
    pub fn write<B: Write>(self, buff: &mut B) -> Result<usize, Error> {
        match self {
            Value::ConstValueIndex { const_value_index } => {
                buff.write_u16::<BigEndian>(const_value_index)?;
                Ok(size_of::<u16>())
            }
            Value::EnumConstValue {
                type_name_index,
                const_name_index,
            } => {
                buff.write_u16::<BigEndian>(type_name_index)?;
                buff.write_u16::<BigEndian>(const_name_index)?;
                Ok(size_of::<u16>() + size_of::<u16>())
            }
            Value::ClassInfoIndex { class_info_index } => {
                buff.write_u16::<BigEndian>(class_info_index)?;
                Ok(size_of::<u16>())
            }
            Value::AnnotationValue { annotation_value } => Ok(annotation_value.write(buff)?),
            Value::ArrayValue { num_values, values } => {
                buff.write_u16::<BigEndian>(num_values)?;
                let mut bytes_written = size_of::<u16>();
                for one_value in values {
                    bytes_written += one_value.write(buff)?;
                }
                Ok(bytes_written)
            }
        }
    }
}

///```javadoc
/// {   u2 name_index;
///     u2 access_flags;
/// } parameters[parameters_count];
///```
#[derive(Copy, Clone, Debug)]
pub struct Parameter {
    pub name_index: type_alias::u2,
    pub access_flags: AccessFlags,
}

impl Parameter {
    pub fn write<B: Write>(self, buff: &mut B) -> Result<usize, Error> {
        buff.write_u16::<BigEndian>(self.name_index)?;
        buff.write_u16::<BigEndian>(self.access_flags.into())?;
        Ok(size_of::<u16>() + size_of::<u16>())
    }
}

///```javadoc
/// union verification_type_info {
///     Top_variable_info;
///     Integer_variable_info;
///     Float_variable_info;
///     Long_variable_info;
///     Double_variable_info;
///     Null_variable_info;
///     UninitializedThis_variable_info;
///     Object_variable_info;
///     Uninitialized_variable_info;
/// }
///```
#[derive(Copy, Clone, Debug)]
pub enum VerificationTypeInfoItem {
    ItemTop = 0,
    ItemInteger = 1,
    ItemFloat = 2,
    ItemDouble = 3,
    ItemLong = 4,
    ItemNull = 5,
    ItemUninitializedThis = 6,
    ItemObject = 7,
    ItemUninitialized = 8,
}

///```javadoc
/// {   type_alias::u2 start_pc;
///     type_alias::u2 length;
///     type_alias::u2 name_index;
///     type_alias::u2 descriptor_index;
///     type_alias::u2 index;
/// } local_variable_table[local_variable_table_length];
///```
#[derive(Clone, Debug)]
pub struct ParameterAnnotation {
    pub num_annotations: type_alias::u2,
    pub annotations: Vec<Annotation>,
}

impl ParameterAnnotation {
    pub fn write<B: Write>(self, buff: &mut B) -> Result<usize, Error> {
        buff.write_u16::<BigEndian>(self.num_annotations)?;
        let mut bytes_written = size_of::<u16>();
        for one_ann in self.annotations {
            bytes_written += one_ann.write(buff)?
        }
        Ok(bytes_written)
    }
}

///```javadoc
/// {   type_alias::u2 start_pc;
///     type_alias::u2 length;
///     type_alias::u2 name_index;
///     type_alias::u2 descriptor_index;
///     type_alias::u2 index;
/// } local_variable_table[local_variable_table_length];
///```
#[derive(Clone, Debug)]
pub struct TypeAnnotation {
    pub target_type: type_alias::u1,
    pub target_info: TargetInfo,
    pub target_path: TypePath,
    pub type_index: type_alias::u2,
    pub num_element_value_pairs: type_alias::u2,
    pub element_value_pairs: Vec<ElementValuePair>,
}

impl TypeAnnotation {
    pub fn write<B: Write>(self, buff: &mut B) -> Result<usize, Error> {
        let mut bytes_written = 0;
        buff.write_u8(self.target_type)?;
        bytes_written += size_of::<u8>();
        bytes_written += self.target_info.write(buff)?;
        bytes_written += self.target_path.write(buff)?;
        buff.write_u16::<BigEndian>(self.type_index)?;
        bytes_written += size_of::<u16>();
        buff.write_u16::<BigEndian>(self.num_element_value_pairs)?;
        bytes_written += size_of::<u16>();
        for entry in self.element_value_pairs {
            bytes_written += entry.write(buff)?;
        }
        Ok(bytes_written)
    }
}

///```javadoc
/// {   type_alias::u2 start_pc;
///     type_alias::u2 length;
///     type_alias::u2 name_index;
///     type_alias::u2 descriptor_index;
///     type_alias::u2 index;
/// } local_variable_table[local_variable_table_length];
///```
#[derive(Copy, Clone, Debug)]
pub struct PathEntry {
    pub type_path_kind: type_alias::u1,
    pub type_argument_index: type_alias::u1,
}

impl PathEntry {
    pub fn write<B: Write>(self, buff: &mut B) -> Result<usize, Error> {
        buff.write_u8(self.type_path_kind)?;
        buff.write_u8(self.type_argument_index)?;
        Ok(size_of::<u8>() + size_of::<u8>())
    }
}

///```javadoc
/// {   type_alias::u2 start_pc;
///     type_alias::u2 length;
///     type_alias::u2 name_index;
///     type_alias::u2 descriptor_index;
///     type_alias::u2 index;
/// } local_variable_table[local_variable_table_length];
///```
#[derive(Clone, Debug)]
pub struct TypePath {
    pub path_length: type_alias::u1,
    pub path: Vec<PathEntry>,
}

impl TypePath {
    pub fn write<B: Write>(self, buff: &mut B) -> Result<usize, Error> {
        let mut bytes_written = 0;
        buff.write_u8(self.path_length)?;
        bytes_written += size_of::<u8>();
        for one_path in self.path {
            bytes_written += one_path.write(buff)?;
        }
        Ok(bytes_written)
    }
}

///```javadoc
/// {   type_alias::u2 start_pc;
///     type_alias::u2 length;
///     type_alias::u2 name_index;
///     type_alias::u2 descriptor_index;
///     type_alias::u2 index;
/// } local_variable_table[local_variable_table_length];
///```
#[derive(Clone, Debug)]
pub enum TargetInfo {
    TypeParameterTarget {
        type_parameter_index: type_alias::u1,
    },
    SupertypeTarget {
        supertype_index: type_alias::u2,
    },
    TypeParameterBoundTarget {
        type_parameter_index: type_alias::u1,
        bound_index: type_alias::u1,
    },
    EmptyTarget {},
    FormalParameterTarget {
        formal_parameter_index: type_alias::u1,
    },
    ThrowsTarget {
        throws_type_index: type_alias::u2,
    },
    LocalvarTarget {
        table_length: type_alias::u2,
        table: Vec<LocalvarTargetTableEntry>,
    },
    CatchTarget {
        exception_table_index: type_alias::u2,
    },
    OffsetTarget {
        offset: type_alias::u2,
    },
    TypeArgumentTarget {
        offset: type_alias::u2,
        type_argument_index: type_alias::u1,
    },
}

impl TargetInfo {
    pub fn write<B: Write>(self, buff: &mut B) -> Result<usize, Error> {
        match self {
            TargetInfo::TypeParameterTarget { type_parameter_index } => {
                buff.write_u8(type_parameter_index)?;
                Ok(size_of::<u8>())
            }
            TargetInfo::SupertypeTarget { supertype_index } => {
                buff.write_u16::<BigEndian>(supertype_index)?;
                Ok(size_of::<u16>())
            }
            TargetInfo::TypeParameterBoundTarget {
                type_parameter_index,
                bound_index,
            } => {
                buff.write_u8(type_parameter_index)?;
                buff.write_u8(bound_index)?;
                Ok(size_of::<u8>() + size_of::<u8>())
            }
            TargetInfo::EmptyTarget {} => Ok(0),
            TargetInfo::FormalParameterTarget { formal_parameter_index } => {
                buff.write_u8(formal_parameter_index)?;
                Ok(size_of::<u8>())
            }
            TargetInfo::ThrowsTarget { throws_type_index } => {
                buff.write_u16::<BigEndian>(throws_type_index)?;
                Ok(size_of::<u16>())
            }
            TargetInfo::LocalvarTarget { table_length, table } => {
                let mut bytes_written = 0;
                buff.write_u16::<BigEndian>(table_length)?;
                bytes_written += size_of::<u16>();
                for entry in table {
                    bytes_written += entry.write(buff)?;
                }
                Ok(bytes_written)
            }
            TargetInfo::CatchTarget { exception_table_index } => {
                buff.write_u16::<BigEndian>(exception_table_index)?;
                Ok(size_of::<u16>())
            }
            TargetInfo::OffsetTarget { offset } => {
                buff.write_u16::<BigEndian>(offset)?;
                Ok(size_of::<u16>())
            }
            TargetInfo::TypeArgumentTarget {
                offset,
                type_argument_index,
            } => {
                buff.write_u16::<BigEndian>(offset)?;
                buff.write_u8(type_argument_index)?;
                Ok(size_of::<u16>() + size_of::<u8>())
            }
        }
    }
}

///```javadoc
/// {   type_alias::u2 start_pc;
///     type_alias::u2 length;
///     type_alias::u2 name_index;
///     type_alias::u2 descriptor_index;
///     type_alias::u2 index;
/// } local_variable_table[local_variable_table_length];
///```
#[derive(Copy, Clone, Debug)]
pub struct LocalvarTargetTableEntry {
    pub start_pc: type_alias::u2,
    pub length: type_alias::u2,
    pub index: type_alias::u2,
}

impl LocalvarTargetTableEntry {
    pub fn write<B: Write>(self, buff: &mut B) -> Result<usize, Error> {
        buff.write_u16::<BigEndian>(self.start_pc)?;
        buff.write_u16::<BigEndian>(self.length)?;
        buff.write_u16::<BigEndian>(self.index)?;
        Ok(size_of::<u16>() + size_of::<u16>() + size_of::<u16>())
    }
}

///```javadoc
/// Module_attribute {
///     type_alias::u2 attribute_name_index;
///     type_alias::u4 attribute_length;
///
///     type_alias::u2 module_name_index;
///     type_alias::u2 module_flags;
///     type_alias::u2 module_version_index;
///
///     type_alias::u2 requires_count;
///     {   type_alias::u2 requires_index;
///         type_alias::u2 requires_flags;
///         type_alias::u2 requires_version_index;
///     } requires[requires_count];
///
///     type_alias::u2 exports_count;
///     {   type_alias::u2 exports_index;
///         type_alias::u2 exports_flags;
///         type_alias::u2 exports_to_count;
///         type_alias::u2 exports_to_index[exports_to_count];
///     } exports[exports_count];
///
///     type_alias::u2 opens_count;
///     {   type_alias::u2 opens_index;
///         type_alias::u2 opens_flags;
///         type_alias::u2 opens_to_count;
///         type_alias::u2 opens_to_index[opens_to_count];
///     } opens[opens_count];
///
///     type_alias::u2 uses_count;
///     type_alias::u2 uses_index[uses_count];
///
///     type_alias::u2 provides_count;
///     {   type_alias::u2 provides_index;
///         type_alias::u2 provides_with_count;
///         type_alias::u2 provides_with_index[provides_with_count];
///     } provides[provides_count];
/// }
///```
///
///```javadoc
/// {   type_alias::u2 start_pc;
///     type_alias::u2 length;
///     type_alias::u2 name_index;
///     type_alias::u2 descriptor_index;
///     type_alias::u2 index;
/// } local_variable_table[local_variable_table_length];
///```
#[derive(Copy, Clone, Debug)]
pub struct RequiresEntry {
    pub requires_index: type_alias::u2,
    pub requires_flags: type_alias::u2,
    pub requires_version_index: type_alias::u2,
}

impl RequiresEntry {
    pub fn write<B: Write>(self, buff: &mut B) -> Result<usize, Error> {
        buff.write_u16::<BigEndian>(self.requires_index)?;
        buff.write_u16::<BigEndian>(self.requires_flags)?;
        buff.write_u16::<BigEndian>(self.requires_version_index)?;
        Ok(size_of::<u16>() + size_of::<u16>() + size_of::<u16>())
    }
}

///```javadoc
/// {   type_alias::u2 start_pc;
///     type_alias::u2 length;
///     type_alias::u2 name_index;
///     type_alias::u2 descriptor_index;
///     type_alias::u2 index;
/// } local_variable_table[local_variable_table_length];
///```
#[derive(Copy, Clone, Debug)]
pub struct ExportsEntry {
    pub exports_index: type_alias::u2,
    pub exports_flags: type_alias::u2,
    pub exports_to_count: type_alias::u2,
}

impl ExportsEntry {
    pub fn write<B: Write>(self, buff: &mut B) -> Result<usize, Error> {
        buff.write_u16::<BigEndian>(self.exports_index)?;
        buff.write_u16::<BigEndian>(self.exports_flags)?;
        buff.write_u16::<BigEndian>(self.exports_to_count)?;
        Ok(size_of::<u16>() + size_of::<u16>() + size_of::<u16>())
    }
}

///```javadoc
/// {   type_alias::u2 start_pc;
///     type_alias::u2 length;
///     type_alias::u2 name_index;
///     type_alias::u2 descriptor_index;
///     type_alias::u2 index;
/// } local_variable_table[local_variable_table_length];
///```
#[derive(Copy, Clone, Debug)]
pub struct OpensEntry {
    pub opens_index: type_alias::u2,
    pub opens_flags: type_alias::u2,
    pub opens_to_count: type_alias::u2,
}

impl OpensEntry {
    pub fn write<B: Write>(self, buff: &mut B) -> Result<usize, Error> {
        buff.write_u16::<BigEndian>(self.opens_index)?;
        buff.write_u16::<BigEndian>(self.opens_flags)?;
        buff.write_u16::<BigEndian>(self.opens_to_count)?;
        Ok(size_of::<u16>() + size_of::<u16>() + size_of::<u16>())
    }
}

///```javadoc
/// {   type_alias::u2 start_pc;
///     type_alias::u2 length;
///     type_alias::u2 name_index;
///     type_alias::u2 descriptor_index;
///     type_alias::u2 index;
/// } local_variable_table[local_variable_table_length];
///```
#[derive(Copy, Clone, Debug)]
pub struct ProvidesEntry {
    pub provides_index: type_alias::u2,
    pub provides_with_count: type_alias::u2,
}

impl ProvidesEntry {
    pub fn write<B: Write>(self, buff: &mut B) -> Result<usize, Error> {
        buff.write_u16::<BigEndian>(self.provides_index)?;
        buff.write_u16::<BigEndian>(self.provides_with_count)?;
        Ok(size_of::<u16>() + size_of::<u16>())
    }
}

#[derive(Copy, Clone, Debug)]
pub enum VerificationTypeInfo {
    TopVariableInfo {
        tag: VerificationTypeInfoItem, /* 0 */
    },
    IntegerVariableInfo {
        tag: VerificationTypeInfoItem, /* 1 */
    },
    FloatVariableInfo {
        tag: VerificationTypeInfoItem, /* 2 */
    },
    DoubleVariableInfo {
        tag: VerificationTypeInfoItem, /* 3 */
    },
    LongVariableInfo {
        tag: VerificationTypeInfoItem, /* 4 */
    },
    NullVariableInfo {
        tag: VerificationTypeInfoItem, /* 5 */
    },
    UninitializedThisVariableInfo {
        tag: VerificationTypeInfoItem, /* 6 */
    },
    ObjectVariableInfo {
        tag: VerificationTypeInfoItem, /* 7 */
        cpool_index: type_alias::u2,
    },
    UninitializedVariableInfo {
        tag: VerificationTypeInfoItem, /* 8 */
        offset: type_alias::u2,
    },
}

impl VerificationTypeInfo {
    pub fn write<B: Write>(self, buff: &mut B) -> Result<usize, Error> {
        match self {
            VerificationTypeInfo::TopVariableInfo { tag } => {
                buff.write_u8(tag as u8)?;
                Ok(size_of::<u8>())
            }
            VerificationTypeInfo::IntegerVariableInfo { tag } => {
                buff.write_u8(tag as u8)?;
                Ok(size_of::<u8>())
            }
            VerificationTypeInfo::FloatVariableInfo { tag } => {
                buff.write_u8(tag as u8)?;
                Ok(size_of::<u8>())
            }
            VerificationTypeInfo::DoubleVariableInfo { tag } => {
                buff.write_u8(tag as u8)?;
                Ok(size_of::<u8>())
            }
            VerificationTypeInfo::LongVariableInfo { tag } => {
                buff.write_u8(tag as u8)?;
                Ok(size_of::<u8>())
            }
            VerificationTypeInfo::NullVariableInfo { tag } => {
                buff.write_u8(tag as u8)?;
                Ok(size_of::<u8>())
            }
            VerificationTypeInfo::UninitializedThisVariableInfo { tag } => {
                buff.write_u8(tag as u8)?;
                Ok(size_of::<u8>())
            }
            VerificationTypeInfo::ObjectVariableInfo { tag, cpool_index } => {
                buff.write_u8(tag as u8)?;
                buff.write_u16::<BigEndian>(cpool_index)?;
                Ok(size_of::<u8>() + size_of::<u16>())
            }
            VerificationTypeInfo::UninitializedVariableInfo { tag, offset } => {
                buff.write_u8(tag as u8)?;
                buff.write_u16::<BigEndian>(offset)?;
                Ok(size_of::<u8>() + size_of::<u16>())
            }
        }
    }
}

impl TryFrom<u8> for VerificationTypeInfoItem {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(VerificationTypeInfoItem::ItemTop),
            1 => Ok(VerificationTypeInfoItem::ItemInteger),
            2 => Ok(VerificationTypeInfoItem::ItemFloat),
            3 => Ok(VerificationTypeInfoItem::ItemDouble),
            4 => Ok(VerificationTypeInfoItem::ItemLong),
            5 => Ok(VerificationTypeInfoItem::ItemNull),
            6 => Ok(VerificationTypeInfoItem::ItemUninitializedThis),
            7 => Ok(VerificationTypeInfoItem::ItemObject),
            8 => Ok(VerificationTypeInfoItem::ItemUninitialized),
            _ => Err(Error::new(
                ErrorKind::InvalidInput,
                format!("unknown verification type info tag: {value}"),
            )),
        }
    }
}

impl TryInto<u8> for VerificationTypeInfoItem {
    type Error = Error;

    fn try_into(self) -> Result<u8, Self::Error> {
        Ok(match self {
            VerificationTypeInfoItem::ItemTop => 0,
            VerificationTypeInfoItem::ItemInteger => 1,
            VerificationTypeInfoItem::ItemFloat => 2,
            VerificationTypeInfoItem::ItemDouble => 3,
            VerificationTypeInfoItem::ItemLong => 4,
            VerificationTypeInfoItem::ItemNull => 5,
            VerificationTypeInfoItem::ItemUninitializedThis => 6,
            VerificationTypeInfoItem::ItemObject => 7,
            VerificationTypeInfoItem::ItemUninitialized => 8,
        })
    }
}

impl<R> TryFrom<&mut BufReader<R>> for VerificationTypeInfo
where
    R: Read,
{
    type Error = Error;

    fn try_from(buff: &mut BufReader<R>) -> Result<Self, Self::Error> {
        let tag = VerificationTypeInfoItem::try_from(buff.read_u8()?)?;
        Ok(match tag {
            VerificationTypeInfoItem::ItemTop => VerificationTypeInfo::TopVariableInfo { tag },
            VerificationTypeInfoItem::ItemInteger => VerificationTypeInfo::IntegerVariableInfo { tag },
            VerificationTypeInfoItem::ItemFloat => VerificationTypeInfo::FloatVariableInfo { tag },
            VerificationTypeInfoItem::ItemDouble => VerificationTypeInfo::DoubleVariableInfo { tag },
            VerificationTypeInfoItem::ItemLong => VerificationTypeInfo::LongVariableInfo { tag },
            VerificationTypeInfoItem::ItemNull => VerificationTypeInfo::NullVariableInfo { tag },
            VerificationTypeInfoItem::ItemUninitializedThis => {
                VerificationTypeInfo::UninitializedThisVariableInfo { tag }
            }
            VerificationTypeInfoItem::ItemObject => VerificationTypeInfo::ObjectVariableInfo {
                tag,
                cpool_index: buff.read_u16::<BigEndian>()?,
            },
            VerificationTypeInfoItem::ItemUninitialized => VerificationTypeInfo::UninitializedVariableInfo {
                tag,
                offset: buff.read_u16::<BigEndian>()?,
            },
        })
    }
}

#[derive(Clone, Debug)]
pub enum StackMapFrame {
    /// 0-63
    SameFrame { frame_type: type_alias::u1 },
    /// 64-127
    SameLocals1StackItemFrame {
        frame_type: type_alias::u1,
        stack: Vec<VerificationTypeInfo>,
    },
    /// 247
    SameLocals1StackItemFrameExtended {
        frame_type: type_alias::u1,
        offset_delta: type_alias::u2,
        stack: Vec<VerificationTypeInfo>,
    },
    /// 248-250, k=251-frame_type
    ChopFrame {
        frame_type: type_alias::u1,
        offset_delta: type_alias::u2,
    },
    /// 251
    SameFrameExtended {
        frame_type: type_alias::u1,
        offset_delta: type_alias::u2,
    },
    /// 252-254
    AppendFrame {
        frame_type: type_alias::u1,
        offset_delta: type_alias::u2,
        /// len is frame_type-251
        locals: Vec<VerificationTypeInfo>,
    },
    /// 255
    FullFrame {
        frame_type: type_alias::u1,
        offset_delta: type_alias::u2,
        number_of_locals: type_alias::u2,
        locals: Vec<VerificationTypeInfo>,
        number_of_stack_items: type_alias::u2,
        stack: Vec<VerificationTypeInfo>,
    },
}

impl StackMapFrame {
    pub fn write<B: Write>(self, buff: &mut B) -> Result<usize, Error> {
        match self {
            StackMapFrame::SameFrame { frame_type } => {
                buff.write_u8(frame_type)?;
                Ok(size_of::<u8>())
            }
            StackMapFrame::SameLocals1StackItemFrame { frame_type, stack } => {
                buff.write_u8(frame_type)?;
                let mut bytes_written = size_of::<u8>();
                for entry in stack {
                    bytes_written += entry.write(buff)?;
                }
                Ok(bytes_written)
            }
            StackMapFrame::SameLocals1StackItemFrameExtended {
                frame_type,
                offset_delta,
                stack,
            } => {
                buff.write_u8(frame_type)?;
                buff.write_u16::<BigEndian>(offset_delta)?;
                let mut bytes_written = size_of::<u8>() + size_of::<u16>();
                for entry in stack {
                    bytes_written += entry.write(buff)?;
                }
                Ok(bytes_written)
            }
            StackMapFrame::ChopFrame {
                frame_type,
                offset_delta,
            } => {
                buff.write_u8(frame_type)?;
                buff.write_u16::<BigEndian>(offset_delta)?;
                Ok(size_of::<u8>() + size_of::<u16>())
            }
            StackMapFrame::SameFrameExtended {
                frame_type,
                offset_delta,
            } => {
                buff.write_u8(frame_type)?;
                buff.write_u16::<BigEndian>(offset_delta)?;
                Ok(size_of::<u8>() + size_of::<u16>())
            }
            StackMapFrame::AppendFrame {
                frame_type,
                offset_delta,
                locals,
            } => {
                buff.write_u8(frame_type)?;
                buff.write_u16::<BigEndian>(offset_delta)?;
                let mut bytes_written = size_of::<u8>() + size_of::<u16>();

                for one_local in locals {
                    bytes_written += one_local.write(buff)?;
                }
                Ok(bytes_written)
            }
            StackMapFrame::FullFrame {
                frame_type,
                offset_delta,
                number_of_locals,
                locals,
                number_of_stack_items,
                stack,
            } => {
                buff.write_u8(frame_type)?;
                buff.write_u16::<BigEndian>(offset_delta)?;
                buff.write_u16::<BigEndian>(number_of_locals)?;
                let mut bytes_written = size_of::<u8>() + size_of::<u16>() + size_of::<u16>();

                for one_local in locals {
                    bytes_written += one_local.write(buff)?;
                }

                buff.write_u16::<BigEndian>(number_of_stack_items)?;
                bytes_written += size_of::<u16>();

                for one_stack in stack {
                    bytes_written += one_stack.write(buff)?;
                }

                Ok(bytes_written)
            }
        }
    }
}

impl Debug for Attribute {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self))?;
        Ok(())
    }
}

impl Display for Attribute {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Attribute::Code { code, .. } => {
                let mut code_iter = code.iter();
                while let Some(&opcode_byte) = code_iter.next() {
                    if let Some(opcode) = OPCODES_MAP[opcode_byte as usize] {
                        let mut opcode_line = String::new();
                        opcode_line.push_str(opcode.opname);
                        for _ in 0..(opcode.oplen - 1) {
                            if let Some(opcode_param) = code_iter.next() {
                                opcode_line.push_str(format!(" {}", opcode_param).as_str());
                            }
                        }
                        opcode_line.push('\n');
                        f.write_str(opcode_line.as_str())?;
                    }
                }
            }
            Attribute::ConstantValue {
                attribute_name_index, ..
            } => {
                f.write_fmt(format_args!("{}", attribute_name_index))?;
            }
            Attribute::StackMapTable {
                attribute_name_index, ..
            } => {
                f.write_fmt(format_args!("{}", attribute_name_index))?;
            }
            Attribute::Exceptions {
                attribute_name_index, ..
            } => {
                f.write_fmt(format_args!("{}", attribute_name_index))?;
            }
            Attribute::InnerClasses {
                attribute_name_index, ..
            } => {
                f.write_fmt(format_args!("{}", attribute_name_index))?;
            }
            Attribute::EnclosingMethod {
                attribute_name_index, ..
            } => {
                f.write_fmt(format_args!("{}", attribute_name_index))?;
            }
            Attribute::Synthetic {
                attribute_name_index, ..
            } => {
                f.write_fmt(format_args!("{}", attribute_name_index))?;
            }
            Attribute::Signature {
                attribute_name_index, ..
            } => {
                f.write_fmt(format_args!("{}", attribute_name_index))?;
            }
            Attribute::SourceFile {
                attribute_name_index, ..
            } => {
                f.write_fmt(format_args!("{}", attribute_name_index))?;
            }
            Attribute::SourceDebugExtension {
                attribute_name_index, ..
            } => {
                f.write_fmt(format_args!("{}", attribute_name_index))?;
            }
            Attribute::LineNumberTable {
                attribute_name_index, ..
            } => {
                f.write_fmt(format_args!("{}", attribute_name_index))?;
            }
            Attribute::LocalVariableTable {
                attribute_name_index, ..
            } => {
                f.write_fmt(format_args!("{}", attribute_name_index))?;
            }
            Attribute::LocalVariableTypeTable {
                attribute_name_index, ..
            } => {
                f.write_fmt(format_args!("{}", attribute_name_index))?;
            }
            Attribute::Deprecated {
                attribute_name_index, ..
            } => {
                f.write_fmt(format_args!("{}", attribute_name_index))?;
            }
            Attribute::RuntimeVisibleAnnotations {
                attribute_name_index, ..
            } => {
                f.write_fmt(format_args!("{}", attribute_name_index))?;
            }
            Attribute::RuntimeInvisibleAnnotations {
                attribute_name_index, ..
            } => {
                f.write_fmt(format_args!("{}", attribute_name_index))?;
            }
            Attribute::RuntimeVisibleParameterAnnotations {
                attribute_name_index, ..
            } => {
                f.write_fmt(format_args!("{}", attribute_name_index))?;
            }
            Attribute::RuntimeInvisibleParameterAnnotations {
                attribute_name_index, ..
            } => {
                f.write_fmt(format_args!("{}", attribute_name_index))?;
            }
            Attribute::RuntimeVisibleTypeAnnotations {
                attribute_name_index, ..
            } => {
                f.write_fmt(format_args!("{}", attribute_name_index))?;
            }
            Attribute::RuntimeInvisibleTypeAnnotations {
                attribute_name_index, ..
            } => {
                f.write_fmt(format_args!("{}", attribute_name_index))?;
            }
            Attribute::AnnotationDefault {
                attribute_name_index, ..
            } => {
                f.write_fmt(format_args!("{}", attribute_name_index))?;
            }
            Attribute::BootstrapMethods {
                attribute_name_index, ..
            } => {
                f.write_fmt(format_args!("{}", attribute_name_index))?;
            }
            Attribute::MethodParameters {
                attribute_name_index, ..
            } => {
                f.write_fmt(format_args!("{}", attribute_name_index))?;
            }
            Attribute::Module {
                attribute_name_index, ..
            } => {
                f.write_fmt(format_args!("{}", attribute_name_index))?;
            }
            Attribute::ModulePackages {
                attribute_name_index, ..
            } => {
                f.write_fmt(format_args!("{}", attribute_name_index))?;
            }
            Attribute::ModuleMainClass {
                attribute_name_index, ..
            } => {
                f.write_fmt(format_args!("{}", attribute_name_index))?;
            }
            Attribute::NestHost {
                attribute_name_index, ..
            } => {
                f.write_fmt(format_args!("{}", attribute_name_index))?;
            }
            Attribute::NestMembers {
                attribute_name_index, ..
            } => {
                f.write_fmt(format_args!("{}", attribute_name_index))?;
            }
            Attribute::Record {
                attribute_name_index, ..
            } => {
                f.write_fmt(format_args!("{}", attribute_name_index))?;
            }
            Attribute::PermittedSubclasses {
                attribute_name_index, ..
            } => {
                f.write_fmt(format_args!("{}", attribute_name_index))?;
            }
            Attribute::ExternalAttribute {
                attribute_name_index, ..
            } => {
                f.write_fmt(format_args!("{}", attribute_name_index))?;
            }
        }
        Ok(())
    }
}

#[derive(Clone)]
pub enum Attribute {
    /// Oracle docs: https://docs.oracle.com/javase/specs/jvms/se22/html/jvms-4.html#jvms-4.7.2
    ConstantValue {
        attribute_name_index: type_alias::u2,
        attribute_length: type_alias::u4,
        constantvalue_index: type_alias::u2,
    },
    /// Oracle docs: https://docs.oracle.com/javase/specs/jvms/se22/html/jvms-4.html#jvms-4.7.3
    Code {
        attribute_name_index: type_alias::u2,
        attribute_length: type_alias::u4,
        max_stack: type_alias::u2,
        max_locals: type_alias::u2,
        code_length: type_alias::u4,
        code: Vec<type_alias::u1>,
        exception_table_length: type_alias::u2,
        exception_table: Vec<ExceptionTableEntry>,
        attributes_count: type_alias::u2,
        attributes: Vec<Attribute>,
    },
    /// Oracle docs: https://docs.oracle.com/javase/specs/jvms/se22/html/jvms-4.html#jvms-4.7.3
    StackMapTable {
        attribute_name_index: type_alias::u2,
        attribute_length: type_alias::u4,
        number_of_entries: type_alias::u2,
        entries: Vec<StackMapFrame>,
    },
    /// Oracle docs: https://docs.oracle.com/javase/specs/jvms/se22/html/jvms-4.html#jvms-4.7.5
    Exceptions {
        attribute_name_index: type_alias::u2,
        attribute_length: type_alias::u4,
        number_of_exceptions: type_alias::u2,
        exception_index_table: Vec<type_alias::u2>,
    },
    /// Oracle docs: https://docs.oracle.com/javase/specs/jvms/se22/html/jvms-4.html#jvms-4.7.6
    InnerClasses {
        attribute_name_index: type_alias::u2,
        attribute_length: type_alias::u4,
        number_of_classes: type_alias::u2,
        classes: Vec<InnerClassEntry>,
    },
    /// Oracle docs: https://docs.oracle.com/javase/specs/jvms/se22/html/jvms-4.html#jvms-4.7.7
    EnclosingMethod {
        attribute_name_index: type_alias::u2,
        attribute_length: type_alias::u4,
        class_index: type_alias::u2,
        method_index: type_alias::u2,
    },
    /// Oracle docs: https://docs.oracle.com/javase/specs/jvms/se22/html/jvms-4.html#jvms-4.7.8
    Synthetic {
        attribute_name_index: type_alias::u2,
        attribute_length: type_alias::u4,
    },
    /// Oracle docs: https://docs.oracle.com/javase/specs/jvms/se22/html/jvms-4.html#jvms-4.7.9
    Signature {
        attribute_name_index: type_alias::u2,
        attribute_length: type_alias::u4,
        signature_index: type_alias::u2,
    },
    /// Oracle docs: https://docs.oracle.com/javase/specs/jvms/se22/html/jvms-4.html#jvms-4.7.10
    SourceFile {
        attribute_name_index: type_alias::u2,
        attribute_length: type_alias::u4,
        sourcefile_index: type_alias::u2,
    },
    /// Oracle docs: https://docs.oracle.com/javase/specs/jvms/se22/html/jvms-4.html#jvms-4.7.11
    SourceDebugExtension {
        attribute_name_index: type_alias::u2,
        attribute_length: type_alias::u4,
        debug_extension: String,
    },
    /// Oracle docs: https://docs.oracle.com/javase/specs/jvms/se22/html/jvms-4.html#jvms-4.7.12
    LineNumberTable {
        attribute_name_index: type_alias::u2,
        attribute_length: type_alias::u4,
        line_number_table_length: type_alias::u2,
        line_number_table: Vec<LineNumberEntry>,
    },
    /// Oracle docs: https://docs.oracle.com/javase/specs/jvms/se22/html/jvms-4.html#jvms-4.7.13
    LocalVariableTable {
        attribute_name_index: type_alias::u2,
        attribute_length: type_alias::u4,
        local_variable_table_length: type_alias::u2,
        local_variable_table: Vec<LocalVariableTableEntry>,
    },
    /// Oracle docs: https://docs.oracle.com/javase/specs/jvms/se22/html/jvms-4.html#jvms-4.7.14
    LocalVariableTypeTable {
        attribute_name_index: type_alias::u2,
        attribute_length: type_alias::u4,
        local_variable_type_table_length: type_alias::u2,
        local_variable_type_table: Vec<LocalVariableTypeTableEntry>,
    },
    /// Oracle docs: https://docs.oracle.com/javase/specs/jvms/se22/html/jvms-4.html#jvms-4.7.15
    Deprecated {
        attribute_name_index: type_alias::u2,
        attribute_length: type_alias::u4,
    },
    /// Oracle docs: https://docs.oracle.com/javase/specs/jvms/se22/html/jvms-4.html#jvms-4.7.16
    RuntimeVisibleAnnotations {
        attribute_name_index: type_alias::u2,
        attribute_length: type_alias::u4,
        num_annotations: type_alias::u2,
        annotations: Vec<Annotation>,
    },
    /// Oracle docs: https://docs.oracle.com/javase/specs/jvms/se22/html/jvms-4.html#jvms-4.7.17
    RuntimeInvisibleAnnotations {
        attribute_name_index: type_alias::u2,
        attribute_length: type_alias::u4,
        num_annotations: type_alias::u2,
        annotations: Vec<Annotation>,
    },
    /// Oracle docs: https://docs.oracle.com/javase/specs/jvms/se22/html/jvms-4.html#jvms-4.7.18
    RuntimeVisibleParameterAnnotations {
        attribute_name_index: type_alias::u2,
        attribute_length: type_alias::u4,
        num_parameters: type_alias::u1,
        parameter_annotations: Vec<ParameterAnnotation>,
    },
    /// Oracle docs: https://docs.oracle.com/javase/specs/jvms/se22/html/jvms-4.html#jvms-4.7.19
    RuntimeInvisibleParameterAnnotations {
        attribute_name_index: type_alias::u2,
        attribute_length: type_alias::u4,
        num_parameters: type_alias::u1,
        parameter_annotations: Vec<ParameterAnnotation>,
    },
    /// Oracle docs: https://docs.oracle.com/javase/specs/jvms/se22/html/jvms-4.html#jvms-4.7.20
    RuntimeVisibleTypeAnnotations {
        attribute_name_index: type_alias::u2,
        attribute_length: type_alias::u4,
        num_parameters: type_alias::u2,
        annotations: Vec<TypeAnnotation>,
    },
    /// Oracle docs: https://docs.oracle.com/javase/specs/jvms/se22/html/jvms-4.html#jvms-4.7.21
    RuntimeInvisibleTypeAnnotations {
        attribute_name_index: type_alias::u2,
        attribute_length: type_alias::u4,
        num_parameters: type_alias::u2,
        annotations: Vec<TypeAnnotation>,
    },
    /// Oracle docs: https://docs.oracle.com/javase/specs/jvms/se22/html/jvms-4.html#jvms-4.7.22
    AnnotationDefault {
        attribute_name_index: type_alias::u2,
        attribute_length: type_alias::u4,
        default_value: ElementValue,
    },
    /// Oracle docs: https://docs.oracle.com/javase/specs/jvms/se22/html/jvms-4.html#jvms-4.7.23
    BootstrapMethods {
        attribute_name_index: type_alias::u2,
        attribute_length: type_alias::u4,
        num_bootstrap_methods: type_alias::u2,
        bootstrap_methods: Vec<BootstrapMethodEntry>,
    },
    /// Oracle docs: https://docs.oracle.com/javase/specs/jvms/se22/html/jvms-4.html#jvms-4.7.24
    MethodParameters {
        attribute_name_index: type_alias::u2,
        attribute_length: type_alias::u4,
        parameters_count: type_alias::u1,
        parameters: Vec<Parameter>,
    },
    /// Oracle docs: https://docs.oracle.com/javase/specs/jvms/se22/html/jvms-4.html#jvms-4.7.25
    Module {
        attribute_name_index: type_alias::u2,
        attribute_length: type_alias::u4,
        module_name_index: type_alias::u2,
        module_flags: type_alias::u2,
        module_version_index: type_alias::u2,
        requires_count: type_alias::u2,
        requires: Vec<RequiresEntry>,
        exports_count: type_alias::u2,
        exports: Vec<ExportsEntry>,
        opens_count: type_alias::u2,
        opens: Vec<OpensEntry>,
        uses_count: type_alias::u2,
        uses_index: Vec<type_alias::u2>,
        provides_count: type_alias::u2,
        provides: Vec<ProvidesEntry>,
    },
    /// Oracle docs: https://docs.oracle.com/javase/specs/jvms/se22/html/jvms-4.html#jvms-4.7.26
    ModulePackages {
        attribute_name_index: type_alias::u2,
        attribute_length: type_alias::u4,
        package_count: type_alias::u2,
        package_index: Vec<type_alias::u2>,
    },
    /// Oracle docs: https://docs.oracle.com/javase/specs/jvms/se22/html/jvms-4.html#jvms-4.7.27
    ModuleMainClass {
        attribute_name_index: type_alias::u2,
        attribute_length: type_alias::u4,
        main_class_index: type_alias::u2,
    },
    /// Oracle docs: https://docs.oracle.com/javase/specs/jvms/se22/html/jvms-4.html#jvms-4.7.28
    NestHost {
        attribute_name_index: type_alias::u2,
        attribute_length: type_alias::u4,
        host_class_index: type_alias::u2,
    },
    /// Oracle docs: https://docs.oracle.com/javase/specs/jvms/se22/html/jvms-4.html#jvms-4.7.29
    NestMembers {
        attribute_name_index: type_alias::u2,
        attribute_length: type_alias::u4,
        number_of_classes: type_alias::u2,
        classes: Vec<type_alias::u2>,
    },
    /// Oracle docs: https://docs.oracle.com/javase/specs/jvms/se22/html/jvms-4.html#jvms-4.7.30
    Record {
        attribute_name_index: type_alias::u2,
        attribute_length: type_alias::u4,
        components_count: type_alias::u2,
        components: Vec<RecordComponentInfo>,
    },
    /// Oracle docs: https://docs.oracle.com/javase/specs/jvms/se22/html/jvms-4.html#jvms-4.7.31
    PermittedSubclasses {
        attribute_name_index: type_alias::u2,
        attribute_length: type_alias::u4,
        number_of_classes: type_alias::u2,
        classes: Vec<type_alias::u2>,
    },
    /// Oracle docs: https://docs.oracle.com/javase/specs/jvms/se22/html/jvms-4.html#jvms-4.7.1
    ExternalAttribute {
        attribute_name_index: type_alias::u2,
        attribute_length: type_alias::u4,
        info: Vec<type_alias::u1>,
    },
}

impl Attribute {
    pub fn write<B: Write>(self, buff: &mut B) -> Result<usize, Error> {
        match self {
            Attribute::ConstantValue {
                attribute_name_index,
                attribute_length,
                constantvalue_index,
            } => {
                buff.write_u16::<BigEndian>(attribute_name_index)?;
                buff.write_u32::<BigEndian>(attribute_length)?;
                buff.write_u16::<BigEndian>(constantvalue_index)?;
                Ok(size_of::<u16>() + size_of::<u32>() + size_of::<u16>())
            }
            Attribute::Code {
                attribute_name_index,
                attribute_length,
                max_stack,
                max_locals,
                code_length,
                code,
                exception_table_length,
                exception_table,
                attributes_count,
                attributes,
            } => {
                buff.write_u16::<BigEndian>(attribute_name_index)?;
                buff.write_u32::<BigEndian>(attribute_length)?;
                buff.write_u16::<BigEndian>(max_stack)?;
                buff.write_u16::<BigEndian>(max_locals)?;
                buff.write_u32::<BigEndian>(code_length)?;
                let mut bytes_written =
                    size_of::<u16>() + size_of::<u32>() + size_of::<u16>() + size_of::<u16>() + size_of::<u32>();

                bytes_written += buff.write(code.as_slice())?;

                buff.write_u16::<BigEndian>(exception_table_length)?;
                bytes_written += size_of::<u16>();
                for entry in exception_table {
                    bytes_written += entry.write(buff)?
                }

                buff.write_u16::<BigEndian>(attributes_count)?;
                bytes_written += size_of::<u16>();
                for attr in attributes {
                    bytes_written += attr.clone().write(buff)?
                }
                Ok(bytes_written)
            }
            Attribute::StackMapTable {
                attribute_name_index,
                attribute_length,
                number_of_entries,
                entries,
            } => {
                buff.write_u16::<BigEndian>(attribute_name_index)?;
                buff.write_u32::<BigEndian>(attribute_length)?;
                buff.write_u16::<BigEndian>(number_of_entries)?;
                let mut bytes_written = size_of::<u16>() + size_of::<u32>() + size_of::<u16>();
                for frame in entries {
                    bytes_written += frame.write(buff)?
                }
                Ok(bytes_written)
            }
            Attribute::Exceptions {
                attribute_name_index,
                attribute_length,
                number_of_exceptions,
                ref exception_index_table,
            } => {
                buff.write_u16::<BigEndian>(attribute_name_index)?;
                buff.write_u32::<BigEndian>(attribute_length)?;
                buff.write_u16::<BigEndian>(number_of_exceptions)?;
                let mut bytes_written = size_of::<u16>() + size_of::<u32>() + size_of::<u16>();

                bytes_written += write_vec_u16_as_bytes(exception_index_table, buff)?;
                Ok(bytes_written)
            }
            Attribute::InnerClasses {
                attribute_name_index,
                attribute_length,
                number_of_classes,
                classes,
            } => {
                buff.write_u16::<BigEndian>(attribute_name_index)?;
                buff.write_u32::<BigEndian>(attribute_length)?;
                buff.write_u16::<BigEndian>(number_of_classes)?;
                let mut bytes_written = size_of::<u16>() + size_of::<u32>() + size_of::<u16>();
                for one_class in classes {
                    bytes_written += one_class.write(buff)?
                }
                Ok(bytes_written)
            }
            Attribute::EnclosingMethod {
                attribute_name_index,
                attribute_length,
                class_index,
                method_index,
            } => {
                buff.write_u16::<BigEndian>(attribute_name_index)?;
                buff.write_u32::<BigEndian>(attribute_length)?;
                buff.write_u16::<BigEndian>(class_index)?;
                buff.write_u16::<BigEndian>(method_index)?;
                Ok(size_of::<u16>() + size_of::<u32>() + size_of::<u16>() + size_of::<u16>())
            }
            Attribute::Synthetic {
                attribute_name_index,
                attribute_length,
            } => {
                buff.write_u16::<BigEndian>(attribute_name_index)?;
                buff.write_u32::<BigEndian>(attribute_length)?;
                Ok(size_of::<u16>() + size_of::<u32>())
            }
            Attribute::Signature {
                attribute_name_index,
                attribute_length,
                signature_index,
            } => {
                buff.write_u16::<BigEndian>(attribute_name_index)?;
                buff.write_u32::<BigEndian>(attribute_length)?;
                buff.write_u16::<BigEndian>(signature_index)?;
                Ok(size_of::<u16>() + size_of::<u32>() + size_of::<u16>())
            }
            Attribute::SourceFile {
                attribute_name_index,
                attribute_length,
                sourcefile_index,
            } => {
                buff.write_u16::<BigEndian>(attribute_name_index)?;
                buff.write_u32::<BigEndian>(attribute_length)?;
                buff.write_u16::<BigEndian>(sourcefile_index)?;
                Ok(size_of::<u16>() + size_of::<u32>() + size_of::<u16>())
            }
            Attribute::SourceDebugExtension {
                attribute_name_index,
                attribute_length,
                debug_extension,
            } => {
                buff.write_u16::<BigEndian>(attribute_name_index)?;
                buff.write_u32::<BigEndian>(attribute_length)?;
                let mut bytes_written = size_of::<u16>() + size_of::<u32>();
                bytes_written += buff.write(debug_extension.as_bytes())?;
                Ok(bytes_written)
            }
            Attribute::LineNumberTable {
                attribute_name_index,
                attribute_length,
                line_number_table_length,
                line_number_table,
            } => {
                buff.write_u16::<BigEndian>(attribute_name_index)?;
                buff.write_u32::<BigEndian>(attribute_length)?;
                buff.write_u16::<BigEndian>(line_number_table_length)?;
                let mut bytes_written = size_of::<u16>() + size_of::<u32>() + size_of::<u16>();
                for entry in line_number_table {
                    bytes_written += entry.write(buff)?
                }
                Ok(bytes_written)
            }
            Attribute::LocalVariableTable {
                attribute_name_index,
                attribute_length,
                local_variable_table_length,
                local_variable_table,
            } => {
                buff.write_u16::<BigEndian>(attribute_name_index)?;
                buff.write_u32::<BigEndian>(attribute_length)?;
                buff.write_u16::<BigEndian>(local_variable_table_length)?;
                let mut bytes_written = size_of::<u16>() + size_of::<u32>() + size_of::<u16>();
                for entry in local_variable_table {
                    bytes_written += entry.write(buff)?
                }
                Ok(bytes_written)
            }
            Attribute::LocalVariableTypeTable {
                attribute_name_index,
                attribute_length,
                local_variable_type_table_length,
                local_variable_type_table,
            } => {
                buff.write_u16::<BigEndian>(attribute_name_index)?;
                buff.write_u32::<BigEndian>(attribute_length)?;
                buff.write_u16::<BigEndian>(local_variable_type_table_length)?;
                let mut bytes_written = size_of::<u16>() + size_of::<u32>() + size_of::<u16>();
                for entry in local_variable_type_table {
                    bytes_written += entry.write(buff)?
                }
                Ok(bytes_written)
            }
            Attribute::Deprecated {
                attribute_name_index,
                attribute_length,
            } => {
                buff.write_u16::<BigEndian>(attribute_name_index)?;
                buff.write_u32::<BigEndian>(attribute_length)?;
                Ok(size_of::<u16>() + size_of::<u32>())
            }
            Attribute::RuntimeVisibleAnnotations {
                attribute_name_index,
                attribute_length,
                num_annotations,
                annotations,
            } => {
                buff.write_u16::<BigEndian>(attribute_name_index)?;
                buff.write_u32::<BigEndian>(attribute_length)?;
                buff.write_u16::<BigEndian>(num_annotations)?;
                let mut bytes_written = size_of::<u16>() + size_of::<u32>() + size_of::<u16>();
                for one_annotation in annotations {
                    bytes_written += one_annotation.write(buff)?;
                }
                Ok(bytes_written)
            }
            Attribute::RuntimeInvisibleAnnotations {
                attribute_name_index,
                attribute_length,
                num_annotations,
                annotations,
            } => {
                buff.write_u16::<BigEndian>(attribute_name_index)?;
                buff.write_u32::<BigEndian>(attribute_length)?;
                buff.write_u16::<BigEndian>(num_annotations)?;
                let mut bytes_written = size_of::<u16>() + size_of::<u32>() + size_of::<u16>();
                for one_annotation in annotations {
                    bytes_written += one_annotation.write(buff)?;
                }
                Ok(bytes_written)
            }
            Attribute::RuntimeVisibleParameterAnnotations {
                attribute_name_index,
                attribute_length,
                num_parameters,
                parameter_annotations,
            } => {
                buff.write_u16::<BigEndian>(attribute_name_index)?;
                buff.write_u32::<BigEndian>(attribute_length)?;
                buff.write_u8(num_parameters)?;
                let mut bytes_written = size_of::<u16>() + size_of::<u32>() + size_of::<u8>();
                for one_annotation in parameter_annotations {
                    bytes_written += one_annotation.write(buff)?;
                }
                Ok(bytes_written)
            }
            Attribute::RuntimeInvisibleParameterAnnotations {
                attribute_name_index,
                attribute_length,
                num_parameters,
                parameter_annotations,
            } => {
                buff.write_u16::<BigEndian>(attribute_name_index)?;
                buff.write_u32::<BigEndian>(attribute_length)?;
                buff.write_u8(num_parameters)?;
                let mut bytes_written = size_of::<u16>() + size_of::<u32>() + size_of::<u8>();
                for one_annotation in parameter_annotations {
                    bytes_written += one_annotation.write(buff)?;
                }
                Ok(bytes_written)
            }
            Attribute::RuntimeVisibleTypeAnnotations {
                attribute_name_index,
                attribute_length,
                num_parameters,
                annotations,
            } => {
                buff.write_u16::<BigEndian>(attribute_name_index)?;
                buff.write_u32::<BigEndian>(attribute_length)?;
                buff.write_u16::<BigEndian>(num_parameters)?;
                let mut bytes_written = size_of::<u16>() + size_of::<u32>() + size_of::<u16>();
                for one_annotation in annotations {
                    bytes_written += one_annotation.write(buff)?;
                }
                Ok(bytes_written)
            }
            Attribute::RuntimeInvisibleTypeAnnotations {
                attribute_name_index,
                attribute_length,
                num_parameters,
                annotations,
            } => {
                buff.write_u16::<BigEndian>(attribute_name_index)?;
                buff.write_u32::<BigEndian>(attribute_length)?;
                buff.write_u16::<BigEndian>(num_parameters)?;
                let mut bytes_written = size_of::<u16>() + size_of::<u32>() + size_of::<u16>();
                for one_annotation in annotations {
                    bytes_written += one_annotation.write(buff)?;
                }
                Ok(bytes_written)
            }
            Attribute::AnnotationDefault {
                attribute_name_index,
                attribute_length,
                default_value,
            } => {
                buff.write_u16::<BigEndian>(attribute_name_index)?;
                buff.write_u32::<BigEndian>(attribute_length)?;
                let mut bytes_written = size_of::<u16>() + size_of::<u32>();
                bytes_written += default_value.write(buff)?;
                Ok(bytes_written)
            }
            Attribute::BootstrapMethods {
                attribute_name_index,
                attribute_length,
                num_bootstrap_methods,
                bootstrap_methods,
            } => {
                buff.write_u16::<BigEndian>(attribute_name_index)?;
                buff.write_u32::<BigEndian>(attribute_length)?;
                buff.write_u16::<BigEndian>(num_bootstrap_methods)?;
                let mut bytes_written = size_of::<u16>() + size_of::<u32>() + size_of::<u16>();
                for one_method in bootstrap_methods {
                    bytes_written += one_method.write(buff)?;
                }
                Ok(bytes_written)
            }
            Attribute::MethodParameters {
                attribute_name_index,
                attribute_length,
                parameters_count,
                parameters,
            } => {
                buff.write_u16::<BigEndian>(attribute_name_index)?;
                buff.write_u32::<BigEndian>(attribute_length)?;
                buff.write_u8(parameters_count)?;
                let mut bytes_written = size_of::<u16>() + size_of::<u32>() + size_of::<u8>();
                for one_parameter in parameters {
                    bytes_written += one_parameter.write(buff)?;
                }
                Ok(bytes_written)
            }
            Attribute::Module {
                attribute_name_index,
                attribute_length,
                module_name_index,
                module_flags,
                module_version_index,
                requires_count,
                requires,
                exports_count,
                exports,
                opens_count,
                opens,
                uses_count,
                ref uses_index,
                provides_count,
                provides,
            } => {
                buff.write_u16::<BigEndian>(attribute_name_index)?;
                buff.write_u32::<BigEndian>(attribute_length)?;
                buff.write_u16::<BigEndian>(module_name_index)?;
                buff.write_u16::<BigEndian>(module_flags)?;
                buff.write_u16::<BigEndian>(module_version_index)?;
                let mut bytes_written =
                    size_of::<u16>() + size_of::<u32>() + size_of::<u16>() + size_of::<u16>() + size_of::<u16>();

                buff.write_u16::<BigEndian>(requires_count)?;
                bytes_written += size_of::<u16>();
                for one_require in requires {
                    bytes_written += one_require.write(buff)?;
                }

                buff.write_u16::<BigEndian>(exports_count)?;
                bytes_written += size_of::<u16>();
                for one_export in exports {
                    bytes_written += one_export.write(buff)?;
                }

                buff.write_u16::<BigEndian>(opens_count)?;
                bytes_written += size_of::<u16>();
                for one_open in opens {
                    bytes_written += one_open.write(buff)?;
                }

                buff.write_u16::<BigEndian>(uses_count)?;
                bytes_written += size_of::<u16>();
                bytes_written += write_vec_u16_as_bytes(uses_index, buff)?;

                buff.write_u16::<BigEndian>(provides_count)?;
                bytes_written += size_of::<u16>();
                for one_provide in provides {
                    bytes_written += one_provide.write(buff)?;
                }
                Ok(bytes_written)
            }
            Attribute::ModulePackages {
                attribute_name_index,
                attribute_length,
                package_count,
                ref package_index,
            } => {
                buff.write_u16::<BigEndian>(attribute_name_index)?;
                buff.write_u32::<BigEndian>(attribute_length)?;
                buff.write_u16::<BigEndian>(package_count)?;
                let mut bytes_written = size_of::<u16>() + size_of::<u32>() + size_of::<u16>();
                bytes_written += write_vec_u16_as_bytes(package_index, buff)?;
                Ok(bytes_written)
            }
            Attribute::ModuleMainClass {
                attribute_name_index,
                attribute_length,
                main_class_index,
            } => {
                buff.write_u16::<BigEndian>(attribute_name_index)?;
                buff.write_u32::<BigEndian>(attribute_length)?;
                buff.write_u16::<BigEndian>(main_class_index)?;
                Ok(size_of::<u16>() + size_of::<u32>() + size_of::<u16>())
            }
            Attribute::NestHost {
                attribute_name_index,
                attribute_length,
                host_class_index,
            } => {
                buff.write_u16::<BigEndian>(attribute_name_index)?;
                buff.write_u32::<BigEndian>(attribute_length)?;
                buff.write_u16::<BigEndian>(host_class_index)?;
                Ok(size_of::<u16>() + size_of::<u32>() + size_of::<u16>())
            }
            Attribute::NestMembers {
                attribute_name_index,
                attribute_length,
                number_of_classes,
                ref classes,
            } => {
                buff.write_u16::<BigEndian>(attribute_name_index)?;
                buff.write_u32::<BigEndian>(attribute_length)?;
                buff.write_u16::<BigEndian>(number_of_classes)?;
                let mut bytes_written = size_of::<u16>() + size_of::<u32>() + size_of::<u16>();
                bytes_written += write_vec_u16_as_bytes(classes, buff)?;
                Ok(bytes_written)
            }
            Attribute::Record {
                attribute_name_index,
                attribute_length,
                components_count,
                components,
            } => {
                buff.write_u16::<BigEndian>(attribute_name_index)?;
                buff.write_u32::<BigEndian>(attribute_length)?;
                buff.write_u16::<BigEndian>(components_count)?;
                let mut bytes_written = size_of::<u16>() + size_of::<u32>() + size_of::<u16>();
                for one_component_info in components {
                    bytes_written += one_component_info.write(buff)?;
                }
                Ok(bytes_written)
            }
            Attribute::PermittedSubclasses {
                attribute_name_index,
                attribute_length,
                number_of_classes,
                ref classes,
            } => {
                buff.write_u16::<BigEndian>(attribute_name_index)?;
                buff.write_u32::<BigEndian>(attribute_length)?;
                buff.write_u16::<BigEndian>(number_of_classes)?;
                let mut bytes_written = size_of::<u16>() + size_of::<u32>() + size_of::<u16>();
                bytes_written += write_vec_u16_as_bytes(classes, buff)?;
                Ok(bytes_written)
            }
            Attribute::ExternalAttribute {
                attribute_name_index,
                attribute_length,
                info: data,
            } => {
                buff.write_u16::<BigEndian>(attribute_name_index)?;
                buff.write_u32::<BigEndian>(attribute_length)?;
                let mut bytes_written = size_of::<u16>() + size_of::<u32>();
                bytes_written += buff.write(&data)?;
                Ok(bytes_written)
            }
        }
    }
}

fn write_vec_u16_as_bytes<B: Write>(vec: &[type_alias::u2], buff: &mut B) -> Result<usize, Error> {
    unsafe {
        buff.write(
            vec.iter()
                .map(|e| e.to_be())
                .collect::<Box<[type_alias::u2]>>()
                .align_to::<u8>()
                .1,
        )
    }
}

impl TryInto<Vec<u8>> for Attribute {
    type Error = Error;

    fn try_into(self) -> Result<Vec<u8>, Self::Error> {
        let mut buff = Vec::new();
        self.write(&mut buff)?;
        Ok(buff)
    }
}
