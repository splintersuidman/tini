use crate::{Identifier, Position};
use std::error;
use std::fmt;

/// The error type of `Interpreter`.
#[derive(Debug)]
pub enum InterpreterError {
    /// An unknown variable name was found.
    UnknownVariable {
        name: Identifier,
        position: Position,
    },
    /// The wrong type was given.
    TypeError {
        expected: &'static str,
        found: &'static str,
        position: Position,
    },
    /// The number of arguments given does not match the expected number of arguments.
    ArgumentError {
        got: usize,
        takes: usize,
        position: Position,
    },
    /// `ArgumentError`, but for built-in functions.
    BuiltinArgumentError {
        name: &'static str,
        got: usize,
        takes: &'static str,
    },
    /// `TypeError`, but for built-in functions.
    BuiltinTypeError {
        name: &'static str,
        expected: &'static str,
        found: &'static str,
    },
}

impl fmt::Display for InterpreterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::InterpreterError::*;
        match self {
            UnknownVariable { name, position } => {
                write!(f, "unknown variable {} at {}", name, position)
            }
            TypeError {
                expected,
                found,
                position,
            } => write!(
                f,
                "type error: expected {} at {}, found {}",
                expected, position, found
            ),
            ArgumentError {
                got,
                takes,
                position,
            } => write!(
                f,
                "function at {} takes {} arguments, but got {}",
                position, takes, got,
            ),
            BuiltinArgumentError { name, got, takes } => write!(
                f,
                "built-in function {} takes {} arguments, but got {}",
                name, takes, got
            ),
            BuiltinTypeError {
                name,
                found,
                expected,
            } => write!(
                f,
                "built-in function {} expected argument of type {}, but got {}",
                name, expected, found
            ),
        }
    }
}

impl error::Error for InterpreterError {}
