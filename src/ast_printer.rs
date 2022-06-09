use crate::{
    error::LoxResult,
    expr::{BinaryExpr, Expr, ExprVisitor, GroupingExpr, LiteralExpr, UnaryExpr},
};

pub struct AstPrinter;

impl AstPrinter {
    pub fn format(&self, expr: &Expr) -> Result<String, LoxResult> {
        expr.accept(self)
    }

    fn parenthesize(&self, name: &str, exprs: &[&Expr]) -> Result<String, LoxResult> {
        let mut builder = format!("({name}");

        for expr in exprs {
            builder = format!("{builder} {}", expr.accept(self)?);
        }
        builder = format!("{builder})");
        Ok(builder)
    }
}
impl ExprVisitor<String> for AstPrinter {
    fn visit_binary_expr(&self, expr: &BinaryExpr) -> Result<String, LoxResult> {
        self.parenthesize(&expr.operator.lexeme, &[&expr.left, &expr.right])
    }
    fn visit_grouping_expr(&self, expr: &GroupingExpr) -> Result<String, LoxResult> {
        self.parenthesize("group", &[&expr.expression])
    }
    fn visit_literal_expr(&self, expr: &LiteralExpr) -> Result<String, LoxResult> {
        if let Some(val) = &expr.value {
            Ok(val.to_string())
        } else {
            Ok("nil".to_string())
        }
    }
    fn visit_unary_expr(&self, expr: &UnaryExpr) -> Result<String, LoxResult> {
        self.parenthesize(&expr.operator.lexeme, &[&expr.right])
    }
}
