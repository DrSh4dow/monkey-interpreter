use monkey_interpreter::token::*;

fn main() {
    println!("Hello, world!");
    let _ = Token {
        token_type: TokenType::LET,
        literal: String::from("let"),
    };
}
