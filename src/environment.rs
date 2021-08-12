use crate::value::*;
use std::collections::HashMap;

#[allow(unused_imports)]
use super::Result;

#[derive(Debug)]
pub struct Environment {
    pub vals: HashMap<String, Value>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            vals: HashMap::new(),
        }
    }

    pub fn get_var(&mut self, name: String) -> Value {
        let output = Value::Nothing;

        if let Some(value) = self.vals.get(&name) {
            value.clone()
        } else { output }
    }
    
    pub fn assign(&mut self, name: String, value: Value) {
        if self.vals.contains_key(&name) {
            self.vals.insert(name, value);
        } 
        panic!("Undefined variable!");
    }

    pub fn define(&mut self, name: String, value: Value) {
        self.vals.insert(name, value.clone());
    }
}
