use std::{
    error::Error,
    fmt::{Display, Formatter, Result},
};

use crate::preprocessor::SourceLocation;

/// An error that occurs during the lexing process containing the position of
/// the error in the source code.
#[derive(Debug)]
pub struct SyntaxError {
    pub line: u32,
    pub column: u32,
    pub error: Box<dyn Error>,
}

impl SyntaxError {
    /// Creates a new syntax error cloned from the given source location and
    /// error.
    pub fn from_source_location(source_location: &SourceLocation, error: Box<dyn Error>) -> Self {
        Self {
            line: source_location.line,
            column: source_location.column,
            error,
        }
    }
}

impl Display for SyntaxError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "{} at line {}, column {}",
            self.error, self.line, self.column
        )
    }
}

impl Error for SyntaxError {}

/// An error that occurs when there is an issue with the loop structure.
#[derive(Debug)]
pub enum LoopError {
    /// An error that occurs when there is a missing opening loop.
    MissingStart,
    /// An error that occurs when there is a missing ending loop.
    MissingEnd,
}

impl Display for LoopError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            LoopError::MissingStart => write!(f, "Missing opening loop for closing loop"),
            LoopError::MissingEnd => write!(f, "Missing closing loop for opening loop"),
        }
    }
}

impl Error for LoopError {}
