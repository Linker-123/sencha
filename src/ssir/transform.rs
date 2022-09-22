use std::collections::HashMap;

use crate::{
    ast::BinaryOp,
    reg::{self, RegisterLabel, RegisterManager, RegisterSize},
    ssir::ins::Instruction,
    typechecker::TaggedType,
};

use super::{
    ins::Function,
    tmp::{TmpChild, TmpNode},
};

pub struct RegisterLabeler {
    rmgr: RegisterManager,
    ref_table: HashMap<usize, RegisterLabel>,
}

impl RegisterLabeler {
    pub fn new() -> RegisterLabeler {
        RegisterLabeler {
            rmgr: RegisterManager::new(),
            ref_table: HashMap::new(),
        }
    }

    pub fn assign_labels(&mut self, mut functions: Vec<Function>) -> Vec<Function> {
        for func in &mut functions {
            for ins in &mut func.instructions {
                match ins {
                    Instruction::TmpNode(node, tipe, label) => {
                        self.process_ins(node, tipe, label);
                    }
                    Instruction::Pop => {
                        self.rmgr.deallocate_all();
                    }
                    _ => {}
                }
            }
        }
        functions
    }

    fn resolve_reg(&mut self, id: usize) -> RegisterLabel {
        self.ref_table.get(&id).unwrap().clone()
    }

    fn process_ins(
        &mut self,
        node: &mut TmpNode,
        tipe: &TaggedType,
        label: &mut Option<RegisterLabel>,
    ) {
        match node {
            TmpNode::ValueTmp(val) => {
                let register = self.rmgr.allocate(reg::size_to_reg_size(tipe.size));
                *label = Some(register.clone());
                self.ref_table.insert(val.id, register);
            }
            TmpNode::BinaryTmp(binary) => {
                match binary.op {
                    BinaryOp::Equal
                    | BinaryOp::Greater
                    | BinaryOp::GreaterEq
                    | BinaryOp::NotEqual
                    | BinaryOp::Less
                    | BinaryOp::LessEq => {
                        *label = Some(self.rmgr.allocate(RegisterSize::Byte));
                        return;
                    }
                    _ => (),
                };

                let mut tmp_ref = false;
                if let TmpChild::TmpRef(ref_id, _, llabel) = &mut binary.lhs {
                    let reg = self.resolve_reg(ref_id.clone());
                    *label = Some(reg.clone());
                    *llabel = Some(reg.clone());
                    self.ref_table.insert(binary.id, reg);
                    tmp_ref = true;
                } else if let TmpChild::TmpRef(ref_id, _, rlabel) = &mut binary.rhs {
                    let reg = self.resolve_reg(ref_id.clone());
                    *label = Some(reg.clone());
                    *rlabel = Some(reg.clone());
                    self.ref_table.insert(binary.id, reg);
                    tmp_ref = true;
                }

                if !tmp_ref {
                    let register = self.rmgr.allocate(reg::size_to_reg_size(tipe.size));
                    *label = Some(register.clone());
                    self.ref_table.insert(binary.id, register);
                }
            }
            TmpNode::AssignTmp(assign) => {
                if let TmpChild::TmpRef(ref_id, _, vlabel) = &mut assign.value {
                    let reg = self.resolve_reg(ref_id.clone());
                    *vlabel = Some(reg.clone());
                    *label = Some(reg.clone());
                    self.ref_table.insert(assign.id, reg);
                } else {
                    unreachable!()
                }
            }
            TmpNode::GroupingTmp(grouping) => {
                if let TmpChild::TmpRef(ref_id, _, llabel) = &mut grouping.expr {
                    let reg = self.resolve_reg(ref_id.clone());
                    *label = Some(reg.clone());
                    *llabel = Some(reg.clone());
                    self.ref_table.insert(grouping.id, reg);
                }
            }
            _ => unimplemented!(),
        }
    }
}
