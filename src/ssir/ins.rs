use super::tmp::{TmpChild, TmpNode};

#[derive(Debug)]
pub enum Instruction {
    TmpNode(TmpNode),
    VarDecl(String, TmpChild),
    VarAssign(String, TmpChild),
    Pop
}
