use crate::{ast::Node, typechecker::TaggedType};

use self::{
    ins::{Function, Instruction, Label},
    tmp::{AssignTmp, BinaryTmp, LogicalTmp, TmpChild, TmpNode, UnaryTmp, ValueTmp},
    var_table::{VarTable, Variable},
};

mod ins;
mod tmp;
mod var_table;

/// Secondary stage intermediate representation
pub struct SSir {
    tmp_count: usize,
    functions: Vec<Function>,
    func: Option<Function>,
    label: Option<Label>,
    label_count: usize,
    variables: VarTable,
}

impl SSir {
    pub fn new() -> SSir {
        SSir {
            tmp_count: 0,
            functions: Vec::new(),
            func: None,
            label: None,
            label_count: 0,
            variables: VarTable::new(),
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
            Instruction::TmpNode(node, tipe) => {
                SSir::print_node(node, tipe);
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

    fn print_node(node: &TmpNode, tipe: &TaggedType) {
        match node {
            TmpNode::ValueTmp(value) => println!("\t{}{{tmp{}}} = {}", tipe, value.id, value.value),
            TmpNode::BinaryTmp(binary) => println!(
                "\t{}{{tmp{}}} = {} {} {}",
                tipe, binary.id, binary.lhs, binary.op, binary.rhs
            ),
            TmpNode::LogicalTmp(logical) => println!(
                "\t{}{{tmp{}}} = {} {} {}",
                tipe, logical.id, logical.lhs, logical.op, logical.rhs
            ),
            TmpNode::UnaryTmp(unary) => {
                println!(
                    "\t{}{{tmp{}}} = {} {}",
                    tipe, unary.id, unary.op, unary.value
                )
            }
            TmpNode::AssignTmp(assign) => {
                println!("\t{}{{tmp{}}} = {}", tipe, assign.id, assign.value)
            }
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

    fn get_child_type(child: &TmpChild) -> TaggedType {
        match child {
            TmpChild::Literal(_, tipe) => tipe.clone(),
            TmpChild::LoadVar(_, tipe) => tipe.clone(),
            TmpChild::TmpRef(_, tipe) => tipe.clone(),
            _ => panic!("No tagged type for tmp child"),
        }
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
                self.variables.add_scope();
                for stmt in &bl.statements {
                    self.process_node(stmt);
                }
                self.variables.end_scope();
                TmpChild::None
            }
            Node::VarDecl(vd) => {
                let tmp = self.process_node(&vd.value);
                self.add_ins(Instruction::VarDecl(vd.name.clone(), tmp, vd.dtype.clone()));
                self.variables
                    .add_var(Variable::new(vd.name.clone(), vd.dtype.clone()));

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

                let lhs_type = Self::get_child_type(&lhs);
                let id = self.get_tmp_id();
                self.add_ins(Instruction::TmpNode(
                    TmpNode::BinaryTmp(BinaryTmp::new(
                        lhs,
                        rhs,
                        bi.op.clone(),
                        id,
                        lhs_type.clone(),
                    )),
                    lhs_type.clone(),
                ));

                return TmpChild::TmpRef(id, lhs_type);
            }
            Node::VarGet(name, _, _) => {
                let id = self.get_tmp_id();
                let var = self.variables.get_var(name.clone()).unwrap();
                self.add_ins(Instruction::TmpNode(
                    TmpNode::ValueTmp(ValueTmp::new(
                        TmpChild::LoadVar(name.clone(), var.tagged_type.clone()),
                        id,
                    )),
                    var.tagged_type.clone(),
                ));

                return TmpChild::TmpRef(id, var.tagged_type.clone());
            }
            Node::Unary(un) => {
                let id = self.get_tmp_id();
                let value = self.process_node(&un.expr);
                let ttype = Self::get_child_type(&value);
                let utmp = UnaryTmp::new(value, un.op.clone(), id);
                self.add_ins(Instruction::TmpNode(TmpNode::UnaryTmp(utmp), ttype.clone()));

                return TmpChild::TmpRef(id, ttype);
            }
            Node::Logical(lg) => {
                let id = self.get_tmp_id();
                let lhs = self.process_node(&lg.lhs);
                let rhs = self.process_node(&lg.rhs);
                let ttype = Self::get_child_type(&lhs);
                let ltmp = LogicalTmp::new(lhs, rhs, lg.op.clone(), id);
                self.add_ins(Instruction::TmpNode(
                    TmpNode::LogicalTmp(ltmp),
                    ttype.clone(),
                ));

                return TmpChild::TmpRef(id, ttype);
            }
            Node::Assign(asi) => {
                let id = self.get_tmp_id();
                let value = self.process_node(&asi.value);
                let ttype = Self::get_child_type(&value);
                let atmp = AssignTmp::new(value, id);

                self.add_ins(Instruction::TmpNode(
                    TmpNode::AssignTmp(atmp),
                    ttype.clone(),
                ));
                self.add_ins(Instruction::VarAssign(
                    asi.name.clone(),
                    TmpChild::TmpRef(id, ttype.clone()),
                ));

                return TmpChild::TmpRef(id, ttype);
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
