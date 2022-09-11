use crate::ast::Node;

use self::{
    ins::Instruction,
    tmp::{BinaryTmp, TmpChild, TmpNode, UnoTmp},
};

mod ins;
mod tmp;

/// Secondary stage intermediate representation
pub struct SSir {
    tmp_count: usize,
    instructions: Vec<Instruction>,
}

impl SSir {
    pub fn new() -> SSir {
        SSir {
            tmp_count: 0,
            instructions: Vec::new(),
        }
    }

    pub fn generate(&mut self, decls: &Vec<Box<Node>>) {
        for decl in decls {
            self.process_node(decl);
        }
        println!("Instructions: {:#?}", self.instructions);
    }

    fn add_ins(&mut self, ins: Instruction) {
        self.instructions.push(ins);
    }

    fn get_tmp_id(&mut self) -> usize {
        self.tmp_count += 1;
        self.tmp_count
    }

    fn process_node(&mut self, node: &Box<Node>) -> TmpChild {
        match &**node {
            // Statements
            Node::Function(fun) => {
                self.process_node(&fun.body);

                TmpChild::None
            }
            Node::Block(bl) => {
                for stmt in &bl.statements {
                    self.process_node(stmt);
                }
                TmpChild::None
            }
            Node::VarDecl(vd) => {
                let tmp = self.process_node(&vd.value);

                self.add_ins(Instruction::VarDecl(vd.name.clone(), tmp));

                TmpChild::None
            }

            Node::Binary(bi) => {
                let lhs = self.process_node(&bi.lhs);
                let rhs = self.process_node(&bi.rhs);

                let id = self.get_tmp_id();
                self.add_ins(Instruction::TmpNode(TmpNode::BinaryTmp(BinaryTmp::new(
                    lhs,
                    rhs,
                    bi.op.clone(),
                    id,
                ))));

                return TmpChild::TmpRef(id);
            }
            Node::VarGet(name, _, _) => {
                let id = self.get_tmp_id();
                self.add_ins(Instruction::TmpNode(TmpNode::UnoTmp(UnoTmp::new(
                    TmpChild::LoadVar(name.clone()),
                    id,
                ))));

                return TmpChild::TmpRef(id);
            }
            Node::Number(n, _, _) => TmpChild::Literal(n.clone()),
            Node::Float(f, _, _) => TmpChild::Literal(f.clone()),
            Node::BoolLiteral(b, _, _) => TmpChild::Literal(b.to_string()),
            _ => unimplemented!(),
        }
    }
}
