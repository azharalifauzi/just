#[derive(Debug)]
pub enum Expression {
    Literal(Literal),
    Unary {
        operator: String,
        right: Box<Expression>,
    },
    Binary {
        left: Box<Expression>,
        operator: String,
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
    Function {
        parameters: Vec<String>,
        body: Box<Statement>,
    },
}

#[derive(Debug)]
pub enum Statement {
    ExpressionStmt(Expression), // An expression used as a statement
    VariableDeclaration {
        name: String,
        initializer: Option<Expression>,
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

#[derive(Debug)]
pub enum Literal {
    Number(f64),
    String(String),
    Boolean(bool),
    Null,
}
