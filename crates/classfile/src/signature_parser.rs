use std::fmt::{Display, Formatter};
use std::iter::Peekable;
use std::str::Chars;

type PrimitiveDescriptor = char;

#[derive(Debug, Copy, Clone)]
pub enum ValueTypePrimitive {
    Byte,
    Short,
    Int,
    Long,
    Char,
    Float,
    Double,
    Boolean,
    Void,
}

impl From<ValueTypePrimitive> for String {
    fn from(value: ValueTypePrimitive) -> Self {
        From::<&ValueTypePrimitive>::from(&value)
    }
}

impl From<&ValueTypePrimitive> for String {
    fn from(value: &ValueTypePrimitive) -> Self {
        match value {
            ValueTypePrimitive::Byte => String::from("byte"),
            ValueTypePrimitive::Short => String::from("short"),
            ValueTypePrimitive::Int => String::from("int"),
            ValueTypePrimitive::Long => String::from("long"),
            ValueTypePrimitive::Char => String::from("char"),
            ValueTypePrimitive::Float => String::from("float"),
            ValueTypePrimitive::Double => String::from("double"),
            ValueTypePrimitive::Boolean => String::from("boolean"),
            ValueTypePrimitive::Void => String::from("void"),
        }
    }
}

impl ValueTypePrimitive {
    fn from(value: PrimitiveDescriptor) -> Option<ValueTypePrimitive> {
        match value.to_ascii_uppercase() {
            'B' => Some(ValueTypePrimitive::Byte),
            'C' => Some(ValueTypePrimitive::Char),
            'D' => Some(ValueTypePrimitive::Double),
            'F' => Some(ValueTypePrimitive::Float),
            'I' => Some(ValueTypePrimitive::Int),
            'J' => Some(ValueTypePrimitive::Long),
            'S' => Some(ValueTypePrimitive::Short),
            'Z' => Some(ValueTypePrimitive::Boolean),
            'V' => Some(ValueTypePrimitive::Void),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub enum ValueType {
    Primitive { r#type: ValueTypePrimitive },
    Object { r#type: String },
    Array { r#type: Box<ValueType> },
    Undefined,
}

impl From<&ValueType> for String {
    fn from(value: &ValueType) -> Self {
        match value {
            ValueType::Primitive { r#type } => Into::<String>::into(r#type),
            ValueType::Object { r#type } => r#type.clone(),
            ValueType::Array { r#type } => Into::<String>::into(r#type.as_ref()).as_str().to_owned() + "[]",
            ValueType::Undefined => String::from("<Undefined ValueType>"),
        }
    }
}

impl Display for ValueType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(Into::<String>::into(self).as_str())
    }
}

#[derive(Debug, Clone)]
pub enum JVMSignature {
    FieldSignature {
        spec: ValueType,
    },
    MethodSignature {
        params_spec: Vec<ValueType>,
        return_spec: ValueType,
    },
}

impl Display for JVMSignature {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            JVMSignature::FieldSignature { spec } => f.write_fmt(format_args!("{}", spec)),
            JVMSignature::MethodSignature {
                params_spec,
                return_spec,
            } => {
                let params_spec_str = params_spec.iter().map(From::from).collect::<Vec<String>>().join(", ");
                f.write_fmt(format_args!("{} ({})", return_spec, params_spec_str))
            }
        }
    }
}

pub fn parse_jvm_descriptor(descriptor: &str) -> Option<JVMSignature> {
    let mut return_spec: ValueType = ValueType::Undefined {};
    let mut params_spec: Vec<ValueType> = vec![];
    let mut is_parsing_method_params = false;
    let mut is_method_descriptor = false;

    let mut descriptor_iter = descriptor.chars().peekable();

    while let Some(&descriptor_symbol) = descriptor_iter.peek() {
        match descriptor_symbol {
            '(' => {
                is_parsing_method_params = true;
                is_method_descriptor = true;
                descriptor_iter.next();
            }

            ')' => {
                is_parsing_method_params = false;
                descriptor_iter.next();
            }

            'B' | 'C' | 'D' | 'F' | 'I' | 'J' | 'S' | 'Z' | 'V' => {
                if is_parsing_method_params {
                    params_spec.push(ValueType::Primitive {
                        r#type: ValueTypePrimitive::from(descriptor_symbol)?,
                    });
                } else {
                    return_spec = ValueType::Primitive {
                        r#type: ValueTypePrimitive::from(descriptor_symbol)?,
                    };
                }
                descriptor_iter.next();
            }

            _ => {
                if let Some(class_or_interface_descriptor) = parse_object_or_array_descriptor(&mut descriptor_iter) {
                    if is_parsing_method_params {
                        params_spec.push(class_or_interface_descriptor);
                    } else {
                        return_spec = class_or_interface_descriptor;
                    }
                }
            }
        };
    }

    if is_method_descriptor {
        Some(JVMSignature::MethodSignature {
            params_spec,
            return_spec,
        })
    } else {
        Some(JVMSignature::FieldSignature { spec: return_spec })
    }
}

pub(crate) fn parse_object_or_array_descriptor(descriptor_iter: &mut Peekable<Chars>) -> Option<ValueType> {
    let next_char = descriptor_iter.next()?;

    match next_char {
        'L' => {
            let mut class_or_interface_descriptor: String = String::new();
            while let Some(symbol) = descriptor_iter.next() {
                match symbol {
                    ';' => break,
                    '/' => class_or_interface_descriptor.push('.'),
                    other => class_or_interface_descriptor.push(other),
                }
            }
            Some(ValueType::Object {
                r#type: class_or_interface_descriptor,
            })
        }
        '[' => Some(ValueType::Array {
            r#type: Box::from(parse_array_descriptor(descriptor_iter)?),
        }),
        _ => None,
    }
}

pub(crate) fn parse_array_descriptor(descriptor_iter: &mut Peekable<Chars>) -> Option<ValueType> {
    match ValueTypePrimitive::from(*descriptor_iter.peek()?) {
        None => parse_object_or_array_descriptor(descriptor_iter),
        Some(primitive_type) => {
            descriptor_iter.next();
            Some(ValueType::Primitive { r#type: primitive_type })
        }
    }
}
