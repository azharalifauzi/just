use crate::{
    ast::{Expression, Literal, Statement},
    environment::{Environment, Value},
    lexer::TokenType,
};

pub struct Interpreter {
    environment: Environment,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            environment: Environment::new(),
        }
    }

    pub fn interpret(&mut self, statements: Vec<Statement>) -> Result<Option<Value>, String> {
        let mut last_value = None;

        for statement in statements {
            match statement {
                Statement::Expression(expr) => {
                    let value = self.evaluate(&expr)?;

                    match value {
                        Value::Void => {}
                        _ => last_value = Some(value),
                    }
                }
                _ => {
                    self.execute(&statement)?;
                }
            }
        }

        Ok(last_value)
    }

    fn execute(&mut self, statement: &Statement) -> Result<(), String> {
        match statement {
            Statement::Expression(expr) => {
                self.evaluate(expr)?;
                Ok(())
            }
            _ => Err("Unknown statement".to_string()),
        }
    }

    fn evaluate(&mut self, expr: &Expression) -> Result<Value, String> {
        match expr {
            Expression::Literal(literal) => match literal {
                Literal::Number(n) => Ok(Value::Number(*n)),
                Literal::String(s) => Ok(Value::String(s.clone())),
                Literal::Boolean(b) => Ok(Value::Boolean(*b)),
                Literal::Null => Ok(Value::Null),
                Literal::Void => Ok(Value::Void),
            },
            Expression::Grouping(expr) => self.evaluate(expr),
            Expression::Unary { operator, right } => {
                let value = self.evaluate(right)?;
                if operator == "!" {
                    Ok(Value::Boolean(!is_truthy(&value)))
                } else {
                    let n = match value {
                        Value::Number(v) => v * -1.0,
                        _ => -1.0,
                    };

                    Ok(Value::Number(n))
                }
            }
            Expression::Binary {
                left,
                operator,
                right,
            } => {
                let left = self.evaluate(left)?;
                let right = self.evaluate(right)?;

                match (operator, left, right) {
                    (TokenType::Plus, Value::Number(a), Value::Number(b)) => {
                        Ok(Value::Number(a + b))
                    }
                    (TokenType::Minus, Value::Number(a), Value::Number(b)) => {
                        Ok(Value::Number(a - b))
                    }
                    (TokenType::Star, Value::Number(a), Value::Number(b)) => {
                        Ok(Value::Number(a * b))
                    }
                    (TokenType::Power, Value::Number(a), Value::Number(b)) => {
                        Ok(Value::Number(a.powf(b)))
                    }
                    (TokenType::Percent, Value::Number(a), Value::Number(b)) => {
                        Ok(Value::Number(a % b))
                    }
                    (TokenType::Slash, Value::Number(a), Value::Number(b)) => {
                        if b == 0.0 {
                            Err("Division by zero".to_string())
                        } else {
                            Ok(Value::Number(a / b))
                        }
                    }

                    _ => Err("Invalid binary operation".to_string()),
                }
            }
            _ => Err("Expression is not implemented yet".to_string()),
        }
    }
}

fn is_truthy(value: &Value) -> bool {
    match value {
        Value::Number(v) => *v > 0.0,
        Value::String(v) => v.clone().len() > 0,
        Value::Boolean(b) => *b,
        Value::Null => false,
        Value::Void => false,
    }
}
