use crate::ast::AST;
use crate::interpreter::InterpreterError;
use crate::Identifier;
use std::fmt;
use std::rc::Rc;

/// The type of a built-in function.
pub type BuiltinFunction = fn(Vec<Rc<Value>>) -> Result<Rc<Value>, InterpreterError>;

/// The representation of a value in `tini`.
#[derive(Clone, Debug)]
pub enum Value {
    /// An integer.
    Integer(i64),
    /// A function.
    Function {
        arguments: Vec<Identifier>,
        value: AST,
    },
    /// A built-in function.
    Builtin(BuiltinFunction),
}

impl Value {
    /// Return the type name of a value.
    pub fn type_name(&self) -> &'static str {
        use self::Value::*;

        match self {
            Integer(_) => "int",
            Function { .. } => "function",
            Builtin(_) => "function",
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Value::*;

        match self {
            Integer(v) => write!(f, "{}", v),
            Function { .. } => write!(f, "<function>"),
            Builtin(_) => write!(f, "<function>"),
        }
    }
}
