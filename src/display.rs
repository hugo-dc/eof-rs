use super::types::*;
use std::fmt;

// TODO: implement this nicely
// TODO: use a markdown library

impl fmt::Display for EOFContainer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            r#""EOF Version {}

| # | Kind | Size | Content |
|---|------|------|---------|
"#,
            self.version,
        )?;
        for i in 0..self.sections.len() {
            match self.sections[i] {
                EOFSection::Code(ref code) => {
                    writeln!(
                        f,
                        //"| {} | Code | {} | {} |",
                        "**Section #{}**\n(len: {})\n{}\n",
                        i,
                        code.len(),
                        hex::encode(code)
                    )?
                }
                EOFSection::Data(ref data) => {
                    writeln!(
                        f,
                        "| {} | Data | {} | {} |",
                        i,
                        data.len(),
                        hex::encode(data)
                    )?
                }
                EOFSection::Type(ref types) => {
                    //                    let type_str = types
                    //                        .iter()
                    //                        .flat_map(|type_entry| {
                    //                            format!("{}->{}", type_entry.inputs, type_entry.outputs)
                    //                        })
                    //                        .collect();
                    //let type_str: Vec<EOFTypeSectionEntry> = types.iter().collect();
                    let type_str = "";

                    writeln!(f, "| {} | Type | {} | {} |", i, types.len() * 2, type_str)?
                }
            }
        }

        write!(f, "")
    }
}

//impl fmt::Display for EOFSection {
//}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::de::*;

    #[test]
    fn complex_container() {
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
        println!("{}", container);
        let formatted = format!("{}", container);
        //        assert!(container.is_valid_eof().is_ok());
        println!("{}", termimad::inline(&formatted));
    }

    #[test]
    fn andreis_code() {
        let bin = "ef000101000c020003003b0017001d0300000000000004010100030101000460043560003560e01c63c766526781145d001c63c6c2ea1781145d00065050600080fd50b0000260005260206000f350b0000160005260206000f3600181115d0004506001b160018103b0000181029050b1600281115d0004506001b160028103b0000260018203b00002019050b1";
        let input = hex::decode(bin).unwrap();
        let deserialized = from_slice(&input[..]).unwrap();
        let formatted = format!("{}", deserialized);
        println!("{}", termimad::inline(&formatted));
    }
}
