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
[LBrace, String("abc"), Colon, Number(15), String("foo"), Colon, LBrace, String("def"), Colon, LBracket, Number(12), Boolean(true), Null, RBracket, RBrace, RBrace]
```