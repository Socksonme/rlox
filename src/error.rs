use crate::{token::Token, token_type::TokenType};

#[derive(Debug)]
pub struct LoxError {
    token: Option<Token>,
    line: usize,
    pub message: String,
}

impl LoxError {
    pub fn error(line: usize, message: &str) -> LoxError {
        let error = LoxError {
            token: None,
            line,
            message: message.to_string(),
        };
        error.report("");
        error
    }

    pub fn parse_error(token: Token, message: &str) -> LoxError {
        let err = LoxError {
            line: token.line,
            token: Some(token),
            message: message.to_string(),
        };
        err.report("");
        err
    }

    pub fn runtime_error(token: Token, message: &str) -> LoxError {
        let err = LoxError {
            line: token.line,
            token: Some(token),
            message: message.to_string(),
        };
        err.report("");
        err
    }

    pub fn to_runtime(&self, token: Token) -> Self {
        Self {
            line: token.line,
            message: self.message.clone(),
            token: Some(token),
        }
    }

    pub fn report(&self, loc: &str) {
        if let Some(token) = &self.token {
            if token.is(TokenType::Eof) {
                eprintln!("{} at end {}", token.line, self.message);
            } else {
                eprintln!("{} at '{}' {}", token.line, token, self.message);
            }
        } else {
            eprintln!("[line: {}] Error{}: {}", self.line, loc, self.message);
        }
    }
}
