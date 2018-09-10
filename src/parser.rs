//! `parser` contains the `Parser`, which turns a stream of `Token`s into a stream of `ASTType`s.

mod error;

pub use self::error::*;

use self::error::ParseResult::*;
use crate::ast::{ASTType, AST};
use crate::lexer::Lexer;
use crate::token::{Token, TokenType};
use std::iter::Peekable;

/// The `Parser` turns a stream of `Token`s into `AST`s.
pub struct Parser<'i> {
    /// The `Lexer`, from which the `Token`s will be read.
    lexer: Peekable<Lexer<'i>>,
}

impl<'i> Parser<'i> {
    /// Create a new `Parser` with a `Lexer` that supplies `Token`s.
    pub fn new(lexer: Lexer<'i>) -> Parser<'i> {
        Parser {
            lexer: lexer.peekable(),
        }
    }

    /// Parse the next expression.
    pub fn parse_expression(&mut self) -> ParseResult<AST> {
        let token = match self.next_token() {
            Ok(t) => t,
            Err(e) => return Err(e),
            Eof => return Eof,
        };

        let position = token.position;
        let ast = match token.token {
            TokenType::Integer(v) => ASTType::Integer(v),
            TokenType::Identifier(ident) => ASTType::Identifier(ident),
            TokenType::LeftBracket => return self.parse_function(),
            // Unexpected tokens. Do not use `_` here, to cause compile errors when a new
            // `TokenType` is added.
            TokenType::RightBracket | TokenType::Define | TokenType::If => {
                return Err(ParseError::UnexpectedToken(token))
            }
        };

        Ok(AST { ast, position })
    }

    /// Peek the `TokenType` of the next `Token`.
    fn peek_token_type(&mut self) -> Option<&TokenType> {
        match self.lexer.peek() {
            Some(Result::Ok(t)) => Some(&t.token),
            _ => None,
        }
    }

    /// Get the next `Token` in the stream.
    fn next_token(&mut self) -> ParseResult<Token> {
        match self.lexer.next() {
            Some(Result::Ok(t)) => Ok(t),
            Some(Result::Err(e)) => Err(ParseError::LexerError(e)),
            None => Eof,
        }
    }

    /// Parse a function call.
    fn parse_function(&mut self) -> ParseResult<AST> {
        match self.peek_token_type() {
            Some(&TokenType::Identifier(_)) => self.parse_function_call(),
            Some(&TokenType::If) => self.parse_if(),
            Some(&TokenType::Define) => self.parse_define(),
            None => Err(ParseError::UnexpectedEof {
                expected: "`if`, `define`, a value, or an identifier",
            }),
            _ => Err(ParseError::UnexpectedToken(self.next_token().unwrap())),
        }
    }

    /// Parse a function call expression.
    fn parse_function_call(&mut self) -> ParseResult<AST> {
        let name = self.next_token().unwrap();
        let position = name.position;

        let name = match name.token {
            TokenType::Identifier(n) => n,
            _ => panic!(
                "parse_function_call may only be called when the next token is an identifier."
            ),
        };

        let mut arguments = Vec::new();
        while self.peek_token_type() != Some(&TokenType::RightBracket) {
            match self.parse_expression() {
                Ok(p) => arguments.push(p),
                Err(e) => return Err(e),
                Eof => {
                    return Err(ParseError::UnexpectedEof {
                        expected: "function parameter or `)`",
                    })
                }
            };
        }

        // Expect RightBracket.
        match self.peek_token_type() {
            Some(&TokenType::RightBracket) => {
                self.next_token();
            }
            None => return Err(ParseError::UnexpectedEof { expected: "`)`" }),
            _ => return Err(ParseError::UnexpectedToken(self.next_token().unwrap())),
        }

        Ok(AST {
            ast: ASTType::FunctionCall { name, arguments },
            position,
        })
    }

    /// Parse an if expression.
    fn parse_if(&mut self) -> ParseResult<AST> {
        let if_token = self.next_token().unwrap();
        assert_eq!(if_token.token, TokenType::If);

        let position = if_token.position;

        let condition = match self.parse_expression() {
            Ok(v) => Box::new(v),
            Err(e) => return Err(e),
            Eof => {
                return Err(ParseError::UnexpectedEof {
                    expected: "condition in if expression",
                })
            }
        };
        let consequence = match self.parse_expression() {
            Ok(v) => Box::new(v),
            Err(e) => return Err(e),
            Eof => {
                return Err(ParseError::UnexpectedEof {
                    expected: "consequence in if expression",
                })
            }
        };
        let alternative = match self.parse_expression() {
            Ok(v) => Box::new(v),
            Err(e) => return Err(e),
            Eof => {
                return Err(ParseError::UnexpectedEof {
                    expected: "alternative in if expression",
                })
            }
        };

        // Next token must be a right bracket.
        let right_bracket = match self.next_token() {
            Ok(t) => t,
            Err(e) => return Err(e),
            Eof => return Err(ParseError::UnexpectedEof { expected: "`)`" }),
        };

        match right_bracket.token {
            TokenType::RightBracket => {}
            _ => return Err(ParseError::UnexpectedToken(right_bracket)),
        }

        Ok(AST {
            ast: ASTType::If {
                condition,
                consequence,
                alternative,
            },
            position,
        })
    }

    /// Parse a define expression.
    fn parse_define(&mut self) -> ParseResult<AST> {
        let define_token = self.next_token().unwrap();
        assert_eq!(define_token.token, TokenType::Define);

        let position = define_token.position;

        let (name, arguments) = match self.parse_expression() {
            Ok(AST {
                ast: ASTType::FunctionCall { name, arguments },
                ..
            }) => {
                let mut identifiers = Vec::new();
                for parameter in arguments {
                    match parameter {
                        AST {
                            ast: ASTType::Identifier(p),
                            ..
                        } => identifiers.push(p),
                        expr => return Err(ParseError::UnexpectedExpression(expr)),
                    }
                }
                (name, Some(identifiers))
            }
            Ok(AST {
                ast: ASTType::Identifier(name),
                ..
            }) => (name, None),
            Ok(expr) => return Err(ParseError::UnexpectedExpression(expr)),
            e => return e,
        };

        let value = match self.parse_expression() {
            Ok(v) => v,
            e => return e,
        };

        // Next token must be a right bracket.
        let right_bracket = match self.next_token() {
            Ok(t) => t,
            Err(e) => return Err(e),
            Eof => return Err(ParseError::UnexpectedEof { expected: "`)`" }),
        };

        match right_bracket.token {
            TokenType::RightBracket => {}
            _ => return Err(ParseError::UnexpectedToken(right_bracket)),
        }

        Ok(AST {
            ast: ASTType::Define {
                name,
                arguments,
                value: Box::new(value),
            },
            position,
        })
    }
}

impl<'i> Iterator for Parser<'i> {
    type Item = Result<AST, ParseError>;

    fn next(&mut self) -> Option<Result<AST, ParseError>> {
        match self.parse_expression() {
            Ok(v) => Some(Result::Ok(v)),
            Err(e) => Some(Result::Err(e)),
            Eof => None,
        }
    }
}
