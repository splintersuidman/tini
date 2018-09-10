use crate::Position;
use std::error;
use std::fmt;

/// The `Result` of `Lexer`.
#[derive(Debug)]
pub enum LexerResult<T> {
    /// A success value.
    Ok(T),
    /// An error value.
    Err(LexerError),
    /// No tokens found; `Eof` must be returned when the end of the file is reached.
    Eof,
}

impl<T> LexerResult<T> {
    /// Returns `true` if the result is `Ok`.
    pub fn is_ok(&self) -> bool {
        match self {
            LexerResult::Ok(_) => true,
            _ => false,
        }
    }

    /// Returns `true` if the result is `Err`.
    pub fn is_err(&self) -> bool {
        match self {
            LexerResult::Err(_) => true,
            _ => false,
        }
    }

    /// Returns `true` if the result is `Eof`.
    pub fn is_eof(&self) -> bool {
        match self {
            LexerResult::Eof => true,
            _ => false,
        }
    }

    /// Unwrap an `Ok` value.
    pub fn unwrap(self) -> T {
        match self {
            LexerResult::Ok(t) => t,
            LexerResult::Err(e) => panic!("unwrapped error value: {}", e),
            LexerResult::Eof => panic!("unwrapped end of file value"),
        }
    }

    /// Map an `Ok` value.
    pub fn map<U>(self, op: impl FnOnce(T) -> U) -> LexerResult<U> {
        match self {
            LexerResult::Ok(t) => LexerResult::Ok(op(t)),
            LexerResult::Err(e) => LexerResult::Err(e),
            LexerResult::Eof => LexerResult::Eof,
        }
    }
}

/// The error type of `Lexer`.
#[derive(Debug)]
pub enum LexerError {
    /// An unexpected character was found.
    UnexpectedCharacter {
        /// The unexpected character.
        ch: char,
        /// The position of the character.
        position: Position,
    },
    /// The end of the file was found, but a character was expected.
    UnexpectedEof { expected: &'static str },
    /// The found escape character is invalid.
    UnknownEscape {
        /// The unknown escape character.
        ch: char,
        /// The position of the character.
        position: Position
    },
    /// A different error.
    Other {
        /// The error value.
        error: Box<dyn error::Error>,
        /// The position at which the error happened.
        position: Position,
    },
}

impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::LexerError::*;
        match self {
            UnexpectedCharacter { ch, position } => {
                write!(f, "unexpected character at {}: '{}' ", position, ch)
            }
            UnknownEscape { ch, position } => {
                write!(f, "invalid escape character at {}: '{}' ", position, ch)
            }
            UnexpectedEof { expected } => {
                write!(f, "unexpected end of file, expected {}", expected)
            }
            Other { error, position } => write!(f, "error at {}: {}", position, error),
        }
    }
}

impl error::Error for LexerError {}
