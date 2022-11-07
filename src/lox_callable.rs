use crate::{error::LoxResult, interpreter::Interpreter, lit::Lit};

pub trait LoxCallable {
    fn call(&self, _interp: &Interpreter, arguments: Vec<Lit>) -> Result<Lit, LoxResult>;
    fn arity(&self) -> usize;
}
