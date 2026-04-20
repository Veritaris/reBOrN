use crate::attributes::Attribute;
use crate::type_alias;
use byteorder::{BigEndian, WriteBytesExt};
use std::io::{Error, Write};

#[derive(Clone, Debug)]
pub struct Method {
    pub access_flags: type_alias::u2,
    pub name_index: type_alias::u2,
    pub descriptor_index: type_alias::u2,
    pub attributes_count: type_alias::u2,
    pub attributes: Vec<Attribute>,
}

impl Method {
    pub fn write(self, buff: &mut Vec<u8>) -> Result<usize, Error> {
        buff.write_u16::<BigEndian>(self.access_flags)?;
        buff.write_u16::<BigEndian>(self.name_index)?;
        buff.write_u16::<BigEndian>(self.descriptor_index)?;
        buff.write_u16::<BigEndian>(self.attributes_count)?;
        let mut bytes_written = size_of::<u16>() + size_of::<u16>() + size_of::<u16>() + size_of::<u16>();

        for attr in self.attributes {
            bytes_written += buff
                .write(TryInto::<Vec<u8>>::try_into(attr)?.as_slice())
                .unwrap_or_else(|err| {
                    eprintln!("{}", err);
                    0
                });
        }
        Ok(bytes_written)
    }
}

impl TryInto<Vec<u8>> for Method {
    type Error = Error;

    fn try_into(self) -> Result<Vec<u8>, Self::Error> {
        let mut output_bytes = Vec::new();

        output_bytes.write_u16::<BigEndian>(self.access_flags)?;
        output_bytes.write_u16::<BigEndian>(self.name_index)?;
        output_bytes.write_u16::<BigEndian>(self.descriptor_index)?;
        output_bytes.write_u16::<BigEndian>(self.attributes_count)?;

        for attr in self.attributes {
            output_bytes.write_all(TryInto::<Vec<u8>>::try_into(attr)?.as_slice())?;
        }

        Ok(output_bytes)
    }
}
