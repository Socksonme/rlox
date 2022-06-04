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