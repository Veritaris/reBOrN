use classfile::access_flags::{AccessFlagContext, AccessFlags};
use classfile::attributes::Attribute;
use classfile::classfile::ClassFile;
use classfile::signature_parser::{parse_jvm_descriptor, JVMSignature, ValueType};
use std::fs::File;
use std::io::{BufReader, BufWriter, Write};

const PARAM_PREFIX: &str = "param_";

const JAVA_BUILTIN_OBJECT: &str = "java/lang/Object";
const JAVA_BUILTIN_OBJECT_DOTTED: &str = "java.lang.Object";
const JVM_PREIMPORTED_PACKAGES: &[&str; 2] = &["java.lang", "java.io"];
const CONSTRUCTOR_METHOD_NAME: &str = "<init>";
const CLASS_INIT_METHOD_NAME: &str = "<clinit>";
const TAB: &str = "    ";

fn main() {
    let file_path = std::path::Path::new("crates/decompiler/compiledJavaClasses/Main.class");
    let file = File::open(file_path).unwrap();
    let buf_reader = BufReader::new(file);

    let classfile = ClassFile::read(buf_reader, None).unwrap();
    let source_file_name: Option<String> = classfile.attributes.iter().find_map(|attr| {
        if let Attribute::SourceFile { sourcefile_index, .. } = attr {
            Some(classfile.get_string_from_cpool(*sourcefile_index))
        } else {
            None
        }
    });

    let source_file_name = match source_file_name {
        Some(it) => it,
        None => file_path
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string()
            .replace(".class", "java"),
    };
    let out_path = format!("crates/decompiler/decompiled/{}", source_file_name);
    let path = std::path::Path::new(out_path.as_str());
    let path_prefix = path.parent().unwrap();
    std::fs::create_dir_all(path_prefix).unwrap();
    let out_file = File::options()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)
        .unwrap();

    let buf_writer = BufWriter::new(out_file);
    write_class(buf_writer, &classfile);
}

fn write_methods(classfile: &ClassFile, buf: &mut Vec<String>, indent: Option<u8>) {
    let indent = match indent {
        None => 1,
        Some(i) => i + 1,
    };

    for method in classfile.methods.iter() {
        let method_name = classfile.get_string_from_cpool(method.name_index);
        if method_name == CONSTRUCTOR_METHOD_NAME || method_name == CLASS_INIT_METHOD_NAME {
            continue;
        }
        let access_flags = AccessFlags::from((AccessFlagContext::Method, method.access_flags));
        let raw_method_descriptor = classfile.get_string_from_cpool(method.descriptor_index);
        let method_signature = parse_jvm_descriptor(&raw_method_descriptor).unwrap();
        if let JVMSignature::MethodSignature { return_spec, .. } = method_signature {
            let method_params_signature = return_spec.to_string();
            let mut method_code = format!(
                "{} {} {}({})",
                access_flags, return_spec, method_name, method_params_signature
            );
            let line_indent = TAB.repeat(indent as usize);
            method_code = line_indent.to_string() + &method_code + "{}";
            buf.push(method_code);
        }
    }
}

fn write_fields(classfile: &ClassFile, buf: &mut Vec<String>, indent: Option<u8>) {
    let indent = match indent {
        None => 1,
        Some(i) => i + 1,
    };

    for field in classfile.fields.iter() {
        let field_name = classfile.get_string_from_cpool(field.name_index);
        let raw_field_descriptor = classfile.get_string_from_cpool(field.descriptor_index);
        let field_descriptor = parse_jvm_descriptor(&raw_field_descriptor).unwrap();
        let access_flags = AccessFlags::from((AccessFlagContext::Field, field.access_flags));
        let mut field_code = format!("{} {} {}", access_flags, field_descriptor, field_name);

        for attr in field.attributes.iter() {
            if let Attribute::ConstantValue {
                constantvalue_index, ..
            } = attr
            {
                if let JVMSignature::FieldSignature { spec } = &field_descriptor {
                    match spec {
                        ValueType::Primitive { .. } => {}
                        ValueType::Object { r#type } => {
                            if r#type == "java.lang.String" {
                                let constval = classfile.get_string_from_cpool(*constantvalue_index);
                                field_code += format!(" = \"{constval}\"").as_str();
                            }
                        }
                        ValueType::Array { .. } => {}
                        ValueType::Undefined => {}
                    }
                }
            }
        }

        let line_indent = TAB.repeat(indent as usize);
        field_code = line_indent.to_string() + &field_code + ";";
        buf.push(field_code);
    }
}

fn extract_inner_classes(classfile: &ClassFile) -> Vec<String> {
    let mut inner_classes: Vec<String> = vec![];
    for attr in classfile.attributes.iter() {
        println!("{:?}", attr);
        if let Attribute::InnerClasses { classes, .. } = attr {
            for inner_class_entry in classes {
                let inner_class_name = classfile.get_string_from_cpool(inner_class_entry.inner_name_index);
                inner_classes.push(inner_class_name);
            }
        };
    }
    inner_classes
}

fn write_class<W>(mut dest: W, classfile: &ClassFile)
where
    W: Write,
{
    let indent: Option<u8> = None;
    let mut code_lines: Vec<String> = Vec::new();
    let this_class_name = classfile.class_name_from_cp(false);
    let super_class_name = classfile.class_name_from_cp(true).replace("/", ".");
    let line = format!(
        "{}class {} extends {} {{",
        classfile.access_flags.as_string(),
        this_class_name,
        super_class_name
    );
    code_lines.push(line);
    // TODO: add scan for all inner classes before any further parsing for proper signatures
    extract_inner_classes(classfile);
    write_fields(classfile, &mut code_lines, indent);
    code_lines.push("\n".to_string());
    write_methods(classfile, &mut code_lines, indent);

    code_lines.push("}".to_string());
    let full_code = code_lines.join("\n");
    let _ = dest.write(full_code.as_bytes()).unwrap();
}
