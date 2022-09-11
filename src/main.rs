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
        x := 10 + 5
        z := x + 50 - 1
        z = 4210

        if z == 4210 {
            z = 412983
            if z == 412983 {
                z = 532
            }
        } else {
            x = 230
        }
    }
    "
    .to_string();
    let tokenizer = Tokenizer::new(&source);
    let mut parser = Parser::new(tokenizer, &source);
    parser.parse();

    let mut ssir = SSir::new();
    ssir.generate(&parser.declarations);
    ssir.export();
}
