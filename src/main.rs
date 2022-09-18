extern crate lazy_static;

use parser::Parser;
use ssir::{print_functions, transform::RegisterLabeler, SSir};
use tokenizer::Tokenizer;
use typechecker::TypeCheck;

mod ast;
mod error;
mod parser;
mod reg;
mod ssir;
mod tokenizer;
mod typechecker;

fn main() {
    env_logger::init();

    let source = "
    func main {
        x := 5 + 5 + 10 - 1
    }
    "
    .to_string();
    let tokenizer = Tokenizer::new(&source);
    let mut parser = Parser::new(tokenizer, &source);
    parser.parse();

    let mut typecheck = TypeCheck::new();
    for decl in &mut parser.declarations {
        typecheck.check(decl);
    }

    let mut ssir = SSir::new();
    ssir.generate(&parser.declarations);

    drop(parser);

    let mut labeler = RegisterLabeler::new();
    let functions = labeler.assign_labels(ssir.get_functions());
    print_functions(&functions);
}
