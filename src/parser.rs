
use crate::{token::*, token_type::*, error::*, expr::*};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            current: 0,
        }
    }

    fn expression(&mut self) -> Result<Expr, LoxError> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.comparison()?;

        while self.is_match(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous();
            let right = Box::new(self.comparison()?);
            expr = Expr::Binary(BinaryExpr {left: Box::new(expr), operator, right});
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.term()?;

        while self.is_match(&[TokenType::Greater, TokenType::GreaterEqual, TokenType::Less, TokenType::LessEqual]) {
            let operator = self.previous();
            let right = Box::new(self.term()?);
            expr = Expr::Binary(BinaryExpr {left: Box::new(expr), operator, right});
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.factor()?;

        while self.is_match(&[TokenType::Plus, TokenType::Minus]) {
            let operator = self.previous();
            let right = Box::new(self.factor()?);
            expr = Expr::Binary(BinaryExpr {left: Box::new(expr), operator, right});
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.unary()?;

        while self.is_match(&[TokenType::Slash, TokenType::Star]) {
            let operator = self.previous();
            let right = Box::new(self.unary()?);
            expr = Expr::Binary(BinaryExpr {left: Box::new(expr), operator, right});
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, LoxError> {
        if self.is_match(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous();
            let right = Box::new(self.unary()?);
            return Ok(Expr::Unary(UnaryExpr {operator, right}));
        }
        self.primary()
    }

    fn primary(&mut self) -> Result<Expr, LoxError> {
        if self.is_match(&[TokenType::False]) {
            return Ok(Expr::Literal(LiteralExpr {value: Some(Lit::False)}));
        } else if self.is_match(&[TokenType::True]) {
            return Ok(Expr::Literal(LiteralExpr {value: Some(Lit::True)}));
        } else if self.is_match(&[TokenType::Nil]) {
            return Ok(Expr::Literal(LiteralExpr {value: Some(Lit::Nil)}));
        }

        if self.is_match(&[TokenType::Number, TokenType::String]) {
            return Ok(Expr::Literal(LiteralExpr {value: self.previous().literal}));
        }

        if self.is_match(&[TokenType::LeftParen]) {
            let expr = Box::new(self.expression()?);
            self.consume(TokenType::RightParen, "Expect ')' after expression.".to_string())?;
            return Ok(Expr::Grouping(GroupingExpr {expression: expr}));
        }
        Err(LoxError::error(0, "Failed primary parse".to_string()))
    }

    fn consume(&mut self, tt: TokenType, message: String) -> Result<Token, LoxError> {
        if self.check(&tt) {
            Ok(self.advance())
        } else {
            let p = self.peek();
            Err(LoxError::error(p.line, message))
        }
    }

    fn is_match(&mut self, ttypes: &[TokenType]) -> bool {
        for tt in ttypes {
            if self.check(tt) {
                self.advance();
                return true;
            }
        }

        false
    }

    fn check(&self, tt: &TokenType) -> bool {
        if self.is_at_end() {
            false
        } else {
            self.peek().ttype == *tt
        }
    }

    fn is_at_end(&self) -> bool {
        self.tokens[self.current].ttype == TokenType::Eof
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