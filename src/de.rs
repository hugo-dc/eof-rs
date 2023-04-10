use super::error::{Error, Result};
use super::types::*;

use std::io::Read;

// TODO: implement complete serde serialiser (see ciborium for an example)

trait ExactReader {
    fn read_u8(&mut self) -> std::io::Result<u8>;
    fn read_u16(&mut self) -> std::io::Result<u16>;
    fn read_bytes(&mut self, len: usize) -> std::io::Result<Vec<u8>>;
}

impl ExactReader for &[u8] {
    fn read_u8(&mut self) -> std::io::Result<u8> {
        let mut tmp = [0u8];
        self.read_exact(&mut tmp)?;
        Ok(tmp[0])
    }

    fn read_u16(&mut self) -> std::io::Result<u16> {
        let mut tmp = [0u8; 2];
        self.read_exact(&mut tmp)?;
        Ok(u16::from_be_bytes(tmp))
    }

    fn read_bytes(&mut self, len: usize) -> std::io::Result<Vec<u8>> {
        let mut tmp = Vec::with_capacity(len);
        unsafe { tmp.set_len(len) };
        self.read_exact(&mut tmp[..])?;
        Ok(tmp)
    }
}

#[derive(Debug)]
struct HeaderEntry {
    kind: u8,
    size: u16,
}

struct Decoder {
    version: u8,
    headers: Vec<HeaderEntry>,
    contents: Vec<Vec<u8>>,
}

impl Decoder {
    fn new() -> Self {
        Self {
            version: 1,
            headers: vec![],
            contents: vec![],
        }
    }

    fn read(&mut self, v: &[u8]) -> Result<()> {
        let mut reader = v;

        if v.len() <= 1 {
            return Err(Error::InvalidMagic);
        }

        if (reader.read_u16()?) != EOF_MAGIC {
            return Err(Error::InvalidMagic);
        }

        match reader.read_u8() {
            Ok(version) => {
                if version != EOF_VERSION_1 {
                    return Err(Error::UnsupportedVersion);
                }
            }
            Err(_) => {
                return Err(Error::UnexpectedEOF);
            }
        }

        let mut type_section_found = false;
        let mut code_section_found = false;
        let mut data_section_found = false;
        // TODO: rewrite this to be more idiomatic
        loop {
            if let Ok(section_kind) = reader.read_u8() {
                if section_kind == EOF_SECTION_TERMINATOR {
                    break;
                }

                if reader.len() == 0 {
                    return Err(Error::IncompleteSections);
                }

                if reader.len() < 2 {
                    return Err(Error::IncompleteSectionSize);
                }

                let section_size = reader.read_u16()?;

                match section_kind {
                    EOF_SECTION_TYPE => {
                        if section_size % 4 != 0 {
                            return Err(Error::InvalidTypeSectionSize);
                        }
                        self.headers.push(HeaderEntry {
                            kind: section_kind,
                            size: section_size,
                        });
                        type_section_found = true;
                    }
                    EOF_SECTION_CODE => {
                        let mut c = 0;
                        loop {
                            if let Ok(code_size) = reader.read_u16() {
                                self.headers.push(HeaderEntry {
                                    kind: section_kind,
                                    size: code_size,
                                });

                                c+=1;
                                if c >= section_size {
                                    break;
                                }
                            } else {
                                break;
                            }
                        }
                        code_section_found = true;
                    },
                    EOF_SECTION_DATA => {
                        self.headers.push(HeaderEntry {
                            kind: section_kind,
                            size: section_size,
                        });
                        data_section_found = true;
                    }
                    _ => {
                        self.headers.push(HeaderEntry {
                            kind: section_kind,
                            size: section_size,
                        });
                    }
                }
            } else {
                return Err(Error::IncompleteSections);
            }
        }

        for i in 0..self.headers.len() {
            if reader.len() < self.headers[i].size as usize || (self.headers[i].size == 0 && self.headers[i].kind != EOF_SECTION_DATA ) {
                return Err(Error::InvalidCodeSize);
            }
            self.contents
                .push(reader.read_bytes(self.headers[i].size as usize)?);
        }

        if !type_section_found {
            return Err(Error::MissingTypeHeader);
        }

        if !code_section_found {
            return Err(Error::MissingCodeHeader);
        }

        if !data_section_found {
            return Err(Error::MissingDataHeader);
        }

        if reader.len() != 0 {
            return Err(Error::InvalidContainerSize);
        }

        Ok(())
    }

