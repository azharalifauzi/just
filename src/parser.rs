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

    pub fn parse(&mut self) -> Result<Vec<Expression>, String> {
        let mut expressions = Vec::new();

        while !self.is_at_end() {
            expressions.push(self.expression()?);
            self.advance();
        }

        Ok(expressions)
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
            let operator = self.peek().lexeme.clone();
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
            let operator = self.peek().lexeme.clone();
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
            let operator = self.peek().lexeme.clone();
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
            let operator = self.peek().lexeme.clone();
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
            TokenType::SemiColon => {
                self.advance();
                Ok(self.expression()?)
            }
            TokenType::Eof => Ok(Expression::Literal(Literal::Null)),
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
}
