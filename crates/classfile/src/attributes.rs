use std::io::{BufReader, Error, ErrorKind, Read, Write};

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use zip::read::ZipFile;

use crate::access_flags::AccessFlags;
use crate::type_alias;

///```javadoc
/// {   type_alias::u2 start_pc;
///     type_alias::u2 end_pc;
///     type_alias::u2 handler_pc;
///     type_alias::u2 catch_type;
/// } exception_table[exception_table_length];
///```
#[derive(Clone, Debug)]
pub struct ExceptionTableEntry {
    pub start_pc: type_alias::u2,
    pub end_pc: type_alias::u2,
    pub handler_pc: type_alias::u2,
    pub catch_type: type_alias::u2,
}

impl ExceptionTableEntry {
    pub fn write(self, buff: &mut Vec<u8>) -> Result<(), Error> {
        buff.write_u16::<BigEndian>(self.start_pc)?;
        buff.write_u16::<BigEndian>(self.end_pc)?;
        buff.write_u16::<BigEndian>(self.handler_pc)?;
        buff.write_u16::<BigEndian>(self.catch_type)?;
        Ok(())
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
pub struct LineNumberEntry {
    pub start_pc: type_alias::u2,
    pub line_number: type_alias::u2,
}

impl LineNumberEntry {
    pub fn write(self, buff: &mut Vec<u8>) -> Result<(), Error> {
        buff.write_u16::<BigEndian>(self.start_pc)?;
        buff.write_u16::<BigEndian>(self.line_number)?;
        Ok(())
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
pub struct LocalVariableTableEntry {
    pub start_pc: type_alias::u2,
    pub length: type_alias::u2,
    pub name_index: type_alias::u2,
    pub descriptor_index: type_alias::u2,
    pub index: type_alias::u2,
}

impl LocalVariableTableEntry {
    pub fn write(self, buff: &mut Vec<u8>) -> Result<(), Error> {
        buff.write_u16::<BigEndian>(self.start_pc)?;
        buff.write_u16::<BigEndian>(self.length)?;
        buff.write_u16::<BigEndian>(self.name_index)?;
        buff.write_u16::<BigEndian>(self.descriptor_index)?;
        buff.write_u16::<BigEndian>(self.index)?;
        Ok(())
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
#[derive(Clone, Debug)]
pub struct LocalVariableTypeTableEntry {
    pub start_pc: type_alias::u2,
    pub length: type_alias::u2,
    pub name_index: type_alias::u2,
    pub signature_index: type_alias::u2,
    pub index: type_alias::u2,
}

impl LocalVariableTypeTableEntry {
    pub fn write(self, buff: &mut Vec<u8>) -> Result<(), Error> {
        buff.write_u16::<BigEndian>(self.start_pc)?;
        buff.write_u16::<BigEndian>(self.length)?;
        buff.write_u16::<BigEndian>(self.name_index)?;
        buff.write_u16::<BigEndian>(self.signature_index)?;
        buff.write_u16::<BigEndian>(self.index)?;
        Ok(())
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
    pub unsafe fn write(self, buff: &mut Vec<u8>) -> Result<(), Error> {
        buff.write_u16::<BigEndian>(self.bootstrap_method_ref)?;
        buff.write_u16::<BigEndian>(self.num_bootstrap_arguments)?;
        buff.write(self.bootstrap_arguments.iter().map(|e| { e.to_be() }).collect::<Vec<type_alias::u2>>().align_to::<u8>().1)?;
        Ok(())
    }
}

///```javadoc
/// {   type_alias::u2 inner_class_info_index;
///     type_alias::u2 outer_class_info_index;
///     type_alias::u2 inner_name_index;
///     type_alias::u2 inner_class_access_flags;
/// } classes[number_of_classes];
/// ```
#[derive(Clone, Debug)]
pub struct InnerClassEntry {
    pub inner_class_info_index: type_alias::u2,
    pub outer_class_info_index: type_alias::u2,
    pub inner_name_index: type_alias::u2,
    pub inner_class_access_flags: type_alias::u2,
}

impl InnerClassEntry {
    pub fn write(self, buff: &mut Vec<u8>) -> Result<(), Error> {
        buff.write_u16::<BigEndian>(self.inner_class_info_index)?;
        buff.write_u16::<BigEndian>(self.outer_class_info_index)?;
        buff.write_u16::<BigEndian>(self.inner_name_index)?;
        buff.write_u16::<BigEndian>(self.inner_class_access_flags)?;
        Ok(())
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
    pub fn write(self, buff: &mut Vec<u8>) -> Result<(), Error> {
        buff.write_u16::<BigEndian>(self.name_index)?;
        buff.write_u16::<BigEndian>(self.descriptor_index)?;
        buff.write_u16::<BigEndian>(self.attributes_count)?;
        for attr in self.attributes {
            attr.write(buff)?;
        }
        Ok(())
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
    pub fn write(self, buff: &mut Vec<u8>) -> Result<(), Error> {
        buff.write_u16::<BigEndian>(self.type_index)?;
        buff.write_u16::<BigEndian>(self.num_element_value_pairs)?;
        for one_pair in self.element_value_pairs {
            one_pair.write(buff)?
        }
        Ok(())
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
    pub fn write(self, buff: &mut Vec<u8>) -> Result<(), Error> {
        buff.write_u16::<BigEndian>(self.element_name_index)?;
        self.value.write(buff)?;
        Ok(())
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
    pub fn write(self, buff: &mut Vec<u8>) -> Result<(), Error> {
        buff.write_u8(self.tag)?;
        self.value.write(buff)?;

        Ok(())
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
/// | tag Item  | Type	                | value Item            | Constant Type     |
/// |-----------|-----------------------|-----------------------|-------------------|
/// | B	        | byte	                | const_value_index     | CONSTANT_Integer  |
/// | C	        | char	                | const_value_index     | CONSTANT_Integer  |
/// | D	        | double	            | const_value_index     | CONSTANT_Double   |
/// | F	        | float	                | const_value_index     | CONSTANT_Float    |
/// | I	        | int	                | const_value_index     | CONSTANT_Integer  |
/// | J	        | long	                | const_value_index     | CONSTANT_Long     |
/// | S	        | short	                | const_value_index     | CONSTANT_Integer  |
/// | Z	        | boolean	            | const_value_index     | CONSTANT_Integer  |
/// | s	        | String	            | const_value_index     | CONSTANT_Utf8     |
/// | e	        | Enum class  	        | enum_const_value      | Not applicable    |
/// | c	        | Class	                | class_info_index      | Not applicable    |
/// | @	        | Annotation interface	| annotation_value      | Not applicable    |
/// | [	        | Array type  	        | array_value           | Not applicable    |
#[derive(Clone, Debug)]
pub enum Value {
    ConstValueIndex { const_value_index: type_alias::u2 },
    EnumConstValue { type_name_index: type_alias::u2, const_name_index: type_alias::u2 },
    ClassInfoIndex { class_info_index: type_alias::u2 },
    AnnotationValue { annotation_value: Annotation },
    ArrayValue { num_values: type_alias::u2, values: Vec<ElementValue> },
}

impl Value {
    pub fn write(self, buff: &mut Vec<u8>) -> Result<(), Error> {
        match self {
            Value::ConstValueIndex { const_value_index } => { buff.write_u16::<BigEndian>(const_value_index)?; }
            Value::EnumConstValue { type_name_index, const_name_index } => {
                buff.write_u16::<BigEndian>(type_name_index)?;
                buff.write_u16::<BigEndian>(const_name_index)?;
            }
            Value::ClassInfoIndex { class_info_index } => { buff.write_u16::<BigEndian>(class_info_index)?; }
            Value::AnnotationValue { annotation_value } => { annotation_value.write(buff)? }
            Value::ArrayValue { num_values, values } => {
                buff.write_u16::<BigEndian>(num_values)?;
                for one_value in values {
                    one_value.write(buff)?
                }
            }
        }
        Ok(())
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
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct Parameter {
    pub name_index: type_alias::u2,
    pub access_flags: AccessFlags,
}

impl Parameter {
    pub fn write(self, buff: &mut Vec<u8>) -> Result<(), Error> {
        buff.write_u16::<BigEndian>(self.name_index)?;
        buff.write_u16::<BigEndian>(self.access_flags.into())?;
        Ok(())
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
    pub fn write(self, buff: &mut Vec<u8>) -> Result<(), Error> {
        buff.write_u16::<BigEndian>(self.num_annotations)?;
        for one_ann in self.annotations {
            one_ann.write(buff)?
        }
        Ok(())
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
    pub fn write(self, buff: &mut Vec<u8>) -> Result<(), Error> {
        buff.write_u8(self.target_type)?;
        self.target_info.write(buff)?;
        self.target_path.write(buff)?;
        buff.write_u16::<BigEndian>(self.type_index)?;
        buff.write_u16::<BigEndian>(self.num_element_value_pairs)?;
        for entry in self.element_value_pairs {
            entry.write(buff)?;
        }
        Ok(())
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
pub struct PathEntry {
    pub type_path_kind: type_alias::u1,
    pub type_argument_index: type_alias::u1,
}

impl PathEntry {
    pub fn write(self, buff: &mut Vec<u8>) -> Result<(), Error> {
        buff.write_u8(self.type_path_kind)?;
        buff.write_u8(self.type_argument_index)?;
        Ok(())
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
    pub fn write(self, buff: &mut Vec<u8>) -> Result<(), Error> {
        buff.write_u8(self.path_length)?;
        for one_path in self.path {
            one_path.write(buff)?;
        }
        Ok(())
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
    TypeParameterTarget { type_parameter_index: type_alias::u1 },
    SupertypeTarget { supertype_index: type_alias::u2 },
    TypeParameterBoundTarget { type_parameter_index: type_alias::u1, bound_index: type_alias::u1 },
    EmptyTarget {},
    FormalParameterTarget { formal_parameter_index: type_alias::u1 },
    ThrowsTarget { throws_type_index: type_alias::u2 },
    LocalvarTarget { table_length: type_alias::u2, table: Vec<LocalvarTargetTableEntry> },
    CatchTarget { exception_table_index: type_alias::u2 },
    OffsetTarget { offset: type_alias::u2 },
    TypeArgumentTarget { offset: type_alias::u2, type_argument_index: type_alias::u1 },
}

impl TargetInfo {
    pub fn write(self, buff: &mut Vec<u8>) -> Result<(), Error> {
        match self {
            TargetInfo::TypeParameterTarget { type_parameter_index } => { buff.write_u8(type_parameter_index)? }
            TargetInfo::SupertypeTarget { supertype_index } => { buff.write_u16::<BigEndian>(supertype_index)? }
            TargetInfo::TypeParameterBoundTarget { type_parameter_index, bound_index } => {
                buff.write_u8(type_parameter_index)?;
                buff.write_u8(bound_index)?;
            }
            TargetInfo::EmptyTarget {} => {}
            TargetInfo::FormalParameterTarget { formal_parameter_index } => { buff.write_u8(formal_parameter_index)? }
            TargetInfo::ThrowsTarget { throws_type_index } => { buff.write_u16::<BigEndian>(throws_type_index)? }
            TargetInfo::LocalvarTarget { table_length, table } => {
                buff.write_u16::<BigEndian>(table_length)?;
                for entry in table {
                    entry.write(buff)?;
                }
            }
            TargetInfo::CatchTarget { exception_table_index } => {
                buff.write_u16::<BigEndian>(exception_table_index)?
            }
            TargetInfo::OffsetTarget { offset } => { buff.write_u16::<BigEndian>(offset)? }
            TargetInfo::TypeArgumentTarget { offset, type_argument_index } => {
                buff.write_u16::<BigEndian>(offset)?;
                buff.write_u8(type_argument_index)?;
            }
        }
        Ok(())
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
pub struct LocalvarTargetTableEntry {
    pub start_pc: type_alias::u2,
    pub length: type_alias::u2,
    pub index: type_alias::u2,
}

impl LocalvarTargetTableEntry {
    pub fn write(self, buff: &mut Vec<u8>) -> Result<(), Error> {
        buff.write_u16::<BigEndian>(self.start_pc)?;
        buff.write_u16::<BigEndian>(self.length)?;
        buff.write_u16::<BigEndian>(self.index)?;
        Ok(())
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

///```javadoc
/// {   type_alias::u2 start_pc;
///     type_alias::u2 length;
///     type_alias::u2 name_index;
///     type_alias::u2 descriptor_index;
///     type_alias::u2 index;
/// } local_variable_table[local_variable_table_length];
///```
#[derive(Clone, Debug)]
pub struct RequiresEntry {
    pub requires_index: type_alias::u2,
    pub requires_flags: type_alias::u2,
    pub requires_version_index: type_alias::u2,
}

impl RequiresEntry {
    pub fn write(self, buff: &mut Vec<u8>) -> Result<(), Error> {
        buff.write_u16::<BigEndian>(self.requires_index)?;
        buff.write_u16::<BigEndian>(self.requires_flags)?;
        buff.write_u16::<BigEndian>(self.requires_version_index)?;
        Ok(())
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
pub struct ExportsEntry {
    pub exports_index: type_alias::u2,
    pub exports_flags: type_alias::u2,
    pub exports_to_count: type_alias::u2,
}

impl ExportsEntry {
    pub fn write(self, buff: &mut Vec<u8>) -> Result<(), Error> {
        buff.write_u16::<BigEndian>(self.exports_index)?;
        buff.write_u16::<BigEndian>(self.exports_flags)?;
        buff.write_u16::<BigEndian>(self.exports_to_count)?;
        Ok(())
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
pub struct OpensEntry {
    pub opens_index: type_alias::u2,
    pub opens_flags: type_alias::u2,
    pub opens_to_count: type_alias::u2,
}

impl OpensEntry {
    pub fn write(self, buff: &mut Vec<u8>) -> Result<(), Error> {
        buff.write_u16::<BigEndian>(self.opens_index)?;
        buff.write_u16::<BigEndian>(self.opens_flags)?;
        buff.write_u16::<BigEndian>(self.opens_to_count)?;
        Ok(())
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
pub struct ProvidesEntry {
    pub provides_index: type_alias::u2,
    pub provides_with_count: type_alias::u2,
}

impl ProvidesEntry {
    pub fn write(self, buff: &mut Vec<u8>) -> Result<(), Error> {
        buff.write_u16::<BigEndian>(self.provides_index)?;
        buff.write_u16::<BigEndian>(self.provides_with_count)?;
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub enum VerificationTypeInfo {
    TopVariableInfo {
        tag: VerificationTypeInfoItem /* 0 */,
    },
    IntegerVariableInfo {
        tag: VerificationTypeInfoItem /* 1 */,
    },
    FloatVariableInfo {
        tag: VerificationTypeInfoItem /* 2 */,
    },
    DoubleVariableInfo {
        tag: VerificationTypeInfoItem /* 3 */,
    },
    LongVariableInfo {
        tag: VerificationTypeInfoItem /* 4 */,
    },
    NullVariableInfo {
        tag: VerificationTypeInfoItem /* 5 */,
    },
    UninitializedThisVariableInfo {
        tag: VerificationTypeInfoItem /* 6 */,
    },
    ObjectVariableInfo {
        tag: VerificationTypeInfoItem /* 7 */,
        cpool_index: type_alias::u2,
    },
    UninitializedVariableInfo {
        tag: VerificationTypeInfoItem /* 8 */,
        offset: type_alias::u2,
    },
}

impl VerificationTypeInfo {
    pub fn write(self, buff: &mut Vec<u8>) -> Result<(), Error> {
        match self {
            VerificationTypeInfo::TopVariableInfo { tag } => { buff.write_u8(tag as u8)? }
            VerificationTypeInfo::IntegerVariableInfo { tag } => { buff.write_u8(tag as u8)? }
            VerificationTypeInfo::FloatVariableInfo { tag } => { buff.write_u8(tag as u8)? }
            VerificationTypeInfo::DoubleVariableInfo { tag } => { buff.write_u8(tag as u8)? }
            VerificationTypeInfo::LongVariableInfo { tag } => { buff.write_u8(tag as u8)? }
            VerificationTypeInfo::NullVariableInfo { tag } => { buff.write_u8(tag as u8)? }
            VerificationTypeInfo::UninitializedThisVariableInfo { tag } => { buff.write_u8(tag as u8)? }
            VerificationTypeInfo::ObjectVariableInfo { tag, cpool_index } => {
                buff.write_u8(tag as u8)?;
                buff.write_u16::<BigEndian>(cpool_index)?;
            }
            VerificationTypeInfo::UninitializedVariableInfo { tag, offset } => {
                buff.write_u8(tag as u8)?;
                buff.write_u16::<BigEndian>(offset)?;
            }
        }
        Ok(())
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
            ))
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
where R: Read {
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
            VerificationTypeInfoItem::ItemUninitializedThis => VerificationTypeInfo::UninitializedThisVariableInfo { tag },
            VerificationTypeInfoItem::ItemObject => VerificationTypeInfo::ObjectVariableInfo {
                tag,
                cpool_index: buff.read_u16::<BigEndian>()?,
            },
            VerificationTypeInfoItem::ItemUninitialized => VerificationTypeInfo::UninitializedVariableInfo {
                tag,
                offset: buff.read_u16::<BigEndian>()?,
            }
        })
    }
}


#[derive(Clone, Debug)]
pub enum StackMapFrame {
    /// 0-63
    SameFrame {
        frame_type: type_alias::u1
    },
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
    pub fn write(self, buff: &mut Vec<u8>) -> Result<(), Error> {
        match self {
            StackMapFrame::SameFrame { frame_type } => {
                buff.write_u8(frame_type)?;
            }
            StackMapFrame::SameLocals1StackItemFrame { frame_type, stack } => {
                buff.write_u8(frame_type)?;
                for entry in stack {
                    entry.write(buff)?;
                }
            }
            StackMapFrame::SameLocals1StackItemFrameExtended { frame_type, offset_delta, stack } => {
                buff.write_u8(frame_type)?;
                buff.write_u16::<BigEndian>(offset_delta)?;
                for entry in stack {
                    entry.write(buff)?;
                }
            }
            StackMapFrame::ChopFrame { frame_type, offset_delta } => {
                buff.write_u8(frame_type)?;
                buff.write_u16::<BigEndian>(offset_delta)?;
            }
            StackMapFrame::SameFrameExtended { frame_type, offset_delta } => {
                buff.write_u8(frame_type)?;
                buff.write_u16::<BigEndian>(offset_delta)?;
            }
            StackMapFrame::AppendFrame { frame_type, offset_delta, locals } => {
                buff.write_u8(frame_type)?;
                buff.write_u16::<BigEndian>(offset_delta)?;
                for one_local in locals {
                    one_local.write(buff)?;
                }
            }
            StackMapFrame::FullFrame { frame_type, offset_delta, number_of_locals, locals, number_of_stack_items, stack } => {
                buff.write_u8(frame_type)?;
                buff.write_u16::<BigEndian>(offset_delta)?;
                buff.write_u16::<BigEndian>(number_of_locals)?;
                for one_local in locals {
                    one_local.write(buff)?;
                }
                buff.write_u16::<BigEndian>(number_of_stack_items)?;
                for one_stack in stack {
                    one_stack.write(buff)?;
                }
            }
        }
        Ok(())
    }
}

#[derive(Clone, Debug)]
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
    pub fn write(self, buff: &mut Vec<u8>) -> Result<(), Error> {
        match self {
            Attribute::ConstantValue { attribute_name_index, attribute_length, constantvalue_index } => {
                buff.write_u16::<BigEndian>(attribute_name_index)?;
                buff.write_u32::<BigEndian>(attribute_length)?;
                buff.write_u16::<BigEndian>(constantvalue_index)?;
            }
            Attribute::Code { attribute_name_index, attribute_length, max_stack, max_locals, code_length, code, exception_table_length, exception_table, attributes_count, attributes } => {
                buff.write_u16::<BigEndian>(attribute_name_index)?;
                buff.write_u32::<BigEndian>(attribute_length)?;
                buff.write_u16::<BigEndian>(max_stack)?;
                buff.write_u16::<BigEndian>(max_locals)?;
                buff.write_u32::<BigEndian>(code_length)?;
                buff.write(code.as_slice())?;
                buff.write_u16::<BigEndian>(exception_table_length)?;
                for entry in exception_table {
                    entry.write(buff)?
                }
                buff.write_u16::<BigEndian>(attributes_count)?;
                for attr in attributes {
                    attr.write(buff)?
                }
            }
            Attribute::StackMapTable { attribute_name_index, attribute_length, number_of_entries, entries } => {
                buff.write_u16::<BigEndian>(attribute_name_index)?;
                buff.write_u32::<BigEndian>(attribute_length)?;
                buff.write_u16::<BigEndian>(number_of_entries)?;
                for frame in entries {
                    frame.write(buff)?
                }
            }
            Attribute::Exceptions { attribute_name_index, attribute_length, number_of_exceptions, exception_index_table } => unsafe {
                buff.write_u16::<BigEndian>(attribute_name_index)?;
                buff.write_u32::<BigEndian>(attribute_length)?;
                buff.write_u16::<BigEndian>(number_of_exceptions)?;
                buff.write(exception_index_table.iter().map(|e| { e.to_be() }).collect::<Vec<type_alias::u2>>().align_to::<u8>().1)?;
            }
            Attribute::InnerClasses { attribute_name_index, attribute_length, number_of_classes, classes } => {
                buff.write_u16::<BigEndian>(attribute_name_index)?;
                buff.write_u32::<BigEndian>(attribute_length)?;
                buff.write_u16::<BigEndian>(number_of_classes)?;
                for one_class in classes {
                    one_class.write(buff)?
                }
            }
            Attribute::EnclosingMethod { attribute_name_index, attribute_length, class_index, method_index } => {
                buff.write_u16::<BigEndian>(attribute_name_index)?;
                buff.write_u32::<BigEndian>(attribute_length)?;
                buff.write_u16::<BigEndian>(class_index)?;
                buff.write_u16::<BigEndian>(method_index)?;
            }
            Attribute::Synthetic { attribute_name_index, attribute_length } => {
                buff.write_u16::<BigEndian>(attribute_name_index)?;
                buff.write_u32::<BigEndian>(attribute_length)?;
            }
            Attribute::Signature { attribute_name_index, attribute_length, signature_index } => {
                buff.write_u16::<BigEndian>(attribute_name_index)?;
                buff.write_u32::<BigEndian>(attribute_length)?;
                buff.write_u16::<BigEndian>(signature_index)?;
            }
            Attribute::SourceFile { attribute_name_index, attribute_length, sourcefile_index } => {
                buff.write_u16::<BigEndian>(attribute_name_index)?;
                buff.write_u32::<BigEndian>(attribute_length)?;
                buff.write_u16::<BigEndian>(sourcefile_index)?;
            }
            Attribute::SourceDebugExtension { attribute_name_index, attribute_length, debug_extension } => {
                buff.write_u16::<BigEndian>(attribute_name_index)?;
                buff.write_u32::<BigEndian>(attribute_length)?;
                buff.write(cesu8::to_java_cesu8(debug_extension.as_str()).into_owned().as_slice())?;
            }
            Attribute::LineNumberTable { attribute_name_index, attribute_length, line_number_table_length, line_number_table } => {
                buff.write_u16::<BigEndian>(attribute_name_index)?;
                buff.write_u32::<BigEndian>(attribute_length)?;
                buff.write_u16::<BigEndian>(line_number_table_length)?;
                for entry in line_number_table {
                    entry.write(buff)?
                }
            }
            Attribute::LocalVariableTable { attribute_name_index, attribute_length, local_variable_table_length, local_variable_table } => {
                buff.write_u16::<BigEndian>(attribute_name_index)?;
                buff.write_u32::<BigEndian>(attribute_length)?;
                buff.write_u16::<BigEndian>(local_variable_table_length)?;
                for entry in local_variable_table {
                    entry.write(buff)?
                }
            }
            Attribute::LocalVariableTypeTable { attribute_name_index, attribute_length, local_variable_type_table_length, local_variable_type_table } => {
                buff.write_u16::<BigEndian>(attribute_name_index)?;
                buff.write_u32::<BigEndian>(attribute_length)?;
                buff.write_u16::<BigEndian>(local_variable_type_table_length)?;
                for entry in local_variable_type_table {
                    entry.write(buff)?
                }
            }
            Attribute::Deprecated { attribute_name_index, attribute_length } => {
                buff.write_u16::<BigEndian>(attribute_name_index)?;
                buff.write_u32::<BigEndian>(attribute_length)?;
            }
            Attribute::RuntimeVisibleAnnotations { attribute_name_index, attribute_length, num_annotations, annotations } => {
                buff.write_u16::<BigEndian>(attribute_name_index)?;
                buff.write_u32::<BigEndian>(attribute_length)?;
                buff.write_u16::<BigEndian>(num_annotations)?;
                for one_annotation in annotations {
                    one_annotation.write(buff)?;
                }
            }
            Attribute::RuntimeInvisibleAnnotations { attribute_name_index, attribute_length, num_annotations, annotations } => {
                buff.write_u16::<BigEndian>(attribute_name_index)?;
                buff.write_u32::<BigEndian>(attribute_length)?;
                buff.write_u16::<BigEndian>(num_annotations)?;
                for one_annotation in annotations {
                    one_annotation.write(buff)?;
                }
            }
            Attribute::RuntimeVisibleParameterAnnotations { attribute_name_index, attribute_length, num_parameters, parameter_annotations } => {
                buff.write_u16::<BigEndian>(attribute_name_index)?;
                buff.write_u32::<BigEndian>(attribute_length)?;
                buff.write_u8(num_parameters)?;
                for one_annotation in parameter_annotations {
                    one_annotation.write(buff)?;
                }
            }
            Attribute::RuntimeInvisibleParameterAnnotations { attribute_name_index, attribute_length, num_parameters, parameter_annotations } => {
                buff.write_u16::<BigEndian>(attribute_name_index)?;
                buff.write_u32::<BigEndian>(attribute_length)?;
                buff.write_u8(num_parameters)?;
                for one_annotation in parameter_annotations {
                    one_annotation.write(buff)?;
                }
            }
            Attribute::RuntimeVisibleTypeAnnotations { attribute_name_index, attribute_length, num_parameters, annotations } => {
                buff.write_u16::<BigEndian>(attribute_name_index)?;
                buff.write_u32::<BigEndian>(attribute_length)?;
                buff.write_u16::<BigEndian>(num_parameters)?;
                for one_annotation in annotations {
                    one_annotation.write(buff)?;
                }
            }
            Attribute::RuntimeInvisibleTypeAnnotations { attribute_name_index, attribute_length, num_parameters, annotations } => {
                buff.write_u16::<BigEndian>(attribute_name_index)?;
                buff.write_u32::<BigEndian>(attribute_length)?;
                buff.write_u16::<BigEndian>(num_parameters)?;
                for one_annotation in annotations {
                    one_annotation.write(buff)?;
                }
            }
            Attribute::AnnotationDefault { attribute_name_index, attribute_length, default_value } => {
                buff.write_u16::<BigEndian>(attribute_name_index)?;
                buff.write_u32::<BigEndian>(attribute_length)?;
                default_value.write(buff)?;
            }
            Attribute::BootstrapMethods { attribute_name_index, attribute_length, num_bootstrap_methods, bootstrap_methods } => unsafe {
                buff.write_u16::<BigEndian>(attribute_name_index)?;
                buff.write_u32::<BigEndian>(attribute_length)?;
                buff.write_u16::<BigEndian>(num_bootstrap_methods)?;
                for one_method in bootstrap_methods {
                    one_method.write(buff)?;
                }
            }
            Attribute::MethodParameters { attribute_name_index, attribute_length, parameters_count, parameters } => {
                buff.write_u16::<BigEndian>(attribute_name_index)?;
                buff.write_u32::<BigEndian>(attribute_length)?;
                buff.write_u8(parameters_count)?;
                for one_parameter in parameters {
                    one_parameter.write(buff)?;
                }
            }
            Attribute::Module { attribute_name_index, attribute_length, module_name_index, module_flags, module_version_index, requires_count, requires, exports_count, exports, opens_count, opens, uses_count, uses_index, provides_count, provides } => unsafe {
                buff.write_u16::<BigEndian>(attribute_name_index)?;
                buff.write_u32::<BigEndian>(attribute_length)?;
                buff.write_u16::<BigEndian>(module_name_index)?;
                buff.write_u16::<BigEndian>(module_flags)?;
                buff.write_u16::<BigEndian>(module_version_index)?;

                buff.write_u16::<BigEndian>(requires_count)?;
                for one_require in requires {
                    one_require.write(buff)?;
                }

                buff.write_u16::<BigEndian>(exports_count)?;
                for one_export in exports {
                    one_export.write(buff)?;
                }

                buff.write_u16::<BigEndian>(opens_count)?;
                for one_open in opens {
                    one_open.write(buff)?;
                }

                buff.write_u16::<BigEndian>(uses_count)?;
                buff.write(uses_index.iter().map(|e| { e.to_be() }).collect::<Vec<type_alias::u2>>().align_to::<u8>().1)?;

                buff.write_u16::<BigEndian>(provides_count)?;
                for one_provide in provides {
                    one_provide.write(buff)?;
                }
            }
            Attribute::ModulePackages { attribute_name_index, attribute_length, package_count, package_index } => unsafe {
                buff.write_u16::<BigEndian>(attribute_name_index)?;
                buff.write_u32::<BigEndian>(attribute_length)?;
                buff.write_u16::<BigEndian>(package_count)?;
                buff.write(package_index.iter().map(|e| { e.to_be() }).collect::<Vec<type_alias::u2>>().align_to::<u8>().1)?;
            }
            Attribute::ModuleMainClass { attribute_name_index, attribute_length, main_class_index } => {
                buff.write_u16::<BigEndian>(attribute_name_index)?;
                buff.write_u32::<BigEndian>(attribute_length)?;
                buff.write_u16::<BigEndian>(main_class_index)?;
            }
            Attribute::NestHost { attribute_name_index, attribute_length, host_class_index } => {
                buff.write_u16::<BigEndian>(attribute_name_index)?;
                buff.write_u32::<BigEndian>(attribute_length)?;
                buff.write_u16::<BigEndian>(host_class_index)?;
            }
            Attribute::NestMembers { attribute_name_index, attribute_length, number_of_classes, classes } => unsafe {
                buff.write_u16::<BigEndian>(attribute_name_index)?;
                buff.write_u32::<BigEndian>(attribute_length)?;
                buff.write_u16::<BigEndian>(number_of_classes)?;
                buff.write(classes.iter().map(|e| { e.to_be() }).collect::<Vec<type_alias::u2>>().align_to::<u8>().1)?;
            }
            Attribute::Record { attribute_name_index, attribute_length, components_count, components } => {
                buff.write_u16::<BigEndian>(attribute_name_index)?;
                buff.write_u32::<BigEndian>(attribute_length)?;
                buff.write_u16::<BigEndian>(components_count)?;
                for one_parameter in components {
                    one_parameter.write(buff)?;
                }
            }
            Attribute::PermittedSubclasses { attribute_name_index, attribute_length, number_of_classes, classes } => unsafe {
                buff.write_u16::<BigEndian>(attribute_name_index)?;
                buff.write_u32::<BigEndian>(attribute_length)?;
                buff.write_u16::<BigEndian>(number_of_classes)?;
                buff.write(classes.iter().map(|e| { e.to_be() }).collect::<Vec<type_alias::u2>>().align_to::<u8>().1)?;
            }
            Attribute::ExternalAttribute { attribute_name_index, attribute_length, info: data } => {
                buff.write_u16::<BigEndian>(attribute_name_index)?;
                buff.write_u32::<BigEndian>(attribute_length)?;
                buff.write(&*data)?;
            }
        }
        Ok(())
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
