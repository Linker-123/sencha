use crate::ast::BinaryOp;

#[derive(Debug)]
pub enum TmpChild {
    Literal(String),
    LoadVar(String),
    TmpRef(usize),
    None
}

#[derive(Debug)]
pub enum TmpNode {
    BinaryTmp(BinaryTmp),
    UnoTmp(UnoTmp),
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
pub struct UnoTmp {
    pub value: TmpChild,
    pub id: usize,
}

impl UnoTmp {
    pub fn new(value: TmpChild, id: usize) -> UnoTmp {
        UnoTmp { value, id }
    }
}
