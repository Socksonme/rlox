use crate::{token::Token, token_type::TokenType};

#[derive(Debug)]
pub enum LoxResult {
    ParseError { token: Token, message: String },
    RuntimeError { token: Token, message: String },
    Error { line: usize, message: String },
}

impl LoxResult {
    pub fn error(line: usize, message: &str) -> LoxResult {
        let error = LoxResult::Error {
            line,
            message: message.to_string(),
        };
        error.report("");
        error
    }

    pub fn parse_error(token: Token, message: &str) -> LoxResult {
        let err = LoxResult::ParseError {
            token,
            message: message.to_string(),
        };
        err.report("");
        err
    }

    pub fn runtime_error(token: Token, message: &str) -> LoxResult {
        let err = LoxResult::RuntimeError {
            token,
            message: message.to_string(),
        };
        err.report("");
        err
    }

    pub fn report(&self, loc: &str) {
        match self {
            LoxResult::Error {line, message} => {
                eprintln!("[line: {}] Error{}: {}", line, loc, message);
            }
            LoxResult::ParseError {token, message} |
            LoxResult::RuntimeError {token, message} => {
                if token.is(TokenType::Eof) {
                    eprintln!("{} at end {}", token.line, message);
                } else {
                    eprintln!("{} at '{}' {}", token.line, token, message);
                }
            }
        }
    }
}
