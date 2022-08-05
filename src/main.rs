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
        z := 5 || 7
        if z == 5 {
            z = 991
        }
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
