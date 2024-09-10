use std::io::{Read, Seek};

use crate::mp4box::nctg::NctgBox;
use crate::mp4box::*;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
#[cfg_attr(feature = "json", derive(serde::Serialize))]
pub struct NcdtBox {
    #[cfg_attr(feature = "json", serde(skip_serializing_if = "Option::is_none"))]
    pub nctg: Option<NctgBox>,
}

impl NcdtBox {
    pub fn get_type(&self) -> BoxType {
        BoxType::NcdtBox
    }

    pub fn get_size(&self) -> u64 {
        let mut size = HEADER_SIZE;
        if let Some(nctg) = &self.nctg {
            size += nctg.box_size();
        }
        size
    }
}

impl Mp4Box for NcdtBox {
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
        Ok(String::new())
    }
}

impl<R: Read + Seek> ReadBox<&mut R> for NcdtBox {
    fn read_box(reader: &mut R, size: u64) -> Result<Self> {
        let start = box_start(reader)?;

        let mut nctg = None;

        let mut current = reader.stream_position()?;
        let end = start + size;
        while current < end {
            // Get box header.
            let header = BoxHeader::read(reader)?;
            let BoxHeader { name, size: s } = header;
            if s > size {
                return Err(Error::InvalidData(
                    "ncdt box contains a box with a larger size than it",
                ));
            }

            match name {
                BoxType::NctgBox => {
                    nctg = Some(NctgBox::read_box(reader, s)?);
                }
                _ => {
                    // XXX warn!()
                    skip_box(reader, s)?;
                }
            }

            current = reader.stream_position()?;
        }

        skip_bytes_to(reader, start + size)?;

        Ok(NcdtBox { nctg })
    }
}

impl<W: Write> WriteBox<&mut W> for NcdtBox {
    fn write_box(&self, writer: &mut W) -> Result<u64> {
        let size = self.box_size();
        BoxHeader::new(self.box_type(), size).write(writer)?;

        if let Some(nctg) = &self.nctg {
            nctg.write_box(writer)?;
        }
        Ok(size)
    }
}
