use crate::{
    ast::{Expression, Literal, Statement},
    environment::{Environment, FunctionExpression, Value},
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

                    last_value = Some(value);
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
            Statement::VariableDeclaration {
                name,
                initializer,
                can_reassign,
            } => match initializer {
                Some(expr) => {
                    let value = self.evaluate(expr)?;
                    self.environment.define(name.to_string(), value);
                    Ok(())
                }
                None => {
                    if *can_reassign {
                        self.environment.define(name.to_string(), Value::Null);
                        Ok(())
                    } else {
                        Err("const missing initializer".to_string())
                    }
                }
            },
            Statement::FunctionDeclaration {
                name,
                parameters,
                body,
            } => {
                self.environment.define(
                    name.to_string(),
                    Value::Function(Box::new(FunctionExpression::new(
                        parameters.clone(),
                        body.to_vec(),
                    ))),
                );
                Ok(())
            }
            Statement::Block(statements) => {
                self.enter_scope();
                for statement in statements {
                    self.execute(statement)?
                }
                self.exit_scope();
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
            Expression::Variable(name) => match self.environment.get(name) {
                Some(v) => Ok(v),
                None => Ok(Value::Null),
            },
            Expression::Call { callee, arguments } => {
                if let Value::Function(func_expr) = self.evaluate(&callee)? {
                    let expr = *func_expr;

                    self.enter_scope();

                    for (index, arg) in arguments.iter().enumerate() {
                        let value = self.evaluate(arg)?;
                        if index <= arguments.len() {
                            let var_name = &expr.parameters[index];
                            self.environment.define(var_name.to_string(), value);
                        } else {
                            break;
                        }
                    }

                    let mut returned_value = Value::Null;

                    for statement in expr.body {
                        match statement {
                            Statement::Return(v) => {
                                if let Some(expr) = v {
                                    returned_value = self.evaluate(&expr)?;
                                }
                            }
                            _ => self.execute(&statement)?,
                        }
                    }

                    self.exit_scope();

                    Ok(returned_value)
                } else {
                    let var_name = self.evaluate(&callee)?;
                    Err(format!("{} is not a function", var_name))
                }
            }
            _ => Err("Expression is not implemented yet".to_string()),
        }
    }

    fn enter_scope(&mut self) {
        let new_env = Environment::with_parent(self.environment.clone());
        self.environment = new_env;
    }

    fn exit_scope(&mut self) {
        if let Some(parent) = self.environment.parent.take() {
            self.environment = *parent;
        } else {
            panic!("Cannot exit from the global scope")
        }
    }
}

fn is_truthy(value: &Value) -> bool {
    match value {
        Value::Number(v) => *v > 0.0,
        Value::String(v) => v.clone().len() > 0,
        Value::Boolean(b) => *b,
        Value::Null => false,
        Value::Function(_) => true,
    }
}
