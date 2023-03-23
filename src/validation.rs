use std::collections::HashSet;
use std::collections::HashMap;

use super::error::{Error, Result};
use super::types::*;
use super::opcodes::*;
use super::de::*; // For testing only


pub fn validate_code(function_id: usize, code: &Vec<u8>, types: &[EOFTypeSectionEntry]) -> Result<()> {
    let mut worklist: HashMap<u16, (u16, bool)> = HashMap::new();
    let mut stack_heights: HashMap<u16, u16> = HashMap::new();
    let mut immediates: HashSet<u16> = HashSet::new();
    let mut rjumpdests: HashSet<u16> = HashSet::new();
    let mut current_stack_height: u16 = types[function_id].inputs as u16;
    let mut max_stack_height: u16 = current_stack_height;
    let mut ends_with_terminating_instruction = false;
    let mut visiting = true;
    let mut i = 0;

    worklist.insert(i as u16, (current_stack_height, visiting));
    loop {
        if i >= code.len() {
            break;
        }

        if let Ok(op) = OpCode::from(code[i]) {
            if current_stack_height < (op.stack_inputs as u16) {
                return Err(Error::StackUnderflow);
            }

            stack_heights.insert(i as u16, current_stack_height);
            current_stack_height = current_stack_height - (op.stack_inputs as u16) + (op.stack_outputs as u16);

            if current_stack_height > max_stack_height {
                max_stack_height = current_stack_height;
            }

            if op.immediates as usize > code[i+1..].len() {
                return Err(Error::TruncatedImmediate);
            }
            if op.name == "CALLF" {
                let section: [u8; 2] = code[i+1..i+3].try_into().unwrap();
                let section = u16::from_be_bytes(section);

                if section as usize >= types.len() {
                    return Err(Error::InvalidSectionArgument);
                }

                if current_stack_height + types[section as usize].max_stack_height as u16 > 1024 {
                    return Err(Error::StackOverflow);
                }
            }
            if op.name == "RJUMP" || op.name == "RJUMPI" {
                let offset: [u8; 2] = code[i+1..i+3].try_into().unwrap();
                let offset = i16::from_be_bytes(offset);
                let dest = (i + 1 + op.immediates as usize) as i32 + offset as i32;

                if dest as usize >= code.len() {
                    return Err(Error::InvalidJumpdest);
                }
                rjumpdests.insert(dest as u16);

                if op.name == "RJUMPI" {
                    worklist.insert(dest as u16, (current_stack_height, visiting));
                }
            }

            if op.name == "RJUMPV" {
                let count = code[i+1];
                if count == 0 {
                    return Err(Error::InvalidBranchCount);
                }
                // Add immediates
                let imm_pc: Vec<u16> = ((i+2) as u16..((i+2+(count *2) as usize) as u16)).collect();
                let imm: HashSet<u16> = HashSet::from_iter(imm_pc.iter().cloned());
                immediates = immediates.union(&imm).cloned().collect();

                let inst_end = i + 1 + op.immediates as usize + (count as usize * 2);

                rjumpdests.insert(inst_end as u16);
                worklist.insert(inst_end as u16, (current_stack_height, visiting));
                for j in 0..(count as usize) {
                    let offset: [u8; 2] = code[i+2+(j*2)..i+4+(j*2)].try_into().unwrap();
                    let offset = i16::from_be_bytes(offset);
                    let dest = inst_end as i32 + offset as i32;
                    if dest as usize >= code.len() {
                        return Err(Error::InvalidJumpdest);
                    }
                    rjumpdests.insert(dest as u16);
                    worklist.insert(dest as u16, (current_stack_height, visiting));
                }
                i += count as usize * 2;
            }

            if op.name == "RETF" {
                if current_stack_height != types[function_id].outputs as u16 {
                    return Err(Error::InvalidOutputs);
                }
            }

            let imm_pc: Vec<u16> = ((i+1) as u16..((i+1+op.immediates as usize) as u16)).collect();
            let imm: HashSet<u16> = HashSet::from_iter(imm_pc.iter().cloned());
            immediates = immediates.union(&imm).cloned().collect();
            i += 1 + op.immediates as usize;

            if op.is_terminating || op.name == "RJUMP" {
                ends_with_terminating_instruction = true;
                visiting = false;
            } else {
                ends_with_terminating_instruction = false;
            }
        } else {
            return Err(Error::UndefinedInstruction(code[i]));
        }
    }

    if !immediates.is_disjoint(&rjumpdests) {
        return Err(Error::InvalidJumpdest);
    }

    // check elements in worklist for stack height
    for (pc, (stack_height, _)) in worklist.iter() {
        if let Some(sh) = stack_heights.get(pc) {
            if sh != stack_height {
                return Err(Error::ConflictingStack);
            }
        }
    }
 

    if max_stack_height != types[function_id].max_stack_height as u16 {
        return Err(Error::InvalidMaxStackHeight);
    }

    if !ends_with_terminating_instruction {
        return Err(Error::InvalidCodeTermination);
    }

    i = 0;
    loop {
        if i >= code.len() {
            break;
        }
        if worklist.contains_key(&(i as u16)) {
            match worklist.get(&(i as u16)) {
                Some((_, v)) => visiting = *v,
                None => visiting = false,
            }
        }

        let op = OpCode::from(code[i]).unwrap();
        if visiting == false {
            return Err(Error::UnreachableCode);
        }

        if op.is_terminating || op.name == "RJUMPI" || op.name == "RJUMPV" {
            visiting = false;
        }

        if op.name == "RJUMPV" {
            let count = code[i+1];
            i += 1 + op.immediates as usize + (count as usize * 2);
        } else {
            i += 1 + op.immediates as usize;
        }
    }

    Ok(())
}

