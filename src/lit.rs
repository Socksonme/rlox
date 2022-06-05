use std::{
    fmt::Display,
    ops::{Div, Mul, Sub, Add},
    cmp::{PartialOrd, Ordering},
};

use crate::error::LoxError;

#[derive(Debug, Clone)]
pub enum Lit {
    Num(f64),
    Str(String),
    Bool(bool),
    Nil,
}

impl Display for Lit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Lit::Num(n) => {
                    n.to_string()
                }
                Lit::Str(s) => {
                    s.clone()
                }
                Lit::Bool(b) => {
                    b.to_string()
                }
                Lit::Nil => {
                    String::from("nil")
                }
            }
        )
    }
}

impl From<Lit> for bool {
    /// Returns true on everything except `Lit::Bool(false)` and [`Lit::Nil`]
    fn from(lit: Lit) -> bool {
        !matches!(lit, Lit::Bool(false) | Lit::Nil)
    }
}

impl Sub for Lit {
    type Output = Result<Lit, LoxError>;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Lit::Num(a), Lit::Num(b)) => Ok(Lit::Num(a - b)),
            _ => Err(LoxError::error(0, "Illegal expression."))
        }
    }
}

impl Div for Lit {
    type Output = Result<Lit, LoxError>;
    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Lit::Num(a), Lit::Num(b)) => Ok(Lit::Num(a / b)),
            _ => Err(LoxError::error(0, "Illegal expression."))
        }
    }
}

impl Mul for Lit {
    type Output = Result<Lit, LoxError>;
    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Lit::Num(a), Lit::Num(b)) => Ok(Lit::Num(a * b)),
            _ => Err(LoxError::error(0, "Illegal expression."))
        }
    }
}

impl Add for Lit {
    type Output = Result<Lit, LoxError>;
    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Lit::Num(a), Lit::Num(b)) => Ok(Lit::Num(a + b)),
            (Lit::Str(a), Lit::Str(b)) => Ok(Lit::Str(format!("{}{}", a, b))),
            _ => Err(LoxError::error(0, "Illegal expression.")),
        }
    }
}

impl PartialOrd for Lit {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Lit::Num(a), Lit::Num(b)) => a.partial_cmp(b),
            _ => None
        }
    }
}

impl PartialEq for Lit {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Lit::Num(a), Lit::Num(b)) => a == b,
            (Lit::Str(a), Lit::Str(b)) => a == b,
            (Lit::Bool(a), Lit::Bool(b)) => a == b,
            (Lit::Nil, o) => {
                if let Lit::Nil = o {
                    true
                } else {
                    false
                }
            }
            _ => false,
        }
    }
    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}