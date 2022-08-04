pub struct FunctionArg {
    dtype: String,
    loc: (usize, usize),
    name: String,
}

pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
}

pub enum Node {
    Signed8(i8, usize, usize),
    Signed16(i16, usize, usize),
    Signed32(i32, usize, usize),
    Signed64(i64, usize, usize),
    Signed128(i128, usize, usize),
    Unsigned8(u8, usize, usize),
    Unsigned16(u16, usize, usize),
    Unsigned32(u32, usize, usize),
    Unsigned64(u64, usize, usize),
    Unsigned128(u128, usize, usize),
    StringLiteral(String, usize, usize),
    Binary {
        lhs: Box<Node>,
        rhs: Box<Node>,
        op: BinaryOp,
    },
    Function {
        name: String,
        args: Vec<FunctionArg>,
        body: Vec<Box<Node>>,
        ret_type: String,
    },
    Call {
        args: Vec<Box<Node>>,
        callee: Box<Node>,
    },
    VarDecl {
        name: String,
        value: Box<Node>,
    },
}
