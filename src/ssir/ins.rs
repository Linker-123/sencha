use super::tmp::{TmpChild, TmpNode};

#[derive(Debug)]
pub enum Instruction {
    TmpNode(TmpNode),
    VarDecl(String, TmpChild),
    VarAssign(String, TmpChild),
    IfNot(TmpChild, usize),
    Jump(usize),
    Pop,
}

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

    pub fn add_label(&mut self, label: Label) {
        self.labels.push(label);
    }
}
