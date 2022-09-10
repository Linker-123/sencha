use std::collections::HashMap;

use crate::{
    ast::{BinaryOp, Node, UnaryOp},
    error,
};

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum TypeKind {
    Numeric,
    Float,
    Bool,
    Textual,
    None,
}

#[derive(Clone, Debug)]
pub struct Type {
    pub name: String,
    pub size: usize,
    pub kind: TypeKind,
    pub signed: Option<bool>,
}

impl Type {
    pub fn new(name: String, size: usize, kind: TypeKind, signed: Option<bool>) -> Type {
        Type {
            name,
            size,
            kind,
            signed,
        }
    }
}

impl PartialEq for Type {
    fn eq(&self, other: &Self) -> bool {
        let mut signed_eq = false;
        if self.signed.is_some() && other.signed.is_some() {
            let signed_self = self.signed.unwrap();
            let signed_other = other.signed.unwrap();
            signed_eq = signed_self == signed_other;
        }
        self.name == other.name && self.size == other.size && self.kind == other.kind && signed_eq
    }
}

#[derive(Debug, Clone)]
pub struct TaggedType {
    pub size: usize,
    pub kind: TypeKind,
    pub signed: Option<bool>,
}

impl TaggedType {
    pub fn new(size: usize, kind: TypeKind, signed: Option<bool>) -> TaggedType {
        TaggedType { size, kind, signed }
    }
}

impl Default for TaggedType {
    fn default() -> Self {
        TaggedType {
            size: 0,
            kind: TypeKind::None,
            signed: None,
        }
    }
}

impl From<Type> for TaggedType {
    fn from(t: Type) -> Self {
        Self::new(t.size, t.kind, t.signed)
    }
}

impl From<&Type> for TaggedType {
    fn from(t: &Type) -> Self {
        Self::new(t.size, t.kind, t.signed)
    }
}

type TypeMap = HashMap<String, Type>;
type LocalsMap = HashMap<String, Type>;

pub struct TypeCheck {
    types: TypeMap,
    locals: LocalsMap,
    created_locals: Option<Vec<String>>,
}

impl TypeCheck {
    pub fn new() -> TypeCheck {
        let mut container = TypeCheck {
            types: HashMap::new(),
            locals: HashMap::new(),
            created_locals: None,
        };

        container.create_type(Type::new(
            "i8".to_string(),
            1,
            TypeKind::Numeric,
            Some(true),
        ));
        container.create_type(Type::new(
            "u8".to_string(),
            1,
            TypeKind::Numeric,
            Some(false),
        ));
        container.create_type(Type::new(
            "i16".to_string(),
            2,
            TypeKind::Numeric,
            Some(true),
        ));
        container.create_type(Type::new(
            "u16".to_string(),
            2,
            TypeKind::Numeric,
            Some(false),
        ));
        container.create_type(Type::new(
            "i32".to_string(),
            4,
            TypeKind::Numeric,
            Some(true),
        ));
        container.create_type(Type::new(
            "u32".to_string(),
            4,
            TypeKind::Numeric,
            Some(false),
        ));
        container.create_type(Type::new("f32".to_string(), 4, TypeKind::Float, Some(true)));
        container.create_type(Type::new(
            "i64".to_string(),
            8,
            TypeKind::Numeric,
            Some(true),
        ));
        container.create_type(Type::new(
            "u64".to_string(),
            8,
            TypeKind::Numeric,
            Some(false),
        ));
        container.create_type(Type::new(
            "f64".to_string(),
            16,
            TypeKind::Float,
            Some(true),
        ));
        container.create_type(Type::new(
            "ptr".to_string(),
            8,
            TypeKind::Numeric,
            Some(false),
        ));
        container.create_type(Type::new("bool".to_string(), 1, TypeKind::Bool, None));
        container.create_type(Type::new("void".to_string(), 0, TypeKind::None, None));
        container
    }

    pub fn create_type(&mut self, tipe: Type) {
        self.types.insert(tipe.name.clone(), tipe);
    }

    pub fn resolve_type(&self, name: &String) -> Type {
        if let Some(tipe) = self.types.get(name) {
            tipe.clone()
        } else {
            error::panic(format!("Undefined reference to type: {}", name));
        }
    }

    pub fn resolve_local(&self, name: &String) -> Type {
        if let Some(tipe) = self.locals.get(name) {
            tipe.clone()
        } else {
            error::panic(format!("Undefined reference to variable: {}", name));
        }
    }

