use std::io::{BufReader, Error, ErrorKind, Read};

use byteorder::{BigEndian, ReadBytesExt};
use linked_hash_map::LinkedHashMap;
use zip::read::ZipFile;

use crate::access_flags::{AccessFlagContext, AccessFlags};
use crate::attributes::*;
use crate::classfile::*;

impl<'a, 'b> ClassFile
    where 'b: 'a {
    pub fn tag_to_display(&self, tag: &ConstantPoolTags) -> String {
        match tag {
            ConstantPoolTags::Utf8 { bytes, length, .. } => {
                format!("Utf8<length={}, bytes='{}'>", length, String::from_utf8(bytes.clone()).unwrap())
            }
            ConstantPoolTags::Integer { _value, .. } => {
                format!("Integer<value={}>", _value)
            }
            ConstantPoolTags::Float { _value, .. } => {
                format!("Float<value={}>", _value)
            }
            ConstantPoolTags::Long { _value, .. } => {
                format!("Long<value={}>", _value)
            }
            ConstantPoolTags::Double { _value, .. } => {
                format!("Double<value={}>", _value)
            }
            ConstantPoolTags::Class { name_index, .. } => {
                let val = self.constant_pool.get(*name_index as usize).unwrap();
                format!("Class<name_index={}, content={}>", name_index, self.tag_to_display(val))
            }
            ConstantPoolTags::String { string_index, .. } => {
                let val = self.constant_pool.get(*string_index as usize).unwrap();
                format!("String<string_index={}, content={}>", string_index, self.tag_to_display(val))
            }
            ConstantPoolTags::Fieldref { class_index, name_and_type_index, .. } => {
                let class = self.constant_pool.get(*class_index as usize).unwrap();
                let name_and_type = self.constant_pool.get(*name_and_type_index as usize).unwrap();
                format!("FieldRef<class={}, name_and_type={}>", self.tag_to_display(class), self.tag_to_display(name_and_type))
            }
            ConstantPoolTags::Methodref { class_index, name_and_type_index, .. } => {
                let class = self.constant_pool.get(*class_index as usize).unwrap();
                let name_and_type = self.constant_pool.get(*name_and_type_index as usize).unwrap();
                format!("MethodRef<class={}, name_and_type={}>", self.tag_to_display(class), self.tag_to_display(name_and_type))
            }
            ConstantPoolTags::InterfaceMethodref { class_index, name_and_type_index, .. } => {
                let class = self.constant_pool.get(*class_index as usize).unwrap();
                let name_and_type = self.constant_pool.get(*name_and_type_index as usize).unwrap();
                format!("InterfaceMethodRef<class={}, name_and_type={}>", self.tag_to_display(class), self.tag_to_display(name_and_type))
            }
            ConstantPoolTags::NameAndType { name_index, descriptor_index, .. } => {
                let name = self.constant_pool.get(*name_index as usize).unwrap();
                let descriptor = self.constant_pool.get(*descriptor_index as usize).unwrap();
                format!("NameAndType<name={}, descriptor={}>", self.tag_to_display(name), self.tag_to_display(descriptor))
            }
            ConstantPoolTags::MethodHandle { .. } => {
                String::from("MethodHandle<TODO>")
            }
            ConstantPoolTags::MethodType { .. } => {
                String::from("MethodType<TODO>")
            }
            ConstantPoolTags::Dynamic { .. } => {
                String::from("Dynamic<TODO>")
            }
            ConstantPoolTags::InvokeDynamic { .. } => {
                String::from("InvokeDynamic<TODO>")
            }
            ConstantPoolTags::Module { name_index, .. } => {
                format!("Module<name={}>", self.get_string_from_cpool(*name_index))
            }
            ConstantPoolTags::Package { name_index, .. } => {
                format!("Module<name={}>", self.get_string_from_cpool(*name_index))
            }
            ConstantPoolTags::ContinuationTag { .. } => {
                String::from("ContinuationTag")
            }
        }
    }

    pub fn class_name_from_cp(&self) -> String {
        match self.constant_pool.get(self.this_class as usize) {
            Some(ConstantPoolTags::Class { name_index, .. }) => {
                match self.constant_pool.get(*name_index as usize) {
                    Some(ConstantPoolTags::Utf8 { bytes, .. }) => String::from_utf8(bytes.clone()).unwrap(),
                    _ => String::new()
                }
            }
            _ => String::new()
        }
    }
    pub fn read(len: u64, mut buff: BufReader<ZipFile>, mappings: Option<&LinkedHashMap<String, String>>) -> Result<ClassFile, Error> {
        let magic = buff.read_u32::<BigEndian>()?;
        if magic != CLASS_HEADER {
            panic!("unexpected magic: {magic}");
        }

        let minor_version = buff.read_u16::<BigEndian>()?;
        let major_version = buff.read_u16::<BigEndian>()?;
        let constant_pool_count = buff.read_u16::<BigEndian>()?;
        let mut constant_pool: Vec<ConstantPoolTags> = Vec::with_capacity(constant_pool_count as usize);
        constant_pool.push(CONTINUATION_TAG);

        let mut cp_index = 1;
        while cp_index < constant_pool_count {
            match ClassFile::read_tag(&mut buff, mappings) {
                Ok(tag) => {
                    match tag {
                        ConstantPoolTags::Long { .. } | ConstantPoolTags::Double { .. } => {
                            constant_pool.push(tag);
                            constant_pool.push(CONTINUATION_TAG);
                            cp_index += 1;
                        }
                        _ => constant_pool.push(tag)
                    };
                }
                Err(err) => {
                    println!("skipping error tag at {}th iter: {}", cp_index, err)
                }
            };
            cp_index += 1;
        }

        let access_flags = AccessFlags::from((AccessFlagContext::Class, buff.read_u16::<BigEndian>()?));
        let this_class = buff.read_u16::<BigEndian>()?;
        let super_class = buff.read_u16::<BigEndian>()?;

        let interfaces_count = buff.read_u16::<BigEndian>()?;
        let mut interfaces: Vec<u2> = Vec::with_capacity(interfaces_count as usize);
        for _ in 0..interfaces_count {
            interfaces.push(buff.read_u16::<BigEndian>()?);
        }

        let fields_count = buff.read_u16::<BigEndian>()?;
        let mut fields: Vec<Field> = Vec::with_capacity(fields_count as usize);
        for _ in 0..fields_count {
            fields.push(Self::read_field(&constant_pool, &mut buff)?);
        }

        let methods_count = buff.read_u16::<BigEndian>()?;
        let mut methods: Vec<Method> = Vec::with_capacity(methods_count as usize);
        for _ in 0..methods_count {
            methods.push(Self::read_method(&constant_pool, &mut buff)?);
        }

        let attributes_count = buff.read_u16::<BigEndian>()?;
        let attributes: Vec<Attribute> = Self::read_attributes_vec(attributes_count, &constant_pool, &mut buff);

        Ok(ClassFile {
            magic,
            minor_version,
            major_version,
            constant_pool_count,
            constant_pool,
            access_flags,
            this_class,
            super_class,
            interfaces_count,
            interfaces,
            fields_count,
            fields,
            methods_count,
            methods,
            attributes_count,
            attributes,

            _len: len,
        })
    }


    ///```javadoc
    /// field_info {
    ///     u2             access_flags;
    ///     u2             name_index;
    ///     u2             descriptor_index;
    ///     u2             attributes_count;
    ///     attribute_info attributes[attributes_count];
    /// }
    ///```
    /// Oracle docs: https://docs.oracle.com/javase/specs/jvms/se22/html/jvms-4.html#jvms-4.5
    ///
    fn read_field(constant_pool: &Vec<ConstantPoolTags>, buff: &mut BufReader<ZipFile<'b>>) -> Result<Field, Error> {
        let access_flags: u2 = buff.read_u16::<BigEndian>()?;
        let name_index: u2 = buff.read_u16::<BigEndian>()?;
        let descriptor_index: u2 = buff.read_u16::<BigEndian>()?;
        let attributes_count: u2 = buff.read_u16::<BigEndian>()?;
        let attributes: Vec<Attribute> = Self::read_attributes_vec(attributes_count, &constant_pool, buff);

        Ok(Field {
            access_flags,
            name_index,
            descriptor_index,
            attributes_count,
            attributes,
        })
    }

    ///```javadoc
    /// method_info {
    ///     u2             access_flags;
    ///     u2             name_index;
    ///     u2             descriptor_index;
    ///     u2             attributes_count;
    ///     attribute_info attributes[attributes_count];
    /// }
    ///```
    /// Oracle docs: https://docs.oracle.com/javase/specs/jvms/se22/html/jvms-4.html#jvms-4.6
    ///
    fn read_method(constant_pool: &Vec<ConstantPoolTags>, buff: &mut BufReader<ZipFile<'b>>) -> Result<Method, Error> {
        let access_flags = buff.read_u16::<BigEndian>()?;
        let name_index = buff.read_u16::<BigEndian>()?;
        let descriptor_index = buff.read_u16::<BigEndian>()?;
        let attributes_count = buff.read_u16::<BigEndian>()?;
        let attributes: Vec<Attribute> = Self::read_attributes_vec(attributes_count, &constant_pool, buff);

        Ok(Method {
            access_flags,
            name_index,
            descriptor_index,
            attributes_count,
            attributes,
        })
    }

    fn read_stack_frames_vec(count: u2, buff: &mut BufReader<ZipFile<'b>>) -> Vec<StackMapFrame> {
        let mut stack_frames: Vec<StackMapFrame> = Vec::with_capacity(count as usize);
        for _ in 0..count {
            match Self::read_stack_frame(buff) {
                Ok(res) => stack_frames.push(res),
                Err(_) => {}
            };
        };
        return stack_frames;
    }

    fn read_stack_frame(buff: &mut BufReader<ZipFile<'b>>) -> Result<StackMapFrame, Error> {
        let frame_type = buff.read_u8()?;

        Ok(
            match frame_type {
                0..=63 => StackMapFrame::SameFrame { frame_type },
                64..=127 => {
                    let verification_type_info = VerificationTypeInfo::try_from(buff)?;
                    StackMapFrame::SameLocals1StackItemFrame {
                        frame_type,
                        stack: vec![verification_type_info],
                    }
                }
                247 => {
                    let offset_delta = buff.read_u16::<BigEndian>()?;
                    let verification_type_info = VerificationTypeInfo::try_from(buff)?;
                    StackMapFrame::SameLocals1StackItemFrameExtended {
                        frame_type,
                        offset_delta,
                        stack: vec![verification_type_info],
                    }
                }
                248..=250 => {
                    let offset_delta = buff.read_u16::<BigEndian>()?;
                    StackMapFrame::ChopFrame {
                        frame_type,
                        offset_delta,
                    }
                }
                251 => {
                    let offset_delta = buff.read_u16::<BigEndian>()?;
                    StackMapFrame::SameFrameExtended {
                        frame_type,
                        offset_delta,
                    }
                }
                252..=254 => {
                    let offset_delta = buff.read_u16::<BigEndian>()?;
                    let mut locals: Vec<VerificationTypeInfo> = vec![];
                    for _ in 0..(frame_type - 251) {
                        locals.push(VerificationTypeInfo::try_from(&mut *buff)?);
                    }
                    StackMapFrame::AppendFrame {
                        frame_type,
                        offset_delta,
                        locals,
                    }
                }
                255 => {
                    let offset_delta = buff.read_u16::<BigEndian>()?;
                    let number_of_locals = buff.read_u16::<BigEndian>()?;
                    let mut locals: Vec<VerificationTypeInfo> = vec![];
                    for _ in 0..number_of_locals {
                        locals.push(VerificationTypeInfo::try_from(&mut *buff)?);
                    }
                    let number_of_stack_items = buff.read_u16::<BigEndian>()?;
                    let mut stack: Vec<VerificationTypeInfo> = vec![];
                    for _ in 0..number_of_stack_items {
                        stack.push(VerificationTypeInfo::try_from(&mut *buff)?);
                    }
                    StackMapFrame::FullFrame {
                        frame_type,
                        offset_delta,
                        number_of_locals,
                        locals,
                        number_of_stack_items,
                        stack,
                    }
                }
                _ => {
                    return Err(Error::new(
                        ErrorKind::InvalidInput,
                        format!("frame_type {} is reserved for future use, is your classfile correct or library up to date?", frame_type))
                    );
                }
            }
        )
    }

    fn read_attributes_vec(count: u2, constant_pool: &Vec<ConstantPoolTags>, buff: &mut BufReader<ZipFile<'b>>) -> Vec<Attribute> {
        let mut attributes: Vec<Attribute> = Vec::with_capacity(count as usize);
        for _ in 0..count {
            match Self::read_attribute(constant_pool, buff) {
                Ok(res) => attributes.push(res),
                Err(err) => println!("unable to read attribute err={err}")
            };
        };
        return attributes;
    }

    ///
    /// Oracle docs: https://docs.oracle.com/javase/specs/jvms/se22/html/jvms-4.html#jvms-4.7
    ///
    fn read_attribute(constant_pool: &Vec<ConstantPoolTags>, buff: &mut BufReader<ZipFile<'b>>) -> Result<Attribute, Error> {
        let attribute_name_index = buff.read_u16::<BigEndian>()?;
        let attribute_length = buff.read_u32::<BigEndian>()?;

        let attribute_name = match constant_pool.get(attribute_name_index as usize) {
            Some(ConstantPoolTags::Utf8 { _value, .. }) => _value.as_str(),
            Some(e) => return Err(Error::new(
                ErrorKind::InvalidData,
                format!("expected Utf8 tag at index {} in constant pool, got {}", attribute_name_index, e))
            ),
            None => return Err(Error::new(
                ErrorKind::NotFound,
                format!("nothing found at index {} in constant pool", attribute_name_index))
            )
        };

        return match attribute_name {
            // critical to work on JVM
            "ConstantValue" => {
                Ok(Attribute::ConstantValue {
                    attribute_name_index,
                    attribute_length,
                    constantvalue_index: buff.read_u16::<BigEndian>()?,
                })
            }
            "Code" => {
                let max_stack = buff.read_u16::<BigEndian>()?;
                let max_locals = buff.read_u16::<BigEndian>()?;
                let code_length = buff.read_u32::<BigEndian>()?;
                let mut code: Vec<u1> = vec![0u8; code_length as usize];
                buff.read_exact(&mut *code)?;
                let exception_table_length = buff.read_u16::<BigEndian>()?;
                let mut exception_table: Vec<ExceptionTableEntry> = vec![];
                for _ in 0..exception_table_length {
                    exception_table.push(ExceptionTableEntry {
                        start_pc: buff.read_u16::<BigEndian>()?,
                        end_pc: buff.read_u16::<BigEndian>()?,
                        handler_pc: buff.read_u16::<BigEndian>()?,
                        catch_type: buff.read_u16::<BigEndian>()?,
                    });
                };
                let attributes_count = buff.read_u16::<BigEndian>()?;
                let attributes: Vec<Attribute> = Self::read_attributes_vec(attributes_count, constant_pool, buff);

                Ok(Attribute::Code {
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
                })
            }
            "StackMapTable" => {
                let number_of_entries = buff.read_u16::<BigEndian>()?;
                let entries: Vec<StackMapFrame> = Self::read_stack_frames_vec(
                    number_of_entries,
                    buff,
                );

                Ok(Attribute::StackMapTable {
                    attribute_name_index,
                    attribute_length,
                    number_of_entries,
                    entries,
                })
            }
            "BootstrapMethods" => {
                let num_bootstrap_methods = buff.read_u16::<BigEndian>()?;
                let mut bootstrap_methods: Vec<BootstrapMethodEntry> = Vec::with_capacity(num_bootstrap_methods as usize);
                for _ in 0..num_bootstrap_methods {
                    let bootstrap_method_ref = buff.read_u16::<BigEndian>()?;
                    let num_bootstrap_arguments = buff.read_u16::<BigEndian>()?;
                    let mut bootstrap_arguments: Vec<u2> = Vec::with_capacity(num_bootstrap_arguments as usize);
                    for _ in 0..num_bootstrap_arguments {
                        bootstrap_arguments.push(buff.read_u16::<BigEndian>()?);
                    }
                    bootstrap_methods.push(BootstrapMethodEntry {
                        bootstrap_method_ref,
                        num_bootstrap_arguments,
                        bootstrap_arguments,
                    });
                };

                Ok(Attribute::BootstrapMethods {
                    attribute_name_index,
                    attribute_length,
                    num_bootstrap_methods,
                    bootstrap_methods,
                })
            }
            "NestHost" => {
                Ok(Attribute::NestHost {
                    attribute_name_index,
                    attribute_length,
                    host_class_index: buff.read_u16::<BigEndian>()?,
                })
            }
            "NestMembers" => {
                let number_of_classes = buff.read_u16::<BigEndian>()?;
                let mut classes: Vec<u2> = Vec::with_capacity(number_of_classes as usize);
                for _ in 0..number_of_classes {
                    classes.push(buff.read_u16::<BigEndian>()?);
                };

                Ok(Attribute::NestMembers {
                    attribute_name_index,
                    attribute_length,
                    number_of_classes,
                    classes,
                })
            }
            "PermittedSubclasses" => {
                let number_of_classes = buff.read_u16::<BigEndian>()?;
                let mut classes: Vec<u2> = Vec::with_capacity(number_of_classes as usize);
                for _ in 0..number_of_classes {
                    classes.push(buff.read_u16::<BigEndian>()?);
                };

                Ok(Attribute::PermittedSubclasses {
                    attribute_name_index,
                    attribute_length,
                    number_of_classes,
                    classes,
                })
            }

            // optional, but critical for class libraries and instrumentation
            "Exceptions" => {
                let number_of_exceptions = buff.read_u16::<BigEndian>()?;
                let mut exception_index_table: Vec<u2> = Vec::with_capacity(number_of_exceptions as usize);
                for _ in 0..number_of_exceptions {
                    exception_index_table.push(buff.read_u16::<BigEndian>()?);
                };

                Ok(Attribute::Exceptions {
                    attribute_name_index,
                    attribute_length,
                    number_of_exceptions,
                    exception_index_table,
                })
            }
            "InnerClasses" => {
                let number_of_classes = buff.read_u16::<BigEndian>()?;
                let mut classes: Vec<InnerClassEntry> = Vec::with_capacity(number_of_classes as usize);
                for _ in 0..number_of_classes {
                    classes.push(InnerClassEntry {
                        inner_class_info_index: buff.read_u16::<BigEndian>()?,
                        outer_class_info_index: buff.read_u16::<BigEndian>()?,
                        inner_name_index: buff.read_u16::<BigEndian>()?,
                        inner_class_access_flags: buff.read_u16::<BigEndian>()?,
                    });
                };

                Ok(Attribute::InnerClasses {
                    attribute_name_index,
                    attribute_length,
                    number_of_classes,
                    classes,
                })
            }
            "EnclosingMethod" => {
                Ok(Attribute::EnclosingMethod {
                    attribute_name_index,
                    attribute_length,
                    class_index: buff.read_u16::<BigEndian>()?,
                    method_index: buff.read_u16::<BigEndian>()?,
                })
            }
            "Synthetic" => {
                Ok(Attribute::Synthetic {
                    attribute_name_index,
                    attribute_length,
                })
            }
            "Signature" => {
                Ok(Attribute::Signature {
                    attribute_name_index,
                    attribute_length,
                    signature_index: buff.read_u16::<BigEndian>()?,
                })
            }
            "Record" => {
                let components_count = buff.read_u16::<BigEndian>()?;
                let mut components: Vec<RecordComponentInfo> = Vec::with_capacity(components_count as usize);
                for _ in 0..components_count {
                    let name_index = buff.read_u16::<BigEndian>()?;
                    let descriptor_index = buff.read_u16::<BigEndian>()?;
                    let attributes_count = buff.read_u16::<BigEndian>()?;
                    components.push(RecordComponentInfo {
                        name_index,
                        descriptor_index,
                        attributes_count,
                        attributes: Self::read_attributes_vec(attributes_count, constant_pool, buff),
                    });
                }

                Ok(Attribute::Record {
                    attribute_name_index,
                    attribute_length,
                    components_count,
                    components,
                })
            }
            "SourceFile" => {
                Ok(Attribute::SourceFile {
                    attribute_name_index,
                    attribute_length,
                    sourcefile_index: buff.read_u16::<BigEndian>()?,
                })
            }
            "LineNumberTable" => {
                let line_number_table_length = buff.read_u16::<BigEndian>()?;
                let mut line_number_table: Vec<LineNumberEntry> = Vec::with_capacity(line_number_table_length as usize);
                for _ in 0..line_number_table_length {
                    line_number_table.push(LineNumberEntry {
                        start_pc: buff.read_u16::<BigEndian>()?,
                        line_number: buff.read_u16::<BigEndian>()?,
                    });
                };

                Ok(Attribute::LineNumberTable {
                    attribute_name_index,
                    attribute_length,
                    line_number_table_length,
                    line_number_table,
                })
            }
            "LocalVariableTable" => {
                let local_variable_table_length = buff.read_u16::<BigEndian>()?;
                let mut local_variable_table: Vec<LocalVariableTableEntry> = Vec::with_capacity(local_variable_table_length as usize);

                for _ in 0..local_variable_table_length {
                    local_variable_table.push(LocalVariableTableEntry {
                        start_pc: buff.read_u16::<BigEndian>()?,
                        length: buff.read_u16::<BigEndian>()?,
                        name_index: buff.read_u16::<BigEndian>()?,
                        descriptor_index: buff.read_u16::<BigEndian>()?,
                        index: buff.read_u16::<BigEndian>()?,
                    });
                }

                Ok(Attribute::LocalVariableTable {
                    attribute_name_index,
                    attribute_length,
                    local_variable_table_length,
                    local_variable_table,
                })
            }
            "LocalVariableTypeTable" => {
                let local_variable_type_table_length = buff.read_u16::<BigEndian>()?;
                let mut local_variable_type_table: Vec<LocalVariableTypeTableEntry> = Vec::with_capacity(local_variable_type_table_length as usize);

                for _ in 0..local_variable_type_table_length {
                    local_variable_type_table.push(LocalVariableTypeTableEntry {
                        start_pc: buff.read_u16::<BigEndian>()?,
                        length: buff.read_u16::<BigEndian>()?,
                        name_index: buff.read_u16::<BigEndian>()?,
                        signature_index: buff.read_u16::<BigEndian>()?,
                        index: buff.read_u16::<BigEndian>()?,
                    });
                }

                Ok(Attribute::LocalVariableTypeTable {
                    attribute_name_index,
                    attribute_length,
                    local_variable_type_table_length,
                    local_variable_type_table,
                })
            }

            // non-critical, but useful for tools and instrumentation
            "SourceDebugExtension" => {
                let mut debug_extension_bytes: Vec<u1> = vec![0u8; attribute_length as usize];
                buff.read_exact(&mut *debug_extension_bytes)?;
                let debug_extension = match cesu8::from_java_cesu8(debug_extension_bytes.as_ref()) {
                    Ok(res) => res.to_string().into_bytes(),
                    Err(_) => {
                        return Err(Error::new(
                            ErrorKind::InvalidInput,
                            format!("unable to read bytes string into utf8 string: {:?}", debug_extension_bytes))
                        );
                    }
                };

                Ok(Attribute::SourceDebugExtension {
                    attribute_name_index,
                    attribute_length,
                    debug_extension: String::from_utf8(debug_extension.clone()).unwrap(),
                })
            }
            "Deprecated" => {
                Ok(Attribute::Deprecated {
                    attribute_name_index,
                    attribute_length,
                })
            }
            "RuntimeVisibleAnnotations" | "RuntimeInvisibleAnnotations" => {
                let num_annotations = buff.read_u16::<BigEndian>()?;
                let annotations = Self::read_annotations_vec(num_annotations, buff)?;

                Ok(match attribute_name {
                    "RuntimeVisibleAnnotations" => Attribute::RuntimeVisibleAnnotations {
                        attribute_name_index,
                        attribute_length,
                        num_annotations,
                        annotations,
                    },
                    "RuntimeInvisibleAnnotations" => Attribute::RuntimeInvisibleAnnotations {
                        attribute_name_index,
                        attribute_length,
                        num_annotations,
                        annotations,
                    },
                    _ => unreachable!()
                })
            }
            "RuntimeVisibleParameterAnnotations" | "RuntimeInvisibleParameterAnnotations" => {
                let num_parameters = buff.read_u8()?;
                let mut parameter_annotations: Vec<ParameterAnnotation> = Vec::with_capacity(num_parameters as usize);
                for _ in 0..num_parameters {
                    let num_annotations = buff.read_u16::<BigEndian>()?;
                    let annotations = Self::read_annotations_vec(num_annotations, buff)?;
                    parameter_annotations.push(ParameterAnnotation {
                        num_annotations,
                        annotations,
                    })
                }

                Ok(match attribute_name {
                    "RuntimeVisibleParameterAnnotations" => Attribute::RuntimeVisibleParameterAnnotations {
                        attribute_name_index,
                        attribute_length,
                        num_parameters,
                        parameter_annotations,
                    },
                    "RuntimeInvisibleParameterAnnotations" => Attribute::RuntimeInvisibleParameterAnnotations {
                        attribute_name_index,
                        attribute_length,
                        num_parameters,
                        parameter_annotations,
                    },
                    _ => unreachable!()
                })
            }
            "RuntimeVisibleTypeAnnotations" | "RuntimeInvisibleTypeAnnotations" => {
                let num_parameters = buff.read_u16::<BigEndian>()?;
                let mut annotations: Vec<TypeAnnotation> = Vec::with_capacity(num_parameters as usize);
                for _ in 0..num_parameters {
                    let target_type: u1 = buff.read_u8()?;
                    let target_info = match target_type {
                        0x00 | 0x01 => TargetInfo::TypeParameterTarget { type_parameter_index: buff.read_u8()? },
                        0x10 => TargetInfo::SupertypeTarget { supertype_index: buff.read_u16::<BigEndian>()? },
                        0x11 | 0x12 => TargetInfo::TypeParameterBoundTarget { type_parameter_index: buff.read_u8()?, bound_index: buff.read_u8()? },
                        0x13 | 0x14 | 0x15 => TargetInfo::EmptyTarget {},
                        0x16 => TargetInfo::FormalParameterTarget { formal_parameter_index: buff.read_u8()? },
                        0x17 => TargetInfo::ThrowsTarget { throws_type_index: buff.read_u16::<BigEndian>()? },
                        0x40 | 0x41 => {
                            let table_length = buff.read_u16::<BigEndian>()?;
                            let mut table: Vec<LocalvarTargetTableEntry> = Vec::with_capacity(table_length as usize);
                            for _ in 0..table_length {
                                table.push(LocalvarTargetTableEntry {
                                    start_pc: buff.read_u16::<BigEndian>()?,
                                    length: buff.read_u16::<BigEndian>()?,
                                    index: buff.read_u16::<BigEndian>()?,
                                });
                            }
                            TargetInfo::LocalvarTarget { table_length, table }
                        }
                        0x42 => TargetInfo::CatchTarget { exception_table_index: buff.read_u16::<BigEndian>()? },
                        0x43 | 0x44 | 0x45 | 0x46 => TargetInfo::OffsetTarget { offset: buff.read_u16::<BigEndian>()? },
                        0x47 | 0x48 | 0x49 | 0x4A | 0x4B => TargetInfo::TypeArgumentTarget { offset: buff.read_u16::<BigEndian>()?, type_argument_index: buff.read_u8()? },
                        _ => continue
                    };
                    let target_path = {
                        let path_length = buff.read_u8()?;
                        let mut path: Vec<PathEntry> = Vec::with_capacity(path_length as usize);
                        for _ in 0..path_length {
                            path.push(PathEntry {
                                type_path_kind: buff.read_u8()?,
                                type_argument_index: buff.read_u8()?,
                            });
                        }
                        TypePath {
                            path_length,
                            path,
                        }
                    };
                    let type_index = buff.read_u16::<BigEndian>()?;
                    let num_element_value_pairs = buff.read_u16::<BigEndian>()?;
                    let element_value_pairs = Self::read_annotations_element_value_pairs_vec(num_element_value_pairs, buff)?;

                    annotations.push(TypeAnnotation {
                        target_type,
                        target_info,
                        target_path,
                        type_index,
                        num_element_value_pairs,
                        element_value_pairs,
                    });
                }

                Ok(match attribute_name {
                    "RuntimeVisibleTypeAnnotations" => Attribute::RuntimeVisibleTypeAnnotations {
                        attribute_name_index,
                        attribute_length,
                        num_parameters,
                        annotations,
                    },
                    "RuntimeInvisibleTypeAnnotations" => Attribute::RuntimeInvisibleTypeAnnotations {
                        attribute_name_index,
                        attribute_length,
                        num_parameters,
                        annotations,
                    },
                    _ => unreachable!()
                })
            }
            "AnnotationDefault" => {
                Ok(Attribute::AnnotationDefault {
                    attribute_name_index,
                    attribute_length,
                    default_value: Self::read_annotation_element_value(buff)?,
                })
            }
            "MethodParameters" => {
                let parameters_count = buff.read_u8()?;
                let mut parameters: Vec<Parameter> = Vec::with_capacity(parameters_count as usize);
                for _ in 0..parameters_count {
                    parameters.push(Parameter {
                        name_index: buff.read_u16::<BigEndian>()?,
                        access_flags: AccessFlags::from((AccessFlagContext::Module, buff.read_u16::<BigEndian>()?)),
                    });
                }

                Ok(Attribute::MethodParameters {
                    attribute_name_index,
                    attribute_length,
                    parameters_count,
                    parameters,
                })
            }
            "Module" => {
                let module_name_index = buff.read_u16::<BigEndian>()?;
                let module_flags = buff.read_u16::<BigEndian>()?;
                let module_version_index = buff.read_u16::<BigEndian>()?;

                let requires_count = buff.read_u16::<BigEndian>()?;
                let mut requires: Vec<RequiresEntry> = Vec::with_capacity(requires_count as usize);
                for _ in 0..requires_count {
                    let requires_index = buff.read_u16::<BigEndian>()?;
                    let requires_flags = buff.read_u16::<BigEndian>()?;
                    let requires_version_index = buff.read_u16::<BigEndian>()?;
                    requires.push(RequiresEntry {
                        requires_index,
                        requires_flags,
                        requires_version_index,
                    });
                }

                let exports_count = buff.read_u16::<BigEndian>()?;
                let mut exports: Vec<ExportsEntry> = Vec::with_capacity(exports_count as usize);
                for _ in 0..exports_count {
                    let exports_index = buff.read_u16::<BigEndian>()?;
                    let exports_flags = buff.read_u16::<BigEndian>()?;
                    let exports_to_count = buff.read_u16::<BigEndian>()?;
                    exports.push(ExportsEntry {
                        exports_index,
                        exports_flags,
                        exports_to_count,
                    });
                }

                let opens_count = buff.read_u16::<BigEndian>()?;
                let mut opens: Vec<OpensEntry> = Vec::with_capacity(opens_count as usize);
                for _ in 0..opens_count {
                    let opens_index = buff.read_u16::<BigEndian>()?;
                    let opens_flags = buff.read_u16::<BigEndian>()?;
                    let opens_to_count = buff.read_u16::<BigEndian>()?;
                    opens.push(OpensEntry {
                        opens_index,
                        opens_flags,
                        opens_to_count,
                    });
                }

                let uses_count = buff.read_u16::<BigEndian>()?;
                let mut uses_index: Vec<u2> = Vec::with_capacity(uses_count as usize);
                for _ in 0..uses_count {
                    uses_index.push(buff.read_u16::<BigEndian>()?);
                }

                let provides_count = buff.read_u16::<BigEndian>()?;
                let mut provides: Vec<ProvidesEntry> = Vec::with_capacity(provides_count as usize);
                for _ in 0..provides_count {
                    let provides_index = buff.read_u16::<BigEndian>()?;
                    let provides_with_count = buff.read_u16::<BigEndian>()?;
                    provides.push(ProvidesEntry {
                        provides_index,
                        provides_with_count,
                    });
                }

                Ok(Attribute::Module {
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
                    uses_index,
                    provides_count,
                    provides,
                })
            }
            "ModulePackages" => {
                let package_count = buff.read_u16::<BigEndian>()?;
                let mut package_index: Vec<u2> = Vec::with_capacity(package_count as usize);
                for _ in 0..package_count {
                    package_index.push(buff.read_u16::<BigEndian>()?);
                };
                Ok(Attribute::ModulePackages {
                    attribute_name_index,
                    attribute_length,
                    package_count,
                    package_index,
                })
            }
            "ModuleMainClass" => {
                Ok(Attribute::ModuleMainClass {
                    attribute_name_index,
                    attribute_length,
                    main_class_index: buff.read_u16::<BigEndian>()?,
                })
            }
            &_ => Err(Error::new(
                ErrorKind::InvalidInput,
                format!("unknown / not implemented attribute '{}' at index {}", attribute_name, attribute_name_index))
            )
        };
    }

    fn read_annotation_element_values_vec(num_values: u2, buff: &mut BufReader<ZipFile<'b>>) -> Result<Vec<ElementValue>, Error> {
        let mut element_values: Vec<ElementValue> = Vec::with_capacity(num_values as usize);
        for _ in 0..num_values {
            match Self::read_annotation_element_value(buff) {
                Ok(res) => element_values.push(res),
                Err(_) => continue
            }
        }

        Ok(element_values)
    }

    fn read_annotations_element_value_pairs_vec(num_element_value_pairs: u2, buff: &mut BufReader<ZipFile<'b>>) -> Result<Vec<ElementValuePair>, Error> {
        let mut element_value_pairs: Vec<ElementValuePair> = Vec::with_capacity(num_element_value_pairs as usize);

        for _ in 0..num_element_value_pairs {
            let element_name_index = buff.read_u16::<BigEndian>()?;
            let element_value = Self::read_annotation_element_value(buff)?;
            element_value_pairs.push(ElementValuePair {
                element_name_index,
                value: element_value,
            });
        }

        Ok(element_value_pairs)
    }

    fn read_annotation_element_value(buff: &mut BufReader<ZipFile<'b>>) -> Result<ElementValue, Error> {
        let tag = buff.read_u8()?;

        Ok(ElementValue {
            tag,
            value: match tag as char {
                'B' => Value::ConstValueIndex { const_value_index: buff.read_u16::<BigEndian>()? },
                'C' => Value::ConstValueIndex { const_value_index: buff.read_u16::<BigEndian>()? },
                'D' => Value::ConstValueIndex { const_value_index: buff.read_u16::<BigEndian>()? },
                'F' => Value::ConstValueIndex { const_value_index: buff.read_u16::<BigEndian>()? },
                'I' => Value::ConstValueIndex { const_value_index: buff.read_u16::<BigEndian>()? },
                'J' => Value::ConstValueIndex { const_value_index: buff.read_u16::<BigEndian>()? },
                'S' => Value::ConstValueIndex { const_value_index: buff.read_u16::<BigEndian>()? },
                'Z' => Value::ConstValueIndex { const_value_index: buff.read_u16::<BigEndian>()? },
                's' => Value::ConstValueIndex { const_value_index: buff.read_u16::<BigEndian>()? },
                'e' => Value::EnumConstValue { type_name_index: buff.read_u16::<BigEndian>()?, const_name_index: buff.read_u16::<BigEndian>()? },
                'c' => Value::ClassInfoIndex { class_info_index: buff.read_u16::<BigEndian>()? },
                '@' => Value::AnnotationValue { annotation_value: Self::read_annotation(buff)? },
                '[' => {
                    let num_values = buff.read_u16::<BigEndian>()?;
                    let values = Self::read_annotation_element_values_vec(num_values, buff)?;
                    Value::ArrayValue { num_values, values }
                }
                _ => return Err(Error::new(ErrorKind::InvalidInput, format!("unknown annotation element value tag: {tag}")))
            },
        })
    }

    fn read_annotation(buff: &mut BufReader<ZipFile<'b>>) -> Result<Annotation, Error> {
        let type_index = buff.read_u16::<BigEndian>()?;
        let num_element_value_pairs = buff.read_u16::<BigEndian>()?;
        let element_value_pairs: Vec<ElementValuePair> = Self::read_annotations_element_value_pairs_vec(num_element_value_pairs, buff)?;

        Ok(Annotation {
            type_index,
            num_element_value_pairs,
            element_value_pairs,
        })
    }

    fn read_annotations_vec(num_annotations: u2, buff: &mut BufReader<ZipFile<'b>>) -> Result<Vec<Annotation>, Error> {
        let mut annotations: Vec<Annotation> = Vec::with_capacity(num_annotations as usize);

        for _ in 0..num_annotations {
            match Self::read_annotation(buff) {
                Ok(res) => annotations.push(res),
                Err(_) => continue
            };
        };

        Ok(annotations)
    }

    fn read_tag(buff: &mut BufReader<ZipFile<'b>>, mappings: Option<&LinkedHashMap<String, String>>) -> Result<ConstantPoolTags, Error> {
        let tag_byte = buff.read_u8()?;
        let tag = ConstantPoolJvmTag::from(tag_byte);
        match tag {
            ConstantPoolJvmTag::Utf8 => {
                let mut length = buff.read_u16::<BigEndian>()?;

                let mut bytes = vec![0u8; length as usize];
                buff.read_exact(&mut *bytes)?;
                let mut _value = String::from_utf8(
                    match cesu8::from_java_cesu8(bytes.as_ref()) {
                        Ok(res) => res.to_string().into_bytes(),
                        Err(_) => {
                            return Err(Error::new(
                                ErrorKind::InvalidInput,
                                format!("unable to read bytes string into utf8 string: {:?}", bytes))
                            );
                        }
                    }
                ).unwrap();

                match mappings {
                    None => {}
                    Some(mapping) => {
                        match mapping.get(&_value) {
                            None => {}
                            Some(val) => {
                                _value = val.clone();
                                bytes = cesu8::to_java_cesu8(&_value.as_str()).into_owned();
                                length = bytes.len() as u16;
                            }
                        }
                    }
                }


                Ok(ConstantPoolTags::Utf8 {
                    tag: ConstantPoolJvmTag::Utf8 as u8,
                    length,
                    bytes,
                    _value,
                })
            }

            ConstantPoolJvmTag::Integer => {
                let bytes = buff.read_u32::<BigEndian>()?;

                Ok(ConstantPoolTags::Integer {
                    tag: ConstantPoolJvmTag::Integer as u8,
                    bytes,
                    _value: bytes as i32,
                })
            }

            ConstantPoolJvmTag::Float => {
                let bytes = buff.read_u32::<BigEndian>()?;

                Ok(ConstantPoolTags::Float {
                    tag: ConstantPoolJvmTag::Float as u8,
                    bytes,
                    _value: bytes as f32,
                })
            }

            ConstantPoolJvmTag::Long => {
                let high_bytes = buff.read_u32::<BigEndian>()?;
                let low_bytes = buff.read_u32::<BigEndian>()?;
                let value = (((high_bytes as u64) << 32) | (low_bytes as u64)) as i64;

                Ok(ConstantPoolTags::Long {
                    tag: ConstantPoolJvmTag::Long as u8,
                    high_bytes,
                    low_bytes,
                    _value: value,
                })
            }

            ConstantPoolJvmTag::Double => {
                let high_bytes = buff.read_u32::<BigEndian>()?;
                let low_bytes = buff.read_u32::<BigEndian>()?;
                let value = (((high_bytes as u64) << 32) | (low_bytes as u64)) as f64;

                Ok(ConstantPoolTags::Double {
                    tag: ConstantPoolJvmTag::Double as u8,
                    high_bytes,
                    low_bytes,
                    _value: value,
                })
            }

            ConstantPoolJvmTag::String => {
                let string_index = buff.read_u16::<BigEndian>()?;

                Ok(ConstantPoolTags::String {
                    tag: ConstantPoolJvmTag::String as u8,
                    string_index,
                })
            }

            ConstantPoolJvmTag::Class => {
                let name_index = buff.read_u16::<BigEndian>()?;

                Ok(ConstantPoolTags::Class {
                    tag: ConstantPoolJvmTag::Class as u8,
                    name_index,
                })
            }

            ConstantPoolJvmTag::NameAndType => {
                let name_index = buff.read_u16::<BigEndian>()?;
                let descriptor_index = buff.read_u16::<BigEndian>()?;

                Ok(ConstantPoolTags::NameAndType {
                    tag: ConstantPoolJvmTag::Class as u8,
                    name_index,
                    descriptor_index,
                })
            }

            ConstantPoolJvmTag::Fieldref | ConstantPoolJvmTag::Methodref | ConstantPoolJvmTag::InterfaceMethodref => {
                let class_index = buff.read_u16::<BigEndian>()?;
                let name_and_type_index = buff.read_u16::<BigEndian>()?;

                Ok(match tag {
                    ConstantPoolJvmTag::Fieldref => ConstantPoolTags::Fieldref {
                        tag: ConstantPoolJvmTag::Fieldref as u8,
                        class_index,
                        name_and_type_index,
                    },
                    ConstantPoolJvmTag::Methodref => ConstantPoolTags::Methodref {
                        tag: ConstantPoolJvmTag::Fieldref as u8,
                        class_index,
                        name_and_type_index,
                    },
                    ConstantPoolJvmTag::InterfaceMethodref => ConstantPoolTags::InterfaceMethodref {
                        tag: ConstantPoolJvmTag::Fieldref as u8,
                        class_index,
                        name_and_type_index,
                    },
                    _ => panic!("pizdec, kak tak?")
                })
            }

            ConstantPoolJvmTag::MethodHandle => {
                let reference_kind = buff.read_u8()?;
                let reference_index = buff.read_u16::<BigEndian>()?;

                Ok(ConstantPoolTags::MethodHandle {
                    tag: ConstantPoolJvmTag::MethodHandle as u8,
                    reference_kind,
                    reference_index,
                })
            }

            ConstantPoolJvmTag::MethodType => {
                let descriptor_index = buff.read_u16::<BigEndian>()?;

                Ok(ConstantPoolTags::MethodType {
                    tag: ConstantPoolJvmTag::MethodType as u8,
                    descriptor_index,
                })
            }

            ConstantPoolJvmTag::Dynamic => {
                let bootstrap_method_attr_index = buff.read_u16::<BigEndian>()?;
                let name_and_type_index = buff.read_u16::<BigEndian>()?;

                Ok(ConstantPoolTags::Dynamic {
                    tag: ConstantPoolJvmTag::Dynamic as u8,
                    bootstrap_method_attr_index,
                    name_and_type_index,
                })
            }

            ConstantPoolJvmTag::InvokeDynamic => {
                let bootstrap_method_attr_index = buff.read_u16::<BigEndian>()?;
                let name_and_type_index = buff.read_u16::<BigEndian>()?;

                Ok(ConstantPoolTags::InvokeDynamic {
                    tag: ConstantPoolJvmTag::InvokeDynamic as u8,
                    bootstrap_method_attr_index,
                    name_and_type_index,
                })
            }

            ConstantPoolJvmTag::Module => {
                let name_index = buff.read_u16::<BigEndian>()?;

                Ok(ConstantPoolTags::Module {
                    tag: ConstantPoolJvmTag::Module as u8,
                    name_index,
                })
            }

            ConstantPoolJvmTag::Package => {
                let name_index = buff.read_u16::<BigEndian>()?;

                Ok(ConstantPoolTags::Package {
                    tag: ConstantPoolJvmTag::Package as u8,
                    name_index,
                })
            }

            ConstantPoolJvmTag::INVALID => {
                Err(Error::new(ErrorKind::InvalidInput, format!("tag {} not implemented!", tag_byte)))
            }
        }
    }
}