    fn finalize(self) -> Result<EOFContainer> {
        let mut container = EOFContainer {
            version: self.version,
            sections: vec![],
        };

        // TODO: make this idiomatic
        for i in 0..self.headers.len() {
            let kind = self.headers[i].kind;
            if kind == EOF_SECTION_CODE {
                container
                    .sections
                    .push(EOFSection::Code(self.contents[i].to_vec()));
            } else if kind == EOF_SECTION_DATA {
                container
                    .sections
                    .push(EOFSection::Data(self.contents[i].to_vec()));
            } else if kind == EOF_SECTION_TYPE {
                let mut reader = &self.contents[i][..];

                if reader.len() % 4 != 0 {
                    return Err(Error::InvalidTypeSectionSize);
                }

                let mut tmp: Vec<EOFTypeSectionEntry> = vec![];
                for _ in 0..(reader.len() / 4) {
                    tmp.push(EOFTypeSectionEntry {
                        inputs: reader.read_u8()?,
                        outputs: reader.read_u8()?,
                        max_stack_height: reader.read_u16()?,
                    });
                }
                container.sections.push(EOFSection::Type(tmp));
            } else {
                return Err(Error::UnsupportedSectionKind);
            }
        }

        Ok(container)
    }
}

pub fn from_slice(value: &[u8]) -> Result<EOFContainer> {
    let mut decoder = Decoder::new();
    decoder.read(value)?;
    decoder.finalize()
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: The following tests should only validate if EOF format is valid, for example,
    // there may exist a container with invalid version, version should be validated in validate.rs 
    #[test]
    fn decode_eof_bytes() {
        let input = hex::decode("ef000101000802000200010001030005000000000001010001fefe0001020304").unwrap();
        let container = EOFContainer {
            version: 1,
            sections: vec![
                EOFSection::Type(vec![
                    EOFTypeSectionEntry {
                        inputs: 0,
                        outputs: 0,
                        max_stack_height: 0,
                    },
                    EOFTypeSectionEntry {
                        inputs: 1,
                        outputs: 1,
                        max_stack_height: 1,
                    },
                ]),
                EOFSection::Code(vec![0xfe]),
                EOFSection::Code(vec![0xfe]),
                EOFSection::Data(vec![0, 1, 2, 3, 4]),
            ],
        };

        let deserialized = from_slice(&input[..]).unwrap();
        assert_eq!(deserialized, container);
    }

    #[test]
    fn unexpected_eof() {
        let input = hex::decode("ef00").unwrap();
        let deserialized = from_slice(&input[..]);
        assert_eq!(deserialized, Err(Error::UnexpectedEOF));
    }

    #[test]
    fn invalid_magic() {
        let input = hex::decode("ef0101010004020001002903000000000000027fef000101000402000100010300000000000000fe00000000000000000000000060005260146000f3").unwrap();
        let deserialized = from_slice(&input[..]);
        assert_eq!(deserialized, Err(Error::InvalidMagic));
    }

    #[test]
    fn invalid_version() {
        let input = hex::decode("ef0002010004020001002903000000000000027fef000101000402000100010300000000000000fe00000000000000000000000060005260146000f3").unwrap();
        let deserialized = from_slice(&input[..]);
        assert_eq!(deserialized, Err(Error::UnsupportedVersion));
    }

    #[test]
    fn invalid_type_section_size() {
        let input = hex::decode("ef000101000202000100010300000000000000fe").unwrap();
        let deserialized = from_slice(&input[..]);
        assert_eq!(deserialized, Err(Error::InvalidTypeSectionSize));
    }

    #[test]
    fn invalid_container_size() {
        let input = hex::decode("ef000101000402000100010300000000000000feaabbcc").unwrap();
        let deserialized = from_slice(&input[..]);
        assert_eq!(deserialized, Err(Error::InvalidContainerSize));
    }

    #[test]
    fn missing_type_header() {
        let input = hex::decode("ef0001020001000103000200feaabb").unwrap();
        let deserialized = from_slice(&input[..]);
        assert_eq!(deserialized, Err(Error::MissingTypeHeader));
    }

    #[test]
    fn missing_code_header() {
        let input = hex::decode("ef00010100040300020000000000feaabb").unwrap();
        let deserialized = from_slice(&input[..]);

        assert_eq!(
            deserialized,
            Err(Error::MissingCodeHeader)
        );
    }

    #[test]
    fn missing_data_header() {
        let input = hex::decode("ef000101000402000100010000000000fefe").unwrap();
        let deserialized = from_slice(&input[..]);
        assert_eq!(deserialized, Err(Error::MissingDataHeader));
    }


    // NOTE: This cannot happen because if the code ends without terminator, the
    // container format is not valid
    /*
    #[test]
    fn missing_terminator() {
        let input = hex::decode("ef0001010004020001000103000001010000fe").unwrap();
        let deserialized = from_slice(&input[..]);
        assert_eq!(deserialized, Err(Error::MissingTerminator));
    }
    */

}






