# JSON-lexer

This a json lexer written in Rust.

# Example

Input: 
```json
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
```

Result:
```
[LBrace, String("abc"), Colon, Number(15), Comma, String("foo"), Colon, LBrace, String("def"), Colon, LBracket, Number(12), Comma, Boolean(true), Comma, Null, RBracket, RBrace, RBrace]
```