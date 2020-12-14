mod lexer;
mod tokens;

use std::fs::read_to_string;

fn main() {
    let code = read_to_string("example.json").expect("Cannot read file");
    let tokens = lexer::tokenize(&code);

    println!("{:?}", tokens);
}
