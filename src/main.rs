use parser::Parser;
use tokenizer::Tokenizer;

mod asm;
mod ast;
mod compile;
mod parser;
mod tokenizer;

fn main() {
    let source = "
    y = 5+1
    
    for x in 2+3/7 || 1 {
        x = 1
    }
    ".to_string();
    let tokenizer = Tokenizer::new(&source);
    let mut parser = Parser::new(tokenizer);
    parser.parse();

    println!("{:#?}", parser.declarations);
}
