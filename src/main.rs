mod lexer;
mod tokens;

fn main() {
    let example_code: &str = r#"
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
    let lex = lexer::Lexer { code: example_code };
    let tokens = lex.tokenize();

    println!("{:?}", tokens);
}
