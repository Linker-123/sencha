use crate::{
    asm::{Asm, AsmLabel},
    ast::{BinaryOp, Block, ExprStmt, Function, Node, VarDecl},
    reg::{self, RegisterManager},
    vartable::VarTable,
};
use std::fs;
use std::io::Write;

pub struct CodeGen {
    var_table: VarTable,
    registers: RegisterManager,
    assembly: Asm,
    label: AsmLabel,
}

impl CodeGen {
    pub fn new() -> CodeGen {
        CodeGen {
            var_table: VarTable::new(),
            registers: RegisterManager::new(),
            assembly: Asm::new(),
            label: AsmLabel::new(),
        }
    }

    pub fn generate(&mut self, declarations: &Vec<Box<Node>>) {
        for decl in declarations {
            self.dispatch(decl);
        }
    }

    pub fn write(self) {
        fs::create_dir_all("build/debug").unwrap();
        let mut file = fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open("build/debug/main.nasm")
            .unwrap();
        let content = self.assembly.export();
        write!(file, "{}", content).unwrap();
    }

    fn dispatch(&mut self, node: &Box<Node>) {
        match &**node {
            Node::Function(f) => self.process_func(f),
            Node::Block(b) => self.process_block(b),
            Node::VarDecl(vd) => self.process_var_decl(vd),
            Node::ExprStmt(es) => self.process_expr_stmt(es),

            // Expressions
            Node::Number(value, tipe, _, _) => {
                self.label.push(value);
                self.var_table.vstack.push(tipe.size, tipe.signed);
            }
            Node::VarGet(name, _, _) => {
                let (offset, size) = self.var_table.find(name);
                let reg_size = reg::size_to_reg_size(size);
                self.label.push(format!(
                    "{} [rbp-{}]",
                    reg::reg_size_to_str(reg_size),
                    offset
                ));
            }
            Node::Binary(bi) => {
                self.dispatch(&bi.lhs);
                self.dispatch(&bi.rhs);

                let len = self.var_table.vstack.len();
                let rhs_offset = len * 8;
                let lhs_offset = len * 8 - 8;
                let sitem = self.var_table.vstack.get(len - 1);
                let reg_size = reg::size_to_reg_size(sitem.size);

                if bi.op != BinaryOp::Div {
                    let reg1 = self.registers.allocate(reg_size.clone());
                    let reg2 = self.registers.allocate(reg_size.clone());

                    let reg1str = format!("{:#?}", reg1).to_lowercase();
                    let reg2str = format!("{:#?}", reg2).to_lowercase();

                    self.label.mov(
                        &reg1str,
                        format!(
                            "{} [rbp-{}]",
                            reg::reg_size_to_str(reg_size.clone()),
                            rhs_offset
                        ),
                    );
                    self.label.mov(
                        &reg2str,
                        format!("{} [rbp-{}]", reg::reg_size_to_str(reg_size), lhs_offset),
                    );

                    match bi.op {
                        BinaryOp::Add => {
                            self.label.add(&reg1str, reg2str);
                            self.label.mov(format!("[rbp-{}]", rhs_offset), reg1str);
                        }
                        BinaryOp::Sub => {
                            self.label.sub(&reg1str, reg2str);
                            self.label.mov(format!("[rbp-{}]", rhs_offset), reg1str);
                        }
                        BinaryOp::Mul => {
                            if sitem.signed.is_some() && sitem.signed.unwrap() == true {
                                self.label.imul(&reg1str, reg2str);
                                self.label.mov(format!("[rbp-{}]", rhs_offset), reg1str);
                            } else {
                                self.label.mul(&reg1str, reg2str);
                                self.label.mov(format!("[rbp-{}]", rhs_offset), reg1str);
                            }
                        }
                        _ => unimplemented!(),
                    }

                    self.registers.deallocate(reg1);
                    self.registers.deallocate(reg2);
                } else {
                    let reg1 = self.registers.allocate(reg_size.clone());
                    // if reg1 != RegisterLabel::Rax
                    //     && reg1 != RegisterLabel::Ax
                    //     && reg1 != RegisterLabel::Eax
                    //     && reg1 != RegisterLabel::Al
                    // {
                    //     error::panic(format!("Allocated register is not an AX based"));
                    // }

                    let reg1str = format!("{:#?}", reg1).to_lowercase();
                    self.label.mov(
                        reg1str,
                        format!(
                            "{} [rbp-{}]",
                            reg::reg_size_to_str(reg_size.clone()),
                            rhs_offset
                        ),
                    );
                    if sitem.signed.is_some() && sitem.signed.unwrap() == true {
                        self.label.idiv(format!("dword [rbp-{}]", lhs_offset));
                    } else {
                        self.label.div(format!("dword [rbp-{}]", lhs_offset));
                    }
                    self.registers.deallocate(reg1);
                }
            }
            Node::GetPtr(get_ptr) => {
                self.dispatch(&get_ptr.expr);
                let len = self.var_table.vstack.len();
                let offset = len * 8;
                let sitem = self.var_table.vstack.get(len - 1);
                let reg_size = reg::size_to_reg_size(sitem.size);

                let reg1 = self.registers.allocate(reg_size.clone());
                let reg1_str = format!("{:#?}", reg1).to_lowercase();

                self.label.lea(
                    &reg1_str,
                    format!(
                        "{} [rbp-{}]",
                        reg::reg_size_to_str(reg_size.clone()),
                        offset
                    ),
                );
                self.label.mov(
                    format!("{} [rbp-{}]", reg::reg_size_to_str(reg_size), offset),
                    reg1_str,
                );
                self.registers.deallocate(reg1);
            }
            _ => unimplemented!(),
        }
    }

    fn process_func(&mut self, func: &Function) {
        self.label.push("rbp");
        self.label.mov("rbp", "rsp");
        self.dispatch(&func.body);
        self.label.pop("rbp");
        self.label.ret();

        let mut label = AsmLabel::new();
        std::mem::swap(&mut self.label, &mut label);

        self.assembly.add_label(func.name.clone(), label);
    }

    fn process_block(&mut self, block: &Block) {
        self.var_table.open_scope();
        for stmt in &block.statements {
            self.dispatch(stmt);
        }

        if self.var_table.items.len() > 0 {
            let mut last = self.var_table.back();

            while self.var_table.items.len() > 0 && last.scope_level == self.var_table.scopes {
                self.var_table.pop();
                self.var_table.vstack.pop_item();
                if self.var_table.items.len() > 0 {
                    last = self.var_table.back();
                }
            }
            self.var_table.close_scope();
        }
    }

    fn process_var_decl(&mut self, decl: &VarDecl) {
        self.var_table
            .push(decl.name.clone(), decl.dtype.size, decl.dtype.signed);
        self.dispatch(&decl.value);
    }

    fn process_expr_stmt(&mut self, es: &ExprStmt) {
        self.dispatch(&es.expr);
        self.label.add("rsp", "8");
    }
}
