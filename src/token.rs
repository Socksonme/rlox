use std::fmt::Display;

use crate::token_type::TokenType;

#[derive(Debug)]
pub enum Lit {
    Num(f64),
    Str(String),
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
            }
        )
    }
}

#[derive(Debug)]
pub struct Token {
    ttype: TokenType,
    lexeme: String,
    literal: Option<Lit>,
    line: usize,
}

impl Token {
    pub fn new(ttype: TokenType, lexeme: String, literal: Option<Lit>, line: usize) -> Self {
        Self {
            ttype,
            lexeme,
            literal,
            line,
        }
    }
    pub fn eof(line: usize) -> Self {
        Self {
            ttype: TokenType::Eof,
            lexeme: String::new(),
            literal: None,
            line,
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?} {} {}",
            self.ttype,
            self.lexeme,
            if let Some(lit) = &self.literal {
                lit.to_string()
            } else {
                String::new()
            }
        )
    }
}
