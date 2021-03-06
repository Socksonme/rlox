use crate::error::*;
use crate::token::*;
use crate::lit::*;

pub enum Expr {
    Assign(AssignExpr),
    Binary(BinaryExpr),
    Call(CallExpr),
    Grouping(GroupingExpr),
    Literal(LiteralExpr),
    Logical(LogicalExpr),
    Unary(UnaryExpr),
    Variable(VariableExpr),
}

impl Expr {
    pub fn accept<T>(&self, visitor: &mut dyn ExprVisitor<T>) -> Result<T, LoxResult> {
        match self {
            Expr::Assign(expr) => {
                expr.accept(visitor)
            }
            Expr::Binary(expr) => {
                expr.accept(visitor)
            }
            Expr::Call(expr) => {
                expr.accept(visitor)
            }
            Expr::Grouping(expr) => {
                expr.accept(visitor)
            }
            Expr::Literal(expr) => {
                expr.accept(visitor)
            }
            Expr::Logical(expr) => {
                expr.accept(visitor)
            }
            Expr::Unary(expr) => {
                expr.accept(visitor)
            }
            Expr::Variable(expr) => {
                expr.accept(visitor)
            }
        }
    }
}

pub struct AssignExpr {
    pub name: Token,
    pub value: Box<Expr>,
}

pub struct BinaryExpr {
    pub left: Box<Expr>,
    pub operator: Token,
    pub right: Box<Expr>,
}

pub struct CallExpr {
    pub callee: Box<Expr>,
    pub paren: Token,
    pub arguments: Vec<Expr>,
}

pub struct GroupingExpr {
    pub expression: Box<Expr>,
}

pub struct LiteralExpr {
    pub value: Option<Lit>,
}

pub struct LogicalExpr {
    pub left: Box<Expr>,
    pub operator: Token,
    pub right: Box<Expr>,
}

pub struct UnaryExpr {
    pub operator: Token,
    pub right: Box<Expr>,
}

pub struct VariableExpr {
    pub name: Token,
}

pub trait ExprVisitor<T> {
    fn visit_assign_expr(&mut self, expr: &AssignExpr) -> Result<T, LoxResult>;
    fn visit_binary_expr(&mut self, expr: &BinaryExpr) -> Result<T, LoxResult>;
    fn visit_call_expr(&mut self, expr: &CallExpr) -> Result<T, LoxResult>;
    fn visit_grouping_expr(&mut self, expr: &GroupingExpr) -> Result<T, LoxResult>;
    fn visit_literal_expr(&mut self, expr: &LiteralExpr) -> Result<T, LoxResult>;
    fn visit_logical_expr(&mut self, expr: &LogicalExpr) -> Result<T, LoxResult>;
    fn visit_unary_expr(&mut self, expr: &UnaryExpr) -> Result<T, LoxResult>;
    fn visit_variable_expr(&mut self, expr: &VariableExpr) -> Result<T, LoxResult>;
}

impl AssignExpr {
    pub fn accept<T>(&self, visitor: &mut dyn ExprVisitor<T>) -> Result<T, LoxResult> {
        visitor.visit_assign_expr(self)
    }
}

impl BinaryExpr {
    pub fn accept<T>(&self, visitor: &mut dyn ExprVisitor<T>) -> Result<T, LoxResult> {
        visitor.visit_binary_expr(self)
    }
}

impl CallExpr {
    pub fn accept<T>(&self, visitor: &mut dyn ExprVisitor<T>) -> Result<T, LoxResult> {
        visitor.visit_call_expr(self)
    }
}

impl GroupingExpr {
    pub fn accept<T>(&self, visitor: &mut dyn ExprVisitor<T>) -> Result<T, LoxResult> {
        visitor.visit_grouping_expr(self)
    }
}

impl LiteralExpr {
    pub fn accept<T>(&self, visitor: &mut dyn ExprVisitor<T>) -> Result<T, LoxResult> {
        visitor.visit_literal_expr(self)
    }
}

impl LogicalExpr {
    pub fn accept<T>(&self, visitor: &mut dyn ExprVisitor<T>) -> Result<T, LoxResult> {
        visitor.visit_logical_expr(self)
    }
}

impl UnaryExpr {
    pub fn accept<T>(&self, visitor: &mut dyn ExprVisitor<T>) -> Result<T, LoxResult> {
        visitor.visit_unary_expr(self)
    }
}

impl VariableExpr {
    pub fn accept<T>(&self, visitor: &mut dyn ExprVisitor<T>) -> Result<T, LoxResult> {
        visitor.visit_variable_expr(self)
    }
}

