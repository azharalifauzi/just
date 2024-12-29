use crate::ast::*;
use crate::lexer::{Token, TokenType};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<Vec<Statement>, String> {
        let mut statements = Vec::new();

        while !self.is_at_end() {
            statements.push(self.statement()?);
            self.advance();
        }

        Ok(statements)
    }

    fn statement(&mut self) -> Result<Statement, String> {
        match self.peek().ttype {
            TokenType::Let | TokenType::Var => self.variable_declaration(true),
            TokenType::Const => self.variable_declaration(false),
            TokenType::LBrace => self.block(),
            TokenType::Function => self.function_declaration(),
            TokenType::Return => {
                self.advance();
                match self.peek().ttype {
                    TokenType::SemiColon => Ok(Statement::Return(None)),
                    TokenType::RBrace => {
                        // Allow function to return none
                        // function add() { return }
                        self.current -= 1;
                        Ok(Statement::Return(None))
                    }
                    _ => Ok(Statement::Return(Some(self.expression()?))),
                }
            }
            TokenType::SemiColon => Ok(Statement::Expression(Expression::Literal(Literal::Void))),
            _ => Ok(Statement::Expression(self.expression()?)),
        }
    }

    fn variable_declaration(&mut self, can_reassign: bool) -> Result<Statement, String> {
        self.advance();
        let token = self.peek();
        let name = match &token.ttype {
            TokenType::Identifier(v) => v.to_string(),
            _ => {
                let err_message = format!("Invalid variable name at {}:{}", token.line, token.col);
                return Err(err_message);
            }
        };

        self.advance();

        let initializer = if self.peek().ttype == TokenType::Equal {
            self.advance();
            Some(self.expression()?)
        } else {
            None
        };

        Ok(Statement::VariableDeclaration {
            name,
            initializer,
            can_reassign,
        })
    }

    fn block(&mut self) -> Result<Statement, String> {
        self.check(TokenType::LBrace, "{")?;
        self.advance();
        let mut statements = Vec::new();

        loop {
            let current_token = self.peek();
            if self.is_at_end() {
                let err = format!(
                    "You must close function block with '}}' at {}:{}",
                    current_token.line, current_token.col
                );
                return Err(err);
            }

            statements.push(self.statement()?);

            if self.peek_next().ttype == TokenType::RBrace {
                self.advance();
                break;
            }

            self.advance();
        }

        Ok(Statement::Block(statements))
    }

    fn function_declaration(&mut self) -> Result<Statement, String> {
        self.advance();

        let token = self.peek();
        let name = match &token.ttype {
            TokenType::Identifier(v) => v.to_string(),
            _ => {
                let err_message = format!("Invalid variable name at {}:{}", token.line, token.col);
                return Err(err_message);
            }
        };

        let mut params = Vec::new();

        self.advance();
        self.check(TokenType::LParen, "(")?;
        self.advance();

        loop {
            match &self.peek().ttype {
                TokenType::Identifier(v) => params.push(v.to_string()),
                TokenType::RParen => {
                    self.advance();
                    break;
                }
                TokenType::Comma => {}
                _ => {
                    let token = self.peek();
                    let err_message = format!(
                        "Syntax Error expected ), but get {} at {}:{}",
                        token.lexeme, token.line, token.col
                    );
                    return Err(err_message);
                }
            }

            self.advance();
        }

        self.check(TokenType::LBrace, "{")?;

        match self.block() {
            Ok(v) => {
                if let Statement::Block(body) = v {
                    Ok(Statement::FunctionDeclaration {
                        name,
                        parameters: params,
                        body,
                    })
                } else {
                    Err("Block is not Vec<Statement>".to_string())
                }
            }
            Err(e) => Err(e),
        }
    }

    fn expression(&mut self) -> Result<Expression, String> {
        let expression = self.equality()?;

        Ok(expression)
    }

    fn equality(&mut self) -> Result<Expression, String> {
        let mut expression = self.comparison()?;
        let mut next_ttype = &self.peek_next().ttype;

        while *next_ttype == TokenType::EqualEqual || *next_ttype == TokenType::BangEqual {
            self.advance();
            let operator = self.peek().ttype.clone();
            self.advance();
            let right = self.expression()?;

            expression = Expression::Binary {
                left: Box::new(expression),
                operator,
                right: Box::new(right),
            };

            next_ttype = &self.peek().ttype;
        }

        Ok(expression)
    }

    fn comparison(&mut self) -> Result<Expression, String> {
        let mut expression = self.term()?;
        let mut next_ttype = &self.peek_next().ttype;

        while *next_ttype == TokenType::Greater
            || *next_ttype == TokenType::GreaterEqual
            || *next_ttype == TokenType::Lesser
            || *next_ttype == TokenType::LesserEqual
        {
            self.advance();
            let operator = self.peek().ttype.clone();
            self.advance();
            let right = self.expression()?;

            expression = Expression::Binary {
                left: Box::new(expression),
                operator,
                right: Box::new(right),
            };

            next_ttype = &self.peek().ttype;
        }

        Ok(expression)
    }

    fn term(&mut self) -> Result<Expression, String> {
        let mut expression = self.factor()?;
        let mut next_ttype = &self.peek_next().ttype;

        while *next_ttype == TokenType::Plus || *next_ttype == TokenType::Minus {
            self.advance();
            let operator = self.peek().ttype.clone();
            self.advance();
            let right = self.expression()?;

            expression = Expression::Binary {
                left: Box::new(expression),
                operator,
                right: Box::new(right),
            };

            next_ttype = &self.peek().ttype;
        }

        Ok(expression)
    }

    fn factor(&mut self) -> Result<Expression, String> {
        let mut expression = self.unary()?;
        let mut next_ttype = &self.peek_next().ttype;

        while *next_ttype == TokenType::Slash
            || *next_ttype == TokenType::Star
            || *next_ttype == TokenType::Power
            || *next_ttype == TokenType::Percent
        {
            self.advance();
            let operator = self.peek().ttype.clone();
            self.advance();
            let right = self.expression()?;

            expression = Expression::Binary {
                left: Box::new(expression),
                operator,
                right: Box::new(right),
            };

            next_ttype = &self.peek().ttype;
        }

        Ok(expression)
    }

    fn unary(&mut self) -> Result<Expression, String> {
        let token = self.peek();

        let result = match &token.ttype {
            TokenType::Minus => {
                self.advance();
                Ok(Expression::Unary {
                    operator: "-".to_string(),
                    right: Box::new(self.expression()?),
                })
            }
            TokenType::Bang => {
                self.advance();
                Ok(Expression::Unary {
                    operator: '!'.to_string(),
                    right: Box::new(self.expression()?),
                })
            }
            _ => self.primary(),
        };

        result
    }

    fn primary(&mut self) -> Result<Expression, String> {
        let token = self.peek().clone();

        let result = match &token.ttype {
            TokenType::Number(v) => Ok(Expression::Literal(Literal::Number(*v))),
            TokenType::String(v) => Ok(Expression::Literal(Literal::String(v.clone()))),
            TokenType::Null => Ok(Expression::Literal(Literal::Null)),
            TokenType::Boolean(v) => Ok(Expression::Literal(Literal::Boolean(*v))),
            TokenType::LParen => {
                self.advance();
                let expression = self.expression()?;

                if self.peek_next().ttype == TokenType::RParen {
                    self.advance();
                    Ok(Expression::Grouping(Box::new(expression)))
                } else {
                    let token = self.peek_next();
                    let err = format!(
                        "Unexpected token, Expected ')' but get {}, at {}:{}",
                        token.lexeme, token.line, token.col
                    );
                    Err(err)
                }
            }
            TokenType::Identifier(v) => match self.peek_next().ttype {
                TokenType::Equal => {
                    self.advance();
                    self.advance();

                    Ok(Expression::Assignment {
                        name: v.to_string(),
                        value: Box::new(self.expression()?),
                    })
                }
                TokenType::LParen => {
                    self.advance();
                    let mut args = Vec::new();

                    while self.peek_next().ttype != TokenType::RParen {
                        self.advance();
                        let current_token = self.peek();
                        if current_token.ttype == TokenType::Comma {
                            self.advance();
                        } else if self.is_at_end() {
                            let err = format!(
                                "You must close function call with ')' at {}:{}",
                                current_token.line, current_token.col
                            );
                            return Err(err);
                        }

                        args.push(self.expression()?);
                    }

                    self.advance();

                    Ok(Expression::Call {
                        callee: Box::new(Expression::Variable(v.to_string())),
                        arguments: args,
                    })
                }
                _ => Ok(Expression::Variable(v.to_string())),
            },
            TokenType::Eof => Ok(Expression::Literal(Literal::Void)),
            _ => {
                let err = format!(
                    "Unexpected token {:?}, at {}:{}",
                    token.lexeme, token.line, token.col
                );
                Err(err)
            }
        };

        result
    }

    fn is_at_end(&self) -> bool {
        matches!(self.peek().ttype, TokenType::Eof)
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn peek_next(&self) -> &Token {
        if self.is_at_end() {
            return &self.tokens[self.current];
        }

        &self.tokens[self.current + 1]
    }

    fn advance(&mut self) {
        if !self.is_at_end() {
            self.current += 1;
        }
    }

    fn check(&self, expected: TokenType, expected_lexeme: &str) -> Result<(), String> {
        let token = self.peek();

        if token.ttype != expected {
            let err_message = format!(
                "Unexpected token, expected {}, but get {} at {}:{}",
                expected_lexeme, token.lexeme, token.line, token.col
            );

            return Err(err_message);
        }

        Ok(())
    }
}
