use std::io::{Error, Write};
use byteorder::{BigEndian, WriteBytesExt};
use crate::attributes::Attribute;
use crate::type_alias;

// #[repr(packed)]
pub struct Method {
    pub access_flags: type_alias::u2,
    pub name_index: type_alias::u2,
    pub descriptor_index: type_alias::u2,
    pub attributes_count: type_alias::u2,
    pub attributes: Vec<Attribute>,
}

impl Method {
    pub fn write(self, buff: &mut Vec<u8>) -> Result<(), Error> {
        buff.write_u16::<BigEndian>(self.access_flags)?;
        buff.write_u16::<BigEndian>(self.name_index)?;
        buff.write_u16::<BigEndian>(self.descriptor_index)?;
        buff.write_u16::<BigEndian>(self.attributes_count)?;

        for attr in self.attributes {
            let bytes: Vec<u8> = attr.try_into()?;
            buff.write(bytes.as_slice())?;
        }
        Ok(())
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
            let bytes: Vec<u8> = attr.try_into()?;
            output_bytes.write(bytes.as_slice())?;
        }

        Ok(output_bytes)
    }
}