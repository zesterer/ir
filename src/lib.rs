pub mod lexer;
pub mod parser;
pub mod src;

use src::SrcRange;

#[derive(Debug, PartialEq)]
pub struct Error {
    kind: ErrorKind,
    range: Option<SrcRange>,
}

#[derive(Debug, PartialEq)]
pub enum ErrorKind {
    // Lexing
    /// An unrecognized character was found while lexing.
    UnexpectedChar(char),

    // Parsing
    /// While parsing the program, it expected to find a block, but did not.
    /// This is most likely due to having an empty program.
    ExpectedBlock,
}

impl Error {
    pub fn new(kind: ErrorKind) -> Self {
        Self { kind, range: None }
    }

    pub fn at(self, range: SrcRange) -> Self {
        Self { kind: self.kind, range: Some(range) }
    }
}

// A crate-wide helper function for creating errors without a lot of verbosity
pub(crate) fn error(kind: ErrorKind) -> Error {
    Error { kind, range: None }
}
