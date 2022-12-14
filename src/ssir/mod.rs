use crate::{
    ast::{BinaryOp, Node},
    reg::RegisterLabel,
    typechecker::{TaggedType, TypeKind},
};

use self::{
    ins::{Function, Instruction, Label},
    tmp::{AssignTmp, BinaryTmp, GroupingTmp, LogicalTmp, TmpChild, TmpNode, UnaryTmp, ValueTmp},
    var_table::{VarTable, Variable},
};

mod ins;
mod reveng;
mod tmp;
pub mod transform;
mod var_table;

fn print_instruction(ins: &Instruction) {
    match ins {
        Instruction::TmpNode(node, tipe, label) => {
            print_node(node, Some(tipe), label);
        }
        Instruction::VarDecl(name, node, size) => {
            println!("\t{}{{{}}} := {}", size, name, node);
        }
        Instruction::Pop => {
            println!("\tpop");
        }
        Instruction::VarAssign(name, id, tipe) => {
            println!("\t{}{{{}}} = {}", tipe, name, id);
        }
        Instruction::If(cond) => {
            print!("\tif ");
            print_node(cond, None, &None);
        }
        Instruction::Jump(lc) => {
            println!("\tjump LC{}", lc);
        }
    }
}

fn print_node(node: &TmpNode, tipe: Option<&TaggedType>, label: &Option<RegisterLabel>) {
    match node {
        TmpNode::ValueTmp(value) => {
            if let Some(tipe) = tipe {
                println!(
                    "\t{} -> {}{{tmp{}}} = {}",
                    label.as_ref().unwrap(),
                    tipe,
                    value.id,
                    value.value
                )
            } else {
                println!("{}", value.value);
            }
        }
        TmpNode::BinaryTmp(binary) => {
            if let Some(tipe) = tipe {
                println!(
                    "\t{} -> {}{{tmp{}}} = {} {} {}",
                    label.as_ref().unwrap(),
                    tipe,
                    binary.id,
                    binary.lhs,
                    binary.op,
                    binary.rhs
                )
            } else {
                println!("{} {} {}", binary.lhs, binary.op, binary.rhs);
            }
        }
        TmpNode::LogicalTmp(logical) => {
            if let Some(tipe) = tipe {
                println!(
                    "\t{} -> {}{{tmp{}}} = {} {} {}",
                    label.as_ref().unwrap(),
                    tipe,
                    logical.id,
                    logical.lhs,
                    logical.op,
                    logical.rhs
                )
            } else {
                println!("{} {} {}", logical.lhs, logical.op, logical.rhs);
            }
        }
        TmpNode::UnaryTmp(unary) => {
            if let Some(tipe) = tipe {
                println!(
                    "\t{} -> {}{{tmp{}}} = {} {}",
                    label.as_ref().unwrap(),
                    tipe,
                    unary.id,
                    unary.op,
                    unary.value
                )
            } else {
                println!("{} {}", unary.op, unary.value);
            }
        }
        TmpNode::AssignTmp(assign) => {
            if let Some(tipe) = tipe {
                println!(
                    "\t{} -> {}{{tmp{}}} = {}",
                    label.as_ref().unwrap(),
                    tipe,
                    assign.id,
                    assign.value
                );
            } else {
                println!("{}", assign.value);
            }
        }
        TmpNode::GroupingTmp(grouping) => {
            println!(
                "\t{}{{tmp{}}} = ({})",
                grouping.tipe, grouping.id, grouping.expr
            );
        }
    }
}

pub fn print_functions(functions: &Vec<Function>) {
    for func in functions {
        println!("func {}:", func.name);
        for ins in &func.instructions {
            print_instruction(ins);
        }
        for label in &func.labels {
            println!("LC{}:", label.id);
            for ins in &label.instructions {
                print_instruction(ins);
            }
        }
    }
}

pub fn get_child_type(child: &TmpChild) -> TaggedType {
    match child {
        TmpChild::Literal(_, tipe) => tipe.clone(),
        TmpChild::LoadVar(_, tipe) => tipe.clone(),
        TmpChild::TmpRef(_, tipe, _) => tipe.clone(),
        _ => panic!("No tagged type for tmp child"),
    }
}

