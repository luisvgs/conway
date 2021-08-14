use crate::value::*;
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

#[allow(unused_imports)]
use super::Result;

#[derive(Debug)]
pub struct Environment {
    pub vals: HashMap<String, Value>,
    pub enclosing: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            vals: HashMap::new(),
            enclosing: None
        }
    }
    
    pub fn with_ref(environment: Rc<RefCell<Environment>>) -> Self {
        Self {
            vals: HashMap::new(),
            enclosing: Some(environment),
        }
    }

    pub fn get_var(&mut self, name: String) -> Value {
        let output = Value::Nothing;

        if let Some(value) = self.vals.get(&name) {
            value.clone()
        } else if let Some(enclosing) = &self.enclosing {
            return (*enclosing.borrow_mut()).get_var(name.clone())
        } 
        else { output }
    }
    
    pub fn assign(&mut self, name: String, value: Value) {
        if self.vals.contains_key(&name) {
            self.vals.insert(name, value);
        } else if let Some(enclosing) = &self.enclosing {
            return (*enclosing.borrow_mut()).assign(name.clone(), value)
        } 
        panic!("Undefined variable!");
    }

    pub fn define(&mut self, name: String, value: Value) {
        self.vals.insert(name, value.clone());
    }
}
