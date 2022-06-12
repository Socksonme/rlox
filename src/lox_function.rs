use crate::interpreter::Interpreter;
use crate::lox_callable::LoxCallable;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LoxFunction {}
impl LoxCallable for LoxFunction {
    fn call(&mut self, _interp: &Interpreter, arguments: Vec<crate::lit::Lit>) -> crate::lit::Lit {todo!()}
    fn arity(&self) -> usize {todo!()}
}