/// Secondary stage intermediate representation
pub struct SSir {
    tmp_count: usize,
    functions: Vec<Function>,
    func: Option<Function>,
    label: Option<Label>,
    label_count: usize,
    variables: VarTable,
    is_condition: bool,
    condition_node: Option<TmpNode>,
}

impl SSir {
    pub fn new() -> SSir {
        SSir {
            tmp_count: 1,
            functions: Vec::new(),
            func: None,
            label: None,
            label_count: 0,
            variables: VarTable::new(),
            is_condition: false,
            condition_node: None,
        }
    }

    pub fn generate(&mut self, decls: &mut Vec<Box<Node>>) {
        for decl in decls {
            self.process_node(decl);
        }
    }

    pub fn get_functions(self) -> Vec<Function> {
        self.functions
    }

    fn add_func(&mut self, name: String) {
        self.func = Some(Function::new(name));
    }

    fn swap_label(&mut self, label_count: usize) {
        self.label = Some(Label::new(label_count))
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
            panic!("Not compiling a function.");
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

    fn process_node(&mut self, node: &mut Box<Node>) -> TmpChild {
        match &mut **node {
            // Statements
            Node::Function(fun) => {
                self.add_func(fun.name.clone());
                self.process_node(&mut fun.body);
                self.end_label();
                self.end_func();

                TmpChild::None
            }
            Node::Block(bl) => {
                self.variables.add_scope();
                for stmt in &mut bl.statements {
                    self.process_node(stmt);
                }
                self.variables.end_scope();
                TmpChild::None
            }
            Node::VarDecl(vd) => {
                let tmp = self.process_node(&mut vd.value);
                self.add_ins(Instruction::VarDecl(vd.name.clone(), tmp, vd.dtype.clone()));
                self.variables
                    .add_var(Variable::new(vd.name.clone(), vd.dtype.clone()));
                self.add_ins(Instruction::Pop);

                TmpChild::None
            }
            Node::ExprStmt(es) => {
                self.process_node(&mut es.expr);
                self.add_ins(Instruction::Pop);

                TmpChild::None
            }
            Node::If(ief) => {
                reveng::reverse_binary(&mut ief.condition);
                self.is_condition = true;
                self.process_node(&mut ief.condition);

                // let mut else_loc: usize = 0;
                // if let Some(_) = &ief.else_block {
                // else_loc = self.label_count + 1;
                // }

                let cond_node = self.condition_node.take();
                self.add_ins(Instruction::If(cond_node.unwrap()));
                if let Some(ealse) = &mut ief.else_block {
                    self.process_node(ealse);
                }

                let reserved = self.label_count + 2;
                self.add_ins(Instruction::Jump(reserved));
                self.add_label();

                self.process_node(&mut ief.then_block);
                self.add_ins(Instruction::Jump(reserved));
                self.end_label();

                self.swap_label(reserved);
                
                // if let Some(els) = &mut ief.else_block {
                //     self.add_label();
                //     self.process_node(els);
                //     self.end_label();
                // }

                self.add_ins(Instruction::Pop);
                TmpChild::None
            }
            Node::Binary(bi) => {
                let is_condition = self.is_condition;
                if is_condition {
                    self.is_condition = false;
                }

                let lhs = self.process_node(&mut bi.lhs);
                let rhs = self.process_node(&mut bi.rhs);

                let lhs_type = get_child_type(&lhs);
                let res_type = match bi.op {
                    BinaryOp::Add | BinaryOp::Div | BinaryOp::Mul | BinaryOp::Sub => {
                        lhs_type.clone()
                    }
                    _ => TaggedType::new(1, TypeKind::Bool, None),
                };

                if is_condition {
                    self.condition_node = Some(TmpNode::BinaryTmp(BinaryTmp::new(
                        lhs,
                        rhs,
                        bi.op.clone(),
                        0,
                        lhs_type.clone(),
                    )));
                    return TmpChild::None;
                } else {
                    let id = self.get_tmp_id();
                    self.add_ins(Instruction::TmpNode(
                        TmpNode::BinaryTmp(BinaryTmp::new(
                            lhs,
                            rhs,
                            bi.op.clone(),
                            id,
                            lhs_type.clone(),
                        )),
                        res_type.clone(),
                        None,
                    ));

                    return TmpChild::TmpRef(id, res_type.clone(), None);
                }
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
                    None,
                ));

                return TmpChild::TmpRef(id, var.tagged_type.clone(), None);
            }
            Node::Unary(un) => {
                let is_condition = self.is_condition;
                if is_condition {
                    self.is_condition = false;
                }

                let value = self.process_node(&mut un.expr);
                let ttype = get_child_type(&value);
                if is_condition {
                    let utmp = UnaryTmp::new(value, un.op.clone(), 0);
                    self.condition_node = Some(TmpNode::UnaryTmp(utmp));

                    return TmpChild::None;
                } else {
                    let id = self.get_tmp_id();
                    let utmp = UnaryTmp::new(value, un.op.clone(), id);
                    self.add_ins(Instruction::TmpNode(
                        TmpNode::UnaryTmp(utmp),
                        ttype.clone(),
                        None,
                    ));

                    return TmpChild::TmpRef(id, ttype, None);
                }
            }
            Node::Logical(lg) => {
                let is_condition = self.is_condition;
                if is_condition {
                    self.is_condition = false;
                }

                let lhs = self.process_node(&mut lg.lhs);
                let rhs = self.process_node(&mut lg.rhs);
                let ttype = get_child_type(&lhs);

                if is_condition {
                    let ltmp = LogicalTmp::new(lhs, rhs, lg.op.clone(), 0);
                    self.condition_node = Some(TmpNode::LogicalTmp(ltmp));

                    return TmpChild::None;
                } else {
                    let id = self.get_tmp_id();
                    let ltmp = LogicalTmp::new(lhs, rhs, lg.op.clone(), id);
                    self.add_ins(Instruction::TmpNode(
                        TmpNode::LogicalTmp(ltmp),
                        ttype.clone(),
                        None,
                    ));

                    return TmpChild::TmpRef(id, ttype, None);
                }
            }
            Node::Assign(asi) => {
                let is_condition = self.is_condition;
                if is_condition {
                    self.is_condition = false;
                }

                let value = self.process_node(&mut asi.value);
                let ttype = get_child_type(&value);

                let id = self.get_tmp_id();
                let atmp = AssignTmp::new(value, id);

                self.add_ins(Instruction::TmpNode(
                    TmpNode::AssignTmp(atmp),
                    ttype.clone(),
                    None,
                ));
                self.add_ins(Instruction::VarAssign(
                    asi.name.clone(),
                    TmpChild::TmpRef(id, ttype.clone(), None),
                    ttype.clone(),
                ));

                return TmpChild::TmpRef(id, ttype, None);
            }
            Node::Number(n, size, _, _) => TmpChild::Literal(n.clone(), size.clone()),
            Node::Float(f, size, _, _) => TmpChild::Literal(f.clone(), size.clone()),
            Node::BoolLiteral(b, size, _, _) => TmpChild::Literal(b.to_string(), size.clone()),
            Node::Grouping(grouping) => {
                let is_condition = self.is_condition;
                if is_condition {
                    self.is_condition = false;
                }

                let expr = self.process_node(&mut grouping.expr);
                let ttype = get_child_type(&expr);

                if is_condition {
                    self.condition_node = Some(TmpNode::GroupingTmp(GroupingTmp::new(
                        expr,
                        ttype.clone(),
                        0,
                    )));
                    return TmpChild::None;
                } else {
                    if let TmpChild::TmpRef(ref_id, _, _) = expr {
                        return TmpChild::TmpRef(ref_id, ttype, None);
                    } else {
                        return expr;
                    }
                }
            }
            _ => {
                println!("{:#?}", node);
                unimplemented!()
            }
        }
    }
}
