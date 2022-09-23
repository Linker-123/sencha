extern crate lazy_static;

use cli::config::Config;
use parser::Parser;
use reg::RegisterManager;
use ssir::{print_functions, transform::RegisterLabeler, SSir};
use tokenizer::Tokenizer;
use typechecker::TypeCheck;

mod ast;
mod cli;
mod error;
mod parser;
mod reg;
mod ssir;
mod tokenizer;
mod typechecker;

fn main() {
    env_logger::init();

    let config = Config::new();
    if config.get_bool("rt") {
        let registers = RegisterManager::new();
        registers.table(None);
        return;
    }

    let source = "
    func main {
        z := 111;

        if z == 222 {
            z = 333;
        } else {
            z = 444;
        }
    }
    "
    .to_string();
    let tokenizer = Tokenizer::new(&source);
    let mut parser = Parser::new(tokenizer, &source);
    parser.parse();

    if config.get_bool("pa") {
        println!("{:#?}", parser.declarations);
    }

    let mut typecheck = TypeCheck::new();
    for decl in &mut parser.declarations {
        typecheck.check(decl);
    }

    if config.get_bool("pat") {
        println!("{:#?}", parser.declarations);
    }

    let mut ssir = SSir::new();
    ssir.generate(&mut parser.declarations);

    drop(parser);

    let mut labeler = RegisterLabeler::new();
    let functions = labeler.assign_labels(ssir.get_functions());

    if config.get_bool("ssir") {
        print_functions(&functions);
    }
}
