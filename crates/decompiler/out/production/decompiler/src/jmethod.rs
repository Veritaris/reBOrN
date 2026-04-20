use crate::jannotation::JavaAnnotation;
use classfile::access_flags::AccessFlags;

pub struct JavaMethod {
    access_flags: AccessFlags,
    name: String,
    annotations: Vec<JavaAnnotation>,
}
