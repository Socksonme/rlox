use std::cell::RefCell;
use std::rc::Rc;

use crate::environment::Environment;
use crate::error::LoxError;
use crate::expr::*;
use crate::lit::*;
use crate::stmt::*;
use crate::token_type::TokenType;

pub struct Interpreter {
    environment: Rc<RefCell<Environment>>,
}

impl Default for Interpreter {
    fn default() -> Self {
        Self::new()
    }
}

impl StmtVisitor<()> for Interpreter {
    fn visit_while_stmt(&mut self, stmt: &WhileStmt) -> Result<(), LoxError> {
        while self.evaluate(&stmt.condition)?.is_truthy() {
            self.execute(&stmt.body)?;
        }
        Ok(())
    }

    fn visit_if_stmt(&mut self, stmt: &IfStmt) -> Result<(), LoxError> {
        if self.evaluate(&stmt.condition)?.is_truthy() {
            self.execute(&stmt.then_branch)?;
        } else if let Some(else_branch) = &stmt.else_branch {
            self.execute(else_branch)?;
        }
        Ok(())
    }

    fn visit_block_stmt(&mut self, stmt: &BlockStmt) -> Result<(), LoxError> {
        // Otheriwse you borrow non-mutably then mutably
        let env = Environment::new_with_enclosing(self.environment.clone());
        self.execute_block(&stmt.statements, env)
    }
    fn visit_expression_stmt(&mut self, stmt: &ExpressionStmt) -> Result<(), LoxError> {
        self.evaluate(&stmt.expression)?;
        Ok(())
    }
    fn visit_print_stmt(&mut self, stmt: &PrintStmt) -> Result<(), LoxError> {
        let value = self.evaluate(&stmt.expression)?;
        println!("{}", value);
        Ok(())
    }
    fn visit_var_stmt(&mut self, stmt: &VarStmt) -> Result<(), LoxError> {
        let value = if let Some(init) = &stmt.initializer {
            Some(self.evaluate(init)?)
        } else {
            None
        };

        self.environment
            .borrow_mut()
            .define(&stmt.name.lexeme, value.unwrap_or(Lit::Nil));
        Ok(())
    }
}

impl ExprVisitor<Lit> for Interpreter {
    fn visit_logical_expr(&mut self, expr: &LogicalExpr) -> Result<Lit, LoxError> {
        let left = self.evaluate(&expr.left)?;

        if expr.operator.ttype == TokenType::Or {
            if left.is_truthy() {
                return Ok(left);
            }
        } else if left.is_truthy() {
            return Ok(left);
        }

        self.evaluate(&expr.right)
    }

