use std::io;

const ONE_BYTE_MASK: u8 = 0b1000_0000;
const ONE_BYTE: u8 = 0b0000_0000;
const TWO_BYTE_MASK: u8 = 0b1110_0000;
const TWO_BYTE: u8 = 0b1100_0000;
const THREE_BYTE_MASK: u8 = 0b1111_0000;
const THREE_BYTE: u8 = 0b1110_0000;
const SURROGATE_BYTE_MASK: u8 = 0b1110_1101;
#[allow(unused)]
const SURROGATE_BYTE: u8 = 0b1110_1100;

pub fn read_modified_utf8(data: &[u8]) -> Result<String, io::Error> {
    let mut string = String::new();
    let mut iter = data.iter();

    while let Some(byte) = iter.next() {
        let code_point: &[u8] = if ((*byte & ONE_BYTE_MASK) ^ ONE_BYTE) == 0 {
            // *byte as u32
            &[*byte]
        } else if ((*byte & TWO_BYTE_MASK) ^ TWO_BYTE) == 0 {
            let x = *byte;
            let y = *match iter.next() {
                None => {
                    println!("Expected byte in two-byte code point, but no data left");
                    continue;
                }
                Some(it) => it,
            };
            &[x, y]
            // ((x & 0x1f) << 6u32) + (y & 0x3f)
        } else if ((*byte & THREE_BYTE_MASK) ^ THREE_BYTE) == 0 {
            let x = *byte;
            let y = *match iter.next() {
                None => {
                    println!("Expected byte in three-byte code point, but no data left");
                    continue;
                }
                Some(it) => it,
            };
            let z = *match iter.next() {
                None => {
                    println!("Expected byte in three-byte code point, but no data left");
                    continue;
                }
                Some(it) => it,
            };
            &[x, y, z]
            // ((x & 0xf) << 12u32) + ((y & 0x3f) << 6u32) + (z & 0x3f)
        } else if *byte == SURROGATE_BYTE_MASK {
            continue;
        } else {
            println!("[warn] unknown octet prefix: {}", *byte);
            string.push(char::REPLACEMENT_CHARACTER);
            continue;
        };

        if let Ok(c) = String::from_utf8(code_point.into()) {
            string.push_str(&c);
        } else {
            #[cfg(feature = "debug-logging")]
            println!("[warn] unable to convert {:?} to Unicode char", code_point);
            string.push(char::REPLACEMENT_CHARACTER);
        }
    }

    Ok(string)
}
