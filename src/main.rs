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
    func main {
        {
            apple_count := 21
        }
        apple_count = apple_count + 1
    }
    "
    .to_string();
    let tokenizer = Tokenizer::new(&source);
    let mut parser = Parser::new(tokenizer, &source);
    parser.parse();

    let mut checker = TypeContainer::new();
    for decl in &parser.declarations {
        checker.check(decl);
    }
}
