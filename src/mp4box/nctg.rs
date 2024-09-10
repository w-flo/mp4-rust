use std::io::{Read, Seek};

use crate::mp4box::*;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "json", derive(serde::Serialize))]
#[cfg_attr(feature = "json", serde(tag = "NCTG"))]
#[cfg_attr(feature = "json", serde(rename_all = "lowercase"))]
pub struct NctgBox {
    pub date_time_original: Option<String>,
    pub time_zone: Option<String>,
}

impl NctgBox {
    pub fn get_type(&self) -> BoxType {
        BoxType::NctgBox
    }

    pub fn get_size(&self) -> u64 {
        todo!();
    }
}

impl Mp4Box for NctgBox {
    fn box_type(&self) -> BoxType {
        self.get_type()
    }

    fn box_size(&self) -> u64 {
        self.get_size()
    }

    #[cfg(feature = "json")]
    fn to_json(&self) -> Result<String> {
        Ok(serde_json::to_string(&self).unwrap())
    }

    fn summary(&self) -> Result<String> {
        todo!();
    }
}

impl<R: Read + Seek> ReadBox<&mut R> for NctgBox {
    fn read_box(reader: &mut R, size: u64) -> Result<Self> {
        let start = box_start(reader)?;
        let end = start + size;

        let mut nctg = NctgBox::default();

        while reader.stream_position()? < end {
            let tag = reader.read_u32::<BigEndian>()?;
            let tag_type = reader.read_u16::<BigEndian>()?;
            let len = reader.read_u16::<BigEndian>()?;

            match tag_type {
                0x01 => reader.seek_relative(len.into())?, // u8
                0x02 => {
                    // String
                    let mut buf = vec![0_u8; len.into()];
                    reader.read_exact(&mut buf)?;
                    buf.pop();
                    let string = String::from_utf8(buf)?;

                    match tag {
                        0x0012 => nctg.date_time_original = Some(string),
                        0x0019 => nctg.time_zone = Some(string),
                        _ => {}
                    }
                }
                0x03 => reader.seek_relative((len * 2).into())?, // u16
                0x04 => reader.seek_relative((len * 4).into())?, // u32
                0x05 => reader.seek_relative((len * 8).into())?, // decimal
                0x07 => reader.seek_relative(len.into())?,       // byte buffer (?)
                _ => todo!(),
            }
        }

        skip_bytes_to(reader, start + size)?;

        Ok(nctg)
    }
}

impl<W: Write> WriteBox<&mut W> for NctgBox {
    fn write_box(&self, _writer: &mut W) -> Result<u64> {
        todo!();
    }
}
