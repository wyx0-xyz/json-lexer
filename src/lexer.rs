use crate::tokens::Token;

pub enum State {
     InString,
     InNumber,
     InKeyword,
     None,
}

pub fn tokenize(code: &str) -> Vec<Token> {
     let mut tokens: Vec<Token> = vec![];
     let mut state: State = State::None;
     let mut current_token: String = String::new();

     for c in code.chars() {
          match state {
               State::InString => {
                    if c != '"' {
                         current_token.push(c);
                         continue;
                    } else {
                         tokens.push(Token::String(current_token.clone()));
                         current_token.clear();
                         state = State::None;
                         continue;
                    }
               }
               State::InNumber => {
                    if c.is_ascii_digit() {
                         current_token.push(c);
                         continue;
                    } else {
                         tokens.push(Token::Number(current_token.clone().parse().unwrap()));
                         current_token.clear();
                         state = State::None;
                         continue;
                    }
               }
               State::InKeyword => {
                    if c.is_ascii_alphabetic() {
                         current_token.push(c);
                         continue;
                    } else {
                         let keyword: Token = match current_token.as_str() {
                              "true" => Token::Boolean(true),
                              "false" => Token::Boolean(false),
                              "null" => Token::Null,
                              _ => Token::Unknow,
                         };

                         tokens.push(keyword);
                         current_token.clear();
                         state = State::None;
                         continue;
                    }
               }
               State::None => (),
          }

          let token: Token = match c {
               ' ' | '\t' | '\n' | '\r' => continue,
               '"' => {
                    state = State::InString;
                    continue;
               }
               c if c.is_ascii_digit() => {
                    state = State::InNumber;
                    current_token.push(c);
                    continue;
               }
               c if c.is_ascii_alphabetic() => {
                    state = State::InKeyword;
                    current_token.push(c);
                    continue;
               }
               '{' => Token::LBrace,
               '}' => Token::RBrace,
               '[' => Token::LBracket,
               ']' => Token::RBracket,
               ':' => Token::Colon,
               ',' => Token::Comma,
               _ => Token::Unknow,
          };

          tokens.push(token);
     }

     tokens
}
