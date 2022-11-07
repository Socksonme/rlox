use crate::interpreter::Interpreter;
use crate::lox_callable::LoxCallable;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LoxFunction {}
impl LoxCallable for LoxFunction {
    fn call(
        &self,
        _interp: &Interpreter,
        arguments: Vec<crate::lit::Lit>,
    ) -> Result<crate::lit::Lit, crate::error::LoxResult> {
        todo!()
    }
    fn arity(&self) -> usize {
        todo!()
    }
}
