extern crate lazy_static;

use parser::Parser;
use ssir::SSir;
use tokenizer::Tokenizer;

mod ast;
mod parser;
mod reg;
mod ssir;
mod tokenizer;
mod error;

fn main() {
    env_logger::init();

    let source = "
    func main {
        x := 10 + 5;
        z := x + 50 - 1;
    }
    "
    .to_string();
    let tokenizer = Tokenizer::new(&source);
    let mut parser = Parser::new(tokenizer, &source);
    parser.parse();

    let mut ssir = SSir::new();
    ssir.generate(&parser.declarations);

}
