use crate::typechecker::TaggedType;
use crate::reg::RegisterLabel;
use super::tmp::{TmpChild, TmpNode};

#[derive(Debug)]
pub enum Instruction {
    TmpNode(TmpNode, TaggedType, Option<RegisterLabel>),
    VarDecl(String, TmpChild, TaggedType),
    VarAssign(String, TmpChild, TaggedType),
    If(TmpNode),
    Pop,
}

#[derive(Debug)]
pub struct Label {
    pub instructions: Vec<Instruction>,
    pub id: usize,
}

impl Label {
    pub fn new(id: usize) -> Label {
        Label {
            instructions: Vec::new(),
            id,
        }
    }

    pub fn add_ins(&mut self, ins: Instruction) {
        self.instructions.push(ins);
    }
}

#[derive(Debug)]
pub struct Function {
    pub instructions: Vec<Instruction>,
    pub labels: Vec<Label>,
    pub name: String,
}

impl Function {
    pub fn new(name: String) -> Function {
        Function {
            instructions: Vec::new(),
            labels: Vec::new(),
            name,
        }
    }

    pub fn add_ins(&mut self, ins: Instruction) {
        self.instructions.push(ins);
    }

    #[allow(dead_code)]
    pub fn add_label(&mut self, label: Label) {
        self.labels.push(label);
    }
}
