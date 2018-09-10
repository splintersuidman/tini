//! `token` contains the `Token` structure.

use crate::{Identifier, Position};
use std::fmt;

/// `Token` represents a token, containing the token's type and its line and column.
#[derive(Clone, Debug, PartialEq)]
pub struct Token {
    /// The `Token`'s type.
    pub token: TokenType,
    /// The `Token`'s position in a file.
    pub position: Position,
}

impl Token {
    /// Create a new `Token`.
    pub fn new(token: TokenType, line: usize, column: usize) -> Token {
        Token {
            token,
            position: Position::new(line, column),
        }
    }
}

/// `TokenType` contains all possible token types.
#[derive(Clone, PartialEq, Debug)]
pub enum TokenType {
    /// An identifier.
    Identifier(Identifier),
    /// An integer literal.
    Integer(i64),
    /// `(`
    LeftBracket,
    /// `)`
    RightBracket,

    /// `if`
    If,
    /// `define`
    Define,
}

impl TokenType {
    /// Turn a name into a `TokenType`. If the name is a keyword, the keyword's `TokenType` will be
    /// returned, otherwise the name is an identifier.
    pub fn identifier_or_keyword(name: String) -> TokenType {
        match name.as_str() {
            "if" => TokenType::If,
            "define" => TokenType::Define,
            _ => TokenType::Identifier(name),
        }
    }
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::TokenType::*;
        match self {
            Identifier(ident) => write!(f, "{}", ident),
            Integer(v) => write!(f, "{}", v),
            LeftBracket => write!(f, "("),
            RightBracket => write!(f, ")"),
            If => write!(f, "if"),
            Define => write!(f, "define"),
        }
    }
}
