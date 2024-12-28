use lexer::Lexer;
use parser::Parser;

mod ast;
mod lexer;
mod parser;

fn main() {
    let source = r#"
    function add(a, b) {
        return a + b
    }

    {
        let a = 1 + 2
        add(a, b)
    }
    "#
    .to_string();
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize();
    let mut parser = Parser::new(tokens);
    let ast = parser.parse().unwrap();

    println!("Token: {:#?}", ast);
}
