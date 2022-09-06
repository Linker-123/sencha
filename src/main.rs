extern crate lazy_static;

use crate::typecheck::TypeContainer;
use parser::Parser;
use reg::{RegisterManager, RegisterSize};
use tokenizer::Tokenizer;

mod ast;
mod compile;
mod error;
mod parser;
mod reg;
mod tokenizer;
mod typecheck;

fn main() {
    env_logger::init();

    let source = "
    func main {
        x := 1 + 2
    }
    "
    .to_string();
    let tokenizer = Tokenizer::new(&source);
    let mut parser = Parser::new(tokenizer, &source);
    parser.parse();

    let mut checker = TypeContainer::new();
    for decl in &mut parser.declarations {
        checker.check(decl);
        println!("{:#?}", decl);
    }

    let mut registers = RegisterManager::new();
    for _ in 0..8 {
        registers.allocate(RegisterSize::Oword);
    }
    registers.table(Some(RegisterSize::Oword));
}