pub trait EOFValidator {
    fn is_valid_eof(&self) -> Result<()>;
}

impl EOFValidator for EOFContainer {
    fn is_valid_eof(&self) -> Result<()> {
        if self.version != EOF_VERSION_1 {
            return Err(Error::UnsupportedVersion);
        }

        if self.sections.is_empty() {
            return Err(Error::NoSections);
        }

        //let mut code_found = false;
        let mut code_count = 0;
        let mut data_found = false;
        let mut type_found: Option<usize> = None;
        let mut last_section_priority = 0u8;

        let mut error = None;
        for i in 0..self.sections.len() {
            let current_priority = self.sections[i].priority();
            if last_section_priority > current_priority {
                error = Some(Error::InvalidSectionOrder);
            }
            last_section_priority = self.sections[i].priority();

            match &self.sections[i] {
                EOFSection::Type(_) => {
                    if type_found.is_some() {
                        return Err(Error::DuplicateTypeSection);
                    }
                    type_found = Some(i);
                }
                EOFSection::Code(c) => {
                    if c.len() == 0 {
                        return Err(Error::InvalidCodeSize);
                    }
                    code_count += 1;
                }
                EOFSection::Data(_) => {
                    data_found = true;
                }
            }
        }

        // TODO: Remove this, it is validated below
        if !type_found.is_some() {
            return Err(Error::MissingTypeHeader);
        }

        if code_count == 0 {
            return Err(Error::MissingCodeHeader);
        }

        if !data_found {
            return Err(Error::MissingDataHeader);
        }

        if let Some(err) = error {
            return Err(err);
        }

        if let Some(type_found) = type_found {
            if let EOFSection::Type(ref types) = self.sections[type_found] {
                if types.len() != code_count {
                    return Err(Error::InvalidCodeHeader);
                }

                // Validate max inputs, outputs and stack height
                for i in 0..types.len() {
                    if types[i].inputs > 127 {
                        return Err(Error::TooManyInputs);
                    }
                    if types[i].outputs > 127 {
                        return Err(Error::TooManyOutputs);
                    }
                    if types[i].max_stack_height > 1024 {
                        return Err(Error::TooLargeMaxStackHeight);
                    }
                    if i == 0 && (types[i].inputs != 0 || types[i].outputs != 0) {
                        return Err(Error::InvalidSection0Type);
                    }
                }

                // Iterate over code sections and validate each one.
                let mut code_sections_count = 0;
                for i in 0..self.sections.len() {
                    if let EOFSection::Code(ref code) = self.sections[i] {
                        validate_code(code_sections_count, code, types)?;
                        code_sections_count += 1;
                    }
                }

                let types_count = types.len();
                if code_sections_count != types_count {
                    return Err(Error::InvalidCodeHeader);
                }
            } else {
                panic!(); // In case the above logic is wrong.
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
                        inputs: 0,
                        outputs: 0,
                        max_stack_height: 0,
                    },
                ]),
                EOFSection::Code(vec![0xfe]),
                EOFSection::Code(vec![0xfe]),
                EOFSection::Data(vec![0, 1, 2, 3, 4]),
            ],
        };

        assert_eq!(container.is_valid_eof().is_ok(), true);
    }

    #[test]
    fn valid_data_container() {
        let container = EOFContainer {
            version: 1,
            sections: vec![
                EOFSection::Type(vec![
                    EOFTypeSectionEntry {
                        inputs: 0,
                        outputs: 0,
                        max_stack_height: 0,
                    }
                ]),
                EOFSection::Code(vec![0xfe]),
                EOFSection::Data(vec![0, 1, 2, 3, 4]),
                EOFSection::Data(vec![0, 1, 2, 3, 4]),
            ],
        };
        assert!(container.is_valid_eof().is_ok());
    }

    #[test]
    fn unsupported_version() {
        let container = EOFContainer {
            version: 2,
            sections: vec![
                EOFSection::Code(vec![0xfe]),
                EOFSection::Data(vec![0, 1, 2, 3, 4]),
            ],
        };
        assert_eq!(
            container.is_valid_eof().err(),
            Some(Error::UnsupportedVersion)
        );
    }

    #[test]
    fn no_sections() {
        let container = EOFContainer {
            version: 1,
            sections: vec![],
        };
        assert_eq!(container.is_valid_eof().err(), Some(Error::NoSections));
    }

    #[test]
    fn no_code() {
        let container = EOFContainer {
            version: 1,
            sections: vec![
                EOFSection::Type(vec![EOFTypeSectionEntry {
                    inputs: 0,
                    outputs: 0,
                    max_stack_height: 0,
                }]),
                EOFSection::Data(vec![0xfe])],
        };
        assert_eq!(
            container.is_valid_eof().err(),
            Some(Error::MissingCodeHeader)
        );
    }

    #[test]
    fn code_before_types() {
        let container = EOFContainer {
            version: 1,
            sections: vec![
                EOFSection::Code(vec![0xfe]),
                EOFSection::Type(vec![EOFTypeSectionEntry {
                    inputs: 0,
                    outputs: 0,
                    max_stack_height: 0,
                }]),
                EOFSection::Data(vec![]),
            ],
        };
        assert_eq!(
            container.is_valid_eof().err(),
            Some(Error::InvalidSectionOrder)
        );
    }

    #[test]
    fn data_before_types() {
        let container = EOFContainer {
            version: 1,
            sections: vec![
                EOFSection::Data(vec![0, 1, 2, 3, 4]),
                EOFSection::Type(vec![EOFTypeSectionEntry {
                    inputs: 0,
                    outputs: 0,
                    max_stack_height: 0,
                }]),
                EOFSection::Code(vec![0xfe]),
            ],
        };
        assert_eq!(
            container.is_valid_eof().err(),
            Some(Error::InvalidSectionOrder)
        );
    }

    #[test]
    fn data_before_code() {
        let container = EOFContainer {
            version: 1,
            sections: vec![
                EOFSection::Type(vec![EOFTypeSectionEntry {
                    inputs: 0,
                    outputs: 0,
                    max_stack_height: 0,
                }]),
                EOFSection::Data(vec![0, 1, 2, 3, 4]),
                EOFSection::Code(vec![0xfe]),
            ],
        };
        assert_eq!(
            container.is_valid_eof().err(),
            Some(Error::InvalidSectionOrder)
        );
    }

    #[test]
    fn type_after_code() {
        let container = EOFContainer {
            version: 1,
            sections: vec![
                EOFSection::Code(vec![0xfe]),
                EOFSection::Type(vec![EOFTypeSectionEntry {
                    inputs: 0,
                    outputs: 0,
                    max_stack_height: 0,
                }]),
                EOFSection::Data(vec![]),
            ],
        };
        assert_eq!(
            container.is_valid_eof().err(),
            Some(Error::InvalidSectionOrder)
        );
    }

    #[test]
    fn multiple_type_sections() {
        let container = EOFContainer {
            version: 1,
            sections: vec![
                EOFSection::Type(vec![EOFTypeSectionEntry {
                    inputs: 0,
                    outputs: 0,
                    max_stack_height: 0,
                }]),
                EOFSection::Type(vec![EOFTypeSectionEntry {
                    inputs: 1,
                    outputs: 1,
                    max_stack_height: 0,
                }]),
                EOFSection::Code(vec![0xfe]),
            ],
        };
        assert_eq!(
            container.is_valid_eof().err(),
            Some(Error::DuplicateTypeSection)
        );
    }

    #[test]
    fn more_type_than_code() {
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
                EOFSection::Data(vec![]),
            ],
        };
        assert_eq!(
            container.is_valid_eof().err(),
            Some(Error::InvalidCodeHeader)
        );
    }

    #[test]
    fn more_code_than_type() {
        let container = EOFContainer {
            version: 1,
            sections: vec![
                EOFSection::Type(vec![EOFTypeSectionEntry {
                    inputs: 0,
                    outputs: 0,
                    max_stack_height: 0,
                }]),
                EOFSection::Code(vec![0xfe]),
                EOFSection::Code(vec![0xfe]),
                EOFSection::Data(vec![]),
            ],
        };
        assert_eq!(
            container.is_valid_eof().err(),
            Some(Error::InvalidCodeHeader)
        );

        for col in (0..3).rev() {
            for row in 0..5 {
            println!("{} {}", col, row);
            }
        }

    }

    #[test]
    fn missing_type_header() {
        let container = EOFContainer {
            version: 1,
            sections: vec![
                EOFSection::Code(vec![0xfe]),
            ],
        };

        assert_eq!(
            container.is_valid_eof().err(),
            Some(Error::MissingTypeHeader)
        );

        let container = EOFContainer {
            version: 1,
            sections: vec![
                EOFSection::Code(vec![0xfe]),
                EOFSection::Data(vec![0xaa, 0xbb]),
            ],
        };


        assert_eq!(
            container.is_valid_eof().err(),
            Some(Error::MissingTypeHeader)
        );

    }

    #[test]
    fn missing_code_header() {
        let container = EOFContainer {
            version: 1,
            sections: vec![
                EOFSection::Type(vec![EOFTypeSectionEntry {
                    inputs: 0,
                    outputs: 0,
                    max_stack_height: 0,
                }]),
                EOFSection::Data(vec![0xaa, 0xbb]),
            ],
        };

        assert_eq!(
            container.is_valid_eof().err(),
            Some(Error::MissingCodeHeader)
        );
    }

    #[test]
    fn invalid_code_header() {
        let code = hex::decode("ef00010100080200010001030000000000000000000000fe").unwrap();
        let container = from_slice(&code).unwrap();

        assert_eq!(
            container.is_valid_eof().err(),
            Some(Error::InvalidCodeHeader)
        );
    }

    #[test]
    fn missing_data_header() {
        let container = EOFContainer {
            version: 1,
            sections: vec![
                EOFSection::Type(vec![EOFTypeSectionEntry {
                    inputs: 0,
                    outputs: 0,
                    max_stack_height: 0,
                }]),
                EOFSection::Code(vec![0xfe, 0xfe]),
            ],
        };

        assert_eq!(
            container.is_valid_eof().err(),
            Some(Error::MissingDataHeader)
        );
    }

    #[test]
    fn too_many_inputs() {
        // TODO: Test from 128 to 255
        let code = hex::decode("ef0001010004020001000103000000ff000000fe").unwrap();
        let container = from_slice(&code).unwrap();

        assert_eq!(
            container.is_valid_eof().err(),
            Some(Error::TooManyInputs)
        );
    }

    #[test]
    fn too_many_outputs() {
        let code = hex::decode("ef000101000402000100010300000000ff0000fe").unwrap();
        let container = from_slice(&code).unwrap();

        assert_eq!(
            container.is_valid_eof().err(),
            Some(Error::TooManyOutputs)
        );
    }

    #[test]
    fn too_large_max_stack_height() {
        // TODO: Test from 1024 to 0xffff 
        let code = hex::decode("ef00010100040200010001030000000000fffffe").unwrap();
        let container = from_slice(&code).unwrap();

        assert_eq!(
            container.is_valid_eof().err(),
            Some(Error::TooLargeMaxStackHeight)
        );
    }

    #[test]
    fn invalid_section_0_type() {
        let code = hex::decode("ef000101000402000100010300000001000000fe").unwrap();
        let container = from_slice(&code).unwrap();

        assert_eq!(
            container.is_valid_eof().err(),
            Some(Error::InvalidSection0Type)
        );

        let code = hex::decode("ef000101000402000100010300000000010000fe").unwrap();
        let container = from_slice(&code).unwrap();

        assert_eq!(
            container.is_valid_eof().err(),
            Some(Error::InvalidSection0Type)
        );
    }

    #[test]
    fn invalid_code_size() {
        // TODO: Is this valid format?
        /*
        let code = hex::decode("ef00010100040200000300000000000000").unwrap();
        let container = from_slice(&code).unwrap();

        assert_eq!(
            container.is_valid_eof().err(),
            Some(Error::InvalidCodeSize)
        );
        */

        let code = hex::decode("ef000101000402000100000300000000000000").unwrap();
        let container = from_slice(&code[..]).unwrap();

        assert_eq!(
            container.is_valid_eof().err(),
            Some(Error::InvalidCodeSize)
        );

    }


    #[test]
    fn undefined_instruction() {
        let code = hex::decode("ef00010100040200010001030000000000000056").unwrap();
        let container = from_slice(&code).unwrap();
        assert_eq!(container.is_valid_eof().err(), 
            Some(Error::UndefinedInstruction(0x56)));

        let code = hex::decode("ef000101000402000100010300000000000000b3").unwrap();
        let container = from_slice(&code).unwrap();
        assert_eq!(container.is_valid_eof().err(), 
            Some(Error::UndefinedInstruction(0xb3)));

    }

    #[test]
    fn truncated_immediate() {
        let code = hex::decode("ef00010100040200010001030000000000000160").unwrap();
        let container = from_slice(&code).unwrap();

        assert_eq!(
            container.is_valid_eof().err(),
            Some(Error::TruncatedImmediate)
        );
    }

    #[test]
    fn invalid_section_argument() {
        let code = hex::decode("ef000101000402000100040300000000000000b0000100").unwrap();
        let container = from_slice(&code).unwrap();

        assert_eq!(
            container.is_valid_eof().err(),
            Some(Error::InvalidSectionArgument)
        );
    }

    #[test]
    fn invalid_jumpdest() {
        // Target header
        let code = hex::decode("ef0001010004020001000303000000000000005cfffb").unwrap();
        let container = from_slice(&code).unwrap();

        assert_eq!(
            container.is_valid_eof().err(),
            Some(Error::InvalidJumpdest)
        );

        // Target before container code
        let code = hex::decode("ef0001010004020001000303000000000000005cffe9").unwrap();
        let container = from_slice(&code).unwrap();

        assert_eq!(
            container.is_valid_eof().err(),
            Some(Error::InvalidJumpdest)
        );

        // Target into data section
        let code = hex::decode("ef0001010004020001000303000400000000005c0002aabbccdd").unwrap();
        let container = from_slice(&code).unwrap();

        assert_eq!(
            container.is_valid_eof().err(),
            Some(Error::InvalidJumpdest)
        );

        // Target after code end
        let code = hex::decode("ef0001010004020001000303000000000000005c0002").unwrap();
        let container = from_slice(&code).unwrap();

        assert_eq!(
            container.is_valid_eof().err(),
            Some(Error::InvalidJumpdest)
        );

        // TODO: Test RJUMPV and PUSH* Immediates
        // Target immediate
        let code = hex::decode("ef0001010004020001000303000000000000005cffff").unwrap();
        let container = from_slice(&code).unwrap();

        assert_eq!(
            container.is_valid_eof().err(),
            Some(Error::InvalidJumpdest)
        );
    }

    #[test]
    fn conflicting_stack() {
        let code = hex::decode("ef0001010004020001000a030000000000000260005d00026001600200").unwrap();
        let container = from_slice(&code).unwrap();

        assert_eq!(
            container.is_valid_eof().err(),
            Some(Error::ConflictingStack)
        );
    }

    #[test]
    fn invalid_branch_count() {
        let code = hex::decode("ef0001010008020002000a000603000000000000010000000260015d00030060015e006001600155b1").unwrap();
        let container = from_slice(&code).unwrap();

        assert_eq!(
            container.is_valid_eof().err(),
            Some(Error::InvalidBranchCount)
        );
    }

    #[test]
    fn stack_underflow() {
        let code = hex::decode("ef00010100040200010004030000000000000160010100").unwrap();
        let container = from_slice(&code).unwrap();

        assert_eq!(
            container.is_valid_eof().err(),
            Some(Error::StackUnderflow)
        );
    }

    #[test]
    fn stack_overflow() {
        let code = hex::decode("ef0001010008020002000b0bff0300000000000002010003ff60016001b000016001550050600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001600160016001505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050505050b1").unwrap();
        let container = from_slice(&code).unwrap();

        assert_eq!(
            container.is_valid_eof().err(),
            Some(Error::StackOverflow)
        );
    }

    #[test]
    fn invalid_outputs() {
        let code = hex::decode("ef000101000802000200040003030000000000000000000001b00001006001b1").unwrap();
        let container = from_slice(&code).unwrap();

        assert_eq!(
            container.is_valid_eof().err(),
            Some(Error::InvalidOutputs)
        );
    }

    #[test]
    fn invalid_max_stack_height() {
        let code = hex::decode("ef0001010004020001000303000000000000026001fe").unwrap();
        let container = from_slice(&code).unwrap();

        assert_eq!(
            container.is_valid_eof().err(),
            Some(Error::InvalidMaxStackHeight)
        );
    }

    #[test]
    fn invalid_code_termination() {
        let code = hex::decode("ef0001010004020001000203000000000000016001").unwrap();
        let container = from_slice(&code).unwrap();
        
        assert_eq!(
            container.is_valid_eof().err(),
            Some(Error::InvalidCodeTermination)
        );
    }

    #[test]
    fn unreachable_code() {
        let code = hex::decode("ef00010100040200010006030000000000000260006000f300").unwrap();
        let container = from_slice(&code).unwrap();

        assert_eq!(
            container.is_valid_eof().err(),
            Some(Error::UnreachableCode)
        );
    }
}
