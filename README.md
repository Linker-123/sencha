## Sencha

A work in progress compiled language written in rust 🦞
The parser and tokenizer part of this language are based on the http://craftinginterpreters.com book. The parser is an implemention of the book's Java descent parser. And the tokenizer is the implemention of the book's C scanner.

## Road Map
- Automatic casting of variable values to the variable's type
#### Example 1:
`var x: i16 = 50`
50 is considered an i32 but the variable's type is an i16 hence the 50 should be converted to an i16

### Example 2:
```
var x: i16 = 10
var y: i16 = 50 + x + 10
```

`x`'s value (10) is considered an i32 but the type of the value is i16, 10 must be converted to type of i16
`50 + x + 10` 50 is considered to be an i32 and 10 is considered to be an i32, x must be converted to type of i32 and then the result must be converted to i16 because `y`'s type is i16
