use crate::interpreter::Value;
use crate::Identifier;
use std::collections::HashMap;
use std::rc::Rc;

/// The `Environment` contains all variable and function bindings.
#[derive(Default)]
pub struct Environment {
    env: HashMap<Identifier, Rc<Value>>,
}

impl Environment {
    /// Create a new `Environment`.
    pub fn new() -> Environment {
        Environment::default()
    }

    /// Get a reference to a variable from the `Environment`.
    pub fn get(&self, key: &Identifier) -> Option<Rc<Value>> {
        self.env.get(key).map(Clone::clone)
    }

    /// Take a value from the `Environment`, getting ownership of the value and removing the
    /// variable from the `Environment`.
    pub fn take(&mut self, key: &Identifier) -> Option<Rc<Value>> {
        self.env.remove(key)
    }

    /// Set a variable in the `Environment` and returns the previous value of that variable, if
    /// any.
    pub fn set(&mut self, key: Identifier, value: Rc<Value>) -> Option<Rc<Value>> {
        self.env.insert(key, value)
    }
}
