use std::{collections::{HashMap, hash_map::Entry}};
use crate::{lit::Lit, error::LoxError, token::Token};

pub struct Environment {
    values: HashMap<String, Lit>,
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
        }
    }

    pub fn define(&mut self, name: &str, value: Lit) {
        self.values.insert(name.to_string(), value);
    }

    pub fn get(&self, name: &Token) -> Result<Lit, LoxError> {
        match self.values.get(&name.lexeme) {
            Some(lit) => Ok(lit.clone()),
            _ => Err(LoxError::runtime_error(name.clone(), &format!("Undefined variable '{}'.", name.lexeme)))
        }
    }

    pub fn assign(&mut self, name: &Token, value: Lit) -> Result<(), LoxError> {
        if let Entry::Occupied(mut ent) = self.values.entry(name.lexeme.clone()) {
            ent.insert(value);
            Ok(())
        } else {
            Err(LoxError::runtime_error(name.clone(), &format!("Undefined variable {}.", name.lexeme)))
        }
    }
}