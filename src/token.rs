use std::fmt::Display;

use crate::lit::*;
use crate::token_type::TokenType;

#[derive(Debug, Clone)]
pub struct Token {
    pub ttype: TokenType,
    pub lexeme: String,
    pub literal: Option<Lit>,
    pub line: usize,
}

impl Token {
    pub fn new(ttype: TokenType, lexeme: &str, literal: Option<Lit>, line: usize) -> Self {
        Self {
            ttype,
            lexeme: lexeme.to_string(),
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
    pub fn is(&self, tt: TokenType) -> bool {
        self.ttype == tt
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
