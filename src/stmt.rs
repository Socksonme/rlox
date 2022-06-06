use crate::error::*;
use crate::expr::*;

pub enum Stmt {
    Expression(ExpressionStmt),
    Print(PrintStmt),
}

impl Stmt {
    pub fn accept<T>(&self, visitor: &dyn StmtVisitor<T>) -> Result<T, LoxError> {
        match self {
            Stmt::Expression(stmt) => {
                stmt.accept(visitor)
            }
            Stmt::Print(stmt) => {
                stmt.accept(visitor)
            }
        }
    }
}

pub struct ExpressionStmt {
    pub expression: Expr,
}

pub struct PrintStmt {
    pub expression: Expr,
}

pub trait StmtVisitor<T> {
    fn visit_expression_stmt(&self, expr: &ExpressionStmt) -> Result<T, LoxError>;
    fn visit_print_stmt(&self, expr: &PrintStmt) -> Result<T, LoxError>;
}

impl ExpressionStmt {
    pub fn accept<T>(&self, visitor: &dyn StmtVisitor<T>) -> Result<T, LoxError> {
        visitor.visit_expression_stmt(self)
    }
}

impl PrintStmt {
    pub fn accept<T>(&self, visitor: &dyn StmtVisitor<T>) -> Result<T, LoxError> {
        visitor.visit_print_stmt(self)
    }
}

