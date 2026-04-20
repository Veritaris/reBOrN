use crate::jannotation::JavaAnnotation;
use classfile::access_flags::AccessFlags;

pub struct JavaField {
    access_flags: AccessFlags,
    name: String,
    annotations: Vec<JavaAnnotation>,
}
