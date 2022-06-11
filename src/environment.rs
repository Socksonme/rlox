use crate::{error::LoxResult, lit::Lit, token::Token};
use std::{
    cell::RefCell,
    collections::{hash_map, HashMap},
    rc::Rc,
};
use ghost_cell::{GhostCell, GhostToken};
use crate::ref_chain::*;

pub struct Environment<'brand, 'enclosing>(pub RefChain<'brand, 'enclosing, HashMap<String, Lit>>);

impl<'brand, 'enclosing> Environment<'brand, 'enclosing> {
    pub fn new(token: &'enclosing mut GhostToken) -> Self {
        Self {
            0: RefChain::new(HashMap::new(), token)
        }
    }

    pub fn new_with_enclosing<'prev_enclosing>(
        enclosing: &'enclosing mut Environment<'brand, 'prev_enclosing>,
    ) -> Self {
        Self(RefChain::with_prev(&mut enclosing.0, HashMap::new()))
    }


    pub fn define(&mut self, name: &str, value: Lit) {
        self.0.get_mut().insert(name.to_string(), value);
    }

    pub fn get(&self, name: &Token) -> Result<Lit, LoxResult> {
        if let Some(lit) = self.0.get().get(&name.lexeme) {
            return Ok(lit.clone());
        } else if let Some(enclosing) = self.0.entry.prev {
            return enclosing.data.get(name);
        }

        Err(LoxResult::runtime_error(
            name.clone(),
            &format!("Undefined variable '{}'.", name.lexeme),
        ))
    }

    pub fn assign(&mut self, name: &Token, value: Lit) -> Result<(), LoxResult> {
        if let hash_map::Entry::Occupied(mut ent) = self.values.entry(name.lexeme.clone()) {
            ent.insert(value);
            Ok(())
        } else if let Some(enclosing) = &self.enclosing {
            enclosing.borrow_mut().assign(name, value)
        } else {
            Err(LoxResult::runtime_error(
                name.clone(),
                &format!("Undefined variable {}.", name.lexeme),
            ))
        }
    }
}
