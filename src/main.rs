use parser::Parser;
use tokenizer::Tokenizer;

mod asm;
mod ast;
mod compile;
mod parser;
mod tokenizer;

fn main() {
    let source = "
    func main asdjiasdu9u132812931231
    }
    ".to_string();
    let tokenizer = Tokenizer::new(&source);
    let mut parser = Parser::new(tokenizer, &source);
    let res = parser.parse();
    match res {
        Err(e) => eprintln!("{}", e),
        _ => ()
    }

    println!("{:#?}", parser.declarations);
}
