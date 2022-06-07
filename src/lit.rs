use std::fmt::Display;
#[derive(Debug, Clone, PartialEq)]
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
                    format!("\"{}\"", s.clone())
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

impl Lit {
    pub fn is_truthy(&self) -> bool {
        !matches!(self, Lit::Bool(false) | Lit::Nil)
    }
}

impl From<Lit> for bool {
    /// Returns true on everything except `Lit::Bool(false)` and [`Lit::Nil`]
    fn from(lit: Lit) -> bool {
        lit.is_truthy()
    }
}
