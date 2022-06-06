use crate::error::LoxError;
use crate::expr::*;
use crate::lit::*;
use crate::token_type::TokenType;

pub struct Interpreter {}

impl ExprVisitor<Lit> for Interpreter {
    fn visit_binary_expr(&self, expr: &BinaryExpr) -> Result<Lit, LoxError> {
        let left = self.evaluate(&expr.left)?;
        let right = self.evaluate(&expr.right)?;

        // This still doesnt work with EqualEqual, kek
        match expr.operator.ttype {
            TokenType::Plus => {
                match (left, right) {
                    (Lit::Num(left), Lit::Num(right)) => Ok(Lit::Num(left + right)),
                    (Lit::Str(left), Lit::Str(right)) => Ok(Lit::Str(format!("{}{}", left, right))),
                    _ => Err(LoxError::runtime_error(expr.operator.clone(), "Expected two numbers or two strings."))
                }
            }
            TokenType::Minus => {
                match (left, right) {
                    (Lit::Num(left), Lit::Num(right)) => Ok(Lit::Num(left - right)),
                    _ => Err(LoxError::runtime_error(expr.operator.clone(), "Expected two numbers."))
                }
            }
            TokenType::Star => {
                match (left, right) {
                    (Lit::Num(left), Lit::Num(right)) => Ok(Lit::Num(left * right)),
                    _ => Err(LoxError::runtime_error(expr.operator.clone(), "Expected two numbers."))
                }
            }
            TokenType::Slash => {
                match (left, right) {
                    (Lit::Num(left), Lit::Num(right)) => Ok(Lit::Num(left / right)),
                    _ => Err(LoxError::runtime_error(expr.operator.clone(), "Expected two numbers."))
                }
            }
            TokenType::Greater => {
                match (left, right) {
                    (Lit::Num(left), Lit::Num(right)) => Ok(Lit::Bool(left > right)),
                    _ => Err(LoxError::runtime_error(expr.operator.clone(), "Expected two numbers."))
                }
            }
            TokenType::GreaterEqual => {
                match (left, right) {
                    (Lit::Num(left), Lit::Num(right)) => Ok(Lit::Bool(left >= right)),
                    _ => Err(LoxError::runtime_error(expr.operator.clone(), "Expected two numbers."))
                }
            }
            TokenType::Less => {
                match (left, right) {
                    (Lit::Num(left), Lit::Num(right)) => Ok(Lit::Bool(left < right)),
                    _ => Err(LoxError::runtime_error(expr.operator.clone(), "Expected two numbers."))
                }
            }
            TokenType::LessEqual => {
                match (left, right) {
                    (Lit::Num(left), Lit::Num(right)) => Ok(Lit::Bool(left <= right)),
                    _ => Err(LoxError::runtime_error(expr.operator.clone(), "Expected two numbers."))
                }
            }
            TokenType::EqualEqual => {
                Ok(Lit::Bool(left == right))
            }
            TokenType::BangEqual => {
                Ok(Lit::Bool(left != right))
            }
            _ => Err(LoxError::runtime_error(
                expr.operator.clone(),
                "Illegal expression.",
            )),
        }
    }
    fn visit_unary_expr(&self, expr: &UnaryExpr) -> Result<Lit, LoxError> {
        let right = self.evaluate(&expr.right)?;

        match expr.operator.ttype {
            TokenType::Minus => match &right {
                Lit::Num(n) => return Ok(Lit::Num(-n)),
                _ => return Ok(Lit::Nil),
            },
            TokenType::Bang => {
                let val: bool = right.into();
                return Ok(Lit::Bool(!val));
            }
            _ => {}
        }
        Err(LoxError::error(expr.operator.line, "Unreachable code."))
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

    pub fn interpret(&self, expr: &Expr) -> bool {
        match self.evaluate(expr) {
            Ok(v) => {
                println!("{}", v);
                true
            }
            Err(e) => {
                e.report("");
                false
            }
        }
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
        let interpreter = Interpreter {};
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
        let interpreter = Interpreter {};
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
        let interpreter = Interpreter {};
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
        let interpreter = Interpreter {};
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
        let interpreter = Interpreter {};
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
        let interpreter = Interpreter {};
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
        let interpreter = Interpreter {};
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
        let interpreter = Interpreter {};
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
        let interpreter = Interpreter {};
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
        let interpreter = Interpreter {};
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
        let interpreter = Interpreter {};
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
        let interpreter = Interpreter {};
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
        let interpreter = Interpreter {};
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
        let interpreter = Interpreter {};
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
        let interpreter = Interpreter {};
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
