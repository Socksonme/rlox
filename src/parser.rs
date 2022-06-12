use crate::{error::*, expr::*, lit::*, stmt::*, token::*, token_type::*};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
    had_error: bool,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            current: 0,
            had_error: false,
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Stmt>, LoxResult> {
        let mut statements = Vec::new();
        while !self.is_at_end() {
            statements.push(self.declaration()?);
        }

        Ok(statements)
    }

    fn statement(&mut self) -> Result<Stmt, LoxResult> {
        if self.matches(&[TokenType::Print]) {
            return self.print_statement();
        }
        if self.matches(&[TokenType::LeftBrace]) {
            return Ok(Stmt::Block(BlockStmt {
                statements: self.block()?,
            }));
        }
        if self.matches(&[TokenType::If]) {
            return self.if_statement();
        }
        if self.matches(&[TokenType::While]) {
            return self.while_statement();
        }
        if self.matches(&[TokenType::For]) {
            return self.for_statement();
        }
        self.expression_statement()
    }

    fn for_statement(&mut self) -> Result<Stmt, LoxResult> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'for'.")?;
        let initiliazer = if self.matches(&[TokenType::Semicolon]) {
            None
        } else if self.matches(&[TokenType::Var]) {
            Some(self.var_declaration()?)
        } else {
            Some(self.expression_statement()?)
        };

        let condition = if self.check(&TokenType::Semicolon) {
            None
        } else {
            Some(self.expression()?)
        };
        self.consume(TokenType::Semicolon, "Expect ';' after  loop condition.")?;

        let increment = if self.check(&TokenType::RightParen) {
            None
        } else {
            Some(self.expression()?)
        };
        self.consume(TokenType::RightParen, "Expect ')' after for clauses.")?;

        let mut body = self.statement()?;

        if let Some(increment) = increment {
            body = Stmt::Block(BlockStmt {
                statements: vec![
                    body,
                    Stmt::Expression(ExpressionStmt {
                        expression: increment,
                    }),
                ],
            });
        }

        body = Stmt::While(WhileStmt {
            condition: condition.unwrap_or(Expr::Literal(LiteralExpr {
                value: Some(Lit::Bool(true)),
            })),
            body: Box::new(body),
        });

        if let Some(initializer) = initiliazer {
            body = Stmt::Block(BlockStmt {
                statements: vec![initializer, body],
            });
        }

        Ok(body)
    }

    fn while_statement(&mut self) -> Result<Stmt, LoxResult> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'while'.")?;
        let condition = self.expression()?;
        self.consume(TokenType::RightParen, "Expect ')' after condition.")?;
        let body = self.statement()?;

        Ok(Stmt::While(WhileStmt {
            condition,
            body: Box::new(body),
        }))
    }

    fn if_statement(&mut self) -> Result<Stmt, LoxResult> {
        self.consume(TokenType::LeftParen, "Expected '(' after if.")?;
        let condition = self.expression()?;
        self.consume(TokenType::RightParen, "Expected ')' after if condition.")?;

        let then_branch = Box::new(self.statement()?);
        let else_branch = if self.matches(&[TokenType::Else]) {
            Some(Box::new(self.statement()?))
        } else {
            None
        };
        Ok(Stmt::If(IfStmt {
            condition,
            then_branch,
            else_branch,
        }))
    }

    fn print_statement(&mut self) -> Result<Stmt, LoxResult> {
        let value = self.expression()?;
        self.consume(TokenType::Semicolon, "Expected ';' after value;")?;
        Ok(Stmt::Print(PrintStmt { expression: value }))
    }

    fn expression_statement(&mut self) -> Result<Stmt, LoxResult> {
        let value = self.expression()?;
        self.consume(TokenType::Semicolon, "Expected ';' after value;")?;
        Ok(Stmt::Expression(ExpressionStmt { expression: value }))
    }

    fn block(&mut self) -> Result<Vec<Stmt>, LoxResult> {
        let mut statements = Vec::new();
        while !self.check(&TokenType::RightBrace) && !self.is_at_end() {
            statements.push(self.declaration()?);
        }
        self.consume(TokenType::RightBrace, "Expect '}' after block.")?;
        Ok(statements)
    }

    fn expression(&mut self) -> Result<Expr, LoxResult> {
        self.assignment()
    }

    fn declaration(&mut self) -> Result<Stmt, LoxResult> {
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

    fn var_declaration(&mut self) -> Result<Stmt, LoxResult> {
        let name = self.consume(TokenType::Identifier, "Expect variable name.")?;

        let initializer = if self.matches(&[TokenType::Equal]) {
            Some(self.expression()?)
        } else {
            None
        };

        self.consume(
            TokenType::Semicolon,
            "Expected ';' after variable declaration.",
        )?;
        Ok(Stmt::Var(VarStmt { name, initializer }))
    }

    fn assignment(&mut self) -> Result<Expr, LoxResult> {
        let expr = self.or()?;

        // Because assignment is right-associative
        if self.matches(&[TokenType::Equal]) {
            let equals = self.previous();
            let value = self.expression()?;

            // Check if expr is a valid l-value (VariableExpr, aka identifier)
            if let Expr::Variable(v) = expr {
                return Ok(Expr::Assign(AssignExpr {
                    name: v.name,
                    value: Box::new(value),
                }));
            }
            self.error(equals, "Invalid assignment target.");
        }
        Ok(expr)
    }

    fn or(&mut self) -> Result<Expr, LoxResult> {
        let mut expr = self.and()?;

        while self.matches(&[TokenType::Or]) {
            let operator = self.previous();
            let right = Box::new(self.and()?);
            expr = Expr::Logical(LogicalExpr {
                left: Box::new(expr),
                operator,
                right,
            });
        }
        Ok(expr)
    }

    fn and(&mut self) -> Result<Expr, LoxResult> {
        let mut expr = self.equality()?;

        while self.matches(&[TokenType::And]) {
            let operator = self.previous();
            let right = Box::new(self.equality()?);
            expr = Expr::Logical(LogicalExpr {
                left: Box::new(expr),
                operator,
                right,
            });
        }
        Ok(expr)
    }

    fn equality(&mut self) -> Result<Expr, LoxResult> {
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

    fn comparison(&mut self) -> Result<Expr, LoxResult> {
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

    fn term(&mut self) -> Result<Expr, LoxResult> {
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

    fn factor(&mut self) -> Result<Expr, LoxResult> {
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

    fn unary(&mut self) -> Result<Expr, LoxResult> {
        if self.matches(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous();
            let right = Box::new(self.unary()?);
            return Ok(Expr::Unary(UnaryExpr { operator, right }));
        }
        self.call()
    }

    fn finish_call(&mut self, callee: Expr) -> Result<Expr, LoxResult> {
        let mut arguments = vec![];
        if !self.check(&TokenType::RightParen) {
            arguments.push(self.expression()?);
            while self.matches(&[TokenType::Comma]) {
                if arguments.len() >= 255 {
                    self.error(self.peek(), "Can't have more than 255 arguments.");
                } else {
                    arguments.push(self.expression()?);
                }
            }
        }

        let paren = self.consume(TokenType::RightParen, "Expected ')' after arguments")?;
        Ok(Expr::Call(CallExpr {
            callee: Box::new(callee),
            arguments,
            paren,
        }))
    }

    fn call(&mut self) -> Result<Expr, LoxResult> {
        let mut expr = self.primary()?;

        loop {
            if self.matches(&[TokenType::LeftParen]) {
                expr = self.finish_call(expr)?;
            } else {
                break;
            }
        }

        Ok(expr)
    }

    fn primary(&mut self) -> Result<Expr, LoxResult> {
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
        let peek = self.peek();
        Err(LoxResult::parse_error(peek, "Expect expression."))
    }

    fn consume(&mut self, tt: TokenType, message: &str) -> Result<Token, LoxResult> {
        if self.check(&tt) {
            Ok(self.advance())
        } else {
            let p = self.peek();
            Err(self.error(p, message))
        }
    }

    fn error(&mut self, t: Token, message: &str) -> LoxResult {
        self.had_error = true;
        LoxResult::parse_error(t, message)
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
