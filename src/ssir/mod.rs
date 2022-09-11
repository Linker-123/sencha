use crate::ast::Node;

use self::{
    ins::Instruction,
    tmp::{AssignTmp, BinaryTmp, LogicalTmp, TmpChild, TmpNode, UnaryTmp, ValueTmp},
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
    }

    pub fn export(&mut self) {
        for ins in &self.instructions {
            match ins {
                Instruction::TmpNode(node) => {
                    SSir::print_node(node);
                }
                Instruction::VarDecl(name, node) => {
                    println!("{} := {}", name, node);
                }
                Instruction::Pop => {
                    println!("pop");
                }
                Instruction::VarAssign(name, id) => {
                    println!("{} = {}", name, id);
                }
            }
        }
    }

    fn print_node(node: &TmpNode) {
        match node {
            TmpNode::ValueTmp(value) => println!("tmp{} = {}", value.id, value.value),
            TmpNode::BinaryTmp(binary) => println!(
                "tmp{} = {} {} {}",
                binary.id, binary.lhs, binary.op, binary.rhs
            ),
            TmpNode::LogicalTmp(logical) => println!(
                "tmp{} = {} {} {}",
                logical.id, logical.lhs, logical.op, logical.rhs
            ),
            TmpNode::UnaryTmp(unary) => println!("tmp{} = {} {}", unary.id, unary.op, unary.value),
            TmpNode::AssignTmp(assign) => println!("tmp{} = {}", assign.id, assign.value),
        }
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
            Node::ExprStmt(es) => {
                self.process_node(&es.expr);
                self.add_ins(Instruction::Pop);

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
                self.add_ins(Instruction::TmpNode(TmpNode::ValueTmp(ValueTmp::new(
                    TmpChild::LoadVar(name.clone()),
                    id,
                ))));

                return TmpChild::TmpRef(id);
            }
            Node::Unary(un) => {
                let id = self.get_tmp_id();
                let value = self.process_node(&un.expr);
                let utmp = UnaryTmp::new(value, un.op.clone(), id);
                self.add_ins(Instruction::TmpNode(TmpNode::UnaryTmp(utmp)));

                return TmpChild::TmpRef(id);
            }
            Node::Logical(lg) => {
                let id = self.get_tmp_id();
                let lhs = self.process_node(&lg.lhs);
                let rhs = self.process_node(&lg.rhs);
                let ltmp = LogicalTmp::new(lhs, rhs, lg.op.clone(), id);
                self.add_ins(Instruction::TmpNode(TmpNode::LogicalTmp(ltmp)));

                return TmpChild::TmpRef(id);
            }
            Node::Assign(asi) => {
                let id = self.get_tmp_id();
                let value = self.process_node(&asi.value);
                let atmp = AssignTmp::new(value, id);

                self.add_ins(Instruction::TmpNode(TmpNode::AssignTmp(atmp)));
                self.add_ins(Instruction::VarAssign(
                    asi.name.clone(),
                    TmpChild::TmpRef(id),
                ));

                return TmpChild::TmpRef(id);
            }
            Node::Number(n, _, _) => TmpChild::Literal(n.clone()),
            Node::Float(f, _, _) => TmpChild::Literal(f.clone()),
            Node::BoolLiteral(b, _, _) => TmpChild::Literal(b.to_string()),
            _ => {
                println!("{:#?}", node);
                unimplemented!()
            }
        }
    }
}