    fn visit_binary_expr(&mut self, expr: &BinaryExpr) -> Result<Lit, LoxError> {
        let left = self.evaluate(&expr.left)?;
        let right = self.evaluate(&expr.right)?;

        // This still doesnt work with EqualEqual, kek
        match expr.operator.ttype {
            TokenType::Plus => match (left, right) {
                (Lit::Num(left), Lit::Num(right)) => Ok(Lit::Num(left + right)),
                (Lit::Str(left), Lit::Str(right)) => Ok(Lit::Str(format!("{}{}", left, right))),
                _ => Err(LoxError::runtime_error(
                    expr.operator.clone(),
                    "Expected two numbers or two strings.",
                )),
            },
            TokenType::Minus => match (left, right) {
                (Lit::Num(left), Lit::Num(right)) => Ok(Lit::Num(left - right)),
                _ => Err(LoxError::runtime_error(
                    expr.operator.clone(),
                    "Expected two numbers.",
                )),
            },
            TokenType::Star => match (left, right) {
                (Lit::Num(left), Lit::Num(right)) => Ok(Lit::Num(left * right)),
                _ => Err(LoxError::runtime_error(
                    expr.operator.clone(),
                    "Expected two numbers.",
                )),
            },
            TokenType::Slash => match (left, right) {
                (Lit::Num(left), Lit::Num(right)) => Ok(Lit::Num(left / right)),
                _ => Err(LoxError::runtime_error(
                    expr.operator.clone(),
                    "Expected two numbers.",
                )),
            },
            TokenType::Greater => match (left, right) {
                (Lit::Num(left), Lit::Num(right)) => Ok(Lit::Bool(left > right)),
                _ => Err(LoxError::runtime_error(
                    expr.operator.clone(),
                    "Expected two numbers.",
                )),
            },
            TokenType::GreaterEqual => match (left, right) {
                (Lit::Num(left), Lit::Num(right)) => Ok(Lit::Bool(left >= right)),
                _ => Err(LoxError::runtime_error(
                    expr.operator.clone(),
                    "Expected two numbers.",
                )),
            },
            TokenType::Less => match (left, right) {
                (Lit::Num(left), Lit::Num(right)) => Ok(Lit::Bool(left < right)),
                _ => Err(LoxError::runtime_error(
                    expr.operator.clone(),
                    "Expected two numbers.",
                )),
            },
            TokenType::LessEqual => match (left, right) {
                (Lit::Num(left), Lit::Num(right)) => Ok(Lit::Bool(left <= right)),
                _ => Err(LoxError::runtime_error(
                    expr.operator.clone(),
                    "Expected two numbers.",
                )),
            },
            TokenType::EqualEqual => Ok(Lit::Bool(left == right)),
            TokenType::BangEqual => Ok(Lit::Bool(left != right)),
            _ => Err(LoxError::runtime_error(
                expr.operator.clone(),
                "Illegal expression.",
            )),
        }
    }
    fn visit_unary_expr(&mut self, expr: &UnaryExpr) -> Result<Lit, LoxError> {
        let right = self.evaluate(&expr.right)?;

        match expr.operator.ttype {
            TokenType::Minus => match &right {
                Lit::Num(n) => return Ok(Lit::Num(-n)),
                _ => return Ok(Lit::Nil),
            },
            TokenType::Bang => {
                let val: bool = right.is_truthy();
                return Ok(Lit::Bool(!val));
            }
            _ => {}
        }
        Err(LoxError::error(expr.operator.line, "Unreachable code."))
    }
    fn visit_grouping_expr(&mut self, expr: &GroupingExpr) -> Result<Lit, LoxError> {
        self.evaluate(&expr.expression)
    }
    fn visit_literal_expr(&mut self, expr: &LiteralExpr) -> Result<Lit, LoxError> {
        Ok(expr.value.clone().unwrap())
    }
    fn visit_variable_expr(&mut self, expr: &VariableExpr) -> Result<Lit, LoxError> {
        self.environment.borrow().get(&expr.name)
    }
    fn visit_assign_expr(&mut self, expr: &AssignExpr) -> Result<Lit, LoxError> {
        let value = self.evaluate(&expr.value)?;
        self.environment
            .borrow_mut()
            .assign(&expr.name, value.clone())?;
        Ok(value)
    }
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            environment: Rc::new(RefCell::new(Environment::new())),
        }
    }

    pub fn evaluate(&mut self, expr: &Expr) -> Result<Lit, LoxError> {
        expr.accept(self)
    }

    pub fn execute(&mut self, statement: &Stmt) -> Result<(), LoxError> {
        statement.accept(self)
    }

    fn execute_block(
        &mut self,
        statements: &[Stmt],
        environment: Environment,
    ) -> Result<(), LoxError> {
        // Because we have to actually change the pointer itself, not the value that it's pointing to
        let previous = self.environment.clone();
        self.environment = Rc::new(RefCell::new(environment));
        let result = statements.iter().try_for_each(|s| self.execute(s));
        self.environment = previous;
        result
    }

    /// Returns `true` on success
    pub fn interpret(&mut self, statements: &[Stmt]) -> bool {
        for statement in statements {
            if self.execute(statement).is_err() {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use crate::token::Token;

    use super::*;

    fn make_literal_num_expr(i: f64) -> Box<Expr> {
        Box::new(Expr::Literal(LiteralExpr {
            value: Some(Lit::Num(i)),
        }))
    }

    fn make_literal_str_expr(s: &str) -> Box<Expr> {
        Box::new(Expr::Literal(LiteralExpr {
            value: Some(Lit::Str(s.to_string())),
        }))
    }

    fn make_literal_bool_expr(b: bool) -> Box<Expr> {
        Box::new(Expr::Literal(LiteralExpr {
            value: Some(Lit::Bool(b)),
        }))
    }

    fn make_literal_nil_expr() -> Box<Expr> {
        Box::new(Expr::Literal(LiteralExpr {
            value: Some(Lit::Nil),
        }))
    }

    #[test]
    fn test_unary_minus() {
        let mut interpreter = Interpreter::new();
        let unary_expr = Expr::Unary(UnaryExpr {
            operator: Token::new(TokenType::Minus, "-", None, 0),
            right: make_literal_num_expr(10.0),
        });
        let result = interpreter.evaluate(&unary_expr);
        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(Lit::Num(-10.0)));
    }

    #[test]
    fn test_unary_not() {
        let mut interpreter = Interpreter::new();
        let unary_expr = Expr::Unary(UnaryExpr {
            operator: Token::new(TokenType::Bang, "!", None, 0),
            right: make_literal_bool_expr(false),
        });
        let result = interpreter.evaluate(&unary_expr);
        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(Lit::Bool(true)));
    }

    #[test]
    fn test_binary_sub() {
        let mut interpreter = Interpreter::new();
        let binary_expr = Expr::Binary(BinaryExpr {
            operator: Token::new(TokenType::Minus, "-", None, 0),
            left: make_literal_num_expr(10.0),
            right: make_literal_num_expr(3.0),
        });
        let result = interpreter.evaluate(&binary_expr);
        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(Lit::Num(7.0)));
    }

    #[test]
    fn test_binary_mul() {
        let mut interpreter = Interpreter::new();
        let binary_expr = Expr::Binary(BinaryExpr {
            operator: Token::new(TokenType::Star, "*", None, 0),
            left: make_literal_num_expr(10.0),
            right: make_literal_num_expr(3.0),
        });
        let result = interpreter.evaluate(&binary_expr);
        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(Lit::Num(30.0)));
    }

    #[test]
    fn test_binary_div() {
        let mut interpreter = Interpreter::new();
        let binary_expr = Expr::Binary(BinaryExpr {
            operator: Token::new(TokenType::Slash, "/", None, 0),
            left: make_literal_num_expr(10.0),
            right: make_literal_num_expr(2.0),
        });
        let result = interpreter.evaluate(&binary_expr);
        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(Lit::Num(5.0)));
    }

    #[test]
    fn test_binary_add() {
        let mut interpreter = Interpreter::new();
        let binary_expr = Expr::Binary(BinaryExpr {
            operator: Token::new(TokenType::Plus, "+", None, 0),
            left: make_literal_num_expr(10.0),
            right: make_literal_num_expr(2.0),
        });
        let result = interpreter.evaluate(&binary_expr);
        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(Lit::Num(12.0)));
    }

    #[test]
    fn test_binary_concat() {
        let mut interpreter = Interpreter::new();
        let binary_expr = Expr::Binary(BinaryExpr {
            operator: Token::new(TokenType::Plus, "+", None, 0),
            left: make_literal_str_expr("abcdef"),
            right: make_literal_str_expr("012345"),
        });
        let result = interpreter.evaluate(&binary_expr);
        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(Lit::Str(String::from("abcdef012345"))));
    }

    #[test]
    fn test_error_str_num_binary_concat() {
        let mut interpreter = Interpreter::new();
        let binary_expr = Expr::Binary(BinaryExpr {
            operator: Token::new(TokenType::Plus, "+", None, 0),
            left: make_literal_str_expr("abcdef"),
            right: make_literal_num_expr(123.0),
        });
        let result = interpreter.evaluate(&binary_expr);
        assert!(result.is_err());
    }

    #[test]
    fn test_binary_greater_than() {
        let mut interpreter = Interpreter::new();
        let binary_expr = Expr::Binary(BinaryExpr {
            operator: Token::new(TokenType::Greater, ">", None, 0),
            left: make_literal_num_expr(10.0),
            right: make_literal_num_expr(2.0),
        });
        let result = interpreter.evaluate(&binary_expr);
        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(Lit::Bool(true)));
    }

    #[test]
    fn test_binary_less_than() {
        let mut interpreter = Interpreter::new();
        let binary_expr = Expr::Binary(BinaryExpr {
            operator: Token::new(TokenType::Less, "<", None, 0),
            left: make_literal_num_expr(10.0),
            right: make_literal_num_expr(2.0),
        });
        let result = interpreter.evaluate(&binary_expr);
        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(Lit::Bool(false)));
    }

    #[test]
    fn test_binary_less_than_equal() {
        let mut interpreter = Interpreter::new();
        let binary_expr = Expr::Binary(BinaryExpr {
            operator: Token::new(TokenType::LessEqual, "<=", None, 0),
            left: make_literal_num_expr(10.0),
            right: make_literal_num_expr(10.0),
        });
        let result = interpreter.evaluate(&binary_expr);
        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(Lit::Bool(true)));
    }

    #[test]
    fn test_binary_greater_than_equal() {
        let mut interpreter = Interpreter::new();
        let binary_expr = Expr::Binary(BinaryExpr {
            operator: Token::new(TokenType::GreaterEqual, ">=", None, 0),
            left: make_literal_num_expr(10.0),
            right: make_literal_num_expr(10.0),
        });
        let result = interpreter.evaluate(&binary_expr);
        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(Lit::Bool(true)));
    }

    #[test]
    fn test_err_binary_greater_than() {
        let mut interpreter = Interpreter::new();
        let binary_expr = Expr::Binary(BinaryExpr {
            operator: Token::new(TokenType::GreaterEqual, ">=", None, 0),
            left: make_literal_str_expr("10.0"),
            right: make_literal_num_expr(10.0),
        });
        let result = interpreter.evaluate(&binary_expr);
        assert!(result.is_err());
    }
    #[test]
    fn test_err_binary_greater_than_equal() {
        let mut interpreter = Interpreter::new();
        let binary_expr = Expr::Binary(BinaryExpr {
            operator: Token::new(TokenType::GreaterEqual, ">=", None, 0),
            left: make_literal_str_expr("10.0"),
            right: make_literal_nil_expr(),
        });
        let result = interpreter.evaluate(&binary_expr);
        assert!(result.is_err());
    }
    #[test]
    fn test_binary_equal() {
        let mut interpreter = Interpreter::new();
        let binary_expr = Expr::Binary(BinaryExpr {
            operator: Token::new(TokenType::EqualEqual, "==", None, 0),
            left: make_literal_nil_expr(),
            right: make_literal_nil_expr(),
        });
        let result = interpreter.evaluate(&binary_expr);
        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(Lit::Bool(true)));
    }
}
