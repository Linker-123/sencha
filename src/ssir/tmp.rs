use crate::{ast::{BinaryOp, LogicalOp, UnaryOp}, typechecker::TaggedType};

#[derive(Debug)]
pub enum TmpChild {
    Literal(String, TaggedType),
    LoadVar(String),
    TmpRef(usize),
    None,
}

impl std::fmt::Display for TmpChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Literal(l, size) => write!(f, "{}{{{}}}", size, l),
            Self::LoadVar(var) => write!(f, "LOAD {}", var),
            Self::TmpRef(tmp) => write!(f, "REF tmp{}", tmp),
            Self::None => unreachable!(),
        }
    }
}

#[derive(Debug)]
pub enum TmpNode {
    BinaryTmp(BinaryTmp),
    ValueTmp(ValueTmp),
    UnaryTmp(UnaryTmp),
    LogicalTmp(LogicalTmp),
    AssignTmp(AssignTmp),
}

#[derive(Debug)]
pub struct BinaryTmp {
    pub lhs: TmpChild,
    pub rhs: TmpChild,
    pub op: BinaryOp,
    pub id: usize,
}

impl BinaryTmp {
    pub fn new(lhs: TmpChild, rhs: TmpChild, op: BinaryOp, id: usize) -> BinaryTmp {
        BinaryTmp { lhs, rhs, op, id }
    }
}

#[derive(Debug)]
pub struct ValueTmp {
    pub value: TmpChild,
    pub id: usize,
}

impl ValueTmp {
    pub fn new(value: TmpChild, id: usize) -> ValueTmp {
        ValueTmp { value, id }
    }
}

#[derive(Debug)]
pub struct UnaryTmp {
    pub value: TmpChild,
    pub op: UnaryOp,
    pub id: usize,
}

impl UnaryTmp {
    pub fn new(value: TmpChild, op: UnaryOp, id: usize) -> UnaryTmp {
        UnaryTmp { value, op, id }
    }
}

#[derive(Debug)]
pub struct LogicalTmp {
    pub lhs: TmpChild,
    pub rhs: TmpChild,
    pub op: LogicalOp,
    pub id: usize,
}

impl LogicalTmp {
    pub fn new(lhs: TmpChild, rhs: TmpChild, op: LogicalOp, id: usize) -> LogicalTmp {
        LogicalTmp { lhs, rhs, op, id }
    }
}

#[derive(Debug)]
pub struct AssignTmp {
    pub value: TmpChild,
    pub id: usize,
}

impl AssignTmp {
    pub fn new(value: TmpChild, id: usize) -> AssignTmp {
        AssignTmp { value, id }
    }
}
