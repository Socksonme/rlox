use crate::error::LoxError;
use crate::expr::*;
use crate::token::*;
use crate::token_type::TokenType;

pub struct Interpreter {}

impl ExprVisitor<Lit> for Interpreter {
    fn visit_binary_expr(&self, expr: &BinaryExpr) -> Result<Lit, LoxError> {
        let left = self.evaluate(&expr.left);
        let right = self.evaluate(&expr.right);

        Ok(Lit::Nil)
    }
    fn visit_unary_expr(&self, expr: &UnaryExpr) -> Result<Lit, LoxError> {
        let right = self.evaluate(&expr.right)?;

        match expr.operator.ttype {
            TokenType::Minus => match &right {
                Lit::Num(n) => return Ok(Lit::Num(-n)),
                _ => return Ok(Lit::Nil),
            },
            TokenType::Bang => {
                if !self.is_truthy(right) {
                    return Ok(Lit::True);
                } else {
                    return Ok(Lit::False);
                }
            }
            _ => {}
        }
        Err(LoxError::error(0, "Unreachable code."))
    }
    fn visit_grouping_expr(&self, expr: &GroupingExpr) -> Result<Lit, LoxError> {
        self.evaluate(&expr.expression)
    }
    fn visit_literal_expr(&self, expr: &LiteralExpr) -> Result<Lit, LoxError> {
        Ok(expr.value.clone().unwrap())
    }
}

impl Interpreter {
    pub fn evaluate(&self, expr: &Expr) -> Result<Lit, LoxError> {
        expr.accept(self)
    }
    /// Returns `true` on everything excpet [`Lit::False`] and [`Lit::Nil`]
    fn is_truthy(&self, lit: Lit) -> bool {
        !matches!(lit, Lit::False | Lit::Nil)   
    }
}
