//! `lexer` contains the `Lexer`, which turns a stream of characters into a stream of `Token`s.

mod error;

pub use self::error::*;

use self::error::LexerResult::*;
use crate::token::{Token, TokenType};
use crate::Position;
use std::iter::Peekable;
use std::str::Chars;

/// `Lexer` turns a stream of characters into `Token`s.
pub struct Lexer<'i> {
    /// The stream of characters that serves as input.
    input: Peekable<Chars<'i>>,
    /// The current position.
    pub position: Position,
}

impl<'i> Lexer<'i> {
    /// Create a new `Lexer` with an input.
    pub fn new(input: &'i str) -> Lexer<'i> {
        Lexer {
            input: input.chars().peekable(),
            position: Position::new(1, 1),
        }
    }

    /// Returns the next token in the input. If there is no next token, returns `None`.
    pub fn next_token(&mut self) -> LexerResult<Token> {
        self.skip_whitespace();

        let position = self.position;

        let peek_char = match self.peek_char() {
            Some(ch) => ch,
            None => return Eof,
        };
        let token = match *peek_char {
            '(' => {
                self.read_char();
                TokenType::LeftBracket
            }
            ')' => {
                self.read_char();
                TokenType::RightBracket
            }
            ';' => {
                self.read_comment();
                return self.next_token();
            }
            ch if Lexer::is_identifier_begin(ch) => {
                TokenType::identifier_or_keyword(self.read_identifier())
            }
            ch if ch.is_digit(10) => self.read_number(),
            ch => {
                return Err(LexerError::UnexpectedCharacter {
                    ch,
                    position: self.position,
                })
            }
        };

        Ok(Token { token, position })
    }

    /// Read the next char.
    fn read_char(&mut self) -> Option<char> {
        let ch = self.input.next()?;

        if ch == '\n' {
            self.position.next_line();
        } else {
            self.position.next_column();
        }

        Some(ch)
    }

    /// Peek the next char.
    fn peek_char(&mut self) -> Option<&char> {
        self.input.peek()
    }

    /// Read a comment, discarding it. A comment begins with ';', and ends with a newline.
    fn read_comment(&mut self) {
        while self.read_char().map(|ch| ch != '\n') == Some(true) {
            // Do nothing; the next character is read by the while predicate.
        }
    }

    /// Read an identifier and collect it into a `String`.
    /// This function must only be called when the next char in the input is an identifier beginner
    /// (see `Lexer::is_identifier_begin`).
    fn read_identifier(&mut self) -> String {
        let mut ident = String::new();

        ident.push(self.read_char().unwrap());

        while self.peek_char().map(|ch| Lexer::is_identifier(*ch)) == Some(true) {
            ident.push(self.read_char().unwrap());
        }

        ident
    }

    /// Read a number and collect it into a `TokenType`. If a '.' is found after reading the first
    /// digits, a `TokenType::Float` will be returned, otherwise a `TokenType::Integer`.
    /// This function must only be called when the next char in the input is a digit.
    fn read_number(&mut self) -> TokenType {
        let mut number = String::new();

        number.push(self.read_char().unwrap());

        while self.peek_char().map(|ch| ch.is_digit(10)) == Some(true) {
            number.push(self.read_char().unwrap());
        }

        TokenType::Integer(number.parse().unwrap())
    }

    /// Skip all whitespace characters.
    fn skip_whitespace(&mut self) {
        while self.peek_char().map(|ch| ch.is_whitespace()) == Some(true) {
            self.read_char();
        }
    }

    /// Returns whether `ch` can be the begin of an identifier.
    fn is_identifier_begin(ch: char) -> bool {
        ch.is_alphabetic() || "'!@#$%^&*-=+|:/?,.<>`~_".contains(ch)
    }

    /// Returns whether `ch` can be inside an identifier.
    fn is_identifier(ch: char) -> bool {
        ch.is_alphabetic() || ch.is_digit(10) || "'!@#$%^&*-=+|:/?,.<>`~_".contains(ch)
    }
}

impl<'i> Iterator for Lexer<'i> {
    type Item = Result<Token, LexerError>;

    fn next(&mut self) -> Option<Result<Token, LexerError>> {
        match self.next_token() {
            Ok(t) => Some(Result::Ok(t)),
            Err(e) => Some(Result::Err(e)),
            Eof => None,
        }
    }
}
