use crate::{interpreter::Interpreter, lit::Lit};

pub trait LoxCallable {
    fn call(&mut self, _interp: &Interpreter, arguments: Vec<Lit>) -> Lit;
    fn arity(&self) -> usize;
}
