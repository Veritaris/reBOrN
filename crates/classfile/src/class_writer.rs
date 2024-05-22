use std::fs::File;
use std::io::{BufWriter, Error, Write};

use byteorder::{BigEndian, WriteBytesExt};
use zip::{CompressionMethod, ZipWriter};
use zip::write::SimpleFileOptions;

use crate::classfile::ClassFile;

impl ClassFile {
    pub fn write(self, writer: &mut ZipWriter<BufWriter<File>>, filename: &str, compress_level: i64) -> Result<(), Error> {
        writer.start_file(filename, SimpleFileOptions::default().compression_method(CompressionMethod::Deflated).compression_level(Some(compress_level)))?;
        let data: Vec<u8> = self.try_into()?;
        writer.write(data.as_slice())?;
        Ok(())
    }
}

impl<'a> TryInto<Vec<u8>> for ClassFile {
    type Error = Error;

    fn try_into(self) -> Result<Vec<u8>, Self::Error> {
        let mut output = Vec::with_capacity(self._len as usize);
        output.write_u32::<BigEndian>(0xCAFEBABE)?;
        output.write_u16::<BigEndian>(self.minor_version)?;
        output.write_u16::<BigEndian>(self.major_version)?;
        output.write_u16::<BigEndian>(self.constant_pool_count)?;

        for entry in self.constant_pool {
            let bytes: Vec<u8> = entry.try_into()?;
            output.write(bytes.as_slice())?;
        }

        output.write_u16::<BigEndian>(self.access_flags.into())?;

        output.write_u16::<BigEndian>(self.this_class)?;
        output.write_u16::<BigEndian>(self.super_class)?;

        output.write_u16::<BigEndian>(self.interfaces_count)?;
        for interface_index in self.interfaces {
            output.write_u16::<BigEndian>(interface_index)?;
        }

        output.write_u16::<BigEndian>(self.fields_count)?;
        for field in self.fields {
            field.write(&mut output)?;
        }

        output.write_u16::<BigEndian>(self.methods_count)?;
        for method in self.methods {
            method.write(&mut output)?;
        }

        output.write_u16::<BigEndian>(self.attributes_count)?;
        for attr in self.attributes {
            attr.write(&mut output)?;
        }

        Ok(output)
    }
}