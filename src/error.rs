use serde::{de, ser};
use std::fmt::{self, Display};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    Message(String),
    UnexpectedEOF,
    InvalidMagic,
    UnsupportedVersion,
    MissingTypeHeader,
    MissingCodeHeader,
    InvalidCodeHeader,
    InvalidTypeSectionSize,
    MissingDataHeader,
    MissingTerminator,
    TooManyInputs,
    TooManyOutputs,
    TooLargeMaxStackHeight,
    InvalidSection0Type,
    InvalidCodeSize,
    InvalidContainerSize,
    UndefinedInstruction(u8),
    TruncatedImmediate,
    InvalidSectionArgument,
    InvalidJumpdest,
    ConflictingStack,
    InvalidBranchCount,
    StackUnderflow,
    StackOverflow,
    InvalidOutputs,
    InvalidMaxStackHeight,
    InvalidCodeTermination,
    UnreachableCode,
    UnsupportedSectionKind,
    NoSections,
    IncompleteSections,
    IncompleteSectionSize,
    InvalidSectionOrder,
    MismatchingCodeAndTypeSections,
    DuplicateTypeSection,
    InvalidStackHeight,
}

impl ser::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error::Message(msg.to_string())
    }
}

impl de::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error::Message(msg.to_string())
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Error::*;

        match self {
            Message(msg) => write!(f, "{}", msg),
            UnexpectedEOF => write!(f, "Unexpected EOF"),
            InvalidMagic => write!(f, "Invalid magic"),
            UnsupportedVersion => write!(f, "Unsupported version"),
            UnsupportedSectionKind => write!(f, "Unsupported section kind"),
            MissingTypeHeader => write!(f, "Missing Type header"),
            MissingCodeHeader => write!(f, "Missing Code header"),
            InvalidCodeHeader => write!(f, "Invalid Code header"),
            MissingDataHeader => write!(f, "Missing Data header"),
            MissingTerminator => write!(f, "Missing terminator"),
            TooManyInputs => write!(f, "Too many inputs"),
            TooManyOutputs => write!(f, "Too many outputs"),
            TooLargeMaxStackHeight => write!(f, "Too large max stack height"),
            InvalidSection0Type => write!(f, "Invalid section 0 type"),
            InvalidCodeSize => write!(f, "Invalid Code section size"),
            InvalidContainerSize => write!(f, "Invalid container size"),
            UndefinedInstruction(op) => write!(f, "Invalid Opcode: {}", op),
            TruncatedImmediate => write!(f, "Truncated immediate"),
            InvalidSectionArgument => write!(f, "Invalid section argument"),
            InvalidJumpdest => write!(f, "Invalid jumpdest"),
            ConflictingStack => write!(f, "Conflicting stack"),
            InvalidBranchCount => write!(f, "Invalid branch count"),
            StackUnderflow => write!(f, "Stack underflow"),
            StackOverflow => write!(f, "Stack overflow"),
            InvalidOutputs => write!(f, "Invalid outputs"),
            InvalidMaxStackHeight => write!(f, "Invalid max stack height"),
            InvalidCodeTermination => write!(f, "Invalid code termination"),
            UnreachableCode => write!(f, "Unreachable code"),
            InvalidTypeSectionSize => write!(f, "Invalid Type section size"),
            NoSections => write!(f, "No sections"),
            IncompleteSections => write!(f, "Incomplete sections"),
            IncompleteSectionSize => write!(f, "Incomplete section size"),
            InvalidSectionOrder => write!(f, "Invalid section order"),
            MismatchingCodeAndTypeSections => {
                write!(f, "Mismatching number of Code and Type sections")
            }
            DuplicateTypeSection => write!(f, "Duplicate Type section"),
            InvalidStackHeight => write!(f, "Invalid stack height"),

        }
    }
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error::Message(error.to_string())
    }
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        Error::Message(error.to_string())
    }
}
