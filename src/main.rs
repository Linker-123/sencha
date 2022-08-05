use parser::Parser;
use tokenizer::Tokenizer;

mod asm;
mod ast;
mod compile;
mod parser;
mod tokenizer;

fn main() {
    let source = "x = -2.4*!3.1>=6/12*2||2;".to_string();
    let tokenizer = Tokenizer::new(&source);
    let mut parser = Parser::new(tokenizer);

    println!("{:#?}", parser.expr_stmt());
}
