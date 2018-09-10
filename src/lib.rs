#![feature(external_doc)]
#![doc(include = "../README.md")]

pub mod ast;
pub mod interpreter;
pub mod lexer;
pub mod parser;
pub mod prelude;
pub mod token;

use std::fmt;

/// The type of an identifier in `tini`.
pub type Identifier = String;

/// A position in a file, consisting of a line number and a column number.
#[derive(Clone, Copy, Default, PartialEq)]
pub struct Position {
    /// The line number in a file.
    pub line: usize,
    /// The column number in a file.
    pub column: usize,
}

impl Position {
    /// Create a new `Position`.
    pub fn new(line: usize, column: usize) -> Position {
        Position { line, column }
    }

    /// Go to the beginning of the next line.
    pub fn next_line(&mut self) {
        self.line += 1;
        self.column = 1;
    }

    /// Go to the next column.
    pub fn next_column(&mut self) {
        self.column += 1;
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.line, self.column)
    }
}

impl fmt::Debug for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Position({}:{})", self.line, self.column)
    }
}
