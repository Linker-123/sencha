use tokenizer::Tokenizer;

mod asm;
mod ast;
mod compile;
mod tokenizer;

fn main() {
    let source = "use std.io.*

    mod SomeModule {
        func sum(a: i32, b: i32, c: i32) -> i32 {
            ret a + b + c
        }
    }
    
    // Parens can be ommitted
    func main {
        res := SomeModule.sum(10, 3, 5)
        print(\"$1\", res) // or std.io.print(\"$1\", res)
    }"
    .to_string();
    let tokenizer = Tokenizer::new(&source);
    for tok in tokenizer {
        println!("Token: {:#?}", tok);
    }
}
