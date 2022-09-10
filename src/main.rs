extern crate lazy_static;

use crate::typecheck::TypeCheck;
use code::CodeGen;
use parser::Parser;
use tokenizer::Tokenizer;

mod asm;
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
        var x: i32[] = {2, 2, 3};
    }
    "
    .to_string();
    let tokenizer = Tokenizer::new(&source);
    let mut parser = Parser::new(tokenizer, &source);
    parser.parse();

    let mut checker = TypeCheck::new();
    for decl in &mut parser.declarations {
        checker.check(decl);
    }

    println!("{:#?}", parser.declarations);

    // let mut code_gen = CodeGen::new();
    // code_gen.generate(&parser.declarations);
    // code_gen.write();
}
