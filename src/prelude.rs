//! The `prelude` exports the most important types of `tini`.

pub use crate::ast::{ASTType, AST};
pub use crate::interpreter::{Environment, Interpreter, Value};
pub use crate::lexer::{Lexer, LexerResult};
pub use crate::parser::{ParseResult, Parser};
pub use crate::token::{Token, TokenType};
pub use crate::{Identifier, Position};
