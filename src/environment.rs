use crate::{error::LoxError, lit::Lit, token::Token};
use std::{
    cell::RefCell,
    collections::{hash_map::Entry, HashMap},
    rc::Rc,
};

#[derive(Debug)]
pub struct Environment {
    values: HashMap<String, Lit>,
    enclosing: Option<Rc<RefCell<Environment>>>,
}

impl Default for Environment {
    fn default() -> Self {
        Self::new()
    }
}

impl Environment {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
            enclosing: None,
        }
    }

    pub fn new_with_enclosing(enclosing: Rc<RefCell<Environment>>) -> Self {
        Self {
            values: HashMap::new(),
            enclosing: Some(enclosing),
        }
    }

    pub fn define(&mut self, name: &str, value: Lit) {
        self.values.insert(name.to_string(), value);
    }

    pub fn get(&self, name: &Token) -> Result<Lit, LoxError> {
        if let Some(lit) = self.values.get(&name.lexeme) {
            return Ok(lit.clone());
        } else if let Some(enclosing) = &self.enclosing {
            return enclosing.borrow().get(name);
        }

        Err(LoxError::runtime_error(
            name.clone(),
            &format!("Undefined variable '{}'.", name.lexeme),
        ))
    }

    pub fn assign(&mut self, name: &Token, value: Lit) -> Result<(), LoxError> {
        if let Entry::Occupied(mut ent) = self.values.entry(name.lexeme.clone()) {
            ent.insert(value);
            Ok(())
        } else if let Some(enclosing) = &self.enclosing {
            enclosing.borrow_mut().assign(name, value)
        } else {
            Err(LoxError::runtime_error(
                name.clone(),
                &format!("Undefined variable {}.", name.lexeme),
            ))
        }
    }
}
