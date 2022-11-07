use core::fmt;
use std::rc::Rc;
use std::time::SystemTime;

use crate::error::LoxResult;
use crate::interpreter::Interpreter;
use crate::lit::Lit;
use crate::lox_callable::LoxCallable;

#[derive(Clone)]
pub struct LoxNative {
    pub func: Rc<dyn LoxCallable>,
}

impl PartialEq for LoxNative {
    fn eq(&self, other: &Self) -> bool {
        Rc::as_ptr(&self.func) == Rc::as_ptr(&other.func)
    }
}

impl fmt::Debug for LoxNative {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<Native-Function>")
    }
}

impl fmt::Display for LoxNative {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<native fn>")
    }
}

pub struct NativeClock;

impl LoxCallable for NativeClock {
    fn call(
        &self,
        _interp: &Interpreter,
        arguments: Vec<crate::lit::Lit>,
    ) -> Result<Lit, LoxResult> {
        match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
            Ok(n) => Ok(Lit::Num(n.as_millis() as f64)),
            Err(e) => Err(LoxResult::system_error(&format!(
                "Clock returned invalid duration: {:?}",
                e.duration()
            ))),
        }
    }

    fn arity(&self) -> usize {
        0
    }
}
