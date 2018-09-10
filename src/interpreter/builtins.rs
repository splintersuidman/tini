//! `builtins` contains built-in function definitions.

use crate::interpreter::{Environment, InterpreterError, Value};
use std::rc::Rc;

/// Add the built-in functions (defined in this module – `builtin`) to an `Environment`.
pub fn add_builtins_to_environment(env: &mut Environment) {
    env.set("+".to_string(), Rc::new(Value::Builtin(builtin_add)));
    env.set("-".to_string(), Rc::new(Value::Builtin(builtin_sub)));
    env.set("*".to_string(), Rc::new(Value::Builtin(builtin_mul)));
    env.set("=".to_string(), Rc::new(Value::Builtin(builtin_equals)));
    env.set(
        ">".to_string(),
        Rc::new(Value::Builtin(builtin_is_greater_than)),
    );
    env.set(
        "<".to_string(),
        Rc::new(Value::Builtin(builtin_is_less_than)),
    );
    env.set("print".to_string(), Rc::new(Value::Builtin(builtin_print)));
}

type Arguments = Vec<Rc<Value>>;
type Return = Result<Rc<Value>, InterpreterError>;

// Name: "=".
fn builtin_equals(args: Arguments) -> Return {
    if args.len() != 2 {
        return Err(InterpreterError::BuiltinArgumentError {
            name: "=",
            got: args.len(),
            takes: "2",
        });
    }

    let equal = match (&*args[0], &*args[1]) {
        (Value::Integer(l), Value::Integer(r)) if l == r => true,
        _ => false,
    };

    Ok(Rc::new(Value::Integer(if equal { 1 } else { 0 })))
}

// Name: "+".
fn builtin_add(args: Arguments) -> Return {
    if args.len() != 2 {
        return Err(InterpreterError::BuiltinArgumentError {
            name: "+",
            got: args.len(),
            takes: "2",
        });
    }

    let value = match (&*args[0], &*args[1]) {
        (Value::Integer(l), Value::Integer(r)) => Value::Integer(l + r),
        (Value::Integer(_), invalid) => {
            return Err(InterpreterError::BuiltinTypeError {
                name: "+",
                expected: "int",
                found: invalid.type_name(),
            })
        }
        (invalid, Value::Integer(_)) => {
            return Err(InterpreterError::BuiltinTypeError {
                name: "+",
                expected: "int",
                found: invalid.type_name(),
            })
        }
        _ => {
            return Err(InterpreterError::BuiltinTypeError {
                name: "+",
                expected: "two integers",
                found: "something non-integer",
            })
        }
    };
    Ok(Rc::new(value))
}

// Name: "-".
fn builtin_sub(args: Arguments) -> Return {
    if args.len() != 2 {
        return Err(InterpreterError::BuiltinArgumentError {
            name: "-",
            got: args.len(),
            takes: "2",
        });
    }

    let value = match (&*args[0], &*args[1]) {
        (Value::Integer(l), Value::Integer(r)) => Value::Integer(l - r),
        (Value::Integer(_), invalid) => {
            return Err(InterpreterError::BuiltinTypeError {
                name: "-",
                expected: "int",
                found: invalid.type_name(),
            })
        }
        (invalid, Value::Integer(_)) => {
            return Err(InterpreterError::BuiltinTypeError {
                name: "-",
                expected: "int",
                found: invalid.type_name(),
            })
        }
        _ => {
            return Err(InterpreterError::BuiltinTypeError {
                name: "-",
                expected: "two integers",
                found: "something non-integer",
            })
        }
    };
    Ok(Rc::new(value))
}

// Name: "*".
fn builtin_mul(args: Arguments) -> Return {
    if args.len() != 2 {
        return Err(InterpreterError::BuiltinArgumentError {
            name: "*",
            got: args.len(),
            takes: "2",
        });
    }

    let value = match (&*args[0], &*args[1]) {
        (Value::Integer(l), Value::Integer(r)) => Value::Integer(l * r),
        (Value::Integer(_), invalid) => {
            return Err(InterpreterError::BuiltinTypeError {
                name: "*",
                expected: "int",
                found: invalid.type_name(),
            })
        }
        (invalid, Value::Integer(_)) => {
            return Err(InterpreterError::BuiltinTypeError {
                name: "*",
                expected: "int",
                found: invalid.type_name(),
            })
        }
        _ => {
            return Err(InterpreterError::BuiltinTypeError {
                name: "*",
                expected: "two integers",
                found: "?",
            })
        }
    };
    Ok(Rc::new(value))
}

// Name: "print".
fn builtin_print(args: Arguments) -> Return {
    for (i, arg) in args.iter().enumerate() {
        if i != 0 {
            print!(" ");
        }
        print!("{}", arg);
    }
    println!();
    Ok(Rc::new(Value::Integer(0)))
}

// Name: ">".
fn builtin_is_greater_than(args: Arguments) -> Return {
    if args.len() != 2 {
        return Err(InterpreterError::BuiltinArgumentError {
            name: ">",
            got: args.len(),
            takes: "2",
        });
    }

    let is_greater = match (&*args[0], &*args[1]) {
        (Value::Integer(l), Value::Integer(r)) => l > r,
        _ => false,
    };
    Ok(Rc::new(Value::Integer(if is_greater { 1 } else { 0 })))
}

// Name "<".
fn builtin_is_less_than(args: Arguments) -> Return {
    if args.len() != 2 {
        return Err(InterpreterError::BuiltinArgumentError {
            name: "<",
            got: args.len(),
            takes: "2",
        });
    }

    let is_less = match (&*args[0], &*args[1]) {
        (Value::Integer(l), Value::Integer(r)) => l < r,
        _ => false,
    };
    Ok(Rc::new(Value::Integer(if is_less { 1 } else { 0 })))
}
