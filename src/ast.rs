use crate::lexer::TokenType;

#[derive(Debug, Clone)]
pub enum Expression {
    Literal(Literal),
    Unary {
        operator: String,
        right: Box<Expression>,
    },
    Binary {
        left: Box<Expression>,
        operator: TokenType,
        right: Box<Expression>,
    },
    Grouping(Box<Expression>),
    Variable(String), // Represents variable usage
    Assignment {
        name: String,
        value: Box<Expression>,
    },
    Call {
        callee: Box<Expression>,
        arguments: Vec<Expression>,
    },
    Member {
        object: Box<Expression>,
        property: String,
        computed: bool, // true for `arr[0]`, false for `obj.key`
    },
    ArrayLiteral(Vec<Expression>),
    ObjectLiteral(Vec<(String, Expression)>),
}

#[derive(Debug, Clone)]
pub enum Statement {
    Expression(Expression), // An expression used as a statement
    VariableDeclaration {
        name: String,
        initializer: Option<Expression>,
        can_reassign: bool,
    },
    FunctionDeclaration {
        name: String,
        parameters: Vec<String>,
        body: Vec<Statement>,
    },
    Block(Vec<Statement>), // Represents `{ ... }`
    If {
        condition: Expression,
        then_branch: Box<Statement>,
        else_branch: Option<Box<Statement>>,
    },
    While {
        condition: Expression,
        body: Box<Statement>,
    },
    Return(Option<Expression>), // Supports `return;` and `return expr;`
}

#[derive(Debug, Clone)]
pub enum Literal {
    Number(f64),
    String(String),
    Boolean(bool),
    Null,
    Void,
}
