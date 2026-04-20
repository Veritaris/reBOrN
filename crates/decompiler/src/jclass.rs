use crate::jannotation::JavaAnnotation;
use crate::jfield::JavaField;
use crate::jmethod::JavaMethod;
use std::cell::{Ref, RefCell};

pub struct JavaClass<'a> {
    minor_version: u16,
    major_version: u16,
    source_file: Option<String>,
    inner_classes: Vec<JavaClass<'a>>,
    this_class: &'a str,
    super_class: Option<&'a JavaClass<'a>>,
    annotations: Vec<JavaAnnotation>,
    fields: Vec<JavaField>,
    methods: Vec<JavaMethod>,
}

const ObjectClass: &JavaClass = &JavaClass {
    minor_version: 0,
    major_version: 0,
    source_file: None,
    inner_classes: vec![],
    this_class: "java.lang.object",
    super_class: None,
    annotations: vec![],
    fields: vec![],
    methods: vec![],
};

impl<'a> JavaClass<'a> {
    fn new(super_class: Option<JavaClass>) -> Self {
        let mut class = JavaClass {
            minor_version: 0,
            major_version: 0,
            source_file: None,
            inner_classes: vec![],
            this_class: "Main",
            super_class: None,
            annotations: vec![],
            fields: vec![],
            methods: vec![],
        };

        class
    }
}
