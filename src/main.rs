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
    function pow(a, b) {
        return a ** b
    }

    function add(a, b) {
        return a + b
    }
        
    add(pow(2, 3), 2);
    "#
    .to_string();
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize();
    let mut parser = Parser::new(tokens);
    let ast = parser.parse().unwrap();
    let mut interpreter = Interpreter::new();

    // println!("{:#?}", ast);

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
