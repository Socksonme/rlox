use crate::{error::*, expr::*, lit::*, token::*, token_type::*, stmt::*};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
    had_error: bool,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0, had_error: false }
    }

    pub fn parse(&mut self) -> Result<Vec<Stmt>, LoxError> {
        let mut statements = Vec::new();
        while !self.is_at_end() {
            statements.push(self.declaration()?);
        }

        Ok(statements)
    }

    fn statement(&mut self) -> Result<Stmt, LoxError> {
        if self.matches(&[TokenType::Print]) {
            return self.print_statement();
        }
        self.expression_statement()
    }

    fn print_statement(&mut self) -> Result<Stmt, LoxError> {
        let value = self.expression()?;
        self.consume(TokenType::Semicolon, "Expected ';' after value;")?;
        Ok(Stmt::Print(PrintStmt {expression: value}))
    }

    fn expression_statement(&mut self) -> Result<Stmt, LoxError> {
        let value = self.expression()?;
        self.consume(TokenType::Semicolon, "Expected ';' after value;")?;
        Ok(Stmt::Expression(ExpressionStmt {expression: value}))
    }

    fn expression(&mut self) -> Result<Expr, LoxError> {
        self.assignment()
    }

    fn declaration(&mut self) -> Result<Stmt, LoxError> {
        let result = if self.matches(&[TokenType::Var]) {
            self.var_declaration()
        } else {
            self.statement()
        };

        if result.is_err() {
            self.synchronize();
        }

        result
    }

    fn var_declaration(&mut self) -> Result<Stmt, LoxError> {
        let name = self.consume(TokenType::Identifier, "Expect variable name.")?;

        let initializer = if self.matches(&[TokenType::Equal]) {
            Some(self.expression()?)
        } else {
            None
        };

        self.consume(TokenType::Semicolon, "Expected ';' after variable declaration.")?;
        Ok(Stmt::Var(VarStmt {name, initializer}))
    }

    fn assignment(&mut self) -> Result<Expr, LoxError> {
        let expr = self.equality()?;

        if self.matches(&[TokenType::Equal]) {
            let equals = self.previous();
            let value = self.expression()?;

            // Check if expr is a valid l-value (VariableExpr, aka identifier)
            if let Expr::Variable(v) = expr {
                return Ok(Expr::Assign(AssignExpr {name: v.name, value: Box::new(value)}));
            }
            self.error(equals, "Invalid assignment target.");
        }
        Ok(expr)
    }

    fn equality(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.comparison()?;

        while self.matches(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous();
            let right = Box::new(self.comparison()?);
            expr = Expr::Binary(BinaryExpr {
                left: Box::new(expr),
                operator,
                right,
            });
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.term()?;

        while self.matches(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous();
            let right = Box::new(self.term()?);
            expr = Expr::Binary(BinaryExpr {
                left: Box::new(expr),
                operator,
                right,
            });
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.factor()?;

        while self.matches(&[TokenType::Plus, TokenType::Minus]) {
            let operator = self.previous();
            let right = Box::new(self.factor()?);
            expr = Expr::Binary(BinaryExpr {
                left: Box::new(expr),
                operator,
                right,
            });
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.unary()?;

        while self.matches(&[TokenType::Slash, TokenType::Star]) {
            let operator = self.previous();
            let right = Box::new(self.unary()?);
            expr = Expr::Binary(BinaryExpr {
                left: Box::new(expr),
                operator,
                right,
            });
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, LoxError> {
        if self.matches(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous();
            let right = Box::new(self.unary()?);
            return Ok(Expr::Unary(UnaryExpr { operator, right }));
        }
        self.primary()
    }

    fn primary(&mut self) -> Result<Expr, LoxError> {
        if self.matches(&[TokenType::False]) {
            return Ok(Expr::Literal(LiteralExpr {
                value: Some(Lit::Bool(false)),
            }));
        } else if self.matches(&[TokenType::True]) {
            return Ok(Expr::Literal(LiteralExpr {
                value: Some(Lit::Bool(true)),
            }));
        } else if self.matches(&[TokenType::Nil]) {
            return Ok(Expr::Literal(LiteralExpr {
                value: Some(Lit::Nil),
            }));
        }

        if self.matches(&[TokenType::Number, TokenType::String]) {
            return Ok(Expr::Literal(LiteralExpr {
                value: self.previous().literal,
            }));
        }

        if self.matches(&[TokenType::Identifier]) {
            return Ok(Expr::Variable(VariableExpr {
                name: self.previous(),
            }));
        }

        if self.matches(&[TokenType::LeftParen]) {
            let expr = Box::new(self.expression()?);
            self.consume(TokenType::RightParen, "Expect ')' after expression.")?;
            return Ok(Expr::Grouping(GroupingExpr { expression: expr }));
        }
        Err(LoxError::error(0, "Expect expression."))
    }

    fn consume(&mut self, tt: TokenType, message: &str) -> Result<Token, LoxError> {
        if self.check(&tt) {
            Ok(self.advance())
        } else {
            let p = self.peek();
            Err(self.error(p, message))
        }
    }

    fn error(&mut self, t: Token, message: &str) -> LoxError {
        self.had_error = true;
        LoxError::parse_error(t, message)
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().is(TokenType::Semicolon) {
                return;
            }

            match self.peek().ttype {
                TokenType::Class
                | TokenType::Fun
                | TokenType::Var
                | TokenType::For
                | TokenType::If
                | TokenType::While
                | TokenType::Print
                | TokenType::Return => {
                    return;
                }
                _ => {}
            }
            self.advance();
        }
    }

    fn matches(&mut self, ttypes: &[TokenType]) -> bool {
        for tt in ttypes {
            if self.check(tt) {
                self.advance();
                return true;
            }
        }

        false
    }

    pub fn success(&self) -> bool {
        !self.had_error
    }

    fn check(&self, tt: &TokenType) -> bool {
        if self.is_at_end() {
            false
        } else {
            self.peek().ttype == *tt
        }
    }

    fn is_at_end(&self) -> bool {
        self.tokens[self.current].is(TokenType::Eof)
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn peek(&self) -> Token {
        self.tokens[self.current].clone()
    }

    fn previous(&self) -> Token {
        self.tokens[self.current - 1].clone()
    }
}
