mod lexer;
mod tokens;

const EXAMPLE_CODE: &str = r#"
{
    "abc": 15,
    "foo": {
        "def": [
            12,
            true,
            null
        ]
    }
}
"#;

fn main() {
    let lex = lexer::Lexer { code: EXAMPLE_CODE };
    let tokens = lex.tokenize();

    println!("{:?}", tokens);
}
