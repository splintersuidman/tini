use crate::ast::AST;
use crate::lexer::LexerError;
use crate::token::Token;
use crate::Position;
use std::error;
use std::fmt;

/// The `Result` of `Parser`.
#[derive(Debug)]
pub enum ParseResult<T> {
    /// A success value.
    Ok(T),
    /// An error value.
    Err(ParseError),
    /// No expressions found; `Eof` must be returned when the end of the file is reached.
    Eof,
}

impl<T> ParseResult<T> {
    /// Returns `true` if the result is `Ok`.
    pub fn is_ok(&self) -> bool {
        match self {
            ParseResult::Ok(_) => true,
            _ => false,
        }
    }

    /// Returns `true` if the result is `Err`.
    pub fn is_err(&self) -> bool {
        match self {
            ParseResult::Err(_) => true,
            _ => false,
        }
    }

    /// Returns `true` if the result is `Eof`.
    pub fn is_eof(&self) -> bool {
        match self {
            ParseResult::Eof => true,
            _ => false,
        }
    }

    /// Unwrap an `Ok` value.
    pub fn unwrap(self) -> T {
        match self {
            ParseResult::Ok(t) => t,
            ParseResult::Err(e) => panic!("unwrapped error value: {}", e),
            ParseResult::Eof => panic!("unwrapped end of file value"),
        }
    }

    /// Map an `Ok` value.
    pub fn map<U>(self, op: impl FnOnce(T) -> U) -> ParseResult<U> {
        match self {
            ParseResult::Ok(t) => ParseResult::Ok(op(t)),
            ParseResult::Err(e) => ParseResult::Err(e),
            ParseResult::Eof => ParseResult::Eof,
        }
    }
}

/// The error type of `Parser`.
#[derive(Debug)]
pub enum ParseError {
    /// The next token in the token stream was not expected.
    UnexpectedToken(Token),
    /// The next expression was not expected.
    UnexpectedExpression(AST),
    /// End of file found, but expected token.
    UnexpectedEof { expected: &'static str },
    /// An error happened in the lexer.
    LexerError(LexerError),
    /// A different error.
    Other {
        /// The error value.
        error: Box<dyn error::Error>,
        /// The position at which the error happened.
        position: Position,
    },
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::ParseError::*;
        match self {
            UnexpectedToken(Token { token, position }) => {
                write!(f, "unexpected token at {}: {}", position, token)
            }
            UnexpectedExpression(AST { ast, position }) => {
                write!(f, "unexpected expression at {}: {:?}", position, ast)
            }
            UnexpectedEof { expected } => write!(f, "found end of file, but expected {}", expected),
            LexerError(e) => write!(f, "{}", e),
            Other { error, position } => write!(f, "error at {}: {}", position, error),
        }
    }
}

impl error::Error for ParseError {}
