//! `interpreter` contains the `Interpreter`, which executes expressions.

pub mod builtins;
mod environment;
mod error;
mod value;

pub use self::environment::*;
pub use self::error::*;
pub use self::value::*;

use crate::ast::{ASTType, AST};
use crate::{Identifier, Position};
use std::rc::Rc;

/// `Interpreter` executes expressions (`AST`s).
pub struct Interpreter {
    /// `env` contains the variable definitions.
    env: Environment,
}

impl Default for Interpreter {
    fn default() -> Interpreter {
        let mut env = Environment::new();
        builtins::add_builtins_to_environment(&mut env);
        Interpreter { env }
    }
}

impl Interpreter {
    /// Create a new `Interpreter`.
    pub fn new() -> Interpreter {
        Interpreter::default()
    }

    /// Evaluate an expression.
    pub fn eval(&mut self, expression: AST) -> Result<Rc<Value>, InterpreterError> {
        use self::ASTType::*;

        let position = expression.position;

        match expression.ast {
            Integer(v) => Ok(Rc::new(Value::Integer(v))),
            // Variable definition.
            Define {
                name,
                arguments: None,
                value,
            } => {
                let value = self.eval(*value)?;
                self.env.set(name, value);
                Ok(Rc::new(Value::Integer(0)))
            }
            // Function definition.
            Define {
                name,
                arguments: Some(arguments),
                value,
            } => {
                let function = Value::Function {
                    arguments,
                    value: *value,
                };
                self.env.set(name, Rc::new(function));
                Ok(Rc::new(Value::Integer(0)))
            }
            If {
                condition,
                consequence,
                alternative,
            } => match &*self.eval(*condition)? {
                Value::Integer(0) => self.eval(*alternative),
                _ => self.eval(*consequence),
            },
            FunctionCall { name, arguments } => {
                let function = match self.env.get(&name) {
                    Some(f) => f,
                    None => return Err(InterpreterError::UnknownVariable { name, position }),
                };
                match &*function {
                    Value::Function {
                        arguments: names,
                        value,
                    } => self.eval_function(
                        names.clone(),
                        arguments.clone(),
                        value.clone(),
                        position,
                    ),
                    Value::Builtin(function) => self.eval_builtin(&function, arguments),
                    v => Err(InterpreterError::TypeError {
                        expected: "function in function call",
                        found: v.type_name(),
                        position,
                    }),
                }
            }
            Identifier(name) => match self.env.get(&name) {
                Some(value) => Ok(value),
                None => Err(InterpreterError::UnknownVariable { name, position }),
            },
        }
    }

    /// Evaluate a built-in function.
    fn eval_builtin(
        &mut self,
        builtin: &BuiltinFunction,
        arguments: Vec<AST>,
    ) -> Result<Rc<Value>, InterpreterError> {
        let arguments = arguments
            .iter()
            .map(|arg| self.eval(arg.clone()))
            .collect::<Result<_, _>>()?;
        builtin(arguments)
    }

    /// Evaluate a non-built-in function.
    fn eval_function(
        &mut self,
        names: Vec<Identifier>,
        arguments: Vec<AST>,
        body: AST,
        position: Position,
    ) -> Result<Rc<Value>, InterpreterError> {
        if names.len() != arguments.len() {
            return Err(InterpreterError::ArgumentError {
                takes: names.len(),
                got: arguments.len(),
                position,
            });
        }

        // Take values with the same name as the arguments out of the environment, to put
        // them back in after the function is called.
        let state: Vec<_> = names
            .iter()
            .map(|arg| (arg.clone(), self.env.get(&arg)))
            .collect();

        for (name, arg) in names.into_iter().zip(arguments) {
            let arg = self.eval(arg)?;
            self.env.set(name, arg);
        }

        let return_value = self.eval(body)?;

        for (name, value) in state {
            if let Some(value) = value {
                self.env.set(name, value);
            } else {
                self.env.take(&name);
            }
        }

        Ok(return_value)
    }
}
