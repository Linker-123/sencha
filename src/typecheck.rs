use std::collections::HashMap;

use crate::ast::{BinaryOp, Node, UnaryOp};

#[derive(PartialEq, Clone)]
pub enum TypeKind {
    Numeric,
    Bool,
    Textual,
    None,
}

#[derive(PartialEq, Clone)]
pub struct Type {
    pub name: String,
    pub size: usize,
    pub kind: TypeKind,
}

impl Type {
    pub fn new(name: String, size: usize, kind: TypeKind) -> Type {
        Type { name, size, kind }
    }
}

type TypeMap = HashMap<String, Type>;
type LocalsMap = HashMap<String, Type>;

pub struct TypeContainer {
    types: TypeMap,
    locals: LocalsMap,
    created_locals: Option<Vec<String>>,
}

impl TypeContainer {
    pub fn new() -> TypeContainer {
        let mut container = TypeContainer {
            types: HashMap::new(),
            locals: HashMap::new(),
            created_locals: None,
        };

        container.create_type(Type::new("i8".to_string(), 1, TypeKind::Numeric));
        container.create_type(Type::new("u8".to_string(), 1, TypeKind::Numeric));
        container.create_type(Type::new("i16".to_string(), 2, TypeKind::Numeric));
        container.create_type(Type::new("u16".to_string(), 2, TypeKind::Numeric));
        container.create_type(Type::new("i32".to_string(), 4, TypeKind::Numeric));
        container.create_type(Type::new("u32".to_string(), 4, TypeKind::Numeric));
        container.create_type(Type::new("f32".to_string(), 4, TypeKind::Numeric));
        container.create_type(Type::new("i64".to_string(), 8, TypeKind::Numeric));
        container.create_type(Type::new("u64".to_string(), 8, TypeKind::Numeric));
        container.create_type(Type::new("f64".to_string(), 8, TypeKind::Numeric));
        container.create_type(Type::new("bool".to_string(), 1, TypeKind::Bool));
        container.create_type(Type::new("void".to_string(), 0, TypeKind::None));
        container
    }

    pub fn create_type(&mut self, tipe: Type) {
        self.types.insert(tipe.name.clone(), tipe);
    }

    pub fn resolve_type(&self, name: &String) -> Type {
        if let Some(tipe) = self.types.get(name) {
            tipe.clone()
        } else {
            panic!("Undefined reference to type: {}", name)
        }
    }

    pub fn resolve_local(&self, name: &String) -> Type {
        if let Some(tipe) = self.locals.get(name) {
            tipe.clone()
        } else {
            panic!("Undefined reference to variable: {}", name)
        }
    }

    pub fn check(&mut self, node: &mut Box<Node>) -> Type {
        match &mut **node {
            Node::Number(_, size, _, _) => {
                let tipe = self.resolve_type(&"i32".to_string());
                *size = tipe.size;
                tipe
            }
            Node::Float(_, size, _, _) => {
                let tipe = self.resolve_type(&"f64".to_string());
                *size = tipe.size;
                tipe
            }
            Node::BoolLiteral(_, size, _, _) => {
                let tipe = self.resolve_type(&"bool".to_string());
                *size = tipe.size;
                tipe
            }
            Node::StringLiteral(literal, _, _) => {
                self.create_type(Type::new(
                    "str".to_string(),
                    literal.len(),
                    TypeKind::Textual,
                ));
                self.resolve_type(&"str".to_string())
            }
            Node::VarGet(name, _, _) => self.resolve_local(name),
            Node::Binary(binary) => {
                let l_type = self.check(&mut binary.lhs);
                let r_type = self.check(&mut binary.rhs);

                if l_type != r_type {
                    panic!("Binary expression has invalid operands");
                }

                match binary.op {
                    BinaryOp::Add | BinaryOp::Sub | BinaryOp::Mul | BinaryOp::Div => {
                        match l_type.kind {
                            TypeKind::Numeric => (),
                            _ => panic!("Cannot do arithmetic on non-numeric types"),
                        }
                        l_type
                    }
                    BinaryOp::Greater
                    | BinaryOp::GreaterEq
                    | BinaryOp::Less
                    | BinaryOp::LessEq
                    | BinaryOp::Equal
                    | BinaryOp::NotEqual => self.resolve_type(&"bool".to_string()),
                }
            }
            Node::Function(func) => {
                let ret_type = match &func.ret_type {
                    Some(str) => str.clone(),
                    None => "void".to_string(),
                };

                let tipe = self.resolve_type(&ret_type);
                func.ret_size = tipe.size;

                self.locals.insert(func.name.clone(), tipe.clone());

                for arg in &mut func.args {
                    let arg_type = self.resolve_type(&arg.dtype);
                    arg.size = arg_type.size;
                    self.locals.insert(arg.name.clone(), arg_type);
                }

                self.check(&mut func.body);

                self.locals.remove(&func.name);
                for arg in &func.args {
                    self.locals.remove(&arg.name);
                }

                tipe
            }
            Node::VarDecl(decl) => {
                let val_type = self.check(&mut decl.value);
                if let Some(locals) = &mut self.created_locals {
                    locals.push(decl.name.clone());
                }

                // If we got an explicit type
                if let Some(ex_dt) = &decl.dtype {
                    let ex_type = self.resolve_type(ex_dt);

                    if ex_type.kind != val_type.kind {
                        panic!("Explicit variable type, doesn't equal the value type");
                    }

                    decl.dtype_size = ex_type.size;
                    self.locals.insert(decl.name.clone(), ex_type.clone());
                    ex_type
                } else {
                    decl.dtype_size = val_type.size;
                    self.locals.insert(decl.name.clone(), val_type.clone());
                    val_type
                }
            }
            Node::Unary(unary) => {
                if unary.op == UnaryOp::Not {
                    self.resolve_type(&"bool".to_string())
                } else {
                    self.check(&mut unary.expr)
                }
            }
            Node::Logical(logical) => {
                let l_type = self.check(&mut logical.lhs);
                let r_type = self.check(&mut logical.rhs);

                if l_type != r_type {
                    panic!("Logical expression has invalid operands");
                }

                l_type
            }
            Node::Assign(assign) => {
                let local = self.resolve_local(&assign.name);
                let val_type = self.check(&mut assign.value);

                if local != val_type {
                    panic!("Original variable data type doesn't equal the assigned one");
                }

                local
            }
            Node::ExprStmt(expr_stmt) => self.check(&mut expr_stmt.expr),
            Node::Block(block) => {
                let mut old_locals = vec![];
                if let Some(locals) = self.created_locals.take() {
                    old_locals = locals;
                }
                self.created_locals = Some(vec![]);

                for node in &mut block.statements {
                    self.check(node);
                }

                if let Some(locals) = &self.created_locals {
                    for local in locals {
                        self.locals.remove(local);
                    }
                }

                self.created_locals = Some(old_locals);
                self.resolve_type(&"void".to_string())
            }
            Node::If(if_stmt) => {
                let cond_type = self.check(&mut if_stmt.condition);
                if cond_type != self.resolve_type(&"bool".to_string()) {
                    panic!("If condition doesn't evaluate to a bool");
                }

                self.check(&mut if_stmt.then_block);

                if let Some(else_block) = &mut if_stmt.else_block {
                    self.check(else_block);
                }

                self.resolve_type(&"void".to_string())
            }
            _ => todo!(),
        }
    }
}
