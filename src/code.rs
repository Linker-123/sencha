use crate::{
    ast::{BinaryOp, Block, ExprStmt, Function, Node, VarDecl},
    reg::{self, RegisterManager},
    vartable::VarTable,
};

pub struct CodeGen {
    var_table: VarTable,
    registers: RegisterManager,
}

impl CodeGen {
    pub fn new() -> CodeGen {
        CodeGen {
            var_table: VarTable::new(),
            registers: RegisterManager::new(),
        }
    }

    pub fn generate(&mut self, declarations: &Vec<Box<Node>>) {
        for decl in declarations {
            self.dispatch(decl);
        }
    }

    fn dispatch(&mut self, node: &Box<Node>) {
        match &**node {
            Node::Function(f) => self.process_func(f),
            Node::Block(b) => self.process_block(b),
            Node::VarDecl(vd) => self.process_var_decl(vd),
            Node::ExprStmt(es) => self.process_expr_stmt(es),

            // Expressions
            Node::Number(value, tipe, _, _) => {
                println!("push {}", value);
                self.var_table.vstack.push(tipe.size, tipe.signed);
            }
            Node::VarGet(name, _, _) => {
                let offset = self.var_table.find(name);
                println!("push qword [rbp-{}]", offset);
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

                    println!(
                        "mov {}, {} [rbp-{}]",
                        reg1str,
                        reg::reg_size_to_str(reg_size.clone()),
                        rhs_offset
                    );
                    println!(
                        "mov {}, {} [rbp-{}]",
                        reg2str,
                        reg::reg_size_to_str(reg_size),
                        lhs_offset
                    );

                    match bi.op {
                        BinaryOp::Add => {
                            println!("add {}, {}", reg1str, reg2str);
                            println!("mov [rbp-{}], {}", rhs_offset, reg1str);
                        }
                        BinaryOp::Sub => {
                            println!("sub {}, {}", reg1str, reg2str);
                            println!("mov [rbp-{}], {}", rhs_offset, reg1str);
                        }
                        BinaryOp::Mul => {
                            if sitem.signed.is_some() && sitem.signed.unwrap() == true {
                                println!("imul {}, {}", reg1str, reg2str);
                                println!("mov [rbp-{}], {}", rhs_offset, reg1str);
                            } else {
                                println!("mul {}, {}", reg1str, reg2str);
                                println!("mov [rbp-{}], {}", rhs_offset, reg1str);
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
                    println!(
                        "mov {}, {} [rbp-{}]\ncdq",
                        reg1str,
                        reg::reg_size_to_str(reg_size.clone()),
                        rhs_offset
                    );
                    if sitem.signed.is_some() && sitem.signed.unwrap() == true {
                        println!("idiv dword [rbp-{}]", lhs_offset);
                    } else {
                        println!("div dword [rbp-{}]", lhs_offset);
                    }
                    self.registers.deallocate(reg1);
                }
            }
            _ => unimplemented!(),
        }
    }

    fn process_func(&mut self, func: &Function) {
        println!("{}:", func.name);
        println!("push rbp\nmov rbp, rsp");
        self.dispatch(&func.body);
        println!("pop rbp");
        println!("ret");
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
        println!("add rsp, 8");
    }
}
