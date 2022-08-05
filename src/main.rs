extern crate lazy_static;

use parser::Parser;
use tokenizer::Tokenizer;
use crate::typecheck::TypeContainer;

mod asm;
mod ast;
mod compile;
mod parser;
mod tokenizer;
mod typecheck;

fn main() {
    let source = "
    func main(a: i32, b: i32, c: i32) -> i32 {
        message1 := \"Abra Kadabra\"
        message2 := \"Hello, World!\"
        var apple_count: i8 = -5
        var x: str = \"test 123\"
    }
    "
    .to_string();
    let tokenizer = Tokenizer::new(&source);
    let mut parser = Parser::new(tokenizer, &source);
    parser.parse();

    let mut checker = TypeContainer::new();
    for decl in &mut parser.declarations {
        checker.check(decl);
    }
    println!("declarations: {:#?}", parser.declarations);
}
