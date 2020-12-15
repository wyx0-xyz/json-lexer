mod lexer;
mod tokens;

fn main() {
    let examples_code: String = String::from(
        r#"
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
    "#,
    );
    let lex = lexer::Lexer {
        code: examples_code,
    };
    let tokens = lex.tokenize();

    println!("{:?}", tokens);
}
