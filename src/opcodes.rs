use super::error::{Error, Result};

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct OpCode {
    pub name: &'static str,
    pub code: u8,
    pub immediates: u8,
    pub stack_inputs: u8,
    pub stack_outputs: u8,
    pub is_terminating: bool,
}

impl OpCode {
    fn load_opcodes() -> Vec<OpCode> {
        let opcodes = vec![
          OpCode{name: "STOP", code: 0x00, immediates: 0, stack_inputs: 0, stack_outputs: 0, is_terminating: true},
          OpCode{name: "ADD",  code: 0x01, immediates: 0, stack_inputs: 2, stack_outputs: 1, is_terminating: false},
          OpCode{name: "MUL",  code: 0x02, immediates: 0, stack_inputs: 2, stack_outputs: 1, is_terminating: false},
          OpCode{name: "SUB",  code: 0x03, immediates: 0, stack_inputs: 2, stack_outputs: 1, is_terminating: false},
          OpCode{name: "DIV",  code: 0x04, immediates: 0, stack_inputs: 2, stack_outputs: 1, is_terminating: false},
          OpCode{name: "SDIV", code: 0x05, immediates: 0, stack_inputs: 2, stack_outputs: 1, is_terminating: false},
          OpCode{name: "MOD", code: 0x06, immediates: 0, stack_inputs: 2, stack_outputs: 1, is_terminating: false},
          OpCode{name: "SMOD", code: 0x07, immediates: 0, stack_inputs: 2, stack_outputs: 1, is_terminating: false},
          OpCode{name: "ADDMOD", code: 0x08, immediates: 0, stack_inputs: 3, stack_outputs: 1, is_terminating: false},
          OpCode{name: "MULMOD", code: 0x09, immediates: 0, stack_inputs: 3, stack_outputs: 1, is_terminating: false},
          OpCode{name: "EXP", code: 0x0a, immediates: 0, stack_inputs: 2, stack_outputs: 1, is_terminating: false},
          OpCode{name: "SIGNEXTEND", code: 0x0b, immediates: 0, stack_inputs: 2, stack_outputs: 1, is_terminating: false},
          OpCode{name: "LT", code: 0x10, immediates: 0, stack_inputs: 2, stack_outputs: 1, is_terminating: false},
          OpCode{name: "GT", code: 0x11, immediates: 0, stack_inputs: 2, stack_outputs: 1, is_terminating: false},
          OpCode{name: "SLT", code: 0x12, immediates: 0, stack_inputs: 2, stack_outputs: 1, is_terminating: false},
          OpCode{name: "SGT", code: 0x13, immediates: 0, stack_inputs: 2, stack_outputs: 1, is_terminating: false},
          OpCode{name: "EQ", code: 0x14, immediates: 0, stack_inputs: 2, stack_outputs: 1, is_terminating: false},
          OpCode{name: "ISZERO", code: 0x15, immediates: 0, stack_inputs: 1, stack_outputs: 1, is_terminating: false},
          OpCode{name: "AND", code: 0x16, immediates: 0, stack_inputs: 2, stack_outputs: 1, is_terminating: false},
          OpCode{name: "OR", code: 0x17, immediates: 0, stack_inputs: 2, stack_outputs: 1, is_terminating: false},
          OpCode{name: "XOR", code: 0x18, immediates: 0, stack_inputs: 2, stack_outputs: 1, is_terminating: false},
          OpCode{name: "NOT", code: 0x19, immediates: 0, stack_inputs: 1, stack_outputs: 1, is_terminating: false},
          OpCode{name: "BYTE", code: 0x1a, immediates: 0, stack_inputs: 2, stack_outputs: 1, is_terminating: false},
          OpCode{name: "SHL", code: 0x1b, immediates: 0, stack_inputs: 2, stack_outputs: 1, is_terminating: false},
          OpCode{name: "SHR", code: 0x1c, immediates: 0, stack_inputs: 2, stack_outputs: 1, is_terminating: false},
          OpCode{name: "SAR", code: 0x1d, immediates: 0, stack_inputs: 2, stack_outputs: 1, is_terminating: false},
          OpCode{name: "KECCAK256", code: 0x20, immediates: 0, stack_inputs: 2, stack_outputs: 1, is_terminating: false},
          OpCode{name: "SHA3", code: 0x20, immediates: 0, stack_inputs: 2, stack_outputs: 1, is_terminating: false},
          OpCode{name: "ADDRESS", code: 0x30, immediates: 0, stack_inputs: 0, stack_outputs: 1, is_terminating: false},
          OpCode{name: "BALANCE", code: 0x31, immediates: 0, stack_inputs: 1, stack_outputs: 1, is_terminating: false},
          OpCode{name: "ORIGIN", code: 0x32, immediates: 0, stack_inputs: 0, stack_outputs: 1, is_terminating: false},
          OpCode{name: "CALLER", code: 0x33, immediates: 0, stack_inputs: 0, stack_outputs: 1, is_terminating: false},
          OpCode{name: "CALLVALUE", code: 0x34, immediates: 0, stack_inputs: 0, stack_outputs: 1, is_terminating: false},
          OpCode{name: "CALLDATALOAD", code: 0x35, immediates: 0, stack_inputs: 1, stack_outputs: 1, is_terminating: false},
          OpCode{name: "CALLDATASIZE", code: 0x36, immediates: 0, stack_inputs: 0, stack_outputs: 1, is_terminating: false},
          OpCode{name: "CALLDATACOPY", code: 0x37, immediates: 0, stack_inputs: 3, stack_outputs: 0, is_terminating: false},
          OpCode{name: "CODESIZE", code: 0x38, immediates: 0, stack_inputs: 0, stack_outputs: 1, is_terminating: false},
          OpCode{name: "CODECOPY", code: 0x39, immediates: 0, stack_inputs: 3, stack_outputs: 0, is_terminating: false},
          OpCode{name: "GASPRICE", code: 0x3a, immediates: 0, stack_inputs: 0, stack_outputs: 1, is_terminating: false},
          OpCode{name: "EXTCODESIZE", code: 0x3b, immediates: 0, stack_inputs: 1, stack_outputs: 1, is_terminating: false},
          OpCode{name: "EXTCODECOPY", code: 0x3c, immediates: 0, stack_inputs: 4, stack_outputs: 0, is_terminating: false},
          OpCode{name: "RETURNDATASIZE", code: 0x3d, immediates: 0, stack_inputs: 0, stack_outputs: 1, is_terminating: false},
          OpCode{name: "RETURNDATACOPY", code: 0x3e, immediates: 0, stack_inputs: 3, stack_outputs: 0, is_terminating: false},
          OpCode{name: "EXTCODEHASH", code: 0x3f, immediates: 0, stack_inputs: 1, stack_outputs: 1, is_terminating: false},
          OpCode{name: "BLOCKHASH", code: 0x40, immediates: 0, stack_inputs: 1, stack_outputs: 1, is_terminating: false},
          OpCode{name: "COINBASE", code: 0x41, immediates: 0, stack_inputs: 0, stack_outputs: 1, is_terminating: false},
          OpCode{name: "TIMESTAMP", code: 0x42, immediates: 0, stack_inputs: 0, stack_outputs: 1, is_terminating: false},
          OpCode{name: "NUMBER", code: 0x43, immediates: 0, stack_inputs: 0, stack_outputs: 1, is_terminating: false},
          OpCode{name: "DIFFICULTY", code: 0x44, immediates: 0, stack_inputs: 0, stack_outputs: 1, is_terminating: false},
          OpCode{name: "GASLIMIT", code: 0x45, immediates: 0, stack_inputs: 0, stack_outputs: 1, is_terminating: false},
          OpCode{name: "CHAINID", code: 0x46, immediates: 0, stack_inputs: 0, stack_outputs: 1, is_terminating: false},
          OpCode{name: "SELFBALANCE", code: 0x47, immediates: 0, stack_inputs: 0, stack_outputs: 1, is_terminating: false},
          OpCode{name: "BASEFEE", code: 0x48, immediates: 0, stack_inputs: 0, stack_outputs: 1, is_terminating: false},
          OpCode{name: "POP", code: 0x50, immediates: 0, stack_inputs: 1, stack_outputs: 0, is_terminating: false},
          OpCode{name: "MLOAD", code: 0x51, immediates: 0, stack_inputs: 1, stack_outputs: 1, is_terminating: false},
          OpCode{name: "MSTORE", code: 0x52, immediates: 0, stack_inputs: 2, stack_outputs: 0, is_terminating: false},
          OpCode{name: "MSTORE8", code: 0x53, immediates: 0, stack_inputs: 2, stack_outputs: 0, is_terminating: false},
          OpCode{name: "SLOAD", code: 0x54, immediates: 0, stack_inputs: 1, stack_outputs: 1, is_terminating: false},
          OpCode{name: "SSTORE", code: 0x55, immediates: 0, stack_inputs: 2, stack_outputs: 0, is_terminating: false},
          // Deprecated in EOF
          /*
          OpCode{name: "JUMP", code: 0x56, immediates: 0, stack_inputs: 1, stack_outputs: 0, is_terminating: false},
          OpCode{name: "JUMPI", code: 0x57, immediates: 0, stack_inputs: 2, stack_outputs: 0, is_terminating: false},
          */
          OpCode{name: "PC", code: 0x58, immediates: 0, stack_inputs: 0, stack_outputs: 1, is_terminating: false},
          OpCode{name: "MSIZE", code: 0x59, immediates: 0, stack_inputs: 0, stack_outputs: 1, is_terminating: false},
          OpCode{name: "GAS", code: 0x5a, immediates: 0, stack_inputs: 0, stack_outputs: 1, is_terminating: false},
          OpCode{name: "JUMPDEST", code: 0x5b, immediates: 0, stack_inputs: 0, stack_outputs: 0, is_terminating: false},
          OpCode{name: "NOP", code: 0x5b, immediates: 0, stack_inputs: 0, stack_outputs: 0, is_terminating: false},
          OpCode{name: "RJUMP", code: 0x5c, immediates: 2, stack_inputs: 0, stack_outputs: 0, is_terminating: false},
          OpCode{name: "RJUMPI", code: 0x5d, immediates: 2, stack_inputs: 1, stack_outputs: 0, is_terminating: false},
          OpCode{name: "RJUMPV", code: 0x5e, immediates: 1, stack_inputs: 1, stack_outputs: 0, is_terminating: false},
          OpCode{name: "PUSH0", code: 0x5f, immediates: 0, stack_inputs: 0, stack_outputs: 1, is_terminating: false},
          OpCode{name: "PUSH1", code: 0x60, immediates: 1, stack_inputs: 0, stack_outputs: 1, is_terminating: false},
          OpCode{name: "PUSH2", code: 0x61, immediates: 2, stack_inputs: 0, stack_outputs: 1, is_terminating: false},
          OpCode{name: "PUSH3", code: 0x62, immediates: 3, stack_inputs: 0, stack_outputs: 1, is_terminating: false},
          OpCode{name: "PUSH4", code: 0x63, immediates: 4, stack_inputs: 0, stack_outputs: 1, is_terminating: false},
          OpCode{name: "PUSH5", code: 0x64, immediates: 5, stack_inputs: 0, stack_outputs: 1, is_terminating: false},
          OpCode{name: "PUSH6", code: 0x65, immediates: 6, stack_inputs: 0, stack_outputs: 1, is_terminating: false},
          OpCode{name: "PUSH7", code: 0x66, immediates: 7, stack_inputs: 0, stack_outputs: 1, is_terminating: false},
          OpCode{name: "PUSH8", code: 0x67, immediates: 8, stack_inputs: 0, stack_outputs: 1, is_terminating: false},
          OpCode{name: "PUSH9", code: 0x68, immediates: 9, stack_inputs: 0, stack_outputs: 1, is_terminating: false},
          OpCode{name: "PUSH10", code: 0x69, immediates: 10, stack_inputs: 0, stack_outputs: 1, is_terminating: false},
          OpCode{name: "PUSH11", code: 0x6a, immediates: 11, stack_inputs: 0, stack_outputs: 1, is_terminating: false},
          OpCode{name: "PUSH12", code: 0x6b, immediates: 12, stack_inputs: 0, stack_outputs: 1, is_terminating: false},
          OpCode{name: "PUSH13", code: 0x6c, immediates: 13, stack_inputs: 0, stack_outputs: 1, is_terminating: false},
          OpCode{name: "PUSH14", code: 0x6d, immediates: 14, stack_inputs: 0, stack_outputs: 1, is_terminating: false},
          OpCode{name: "PUSH15", code: 0x6e, immediates: 15, stack_inputs: 0, stack_outputs: 1, is_terminating: false},
          OpCode{name: "PUSH16", code: 0x6f, immediates: 16, stack_inputs: 0, stack_outputs: 1, is_terminating: false},
          OpCode{name: "PUSH17", code: 0x70, immediates: 17, stack_inputs: 0, stack_outputs: 1, is_terminating: false},
          OpCode{name: "PUSH18", code: 0x71, immediates: 18, stack_inputs: 0, stack_outputs: 1, is_terminating: false},
          OpCode{name: "PUSH19", code: 0x72, immediates: 19, stack_inputs: 0, stack_outputs: 1, is_terminating: false},
          OpCode{name: "PUSH20", code: 0x73, immediates: 20, stack_inputs: 0, stack_outputs: 1, is_terminating: false},
          OpCode{name: "PUSH21", code: 0x74, immediates: 21, stack_inputs: 0, stack_outputs: 1, is_terminating: false},
          OpCode{name: "PUSH22", code: 0x75, immediates: 22, stack_inputs: 0, stack_outputs: 1, is_terminating: false},
          OpCode{name: "PUSH23", code: 0x76, immediates: 23, stack_inputs: 0, stack_outputs: 1, is_terminating: false},
          OpCode{name: "PUSH24", code: 0x77, immediates: 24, stack_inputs: 0, stack_outputs: 1, is_terminating: false},
          OpCode{name: "PUSH25", code: 0x78, immediates: 25, stack_inputs: 0, stack_outputs: 1, is_terminating: false},
          OpCode{name: "PUSH26", code: 0x79, immediates: 26, stack_inputs: 0, stack_outputs: 1, is_terminating: false},
          OpCode{name: "PUSH27", code: 0x7a, immediates: 27, stack_inputs: 0, stack_outputs: 1, is_terminating: false},
          OpCode{name: "PUSH28", code: 0x7b, immediates: 28, stack_inputs: 0, stack_outputs: 1, is_terminating: false},
          OpCode{name: "PUSH29", code: 0x7c, immediates: 29, stack_inputs: 0, stack_outputs: 1, is_terminating: false},
          OpCode{name: "PUSH30", code: 0x7d, immediates: 30, stack_inputs: 0, stack_outputs: 1, is_terminating: false},
          OpCode{name: "PUSH31", code: 0x7e, immediates: 31, stack_inputs: 0, stack_outputs: 1, is_terminating: false},
          OpCode{name: "PUSH32", code: 0x7f, immediates: 32, stack_inputs: 0, stack_outputs: 1, is_terminating: false},
          OpCode{name: "DUP1", code: 0x80, immediates: 0, stack_inputs: 1, stack_outputs: 2, is_terminating: false},
          OpCode{name: "DUP2", code: 0x81, immediates: 0, stack_inputs: 2, stack_outputs: 3, is_terminating: false},
          OpCode{name: "DUP3", code: 0x82, immediates: 0, stack_inputs: 3, stack_outputs: 4, is_terminating: false},
          OpCode{name: "DUP4", code: 0x83, immediates: 0, stack_inputs: 4, stack_outputs: 5, is_terminating: false},
          OpCode{name: "DUP5", code: 0x84, immediates: 0, stack_inputs: 5, stack_outputs: 6, is_terminating: false},
          OpCode{name: "DUP6", code: 0x85, immediates: 0, stack_inputs: 6, stack_outputs: 7, is_terminating: false},
          OpCode{name: "DUP7", code: 0x86, immediates: 0, stack_inputs: 7, stack_outputs: 8, is_terminating: false},
          OpCode{name: "DUP8", code: 0x87, immediates: 0, stack_inputs: 8, stack_outputs: 9, is_terminating: false},
          OpCode{name: "DUP9", code: 0x88, immediates: 0, stack_inputs: 9, stack_outputs: 10, is_terminating: false},
          OpCode{name: "DUP10", code: 0x89, immediates: 0, stack_inputs: 10, stack_outputs: 11, is_terminating: false},
          OpCode{name: "DUP11", code: 0x8a, immediates: 0, stack_inputs: 11, stack_outputs: 12, is_terminating: false},
          OpCode{name: "DUP12", code: 0x8b, immediates: 0, stack_inputs: 12, stack_outputs: 13, is_terminating: false},
          OpCode{name: "DUP13", code: 0x8c, immediates: 0, stack_inputs: 13, stack_outputs: 14, is_terminating: false},
          OpCode{name: "DUP14", code: 0x8d, immediates: 0, stack_inputs: 14, stack_outputs: 15, is_terminating: false},
          OpCode{name: "DUP15", code: 0x8e, immediates: 0, stack_inputs: 15, stack_outputs: 16, is_terminating: false},
          OpCode{name: "DUP16", code: 0x8f, immediates: 0, stack_inputs: 16, stack_outputs: 17, is_terminating: false},
          OpCode{name: "SWAP1", code: 0x90, immediates: 0, stack_inputs: 2, stack_outputs: 2, is_terminating: false},
          OpCode{name: "SWAP2", code: 0x91, immediates: 0, stack_inputs: 3, stack_outputs: 3, is_terminating: false},
          OpCode{name: "SWAP3", code: 0x92, immediates: 0, stack_inputs: 4, stack_outputs: 4, is_terminating: false},
          OpCode{name: "SWAP4", code: 0x93, immediates: 0, stack_inputs: 5, stack_outputs: 5, is_terminating: false},
          OpCode{name: "SWAP5", code: 0x94, immediates: 0, stack_inputs: 6, stack_outputs: 6, is_terminating: false},
          OpCode{name: "SWAP6", code: 0x95, immediates: 0, stack_inputs: 7, stack_outputs: 7, is_terminating: false},
          OpCode{name: "SWAP7", code: 0x96, immediates: 0, stack_inputs: 8, stack_outputs: 8, is_terminating: false},
          OpCode{name: "SWAP8", code: 0x97, immediates: 0, stack_inputs: 9, stack_outputs: 9, is_terminating: false},
          OpCode{name: "SWAP9", code: 0x98, immediates: 0, stack_inputs: 10, stack_outputs: 10, is_terminating: false},
          OpCode{name: "SWAP10", code: 0x99, immediates: 0, stack_inputs: 11, stack_outputs: 11, is_terminating: false},
          OpCode{name: "SWAP11", code: 0x9a, immediates: 0, stack_inputs: 12, stack_outputs: 12, is_terminating: false},
          OpCode{name: "SWAP12", code: 0x9b, immediates: 0, stack_inputs: 13, stack_outputs: 13, is_terminating: false},
          OpCode{name: "SWAP13", code: 0x9c, immediates: 0, stack_inputs: 14, stack_outputs: 14, is_terminating: false},
          OpCode{name: "SWAP14", code: 0x9d, immediates: 0, stack_inputs: 15, stack_outputs: 15, is_terminating: false},
          OpCode{name: "SWAP15", code: 0x9e, immediates: 0, stack_inputs: 16, stack_outputs: 16, is_terminating: false},
          OpCode{name: "SWAP16", code: 0x9f, immediates: 0, stack_inputs: 17, stack_outputs: 17, is_terminating: false},
          OpCode{name: "LOG0", code: 0xa0, immediates: 0, stack_inputs: 2, stack_outputs: 0, is_terminating: false},
          OpCode{name: "LOG1", code: 0xa1, immediates: 0, stack_inputs: 3, stack_outputs: 0, is_terminating: false},
          OpCode{name: "LOG2", code: 0xa2, immediates: 0, stack_inputs: 4, stack_outputs: 0, is_terminating: false},
          OpCode{name: "LOG3", code: 0xa3, immediates: 0, stack_inputs: 5, stack_outputs: 0, is_terminating: false},
          OpCode{name: "LOG4", code: 0xa4, immediates: 0, stack_inputs: 6, stack_outputs: 0, is_terminating: false},
          OpCode{name: "CALLF", code: 0xb0, immediates: 2, stack_inputs: 0, stack_outputs: 0, is_terminating: false},
          OpCode{name: "RETF", code: 0xb1, immediates: 0, stack_inputs: 0, stack_outputs: 0, is_terminating: true},
          OpCode{name: "JUMPF", code: 0xb2, immediates: 2, stack_inputs: 0, stack_outputs: 0, is_terminating: false},
          OpCode{name: "CREATE", code: 0xf0, immediates: 0, stack_inputs: 3, stack_outputs: 1, is_terminating: false},
          OpCode{name: "CALL", code: 0xf1, immediates: 0, stack_inputs: 7, stack_outputs: 1, is_terminating: false},
          //OpCode{name: "CALLCODE", code: 0xf2, immediates: 0, stack_inputs: 7, stack_outputs: 1, is_terminating: false}, // Deprecated by EOF
          OpCode{name: "RETURN", code: 0xf3, immediates: 0, stack_inputs: 2, stack_outputs: 0, is_terminating: true},
          OpCode{name: "DELEGATECALL", code: 0xf4, immediates: 0, stack_inputs: 6, stack_outputs: 1, is_terminating: false},
          OpCode{name: "CREATE2", code: 0xf5, immediates: 0, stack_inputs: 4, stack_outputs: 1, is_terminating: false},
          OpCode{name: "STATICCALL", code: 0xfa, immediates: 0, stack_inputs: 6, stack_outputs: 1, is_terminating: false},
          OpCode{name: "REVERT", code: 0xfd, immediates: 0, stack_inputs: 2, stack_outputs: 0, is_terminating: true},
          OpCode{name: "INVALID", code: 0xfe, immediates: 0, stack_inputs: 0, stack_outputs: 0, is_terminating: true},
          //OpCode{name: "SELFDESTRUCT", code: 0xff, immediates: 0, stack_inputs: 1, stack_outputs: 0, is_terminating: true}, // Deprecated by EOF

        ];

        return opcodes;
    }
    /*
    pub fn is_push(&self) -> bool {
        self.code >= 0x60 && self.code <= 0x7f
    }
    */
    pub fn from(code: u8) -> Result<OpCode> {
        let opcodes = OpCode::load_opcodes();
        opcodes
            .iter()
            .find(|opcode| opcode.code == code)
            .cloned()
            .ok_or_else(|| Error::UndefinedInstruction(code))
    }
}






