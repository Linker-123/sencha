extern crate lazy_static;

use crate::typecheck::TypeContainer;
use code::CodeGen;
use parser::Parser;
use tokenizer::Tokenizer;

mod ast;
mod code;
mod error;
mod parser;
mod reg;
mod tokenizer;
mod typecheck;
mod vartable;
mod vstack;

fn main() {
    env_logger::init();

    let source = "
    func main {
        x := 10.5;
        y := 2.2;
        z := x / y + 10.0 - 12.0;
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

    let mut code_gen = CodeGen::new();
    code_gen.generate(&parser.declarations);
}
