use interpreter::Interpreter;
use lexer::Lexer;
use parser::Parser;

mod ast;
mod environment;
mod interpreter;
mod lexer;
mod parser;

fn main() {
    let source = r#"
    (1 + 2) ** (3 - 1);
    "#
    .to_string();
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize();
    let mut parser = Parser::new(tokens);
    let ast = parser.parse().unwrap();
    let mut interpreter = Interpreter::new();

    match interpreter.interpret(ast) {
        Ok(v) => match v {
            Some(v) => println!("{}", v),
            None => {}
        },
        Err(err) => {
            eprintln!("Error: {}", err);
        }
    }
}
