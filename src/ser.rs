use super::error::Result;
use super::types::*;

// TODO: implement complete serde serialiser (see ciborium for an example)

struct HeaderEntry {
    kind: u8,
    size: u16,
}

struct Encoder {
    version: u8,
    headers: Vec<HeaderEntry>,
    contents: Vec<Vec<u8>>,
}

// TODO: use proper encoder and not typecasting

impl Encoder {
    fn encode_types(types: Vec<EOFTypeSectionEntry>) -> Vec<u8> {
        types
            .into_iter()
            .flat_map(|type_entry| {
                vec![
                    type_entry.inputs,
                    type_entry.outputs,
                    (type_entry.max_stack_height >> 8) as u8,
                    (type_entry.max_stack_height & 0xff) as u8,
                ]
            })
            .collect()
    }

    fn push_section(&mut self, section: EOFSection) -> Result<()> {
        let section_kind = section.kind();

        // Encode content
        let content = match section {
            EOFSection::Code(code) => code,
            EOFSection::Data(data) => data,
            EOFSection::Type(types) => Self::encode_types(types),
        };

        let content_len = content.len();
        self.contents.push(content);

        // Store header
        self.headers.push(HeaderEntry {
            kind: section_kind,
            size: content_len as u16,
        });

        Ok(())
    }

    fn finalize(self) -> Result<Vec<u8>> {
        let mut type_headers: Vec<u8> = self.headers
            .iter()
            .filter(|header| header.kind == EOF_SECTION_TYPE)
            .flat_map(|header| {
                vec![
                    header.kind,
                    (header.size >> 8) as u8,
                    (header.size & 0xff) as u8,
                ]
            })
            .collect();

        let mut code_sizes: Vec<u8> = self.headers
            .iter()
            .filter(|header| header.kind == EOF_SECTION_CODE)
            .flat_map(|header| {
                vec![(header.size >> 8) as u8, (header.size & 0xff) as u8]
            })
            .collect();

        let mut code_header: Vec<u8> = vec![
            EOF_SECTION_CODE,
            ((code_sizes.len() / 2) >> 8) as u8,
            ((code_sizes.len() / 2) & 0xff) as u8,
        ];

        let mut data_header: Vec<u8> = self.headers
            .iter()
            .filter(|header| header.kind == EOF_SECTION_DATA)
            .flat_map(|header| {
                vec![
                    header.kind,
                    (header.size >> 8) as u8,
                    (header.size & 0xff) as u8,
                ]
            })
            .collect();

        let mut encoded_contents: Vec<u8> = self.contents.into_iter().flatten().collect();

        let mut ret = EOF_MAGIC.to_be_bytes().to_vec();
        ret.push(self.version);
        ret.append(&mut type_headers);
        ret.append(&mut code_header);
        ret.append(&mut code_sizes);
        ret.append(&mut data_header);
        ret.push(EOF_SECTION_TERMINATOR);
        ret.append(&mut encoded_contents);

        Ok(ret)
    }
}

pub fn to_bytes(value: EOFContainer) -> Result<Vec<u8>> {
    let mut encoder = Encoder {
        version: value.version,
        headers: vec![],
        contents: vec![],
    };
    for section in value.sections {
        encoder.push_section(section)?;
    }
    encoder.finalize()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_eof_bytes() {
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
                        max_stack_height: 0,
                    },
                ]),
                EOFSection::Code(vec![0xfe]),
                EOFSection::Code(vec![0xfe]),
                EOFSection::Data(vec![0, 1, 2, 3, 4]),
            ],
        };

        let serialized = to_bytes(container).unwrap();
        assert_eq!(
            hex::encode(serialized),
            "ef000101000802000200010001030005000000000001010000fefe0001020304"
        );
    }
}
