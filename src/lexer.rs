#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    // Literals
    Number(f64),
    String(String),
    Boolean(bool),
    Null,
    Identifier(String),

    // Keywords
    Let,
    Const,
    Var,
    Function,
    Return,
    If,
    Else,
    For,
    While,
    Break,
    Continue,
    Throw,
    Typeof,

    // Operators
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Power,
    Equal,
    EqualEqual,
    BangEqual,
    Bang,
    And,
    Or,
    Question,
    Lesser,
    LesserEqual,
    Greater,
    GreaterEqual,

    // Punctuation
    LParen,   // (
    RParen,   // )
    LBrace,   // {
    RBrace,   // }
    LBracket, // [
    RBracket, // ]
    Comma,
    Dot,
    SemiColon,
    Colon,

    // Comments
    LineComment(String),

    Eof, // End of file/input
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub ttype: TokenType,
    pub lexeme: String,
    pub line: usize,
    pub col: usize,
    pub start_pos: usize,
    pub end_pos: usize,
}

pub struct Lexer {
    source: String,
    tokens: Vec<Token>,
    line: usize,
    current: usize,
    col: usize,
    start: usize,
}

impl Lexer {
    pub fn new(source: String) -> Self {
        Self {
            source,
            tokens: vec![],
            line: 1,
            current: 0,
            start: 0,
            col: 1,
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            let c = self.peek().unwrap();
            self.advance();

            match c {
                '(' => self.add_token(TokenType::LParen),
                ')' => self.add_token(TokenType::RParen),
                '{' => self.add_token(TokenType::LBrace),
                '}' => self.add_token(TokenType::RBrace),
                '[' => self.add_token(TokenType::LBracket),
                ']' => self.add_token(TokenType::RBracket),
                '.' => self.add_token(TokenType::Dot),
                ',' => self.add_token(TokenType::Comma),
                ';' => self.add_token(TokenType::SemiColon),
                ':' => self.add_token(TokenType::Colon),
                '+' => self.add_token(TokenType::Plus),
                '-' => self.add_token(TokenType::Minus),
                '*' => {
                    match self.peek() {
                        Some('*') => {
                            self.advance();
                            self.add_token(TokenType::Power);
                        }
                        Some(_) => self.add_token(TokenType::Star),
                        None => {}
                    };
                }
                '/' => {
                    match self.peek() {
                        Some('/') => self.comment(),
                        Some(_) => {
                            self.add_token(TokenType::Slash);
                        }
                        None => {}
                    };
                }
                '%' => self.add_token(TokenType::Percent),
                '=' => match self.peek() {
                    Some('=') => {
                        self.advance();
                        self.add_token(TokenType::EqualEqual);
                    }
                    Some(_) => self.add_token(TokenType::Equal),
                    None => {}
                },
                '!' => match self.peek() {
                    Some('=') => {
                        self.advance();
                        self.add_token(TokenType::BangEqual);
                    }
                    Some(_) => {
                        self.add_token(TokenType::Bang);
                    }
                    None => {}
                },
                '&' => match self.peek() {
                    Some('&') => {
                        self.advance();
                        self.add_token(TokenType::And);
                    }
                    Some(_) => {}
                    None => {}
                },
                '|' => match self.peek() {
                    Some('|') => {
                        self.advance();
                        self.add_token(TokenType::Or);
                    }
                    Some(_) => {}
                    None => {}
                },
                '?' => self.add_token(TokenType::Question),
                '<' => match self.peek() {
                    Some('=') => {
                        self.advance();
                        self.add_token(TokenType::LesserEqual);
                    }
                    Some(_) => {
                        self.add_token(TokenType::Lesser);
                    }
                    None => {}
                },
                '>' => match self.peek() {
                    Some('=') => {
                        self.advance();
                        self.add_token(TokenType::GreaterEqual);
                    }
                    Some(_) => {
                        self.add_token(TokenType::Greater);
                    }
                    None => {}
                },

                // Whitespace and newlines
                ' ' | '\r' | '\t' => {
                    // Ignore whitespace
                }
                '\n' => {
                    self.advance_line();
                }

                // Numbers
                '0'..='9' => self.number(),
                // String literals
                '"' => self.string(),
                // Identifiers and keywords
                'a'..='z' | 'A'..='Z' | '_' => self.identifier(),

                // Unknown character
                _ => {
                    panic!(
                        "Unexpected character: {} at line {} at col {}",
                        c, self.line, self.col
                    );
                }
            }

            self.col += 1;
        }

        self.add_token(TokenType::Eof);

        self.tokens.clone()
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn add_token(&mut self, ttype: TokenType) {
        let lexeme = self.source[self.start..self.current].to_string();
        self.tokens.push(Token {
            ttype,
            lexeme,
            col: self.col,
            line: self.line,
            start_pos: self.start,
            end_pos: self.current,
        })
    }

    fn peek(&self) -> Option<char> {
        self.source.chars().nth(self.current)
    }

    fn advance(&mut self) {
        self.current += 1;
    }

    fn advance_line(&mut self) {
        self.line += 1;
        self.col = 1;
    }

    fn number(&mut self) {
        while let Some(c) = self.peek() {
            if !c.is_ascii_digit() && c != '.' {
                break;
            }

            self.advance();
        }

        let value = self.source[self.start..self.current].parse::<f64>();

        match value {
            Ok(v) => {
                self.add_token(TokenType::Number(v));
            }
            Err(_) => {
                panic!(
                    "Failed to parse number {}",
                    self.source[self.start..self.current].to_string()
                );
            }
        }
    }

    fn string(&mut self) {
        while let Some(c) = self.peek() {
            if c == '"' || self.is_at_end() {
                break;
            }

            self.advance();
        }

        if self.is_at_end() {
            panic!("Unterminated string")
        }

        // Consume closing "
        self.advance();
        let value = self.source[self.start + 1..self.current - 1].to_string();
        self.add_token(TokenType::String(value));
    }

    fn identifier(&mut self) {
        while let Some(c) = self.peek() {
            if !c.is_alphanumeric() && c != '_' {
                break;
            }

            self.advance();
        }

        let text = &self.source[self.start..self.current];

        match text {
            "function" => self.add_token(TokenType::Function),
            "let" => self.add_token(TokenType::Let),
            "const" => self.add_token(TokenType::Const),
            "var" => self.add_token(TokenType::Var),
            "return" => self.add_token(TokenType::Return),
            "if" => self.add_token(TokenType::If),
            "else" => self.add_token(TokenType::Else),
            "for" => self.add_token(TokenType::For),
            "while" => self.add_token(TokenType::While),
            "break" => self.add_token(TokenType::Break),
            "continue" => self.add_token(TokenType::Continue),
            "throw" => self.add_token(TokenType::Throw),
            "typeof" => self.add_token(TokenType::Typeof),
            "null" => self.add_token(TokenType::Null),
            "true" => self.add_token(TokenType::Boolean(true)),
            "false" => self.add_token(TokenType::Boolean(false)),
            _ => self.add_token(TokenType::Identifier(text.to_string())),
        }
    }

    fn comment(&mut self) {
        while let Some(c) = self.peek() {
            if c == '\n' {
                break;
            }

            self.advance();
        }

        let value = self.source[self.start..self.current].to_string();
        self.add_token(TokenType::LineComment(value));
    }
}
