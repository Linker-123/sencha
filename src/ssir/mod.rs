use crate::ast::{Node, VarDecl};

use self::{
    ins::{Function, Instruction, Label},
    tmp::{AssignTmp, BinaryTmp, LogicalTmp, TmpChild, TmpNode, UnaryTmp, ValueTmp},
};

mod ins;
mod tmp;

/// Secondary stage intermediate representation
pub struct SSir {
    tmp_count: usize,
    functions: Vec<Function>,
    func: Option<Function>,
    label: Option<Label>,
    label_count: usize,
}

impl SSir {
    pub fn new() -> SSir {
        SSir {
            tmp_count: 0,
            functions: Vec::new(),
            func: None,
            label: None,
            label_count: 0,
        }
    }

    pub fn generate(&mut self, decls: &Vec<Box<Node>>) {
        for decl in decls {
            self.process_node(decl);
        }
    }

    pub fn export(&mut self) {
        for func in &self.functions {
            println!("func {}:", func.name);
            for ins in &func.instructions {
                Self::print_instruction(ins);
            }
            for label in &func.labels {
                println!("LC{}:", label.id);
                for ins in &label.instructions {
                    Self::print_instruction(ins);
                }
            }
        }
    }

    fn print_instruction(ins: &Instruction) {
        match ins {
            Instruction::TmpNode(node) => {
                SSir::print_node(node);
            }
            Instruction::VarDecl(name, node, size) => {
                println!("\t{}{{{}}} := {}", size, name, node);
            }
            Instruction::Pop => {
                println!("\tpop");
            }
            Instruction::VarAssign(name, id) => {
                println!("\t{} = {}", name, id);
            }
            Instruction::IfNot(cond, ealse) => {
                println!("if NOT {}", cond);
                if *ealse != 0 {
                    println!("\tjump LC{}", ealse);
                }
                println!("else");
            }
        }
    }

    fn print_node(node: &TmpNode) {
        match node {
            TmpNode::ValueTmp(value) => println!("\ttmp{} = {}", value.id, value.value),
            TmpNode::BinaryTmp(binary) => println!(
                "\ttmp{} = {} {} {}",
                binary.id, binary.lhs, binary.op, binary.rhs
            ),
            TmpNode::LogicalTmp(logical) => println!(
                "\ttmp{} = {} {} {}",
                logical.id, logical.lhs, logical.op, logical.rhs
            ),
            TmpNode::UnaryTmp(unary) => {
                println!("\ttmp{} = {} {}", unary.id, unary.op, unary.value)
            }
            TmpNode::AssignTmp(assign) => println!("\ttmp{} = {}", assign.id, assign.value),
        }
    }

    fn add_func(&mut self, name: String) {
        self.func = Some(Function::new(name));
    }

    fn add_label(&mut self) {
        self.label_count += 1;
        self.label = Some(Label::new(self.label_count));
    }

    fn end_func(&mut self) {
        self.functions.push(self.func.take().unwrap());
    }

    fn end_label(&mut self) {
        if let Some(f) = &mut self.func {
            f.labels.push(self.label.take().unwrap());
        } else {
            panic!("Not compile a function.");
        }
    }

    fn add_ins(&mut self, ins: Instruction) {
        if let Some(l) = &mut self.label {
            l.add_ins(ins);
        } else if let Some(f) = &mut self.func {
            f.add_ins(ins);
        } else {
            panic!("Not compiling a function")
        }
    }

    fn get_tmp_id(&mut self) -> usize {
        self.tmp_count += 1;
        self.tmp_count
    }

    fn process_node(&mut self, node: &Box<Node>) -> TmpChild {
        match &**node {
            // Statements
            Node::Function(fun) => {
                self.add_func(fun.name.clone());
                self.process_node(&fun.body);
                self.end_func();

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
                self.add_ins(Instruction::VarDecl(vd.name.clone(), tmp, vd.dtype.clone()));
                
                TmpChild::None
            }
            Node::ExprStmt(es) => {
                self.process_node(&es.expr);
                self.add_ins(Instruction::Pop);

                TmpChild::None
            }
            Node::If(ief) => {
                let cond = self.process_node(&ief.condition);

                let mut else_loc: usize = 0;
                if let Some(_) = &ief.else_block {
                    else_loc = self.label_count + 1;
                }

                self.add_ins(Instruction::IfNot(cond, else_loc));
                self.process_node(&ief.then_block);

                if let Some(els) = &ief.else_block {
                    self.add_label();
                    self.process_node(els);
                    self.end_label();
                }
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
            Node::Number(n, size, _, _) => TmpChild::Literal(n.clone(), size.clone()),
            Node::Float(f, size, _, _) => TmpChild::Literal(f.clone(), size.clone()),
            Node::BoolLiteral(b, size, _, _) => TmpChild::Literal(b.to_string(), size.clone()),
            _ => {
                println!("{:#?}", node);
                unimplemented!()
            }
        }
    }
}
