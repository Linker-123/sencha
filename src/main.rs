use asm::*;
use tokenizer::Tokenizer;

mod asm;
mod compile;
mod tokenizer;

fn main() {
    let source = "\"EEE\" 4357.1".to_string();
    let mut tokenizer = Tokenizer::new(&source);

    for tok in tokenizer.into_iter() {
        println!("Token: {:#?}", tok);
    }

    // let mut context = Context::new();
    // context.add_external_sym("printf".to_string());
    // context.add_external_sym("exit".to_string());

    // let mut data = Section::new(SectionKind::Data);
    // data.add_instruction(Ins::IData(
    //     "format".to_string(),
    //     DataKind::Byte,
    //     "\"%#llu\", 10, 0".to_string(),
    // ));

    // let mut bss = Section::new(SectionKind::Bss);
    // bss.add_instruction(Ins::UData("sum".to_string(), DataKind::Qword));

    // let mut text = Section::new(SectionKind::Text);
    // text.add_instruction(Ins::Label(
    //     "main".to_string(),
    //     vec![
    //         Ins::Inline(Op::Mov, vec![Tgt::Rax, Tgt::Value("9007199254740991279")]),
    //         Ins::Inline(Op::Mov, vec![Tgt::Rbx, Tgt::Value("9007199254740991279")]),
    //         Ins::Inline(Op::Add, vec![Tgt::Rax, Tgt::Rbx]),
    //         Ins::Inline(Op::Mov, vec![Tgt::Vars("sum"), Tgt::Rax]),
    //         Ins::Inline(Op::Mov, vec![Tgt::Rsi, Tgt::Vars("sum")]),
    //         Ins::Inline(Op::Lea, vec![Tgt::Rdi, Tgt::Vars("rel format")]),
    //         Ins::Inline(Op::Xor, vec![Tgt::Rax, Tgt::Rax]),
    //         Ins::Inline(Op::Call, vec![Tgt::Value("printf")]),
    //         Ins::Inline(Op::Xor, vec![Tgt::Rax, Tgt::Rax]),
    //         Ins::Inline(Op::Mov, vec![Tgt::Rax, Tgt::Value("1")]),
    //         Ins::Inline(Op::Int, vec![Tgt::Value("0x80")]),
    //     ],
    // ));

    // context.add_section(data);
    // context.add_section(bss);
    // context.add_section(text);

    // compile::compile_ctx(context).unwrap();
}
