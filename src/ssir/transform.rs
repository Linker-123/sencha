use std::collections::HashMap;

use crate::{
    reg::{self, RegisterLabel, RegisterManager},
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
        node: &TmpNode,
        tipe: &TaggedType,
        label: &mut Option<RegisterLabel>,
    ) {
        match &node {
            TmpNode::ValueTmp(val) => {
                let register = self.rmgr.allocate(reg::size_to_reg_size(tipe.size));
                *label = Some(register.clone());
                println!("Value tmp");
                self.ref_table.insert(val.id, register);
            }
            TmpNode::BinaryTmp(binary) => {
                let mut id: Option<usize> = None;

                if let TmpChild::TmpRef(ref_id, _) = &binary.lhs {
                    id = Some(ref_id.clone());
                } else if let TmpChild::TmpRef(ref_id, _) = &binary.rhs {
                    id = Some(ref_id.clone());
                }

                if let Some(id) = id {
                    let reg = self.resolve_reg(id);
                    *label = Some(reg.clone());
                    self.ref_table.insert(binary.id, reg);
                } else {
                    let register = self.rmgr.allocate(reg::size_to_reg_size(tipe.size));
                    *label = Some(register.clone());
                    self.ref_table.insert(binary.id, register);
                }
            }
            _ => unimplemented!(),
        }
    }
}