    pub fn check(&mut self, node: &mut Box<Node>) -> Type {
        match &mut **node {
            Node::Number(_, size, _, _) => {
                let tipe = self.resolve_type(&"i32".to_string());
                *size = (&tipe).into();
                tipe
            }
            Node::Float(_, size, _, _) => {
                let tipe = self.resolve_type(&"f64".to_string());
                *size = (&tipe).into();
                tipe
            }
            Node::BoolLiteral(_, size, _, _) => {
                let tipe = self.resolve_type(&"bool".to_string());
                *size = (&tipe).into();
                tipe
            }
            Node::ArrayLiteral(items, size, _, _) => {
                let tipe = self.check(&mut items[0]);
                *size = (&tipe).into();
                size.size = size.size * items.len();

                tipe
            }
            Node::StringLiteral(literal, _, _) => {
                self.create_type(Type::new(
                    "str".to_string(),
                    literal.len(),
                    TypeKind::Textual,
                    None,
                ));
                self.resolve_type(&"str".to_string())
            }
            Node::VarGet(name, _, _) => self.resolve_local(name),
            Node::Binary(binary) => {
                let l_type = self.check(&mut binary.lhs);
                let r_type = self.check(&mut binary.rhs);
                let ptr_type = self.resolve_type(&"ptr".to_string());

                if l_type == ptr_type && r_type.kind != TypeKind::Numeric {
                    error::panic_str("Cannot perform pointer arithmetic with non-numeric types");
                } else if r_type == ptr_type && l_type.kind != TypeKind::Numeric {
                    error::panic_str("Cannot perform pointer arithmetic with non-numeric types");
                } else if l_type != r_type && l_type != ptr_type && r_type != ptr_type {
                    error::panic_str("Binary expression has invalid operands");
                }

                match binary.op {
                    BinaryOp::Add | BinaryOp::Sub | BinaryOp::Mul | BinaryOp::Div => {
                        match l_type.kind {
                            TypeKind::Numeric | TypeKind::Float => (),
                            _ => error::panic_str("Cannot do arithmetic on non-numeric types"),
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
                let ret_type = match &func.ret_type_str {
                    Some(str) => str.clone(),
                    None => "void".to_string(),
                };

                let tipe = self.resolve_type(&ret_type);
                func.ret_type = (&tipe).into();

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
                if let Some(ex_dt) = &decl.dtype_str {
                    let mut ex_type = self.resolve_type(ex_dt);

                    if ex_type.kind == TypeKind::Numeric {
                        if ex_type.kind != val_type.kind {
                            error::panic_str(
                                "Explicit variable type, doesn't equal the value type",
                            );
                        }
                    } else if ex_type.kind == TypeKind::Float {
                        if ex_type.kind != val_type.kind {
                            error::panic_str(
                                "Explicit variable type, doesn't equal the value type",
                            );
                        }
                    } else {
                        if ex_type != val_type {
                            error::panic_str(
                                "Explicit variable type, doesn't equal the value type",
                            );
                        }
                    }

                    let mut array_size = 1;
                    match &*decl.value {
                        Node::ArrayLiteral(it, _, _, _) => array_size = it.len(),
                        _ => (),
                    }

                    ex_type.size = ex_type.size * array_size;
                    
                    decl.dtype = (&ex_type).into();
                    self.locals.insert(decl.name.clone(), ex_type.clone());
                    ex_type
                } else {
                    decl.dtype = (&val_type).into();
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
                    error::panic_str("Logical expression has invalid operands");
                }

                l_type
            }
            Node::Assign(assign) => {
                let local = self.resolve_local(&assign.name);
                let val_type = self.check(&mut assign.value);

                if val_type.kind == TypeKind::Numeric {
                    if val_type.kind != local.kind {
                        error::panic_str("Original variable type, doesn't equal the value type");
                    }
                } else if val_type.kind == TypeKind::Float {
                    if val_type.kind != local.kind {
                        error::panic_str("Original variable type, doesn't equal the value type");
                    }
                } else {
                    if val_type != local {
                        error::panic_str("Original variable type, doesn't equal the value type");
                    }
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
                    error::panic_str("If condition doesn't evaluate to a bool");
                }

                self.check(&mut if_stmt.then_block);

                if let Some(else_block) = &mut if_stmt.else_block {
                    self.check(else_block);
                }

                self.resolve_type(&"void".to_string())
            }
            Node::GetPtr(_) => self.resolve_type(&"ptr".to_string()),
            _ => todo!(),
        }
    }
}
