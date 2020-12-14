#[derive(Debug)]
pub enum Token {
     LBrace,         // {
     RBrace,         // }
     LBracket,       // [
     RBracket,       // ]
     Comma,          // ,
     Colon,          // :
     String(String), // "string"
     Number(isize),  // 32
     Boolean(bool),  // true
     Null,           // null
     Unknow,
}
