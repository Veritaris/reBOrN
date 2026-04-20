use byteorder::{BigEndian, ReadBytesExt};
use linked_hash_map::LinkedHashMap;
use std::io::{BufReader, Error, ErrorKind, Read, Seek};

use crate::access_flags::{AccessFlagContext, AccessFlags};
use crate::attributes::*;
use crate::classfile::*;
use crate::constant_pool_tag::{ConstantPoolJvmTag, ConstantPoolTag, CONTINUATION_TAG};
use crate::field::Field;
use crate::method::Method;
use crate::mutf8::read_modified_utf8;
use crate::type_alias;

impl<'a, 'b> ClassFile
where
    'b: 'a,
{
    pub fn tag_to_display(&self, tag: &ConstantPoolTag) -> String {
        match tag {
            ConstantPoolTag::Utf8 { bytes, length, .. } => {
                let bytes_stringified = match read_modified_utf8(&bytes) {
                    Ok(res) => res,
                    Err(err) => {
                        eprintln!("[TagDisplayError]: err={}, raw data: {:?}", err, bytes);
                        String::from("<error>")
                    }
                };
                format!(
                    "Utf8<length={}, bytes='{:?}', stringified='{}'>",
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
                format!(
                    "Class<name_index={}, content={}>",
                    name_index,
                    self.tag_to_display(val)
                )
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
                let name_and_type = self
                    .constant_pool
                    .get(*name_and_type_index as usize)
                    .unwrap();
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
                let name_and_type = self
                    .constant_pool
                    .get(*name_and_type_index as usize)
                    .unwrap();
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
                let name_and_type = self
                    .constant_pool
                    .get(*name_and_type_index as usize)
                    .unwrap();
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

    pub fn class_name_from_cp(&self) -> String {
        match self.constant_pool.get(self.this_class as usize) {
            Some(ConstantPoolTag::Class { name_index, .. }) => {
                match self.constant_pool.get(*name_index as usize) {
                    Some(ConstantPoolTag::Utf8 { bytes, .. }) => {
                        String::from_utf8(bytes.clone()).unwrap()
                    }
                    _ => String::new(),
                }
            }
            _ => String::new(),
        }
    }

    pub fn read<R>(
        mut buff: BufReader<R>,
        mappings: Option<&LinkedHashMap<String, String>>,
    ) -> Result<ClassFile, Error>
    where
        R: Read + Seek,
    {
        let magic = buff.read_u32::<BigEndian>()?;
        if magic != CLASS_HEADER {
            panic!("unexpected magic: {magic}");
        }

        let minor_version = buff.read_u16::<BigEndian>()?;
        let major_version = buff.read_u16::<BigEndian>()?;
        let constant_pool_count = buff.read_u16::<BigEndian>()?;
        let mut constant_pool: Vec<ConstantPoolTag> =
            Vec::with_capacity(constant_pool_count as usize);
        constant_pool.push(CONTINUATION_TAG);
        let mut read_bytes = buff.stream_position().unwrap_or(10);

        #[cfg(feature = "debug-logging")]
        {
            println!("    Reading tags");
        }
        let mut cp_index = 1;
        while cp_index < constant_pool_count {
            match ClassFile::read_tag(&mut buff, mappings) {
                Ok(tag) => {
                    #[cfg(feature = "debug-logging")]
                    println!(
                        "#{} tag {:?} at {}(0x{:x}) position",
                        cp_index, tag, read_bytes, read_bytes
                    );
                    match tag {
                        ConstantPoolTag::Long { .. } | ConstantPoolTag::Double { .. } => {
                            constant_pool.push(tag);
                            constant_pool.push(CONTINUATION_TAG);
                            cp_index += 1;
                        }
                        _ => constant_pool.push(tag),
                    };
                    read_bytes = buff
                        .stream_position()
                        .expect("error while fetching buffer position");
                }
                Err(err) => {
                    println!(
                        "skipping error tag at {}th iter: {} and pushing continuation tag; offset: {}",
                        cp_index, err, read_bytes
                    );
                    constant_pool.push(CONTINUATION_TAG);
                }
            };
            cp_index += 1;
        }

        #[cfg(feature = "debug-logging")]
        {
            let read_bytes = buff.stream_position().expect(
                format!(
                    "unexpected EOF after, previous success position: {}",
                    read_bytes
                )
                .as_str(),
            );
            println!(
                "    Reading class access flags at {} (0x{:x})",
                read_bytes, read_bytes
            );
        }
        let access_flags =
            AccessFlags::from((AccessFlagContext::Class, buff.read_u16::<BigEndian>()?));
        #[cfg(feature = "debug-logging")]
        {
            let read_bytes = buff.stream_position().expect(
                format!(
                    "unexpected EOF after, previous success position: {}",
                    read_bytes
                )
                .as_str(),
            );
            println!(
                "    Reading this class at {} (0x{:x})",
                read_bytes, read_bytes
            );
        }
        let this_class = buff.read_u16::<BigEndian>()?;
        #[cfg(feature = "debug-logging")]
        {
            let read_bytes = buff.stream_position().expect(
                format!(
                    "unexpected EOF after, previous success position: {}",
                    read_bytes
                )
                .as_str(),
            );
            println!(
                "    Reading super class at {} (0x{:x})",
                read_bytes, read_bytes
            );
        }
        let super_class = buff.read_u16::<BigEndian>()?;

        #[cfg(feature = "debug-logging")]
        {
            let read_bytes = buff.stream_position().expect(
                format!(
                    "unexpected EOF after, previous success position: {}",
                    read_bytes
                )
                .as_str(),
            );
            println!(
                "    Reading interfaces at {} (0x{:x})",
                read_bytes, read_bytes
            );
        }
        let interfaces_count = buff.read_u16::<BigEndian>()?;
        let mut interfaces: Vec<type_alias::u2> = Vec::with_capacity(interfaces_count as usize);
        for _ in 0..interfaces_count {
            interfaces.push(buff.read_u16::<BigEndian>()?);
        }

        #[cfg(feature = "debug-logging")]
        {
            let read_bytes = buff.stream_position().expect(
                format!(
                    "unexpected EOF after, previous success position: {}",
                    read_bytes
                )
                .as_str(),
            );
            println!("    Reading fields at {} (0x{:x})", read_bytes, read_bytes);
        }
        let fields_count = buff.read_u16::<BigEndian>()?;
        let mut fields: Vec<Field> = Vec::with_capacity(fields_count as usize);
        for _ in 0..fields_count {
            fields.push(Self::read_field(&constant_pool, &mut buff)?);
        }

        #[cfg(feature = "debug-logging")]
        {
            let read_bytes = buff.stream_position().expect(
                format!(
                    "unexpected EOF after, previous success position: {}",
                    read_bytes
                )
                .as_str(),
            );
            println!("    Reading methods at {} (0x{:x})", read_bytes, read_bytes);
        }
        let methods_count = buff.read_u16::<BigEndian>()?;
        let mut methods: Vec<Method> = Vec::with_capacity(methods_count as usize);
        for _ in 0..methods_count {
            methods.push(Self::read_method(&constant_pool, &mut buff)?);
        }

        #[cfg(feature = "debug-logging")]
        {
            let read_bytes = buff.stream_position().expect(
                format!(
                    "unexpected EOF after, previous success position: {}",
                    read_bytes
                )
                .as_str(),
            );
            println!(
                "    Reading attributes at {} (0x{:x})",
                read_bytes, read_bytes
            );
        }
        let attributes_count = buff.read_u16::<BigEndian>()?;
        let attributes: Vec<Attribute> =
            Self::read_attributes_vec(attributes_count, &constant_pool, &mut buff);

        let len = buff.stream_position().unwrap_or(0);
        #[cfg(feature = "debug-logging")]
        {
            println!("read {} bytes", len);
        }

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
    ///     type_alias::u2             access_flags;
    ///     type_alias::u2             name_index;
    ///     type_alias::u2             descriptor_index;
    ///     type_alias::u2             attributes_count;
    ///     attribute_info attributes[attributes_count];
    /// }
    ///```
    /// Oracle docs: https://docs.oracle.com/javase/specs/jvms/se22/html/jvms-4.html#jvms-4.5
    ///
    fn read_field<R>(
        constant_pool: &Vec<ConstantPoolTag>,
        buff: &mut BufReader<R>,
    ) -> Result<Field, Error>
    where
        R: Read + Seek,
    {
        let access_flags: type_alias::u2 = buff.read_u16::<BigEndian>()?;
        let name_index: type_alias::u2 = buff.read_u16::<BigEndian>()?;
        let descriptor_index: type_alias::u2 = buff.read_u16::<BigEndian>()?;
        let attributes_count: type_alias::u2 = buff.read_u16::<BigEndian>()?;
        #[cfg(feature = "debug-logging")]
        {
            println!("    Reading field attributes");
        }
        let attributes: Vec<Attribute> =
            Self::read_attributes_vec(attributes_count, &constant_pool, buff);

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
    ///     type_alias::u2             access_flags;
    ///     type_alias::u2             name_index;
    ///     type_alias::u2             descriptor_index;
    ///     type_alias::u2             attributes_count;
    ///     attribute_info attributes[attributes_count];
    /// }
    ///```
    /// Oracle docs: https://docs.oracle.com/javase/specs/jvms/se22/html/jvms-4.html#jvms-4.6
    ///
    fn read_method<R>(
        constant_pool: &Vec<ConstantPoolTag>,
        buff: &mut BufReader<R>,
    ) -> Result<Method, Error>
    where
        R: Read + Seek,
    {
        let access_flags = buff.read_u16::<BigEndian>()?;
        let name_index = buff.read_u16::<BigEndian>()?;
        let descriptor_index = buff.read_u16::<BigEndian>()?;
        let attributes_count = buff.read_u16::<BigEndian>()?;
        #[cfg(feature = "debug-logging")]
        {
            println!("    Reading method #{} (0x{:x})", name_index, name_index);
            println!("    Reading method attributes");
        }
        let attributes: Vec<Attribute> =
            Self::read_attributes_vec(attributes_count, &constant_pool, buff);

        Ok(Method {
            access_flags,
            name_index,
            descriptor_index,
            attributes_count,
            attributes,
        })
    }

    fn read_stack_frames_vec<R>(
        count: type_alias::u2,
        buff: &mut BufReader<R>,
    ) -> Vec<StackMapFrame>
    where
        R: Read + Seek,
    {
        let mut stack_frames: Vec<StackMapFrame> = Vec::with_capacity(count as usize);
        for i in 0..count {
            #[cfg(feature = "debug-logging")]
            {
                println!("    Reading {}th entry of StackMapTable", i);
            }
            match Self::read_stack_frame(buff) {
                Ok(res) => stack_frames.push(res),
                Err(err) => println!("error while reading {}th stack frame: {}", i, err),
            };
        }
        stack_frames
    }

    fn read_stack_frame<R>(buff: &mut BufReader<R>) -> Result<StackMapFrame, Error>
    where
        R: Read + Seek,
    {
        let frame_type = buff.read_u8()?;
        #[cfg(feature = "debug-logging")]
        {
            println!("    Frame type: {}", frame_type);
        }

        Ok(match frame_type {
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
        })
    }

    fn read_attributes_vec<R>(
        count: type_alias::u2,
        constant_pool: &Vec<ConstantPoolTag>,
        buff: &mut BufReader<R>,
    ) -> Vec<Attribute>
    where
        R: Read + Seek,
    {
        let mut attributes: Vec<Attribute> = Vec::with_capacity(count as usize);
        for _ in 0..count {
            match Self::read_attribute(constant_pool, buff) {
                Ok(res) => attributes.push(res),
                Err(err) => eprintln!("unable to read attribute err={err}"),
            };
        }
        attributes
    }

    ///
    /// Oracle docs: https://docs.oracle.com/javase/specs/jvms/se22/html/jvms-4.html#jvms-4.7
    ///
    fn read_attribute<R>(
        constant_pool: &Vec<ConstantPoolTag>,
        buff: &mut BufReader<R>,
    ) -> Result<Attribute, Error>
    where
        R: Read + Seek,
    {
        #[cfg(feature = "debug-logging")]
        {
            println!(
                "    Reading attribute name index at 0x{:x}",
                buff.stream_position()?
            );
        }
        let attribute_name_index = buff.read_u16::<BigEndian>()?;
        #[cfg(feature = "debug-logging")]
        {
            println!(
                "    Reading attribute length at 0x{:x}",
                buff.stream_position()?
            );
        }
        let attribute_length = buff.read_u32::<BigEndian>()?;

        let attribute_name = match constant_pool.get(attribute_name_index as usize) {
            Some(ConstantPoolTag::Utf8 { _value, .. }) => _value.as_str(),
            Some(e) => {
                return Err(Error::new(
                    ErrorKind::InvalidData,
                    format!(
                        "expected Utf8 tag at index {} in constant pool, got {}",
                        attribute_name_index, e
                    ),
                ))
            }
            None => {
                return Err(Error::new(
                    ErrorKind::NotFound,
                    format!(
                        "nothing found at index {} in constant pool",
                        attribute_name_index
                    ),
                ))
            }
        };
        #[cfg(feature = "debug-logging")]
        {
            println!("    Reading attribute {}", attribute_name);
        }

        match attribute_name {
            // critical to work on JVM
            "ConstantValue" => {
                #[cfg(feature = "debug-logging")]
                {
                    println!("    Reading ConstantValue attribute");
                }
                Ok(Attribute::ConstantValue {
                    attribute_name_index,
                    attribute_length,
                    constantvalue_index: buff.read_u16::<BigEndian>()?,
                })
            }
            "Code" => {
                #[cfg(feature = "debug-logging")]
                {
                    println!("    Reading Code attribute");
                }
                let max_stack = buff.read_u16::<BigEndian>()?;
                let max_locals = buff.read_u16::<BigEndian>()?;
                let code_length = buff.read_u32::<BigEndian>()?;
                let mut code: Vec<type_alias::u1> = vec![0u8; code_length as usize];
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
                }
                let attributes_count = buff.read_u16::<BigEndian>()?;
                let attributes: Vec<Attribute> =
                    Self::read_attributes_vec(attributes_count, constant_pool, buff);

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
                #[cfg(feature = "debug-logging")]
                {
                    println!("    Reading StackMapTable attribute");
                }
                let number_of_entries = buff.read_u16::<BigEndian>()?;
                #[cfg(feature = "debug-logging")]
                {
                    println!("    Found {} entries", number_of_entries);
                }
                let entries: Vec<StackMapFrame> =
                    Self::read_stack_frames_vec(number_of_entries, buff);

                Ok(Attribute::StackMapTable {
                    attribute_name_index,
                    attribute_length,
                    number_of_entries,
                    entries,
                })
            }
            "BootstrapMethods" => {
                #[cfg(feature = "debug-logging")]
                {
                    println!("    Reading BootstrapMethods attribute");
                }
                let num_bootstrap_methods = buff.read_u16::<BigEndian>()?;
                let mut bootstrap_methods: Vec<BootstrapMethodEntry> =
                    Vec::with_capacity(num_bootstrap_methods as usize);
                for _ in 0..num_bootstrap_methods {
                    let bootstrap_method_ref = buff.read_u16::<BigEndian>()?;
                    let num_bootstrap_arguments = buff.read_u16::<BigEndian>()?;
                    let mut bootstrap_arguments: Vec<type_alias::u2> =
                        Vec::with_capacity(num_bootstrap_arguments as usize);
                    for _ in 0..num_bootstrap_arguments {
                        bootstrap_arguments.push(buff.read_u16::<BigEndian>()?);
                    }
                    bootstrap_methods.push(BootstrapMethodEntry {
                        bootstrap_method_ref,
                        num_bootstrap_arguments,
                        bootstrap_arguments,
                    });
                }

                Ok(Attribute::BootstrapMethods {
                    attribute_name_index,
                    attribute_length,
                    num_bootstrap_methods,
                    bootstrap_methods,
                })
            }
            "NestHost" => {
                #[cfg(feature = "debug-logging")]
                {
                    println!("    Reading NestHost attribute");
                }
                Ok(Attribute::NestHost {
                    attribute_name_index,
                    attribute_length,
                    host_class_index: buff.read_u16::<BigEndian>()?,
                })
            }
            "NestMembers" => {
                #[cfg(feature = "debug-logging")]
                {
                    println!("    Reading NestMembers attribute");
                }
                let number_of_classes = buff.read_u16::<BigEndian>()?;
                let mut classes: Vec<type_alias::u2> =
                    Vec::with_capacity(number_of_classes as usize);
                for _ in 0..number_of_classes {
                    classes.push(buff.read_u16::<BigEndian>()?);
                }

                Ok(Attribute::NestMembers {
                    attribute_name_index,
                    attribute_length,
                    number_of_classes,
                    classes,
                })
            }
            "PermittedSubclasses" => {
                #[cfg(feature = "debug-logging")]
                {
                    println!("    Reading PermittedSubclasses attribute");
                }
                let number_of_classes = buff.read_u16::<BigEndian>()?;
                let mut classes: Vec<type_alias::u2> =
                    Vec::with_capacity(number_of_classes as usize);
                for _ in 0..number_of_classes {
                    classes.push(buff.read_u16::<BigEndian>()?);
                }

                Ok(Attribute::PermittedSubclasses {
                    attribute_name_index,
                    attribute_length,
                    number_of_classes,
                    classes,
                })
            }

            // optional, but critical for class libraries and instrumentation
            "Exceptions" => {
                #[cfg(feature = "debug-logging")]
                {
                    println!("    Reading Exceptions attribute");
                }
                let number_of_exceptions = buff.read_u16::<BigEndian>()?;
                let mut exception_index_table: Vec<type_alias::u2> =
                    Vec::with_capacity(number_of_exceptions as usize);
                for _ in 0..number_of_exceptions {
                    exception_index_table.push(buff.read_u16::<BigEndian>()?);
                }

                Ok(Attribute::Exceptions {
                    attribute_name_index,
                    attribute_length,
                    number_of_exceptions,
                    exception_index_table,
                })
            }
            "InnerClasses" => {
                #[cfg(feature = "debug-logging")]
                {
                    println!("    Reading InnerClasses attribute");
                }
                let number_of_classes = buff.read_u16::<BigEndian>()?;
                let mut classes: Vec<InnerClassEntry> =
                    Vec::with_capacity(number_of_classes as usize);
                for _ in 0..number_of_classes {
                    classes.push(InnerClassEntry {
                        inner_class_info_index: buff.read_u16::<BigEndian>()?,
                        outer_class_info_index: buff.read_u16::<BigEndian>()?,
                        inner_name_index: buff.read_u16::<BigEndian>()?,
                        inner_class_access_flags: buff.read_u16::<BigEndian>()?,
                    });
                }

                Ok(Attribute::InnerClasses {
                    attribute_name_index,
                    attribute_length,
                    number_of_classes,
                    classes,
                })
            }
            "EnclosingMethod" => {
                #[cfg(feature = "debug-logging")]
                {
                    println!("    Reading EnclosingMethod attribute");
                }
                Ok(Attribute::EnclosingMethod {
                    attribute_name_index,
                    attribute_length,
                    class_index: buff.read_u16::<BigEndian>()?,
                    method_index: buff.read_u16::<BigEndian>()?,
                })
            }
            "Synthetic" => {
                #[cfg(feature = "debug-logging")]
                {
                    println!("    Reading Synthetic attribute");
                }
                Ok(Attribute::Synthetic {
                    attribute_name_index,
                    attribute_length,
                })
            }
            "Signature" => {
                #[cfg(feature = "debug-logging")]
                {
                    println!("    Reading Signature attribute");
                }
                Ok(Attribute::Signature {
                    attribute_name_index,
                    attribute_length,
                    signature_index: buff.read_u16::<BigEndian>()?,
                })
            }
            "Record" => {
                #[cfg(feature = "debug-logging")]
                {
                    println!("    Reading Record attribute");
                }
                let components_count = buff.read_u16::<BigEndian>()?;
                let mut components: Vec<RecordComponentInfo> =
                    Vec::with_capacity(components_count as usize);
                for _ in 0..components_count {
                    let name_index = buff.read_u16::<BigEndian>()?;
                    let descriptor_index = buff.read_u16::<BigEndian>()?;
                    let attributes_count = buff.read_u16::<BigEndian>()?;
                    components.push(RecordComponentInfo {
                        name_index,
                        descriptor_index,
                        attributes_count,
                        attributes: Self::read_attributes_vec(
                            attributes_count,
                            constant_pool,
                            buff,
                        ),
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
                #[cfg(feature = "debug-logging")]
                {
                    println!("    Reading SourceFile attribute");
                }
                Ok(Attribute::SourceFile {
                    attribute_name_index,
                    attribute_length,
                    sourcefile_index: buff.read_u16::<BigEndian>()?,
                })
            }
            "LineNumberTable" => {
                #[cfg(feature = "debug-logging")]
                {
                    println!("    Reading LineNumberTable attribute");
                }
                let line_number_table_length = buff.read_u16::<BigEndian>()?;
                let mut line_number_table: Vec<LineNumberEntry> =
                    Vec::with_capacity(line_number_table_length as usize);
                for _ in 0..line_number_table_length {
                    line_number_table.push(LineNumberEntry {
                        start_pc: buff.read_u16::<BigEndian>()?,
                        line_number: buff.read_u16::<BigEndian>()?,
                    });
                }

                Ok(Attribute::LineNumberTable {
                    attribute_name_index,
                    attribute_length,
                    line_number_table_length,
                    line_number_table,
                })
            }
            "LocalVariableTable" => {
                #[cfg(feature = "debug-logging")]
                {
                    println!("    Reading LocalVariableTable attribute");
                }
                let local_variable_table_length = buff.read_u16::<BigEndian>()?;
                let mut local_variable_table: Vec<LocalVariableTableEntry> =
                    Vec::with_capacity(local_variable_table_length as usize);

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
                #[cfg(feature = "debug-logging")]
                {
                    println!("    Reading LocalVariableTypeTable attribute");
                }
                let local_variable_type_table_length = buff.read_u16::<BigEndian>()?;
                let mut local_variable_type_table: Vec<LocalVariableTypeTableEntry> =
                    Vec::with_capacity(local_variable_type_table_length as usize);

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
                #[cfg(feature = "debug-logging")]
                {
                    println!("    Reading SourceDebugExtension attribute");
                }
                let mut debug_extension_bytes: Vec<type_alias::u1> =
                    vec![0u8; attribute_length as usize];
                buff.read_exact(&mut *debug_extension_bytes)?;
                let debug_extension = match read_modified_utf8(&debug_extension_bytes.as_ref()) {
                    Ok(res) => res,
                    Err(err) => {
                        return Err(Error::new(
                            ErrorKind::InvalidInput,
                            format!(
                                "unable to read bytes string into utf8 string: {:?}, err={}",
                                debug_extension_bytes, err
                            ),
                        ));
                    }
                };

                Ok(Attribute::SourceDebugExtension {
                    attribute_name_index,
                    attribute_length,
                    debug_extension,
                })
            }
            "Deprecated" => {
                #[cfg(feature = "debug-logging")]
                {
                    println!("    Reading Deprecated attribute");
                }
                Ok(Attribute::Deprecated {
                    attribute_name_index,
                    attribute_length,
                })
            }
            "RuntimeVisibleAnnotations" | "RuntimeInvisibleAnnotations" => {
                #[cfg(feature = "debug-logging")]
                {
                    println!("    Reading RuntimeVisibleAnnotations or RuntimeInvisibleAnnotations attribute");
                }
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
                    _ => unreachable!(),
                })
            }
            "RuntimeVisibleParameterAnnotations" | "RuntimeInvisibleParameterAnnotations" => {
                #[cfg(feature = "debug-logging")]
                {
                    println!("    Reading RuntimeVisibleParameterAnnotations or RuntimeInvisibleParameterAnnotations attribute");
                }
                let num_parameters = buff.read_u8()?;
                let mut parameter_annotations: Vec<ParameterAnnotation> =
                    Vec::with_capacity(num_parameters as usize);
                for _ in 0..num_parameters {
                    let num_annotations = buff.read_u16::<BigEndian>()?;
                    let annotations = Self::read_annotations_vec(num_annotations, buff)?;
                    parameter_annotations.push(ParameterAnnotation {
                        num_annotations,
                        annotations,
                    })
                }

                Ok(match attribute_name {
                    "RuntimeVisibleParameterAnnotations" => {
                        Attribute::RuntimeVisibleParameterAnnotations {
                            attribute_name_index,
                            attribute_length,
                            num_parameters,
                            parameter_annotations,
                        }
                    }
                    "RuntimeInvisibleParameterAnnotations" => {
                        Attribute::RuntimeInvisibleParameterAnnotations {
                            attribute_name_index,
                            attribute_length,
                            num_parameters,
                            parameter_annotations,
                        }
                    }
                    _ => unreachable!(),
                })
            }
            "RuntimeVisibleTypeAnnotations" | "RuntimeInvisibleTypeAnnotations" => {
                #[cfg(feature = "debug-logging")]
                {
                    println!("    Reading RuntimeVisibleTypeAnnotations or RuntimeInvisibleTypeAnnotations attribute");
                }
                let num_parameters = buff.read_u16::<BigEndian>()?;
                let mut annotations: Vec<TypeAnnotation> =
                    Vec::with_capacity(num_parameters as usize);
                for _ in 0..num_parameters {
                    let target_type: type_alias::u1 = buff.read_u8()?;
                    let target_info = match target_type {
                        0x00 | 0x01 => TargetInfo::TypeParameterTarget {
                            type_parameter_index: buff.read_u8()?,
                        },
                        0x10 => TargetInfo::SupertypeTarget {
                            supertype_index: buff.read_u16::<BigEndian>()?,
                        },
                        0x11 | 0x12 => TargetInfo::TypeParameterBoundTarget {
                            type_parameter_index: buff.read_u8()?,
                            bound_index: buff.read_u8()?,
                        },
                        0x13 | 0x14 | 0x15 => TargetInfo::EmptyTarget {},
                        0x16 => TargetInfo::FormalParameterTarget {
                            formal_parameter_index: buff.read_u8()?,
                        },
                        0x17 => TargetInfo::ThrowsTarget {
                            throws_type_index: buff.read_u16::<BigEndian>()?,
                        },
                        0x40 | 0x41 => {
                            let table_length = buff.read_u16::<BigEndian>()?;
                            let mut table: Vec<LocalvarTargetTableEntry> =
                                Vec::with_capacity(table_length as usize);
                            for _ in 0..table_length {
                                table.push(LocalvarTargetTableEntry {
                                    start_pc: buff.read_u16::<BigEndian>()?,
                                    length: buff.read_u16::<BigEndian>()?,
                                    index: buff.read_u16::<BigEndian>()?,
                                });
                            }
                            TargetInfo::LocalvarTarget {
                                table_length,
                                table,
                            }
                        }
                        0x42 => TargetInfo::CatchTarget {
                            exception_table_index: buff.read_u16::<BigEndian>()?,
                        },
                        0x43 | 0x44 | 0x45 | 0x46 => TargetInfo::OffsetTarget {
                            offset: buff.read_u16::<BigEndian>()?,
                        },
                        0x47 | 0x48 | 0x49 | 0x4A | 0x4B => TargetInfo::TypeArgumentTarget {
                            offset: buff.read_u16::<BigEndian>()?,
                            type_argument_index: buff.read_u8()?,
                        },
                        _ => continue,
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
                        TypePath { path_length, path }
                    };
                    let type_index = buff.read_u16::<BigEndian>()?;
                    let num_element_value_pairs = buff.read_u16::<BigEndian>()?;
                    let element_value_pairs = Self::read_annotations_element_value_pairs_vec(
                        num_element_value_pairs,
                        buff,
                    )?;

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
                    "RuntimeInvisibleTypeAnnotations" => {
                        Attribute::RuntimeInvisibleTypeAnnotations {
                            attribute_name_index,
                            attribute_length,
                            num_parameters,
                            annotations,
                        }
                    }
                    _ => unreachable!(),
                })
            }
            "AnnotationDefault" => {
                #[cfg(feature = "debug-logging")]
                {
                    println!("    Reading AnnotationDefault attribute");
                }
                Ok(Attribute::AnnotationDefault {
                    attribute_name_index,
                    attribute_length,
                    default_value: Self::read_annotation_element_value(buff)?,
                })
            }
            "MethodParameters" => {
                #[cfg(feature = "debug-logging")]
                {
                    println!("    Reading MethodParameters attribute");
                }
                let parameters_count = buff.read_u8()?;
                let mut parameters: Vec<Parameter> = Vec::with_capacity(parameters_count as usize);
                for _ in 0..parameters_count {
                    parameters.push(Parameter {
                        name_index: buff.read_u16::<BigEndian>()?,
                        access_flags: AccessFlags::from((
                            AccessFlagContext::Module,
                            buff.read_u16::<BigEndian>()?,
                        )),
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
                #[cfg(feature = "debug-logging")]
                {
                    println!("    Reading Module attribute");
                }
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
                let mut uses_index: Vec<type_alias::u2> = Vec::with_capacity(uses_count as usize);
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
                #[cfg(feature = "debug-logging")]
                {
                    println!("    Reading ModulePackages attribute");
                }
                let package_count = buff.read_u16::<BigEndian>()?;
                let mut package_index: Vec<type_alias::u2> =
                    Vec::with_capacity(package_count as usize);
                for _ in 0..package_count {
                    package_index.push(buff.read_u16::<BigEndian>()?);
                }
                Ok(Attribute::ModulePackages {
                    attribute_name_index,
                    attribute_length,
                    package_count,
                    package_index,
                })
            }
            "ModuleMainClass" => {
                #[cfg(feature = "debug-logging")]
                {
                    println!("    Reading ModuleMainClass attribute");
                }
                Ok(Attribute::ModuleMainClass {
                    attribute_name_index,
                    attribute_length,
                    main_class_index: buff.read_u16::<BigEndian>()?,
                })
            }
            &_ => {
                let mut info: Vec<type_alias::u1> = vec![attribute_length as u8];
                if attribute_length > 0 {
                    buff.read_exact(info.as_mut())?;
                }
                Ok(Attribute::ExternalAttribute {
                    attribute_name_index,
                    attribute_length,
                    info,
                })
            }
        }
    }

    fn read_annotation_element_values_vec<R>(
        num_values: type_alias::u2,
        buff: &mut BufReader<R>,
    ) -> Result<Vec<ElementValue>, Error>
    where
        R: Read + Seek,
    {
        let mut element_values: Vec<ElementValue> = Vec::with_capacity(num_values as usize);
        for _ in 0..num_values {
            match Self::read_annotation_element_value(buff) {
                Ok(res) => element_values.push(res),
                Err(_) => continue,
            }
        }

        Ok(element_values)
    }

    fn read_annotations_element_value_pairs_vec<R>(
        num_element_value_pairs: type_alias::u2,
        buff: &mut BufReader<R>,
    ) -> Result<Vec<ElementValuePair>, Error>
    where
        R: Read + Seek,
    {
        let mut element_value_pairs: Vec<ElementValuePair> =
            Vec::with_capacity(num_element_value_pairs as usize);

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

    fn read_annotation_element_value<R>(buff: &mut BufReader<R>) -> Result<ElementValue, Error>
    where
        R: Read + Seek,
    {
        let tag = buff.read_u8()?;

        Ok(ElementValue {
            tag,
            value: match tag as char {
                'B' => Value::ConstValueIndex {
                    const_value_index: buff.read_u16::<BigEndian>()?,
                },
                'C' => Value::ConstValueIndex {
                    const_value_index: buff.read_u16::<BigEndian>()?,
                },
                'D' => Value::ConstValueIndex {
                    const_value_index: buff.read_u16::<BigEndian>()?,
                },
                'F' => Value::ConstValueIndex {
                    const_value_index: buff.read_u16::<BigEndian>()?,
                },
                'I' => Value::ConstValueIndex {
                    const_value_index: buff.read_u16::<BigEndian>()?,
                },
                'J' => Value::ConstValueIndex {
                    const_value_index: buff.read_u16::<BigEndian>()?,
                },
                'S' => Value::ConstValueIndex {
                    const_value_index: buff.read_u16::<BigEndian>()?,
                },
                'Z' => Value::ConstValueIndex {
                    const_value_index: buff.read_u16::<BigEndian>()?,
                },
                's' => Value::ConstValueIndex {
                    const_value_index: buff.read_u16::<BigEndian>()?,
                },
                'e' => Value::EnumConstValue {
                    type_name_index: buff.read_u16::<BigEndian>()?,
                    const_name_index: buff.read_u16::<BigEndian>()?,
                },
                'c' => Value::ClassInfoIndex {
                    class_info_index: buff.read_u16::<BigEndian>()?,
                },
                '@' => Value::AnnotationValue {
                    annotation_value: Self::read_annotation(buff)?,
                },
                '[' => {
                    let num_values = buff.read_u16::<BigEndian>()?;
                    let values = Self::read_annotation_element_values_vec(num_values, buff)?;
                    Value::ArrayValue { num_values, values }
                }
                _ => {
                    return Err(Error::new(
                        ErrorKind::InvalidInput,
                        format!("unknown annotation element value tag: {tag}"),
                    ))
                }
            },
        })
    }

    fn read_annotation<R>(buff: &mut BufReader<R>) -> Result<Annotation, Error>
    where
        R: Read + Seek,
    {
        let type_index = buff.read_u16::<BigEndian>()?;
        let num_element_value_pairs = buff.read_u16::<BigEndian>()?;
        let element_value_pairs: Vec<ElementValuePair> =
            Self::read_annotations_element_value_pairs_vec(num_element_value_pairs, buff)?;

        Ok(Annotation {
            type_index,
            num_element_value_pairs,
            element_value_pairs,
        })
    }

    fn read_annotations_vec<R>(
        num_annotations: type_alias::u2,
        buff: &mut BufReader<R>,
    ) -> Result<Vec<Annotation>, Error>
    where
        R: Read + Seek,
    {
        let mut annotations: Vec<Annotation> = Vec::with_capacity(num_annotations as usize);

        for _ in 0..num_annotations {
            match Self::read_annotation(buff) {
                Ok(res) => annotations.push(res),
                Err(_) => continue,
            };
        }

        Ok(annotations)
    }

    fn read_tag<R>(
        buff: &mut BufReader<R>,
        mappings: Option<&LinkedHashMap<String, String>>,
    ) -> Result<ConstantPoolTag, Error>
    where
        R: Read + Seek,
    {
        let tag_byte = buff.read_u8()?;
        let tag = ConstantPoolJvmTag::from(tag_byte);
        match tag {
            ConstantPoolJvmTag::Utf8 => {
                let mut length = buff.read_u16::<BigEndian>()?;

                let mut bytes = vec![0u8; length as usize];
                #[cfg(feature = "debug-logging")]
                {
                    println!("reading {} bytes of string", length);
                }
                buff.read_exact(&mut *bytes)?;
                let mut value_read_correctly = true;
                let mut _value = match read_modified_utf8(&bytes) {
                    Ok(res) => res,
                    Err(err) => {
                        println!(
                            "unable to read bytes string into utf8 string: {:?}, err={}",
                            bytes, err
                        );
                        value_read_correctly = false;
                        String::new()
                    }
                };

                if value_read_correctly {
                    match mappings {
                        None => {}
                        Some(mapping) => match mapping.get(&_value) {
                            None => {}
                            Some(val) => {
                                _value = val.clone();
                                bytes = Vec::from(_value.as_bytes());
                                length = bytes.len() as u16;
                            }
                        },
                    }
                }

                Ok(ConstantPoolTag::Utf8 {
                    tag: ConstantPoolJvmTag::Utf8 as u8,
                    length,
                    bytes,
                    _value,
                })
            }

            ConstantPoolJvmTag::Integer => {
                let bytes = buff.read_u32::<BigEndian>()?;

                Ok(ConstantPoolTag::Integer {
                    tag: ConstantPoolJvmTag::Integer as u8,
                    bytes,
                    _value: bytes as i32,
                })
            }

            ConstantPoolJvmTag::Float => {
                let bytes = buff.read_u32::<BigEndian>()?;

                Ok(ConstantPoolTag::Float {
                    tag: ConstantPoolJvmTag::Float as u8,
                    bytes,
                    _value: bytes as f32,
                })
            }

            ConstantPoolJvmTag::Long => {
                let high_bytes = buff.read_u32::<BigEndian>()?;
                let low_bytes = buff.read_u32::<BigEndian>()?;
                let value = (((high_bytes as u64) << 32) | (low_bytes as u64)) as i64;

                Ok(ConstantPoolTag::Long {
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

                Ok(ConstantPoolTag::Double {
                    tag: ConstantPoolJvmTag::Double as u8,
                    high_bytes,
                    low_bytes,
                    _value: value,
                })
            }

            ConstantPoolJvmTag::String => {
                let string_index = buff.read_u16::<BigEndian>()?;

                Ok(ConstantPoolTag::String {
                    tag: ConstantPoolJvmTag::String as u8,
                    string_index,
                })
            }

            ConstantPoolJvmTag::Class => {
                let name_index = buff.read_u16::<BigEndian>()?;

                Ok(ConstantPoolTag::Class {
                    tag: ConstantPoolJvmTag::Class as u8,
                    name_index,
                })
            }

            ConstantPoolJvmTag::NameAndType => {
                let name_index = buff.read_u16::<BigEndian>()?;
                let descriptor_index = buff.read_u16::<BigEndian>()?;

                Ok(ConstantPoolTag::NameAndType {
                    tag: ConstantPoolJvmTag::Class as u8,
                    name_index,
                    descriptor_index,
                })
            }

            ConstantPoolJvmTag::Fieldref
            | ConstantPoolJvmTag::Methodref
            | ConstantPoolJvmTag::InterfaceMethodref => {
                let class_index = buff.read_u16::<BigEndian>()?;
                let name_and_type_index = buff.read_u16::<BigEndian>()?;

                Ok(match tag {
                    ConstantPoolJvmTag::Fieldref => ConstantPoolTag::Fieldref {
                        tag: ConstantPoolJvmTag::Fieldref as u8,
                        class_index,
                        name_and_type_index,
                    },
                    ConstantPoolJvmTag::Methodref => ConstantPoolTag::Methodref {
                        tag: ConstantPoolJvmTag::Fieldref as u8,
                        class_index,
                        name_and_type_index,
                    },
                    ConstantPoolJvmTag::InterfaceMethodref => ConstantPoolTag::InterfaceMethodref {
                        tag: ConstantPoolJvmTag::Fieldref as u8,
                        class_index,
                        name_and_type_index,
                    },
                    _ => panic!("pizdec, kak tak?"),
                })
            }

            ConstantPoolJvmTag::MethodHandle => {
                let reference_kind = buff.read_u8()?;
                let reference_index = buff.read_u16::<BigEndian>()?;

                Ok(ConstantPoolTag::MethodHandle {
                    tag: ConstantPoolJvmTag::MethodHandle as u8,
                    reference_kind,
                    reference_index,
                })
            }

            ConstantPoolJvmTag::MethodType => {
                let descriptor_index = buff.read_u16::<BigEndian>()?;

                Ok(ConstantPoolTag::MethodType {
                    tag: ConstantPoolJvmTag::MethodType as u8,
                    descriptor_index,
                })
            }

            ConstantPoolJvmTag::Dynamic => {
                let bootstrap_method_attr_index = buff.read_u16::<BigEndian>()?;
                let name_and_type_index = buff.read_u16::<BigEndian>()?;

                Ok(ConstantPoolTag::Dynamic {
                    tag: ConstantPoolJvmTag::Dynamic as u8,
                    bootstrap_method_attr_index,
                    name_and_type_index,
                })
            }

            ConstantPoolJvmTag::InvokeDynamic => {
                let bootstrap_method_attr_index = buff.read_u16::<BigEndian>()?;
                let name_and_type_index = buff.read_u16::<BigEndian>()?;

                Ok(ConstantPoolTag::InvokeDynamic {
                    tag: ConstantPoolJvmTag::InvokeDynamic as u8,
                    bootstrap_method_attr_index,
                    name_and_type_index,
                })
            }

            ConstantPoolJvmTag::Module => {
                let name_index = buff.read_u16::<BigEndian>()?;

                Ok(ConstantPoolTag::Module {
                    tag: ConstantPoolJvmTag::Module as u8,
                    name_index,
                })
            }

            ConstantPoolJvmTag::Package => {
                let name_index = buff.read_u16::<BigEndian>()?;

                Ok(ConstantPoolTag::Package {
                    tag: ConstantPoolJvmTag::Package as u8,
                    name_index,
                })
            }

            ConstantPoolJvmTag::INVALID => Err(Error::new(
                ErrorKind::InvalidInput,
                format!("tag {} not implemented!", tag_byte),
            )),
        }
    }
}
