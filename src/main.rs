use parser::Parser;
use tokenizer::Tokenizer;

mod asm;
mod ast;
mod compile;
mod parser;
mod tokenizer;

fn main() {
    let source = "
    func main {
        x = 3
    }
    ".to_string();
    let tokenizer = Tokenizer::new(&source);
    let mut parser = Parser::new(tokenizer);
    parser.parse();

    println!("{:#?}", parser.declarations);
}
