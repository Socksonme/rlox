use crate::{token::Token, token_type::TokenType};

#[derive(Debug)]
pub struct LoxError {
    token: Option<Token>,
    line: usize,
    message: String,
}

impl LoxError {
    pub fn error(line: usize, message: String) -> LoxError {
        let error = LoxError {
            token: None,
            line,
            message,
        };
        error.report("".to_string());
        error
    }

    pub fn parse_error(token: Token, message: String) -> LoxError {
        let err = LoxError {
            line: token.line,
            token: Some(token),
            message,
        };
        err.report("".to_string());
        err
    }

    pub fn report(&self, loc: String) {
        if let Some(token) = &self.token {
            if token.is(TokenType::Eof) {
                eprintln!("{} at end {}", token.line, self.message);
            } else {
                eprintln!("{} at {} {}", token.line, token.lexeme, self.message);
            }
        }
        eprintln!("[line: {}] Error{}: {}", self.line, loc, self.message);
    }
}
