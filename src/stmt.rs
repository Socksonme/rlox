use crate::error::*;
use crate::token::*;
use crate::expr::*;

pub enum Stmt {
    Block(BlockStmt),
    Expression(ExpressionStmt),
    If(IfStmt),
    Print(PrintStmt),
    Var(VarStmt),
    While(WhileStmt),
}

impl Stmt {
    pub fn accept<T>(&self, visitor: &mut dyn StmtVisitor<T>) -> Result<T, LoxError> {
        match self {
            Stmt::Block(stmt) => {
                stmt.accept(visitor)
            }
            Stmt::Expression(stmt) => {
                stmt.accept(visitor)
            }
            Stmt::If(stmt) => {
                stmt.accept(visitor)
            }
            Stmt::Print(stmt) => {
                stmt.accept(visitor)
            }
            Stmt::Var(stmt) => {
                stmt.accept(visitor)
            }
            Stmt::While(stmt) => {
                stmt.accept(visitor)
            }
        }
    }
}

pub struct BlockStmt {
    pub statements: Vec<Stmt>,
}

pub struct ExpressionStmt {
    pub expression: Expr,
}

pub struct IfStmt {
    pub condition: Expr,
    pub then_branch: Box<Stmt>,
    pub else_branch: Option<Box<Stmt>>,
}

pub struct PrintStmt {
    pub expression: Expr,
}

pub struct VarStmt {
    pub name: Token,
    pub initializer: Option<Expr>,
}

pub struct WhileStmt {
    pub condition: Expr,
    pub body: Box<Stmt>,
}

pub trait StmtVisitor<T> {
    fn visit_block_stmt(&mut self, stmt: &BlockStmt) -> Result<T, LoxError>;
    fn visit_expression_stmt(&mut self, stmt: &ExpressionStmt) -> Result<T, LoxError>;
    fn visit_if_stmt(&mut self, stmt: &IfStmt) -> Result<T, LoxError>;
    fn visit_print_stmt(&mut self, stmt: &PrintStmt) -> Result<T, LoxError>;
    fn visit_var_stmt(&mut self, stmt: &VarStmt) -> Result<T, LoxError>;
    fn visit_while_stmt(&mut self, stmt: &WhileStmt) -> Result<T, LoxError>;
}

impl BlockStmt {
    pub fn accept<T>(&self, visitor: &mut dyn StmtVisitor<T>) -> Result<T, LoxError> {
        visitor.visit_block_stmt(self)
    }
}

impl ExpressionStmt {
    pub fn accept<T>(&self, visitor: &mut dyn StmtVisitor<T>) -> Result<T, LoxError> {
        visitor.visit_expression_stmt(self)
    }
}

impl IfStmt {
    pub fn accept<T>(&self, visitor: &mut dyn StmtVisitor<T>) -> Result<T, LoxError> {
        visitor.visit_if_stmt(self)
    }
}

impl PrintStmt {
    pub fn accept<T>(&self, visitor: &mut dyn StmtVisitor<T>) -> Result<T, LoxError> {
        visitor.visit_print_stmt(self)
    }
}

impl VarStmt {
    pub fn accept<T>(&self, visitor: &mut dyn StmtVisitor<T>) -> Result<T, LoxError> {
        visitor.visit_var_stmt(self)
    }
}

impl WhileStmt {
    pub fn accept<T>(&self, visitor: &mut dyn StmtVisitor<T>) -> Result<T, LoxError> {
        visitor.visit_while_stmt(self)
    }
}

