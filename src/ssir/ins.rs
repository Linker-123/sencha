use super::tmp;

#[derive(Debug)]
pub enum Instruction {
    TmpNode(tmp::TmpNode),
    VarDecl(String, tmp::TmpChild),
}
