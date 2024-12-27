use lexer::Lexer;

mod ast;
mod lexer;
mod parser;

fn main() {
    let source = r#"
    if (5 != 12) {};
    "#
    .to_string();
    let mut tokenizer = Lexer::new(source);
    let tokens = tokenizer.tokenize();
    println!("Token: {:#?}", tokens);
}
