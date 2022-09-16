extern crate lazy_static;

use parser::Parser;
use ssir::SSir;
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
    func test {
        var x: i16 = 41023;
        if x == true {
            x = 124;
        }
    }

    func main {
        var z: i16 = 10 + 50
        var x: i16 = 10 + z + 50
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
    ssir.export();
}
