use lexer::Lexer;
use parser::Parser;

mod ast;
mod lexer;
mod parser;

fn main() {
    let source = r#"
    -5 * -(2 * 5); 1 + 2;
    "#
    .to_string();
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize();
    let mut parser = Parser::new(tokens);
    let ast = parser.parse().unwrap();

    println!("Token: {:#?}", ast);
}
