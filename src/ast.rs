//! `ast` contains the Abstract Syntax Tree (`AST`) representation.

use crate::{Identifier, Position};

/// An Abstract Syntax Tree with the position in the file.
#[derive(Debug, Clone, PartialEq)]
pub struct AST {
    /// The `AST` type.
    pub ast: ASTType,
    /// The position in a file of the `AST`.
    pub position: Position,
}

/// The Abstract Syntax Tree (`AST`) representation.
#[derive(Debug, Clone, PartialEq)]
pub enum ASTType {
    /// A `define` expression. This expression has two forms:
    ///
    /// 1. `(define x foo)`, to give `x` the value of `foo`.
    /// 2. `(define (f x) (foo x))`, to give `f` the value of a function on `x`.
    Define {
        name: Identifier,
        arguments: Option<Vec<Identifier>>,
        value: Box<AST>,
    },
    /// An `if` expression, in the form `(if condition consequence alternative)`.
    If {
        condition: Box<AST>,
        consequence: Box<AST>,
        alternative: Box<AST>,
    },
    /// A function call, in the form `(function param1 param2 ...)`.
    FunctionCall {
        name: Identifier,
        arguments: Vec<AST>,
    },
    /// An identifier.
    Identifier(Identifier),
    /// An integer.
    Integer(i64),
}
