use crate::{
    ast::{BinaryOp, LogicalOp, UnaryOp},
    reg::RegisterLabel,
    typechecker::TaggedType,
};

#[derive(Debug, Clone)]
pub enum TmpChild {
    Literal(String, TaggedType),
    LoadVar(String, TaggedType),
    TmpRef(usize, TaggedType, Option<RegisterLabel>),
    None,
}

impl std::fmt::Display for TmpChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Literal(l, tipe) => write!(f, "{}{{{}}}", tipe, l),
            Self::LoadVar(var, tipe) => write!(f, "LOAD {}{{{}}}", tipe, var),
            Self::TmpRef(tmp, tipe, label) => {
                if let Some(l) = label {
                    write!(f, "{} -> {}{{tmp{}}}", l, tipe, tmp)
                } else {
                    write!(f, "{}{{tmp{}}}", tipe, tmp)
                }
            }
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
    GroupingTmp(GroupingTmp),
}

#[derive(Debug)]
pub struct BinaryTmp {
    pub lhs: TmpChild,
    pub rhs: TmpChild,
    pub op: BinaryOp,
    pub id: usize,
    pub tipe: TaggedType,
}

impl BinaryTmp {
    pub fn new(
        lhs: TmpChild,
        rhs: TmpChild,
        op: BinaryOp,
        id: usize,
        tipe: TaggedType,
    ) -> BinaryTmp {
        BinaryTmp {
            lhs,
            rhs,
            op,
            id,
            tipe,
        }
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

#[derive(Debug)]
pub struct GroupingTmp {
    pub expr: TmpChild,
    pub id: usize,
    pub tipe: TaggedType,
}

impl GroupingTmp {
    pub fn new(expr: TmpChild, tipe: TaggedType, id: usize) -> GroupingTmp {
        GroupingTmp { expr, id, tipe }
    }
}
