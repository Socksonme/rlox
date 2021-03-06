use crate::{error::LoxResult, lit::*, token::Token, token_type::*};

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Self {
            source,
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> Result<Vec<Token>, LoxResult> {
        let mut had_error = None;

        while !self.is_at_end() {
            self.start = self.current;
            match self.scan_token() {
                Ok(_) => {}
                Err(e) => {
                    had_error = Some(e);
                }
            }
        }

        self.tokens.push(Token::eof(self.line));

        if let Some(err) = had_error {
            Err(err)
        } else {
            Ok(self.tokens.clone())
        }
    }

    pub fn scan_token(&mut self) -> Result<(), LoxResult> {
        let c = if let Some(c) = self.bump() {
            c
        } else {
            return Ok(());
        };

        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),
            '!' => {
                let tok = if self.matches('=') {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                };
                self.add_token(tok);
            }
            '=' => {
                let tok = if self.matches('=') {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                };
                self.add_token(tok);
            }
            '<' => {
                let tok = if self.matches('=') {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                };
                self.add_token(tok);
            }
            '>' => {
                let tok = if self.matches('=') {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                };
                self.add_token(tok);
            }
            '/' => {
                if self.matches('/') {
                    // A comment that goes until the end of the line
                    while let Some(pk) = self.peek() {
                        if pk == '\n' {
                            break;
                        }
                        self.advance();
                    }
                } else if self.matches('*') {
                    // block comment start
                    self.scan_comment()?;
                } else {
                    self.add_token(TokenType::Slash)
                }
            }
            ' ' | '\r' | '\t' => {}
            '\n' => {
                self.line += 1;
            }
            '"' => {
                self.string()?;
            }
            c if c.is_numeric() => {
                self.number()?;
            }
            c if c.is_alphabetic() => {
                self.identifier()?;
            }
            _ => {
                return Err(LoxResult::error(self.line, "Unexpected character."));
            }
        }
        Ok(())
    }

    fn scan_comment(&mut self) -> Result<(), LoxResult> {
        loop {
            match self.peek() {
                Some('*') => {
                    self.advance();
                    if self.matches('/') {
                        return Ok(());
                    }
                }
                Some('/') => {
                    self.advance();
                    if self.matches('*') {
                        self.scan_comment()?;
                    }
                }
                Some('\n') => {
                    self.line += 1;
                }
                None => {
                    return Err(LoxResult::error(
                        self.line,
                        "Unterminated multi-line comment.",
                    ));
                }
                _ => {
                    self.advance();
                }
            }
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) {
        self.current += 1;
    }

    fn bump(&mut self) -> Option<char> {
        let res = self.peek();
        self.advance();
        res
    }

    fn add_token(&mut self, ttype: TokenType) {
        self.add_token_lit(ttype, None);
    }

    fn add_token_lit(&mut self, ttype: TokenType, lit: Option<Lit>) {
        let lexeme = &self.source[self.start..self.current];
        self.tokens.push(Token::new(ttype, lexeme, lit, self.line))
    }

    /// If the next character matches the expected character, advance to it and then return true.
    fn matches(&mut self, expected: char) -> bool {
        if self.peek().map_or(false, |cur| cur == expected) {
            self.current += 1;
            return true;
        }
        false
    }

    /// Peeks at the next character. Returns `None` on the end of source
    pub fn peek(&self) -> Option<char> {
        if self.is_at_end() {
            return None;
        }
        Some(self.source.chars().nth(self.current).unwrap())
    }

    /// Peeks at the character after the next character. Returns `None` on the end of source
    pub fn peek_next(&self) -> Option<char> {
        if self.is_at_end() {
            return None;
        }
        Some(self.source.chars().nth(self.current + 1).unwrap())
    }

    pub fn string(&mut self) -> Result<(), LoxResult> {
        while let Some(pk) = self.peek() {
            match pk {
                '"' => break,
                '\n' => self.line += 1,
                _ => {}
            }
            self.advance();
        }
        // If not at the end, then guranteed next to to be '"'
        if self.is_at_end() {
            return Err(LoxResult::error(self.line, "Unterminated String."));
        }
        // TODO: Handle escape sequences such ads "\\" or "\n" etc.
        self.advance();
        let value = String::from(&self.source[self.start + 1..self.current - 1]);
        self.add_token_lit(TokenType::String, Some(Lit::Str(value)));
        Ok(())
    }

    fn identifier(&mut self) -> Result<(), LoxResult> {
        while self.peek().map_or(false, |c| c.is_alphanumeric()) {
            self.advance();
        }

        let value = &self.source[self.start..self.current];
        if let Some(keyword) = Self::keywords(value) {
            self.add_token(keyword);
        } else {
            self.add_token(TokenType::Identifier);
        }

        Ok(())
    }

    pub fn number(&mut self) -> Result<(), LoxResult> {
        while let Some(c) = self.peek() {
            match c {
                c if c.is_numeric() => {
                    self.advance();
                    continue;
                }
                _ => {
                    break;
                }
            }
        }

        if let Some(c) = self.peek() {
            if c == '.' && self.peek_next().map_or(false, |pk| pk.is_numeric()) {
                self.advance();

                while self.peek().map_or(false, |pk| pk.is_numeric()) {
                    self.advance();
                }
            }
        }

        self.add_token_lit(
            TokenType::Number,
            Some(Lit::Num(
                (&self.source[self.start..self.current]).parse().unwrap(),
            )),
        );
        Ok(())
    }

    pub fn keywords(s: &str) -> Option<TokenType> {
        match s {
            "and" => Some(TokenType::And),
            "class" => Some(TokenType::Class),
            "else" => Some(TokenType::Else),
            "false" => Some(TokenType::False),
            "for" => Some(TokenType::For),
            "fun" => Some(TokenType::Fun),
            "if" => Some(TokenType::If),
            "nil" => Some(TokenType::Nil),
            "or" => Some(TokenType::Or),
            "print" => Some(TokenType::Print),
            "return" => Some(TokenType::Return),
            "super" => Some(TokenType::Super),
            "this" => Some(TokenType::This),
            "true" => Some(TokenType::True),
            "var" => Some(TokenType::Var),
            "while" => Some(TokenType::While),
            _ => None,
        }
    }
}
