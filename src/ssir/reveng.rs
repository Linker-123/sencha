use crate::ast::{BinaryOp, Node};

pub fn reverse_binary(node: &mut Box<Node>) {
    match &mut **node {
        Node::Binary(binary) => {
            reverse_binary(&mut binary.lhs);
            reverse_binary(&mut binary.rhs);

            match binary.op {
                BinaryOp::Equal => {
                    binary.op = BinaryOp::NotEqual;
                }
                BinaryOp::NotEqual => {
                    binary.op = BinaryOp::Equal;
                }
                BinaryOp::Greater => {
                    binary.op = BinaryOp::LessEq;
                }
                BinaryOp::GreaterEq => {
                    binary.op = BinaryOp::Less;
                }
                BinaryOp::Less => {
                    binary.op = BinaryOp::GreaterEq;
                }
                BinaryOp::LessEq => {
                    binary.op = BinaryOp::Greater;
                }
                _ => (),
            }
        }
        _ => (),
    }
}
