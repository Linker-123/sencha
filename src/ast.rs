#[derive(Debug)]
pub struct FunctionArg {
    dtype: String,
    loc: (usize, usize),
    name: String,
}

#[derive(Debug)]
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
    F32(f32, usize, usize),
    F64(f64, usize, usize),
    StringLiteral(String, usize, usize),
    BoolLiteral(bool, usize, usize),
    VarGet(String, usize, usize),
    Binary(Binary),
    Function(Function),
    Call(Call),
    VarDecl(VarDecl),
    Grouping(Grouping),
    Unary(Unary),
    Logical(Logical),
    Assign(Assign),
    For(For),
    If(If),
    Use(Use),
    Ret(Ret),
    Block(Block),
    ExprStmt(ExprStmt),
}

#[derive(PartialEq, Debug)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Greater,
    GreaterEq,
    Less,
    LessEq,
    Equal,
    NotEqual,
    None,
}

#[derive(PartialEq, Debug)]
pub enum LogicalOp {
    And,
    Or,
    None,
}

#[derive(PartialEq, Debug)]
pub enum UnaryOp {
    Not,
    Negate,
    None,
}

#[derive(Debug)]
pub struct Binary {
    lhs: Box<Node>,
    rhs: Box<Node>,
    op: BinaryOp,
}

impl Binary {
    pub fn new(lhs: Box<Node>, rhs: Box<Node>, op: BinaryOp) -> Box<Node> {
        Box::new(Node::Binary(Binary { lhs, rhs, op }))
    }
}

#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub loc: (usize, usize),
    pub args: Vec<FunctionArg>,
    pub body: Vec<Box<Node>>,
    pub ret_type: String,
}

impl Function {
    pub fn new(
        name: String,
        loc: (usize, usize),
        args: Vec<FunctionArg>,
        body: Vec<Box<Node>>,
        ret_type: String,
    ) -> Box<Node> {
        Box::new(Node::Function(Function {
            name,
            loc,
            args,
            body,
            ret_type,
        }))
    }
}

#[derive(Debug)]
pub struct Call {
    pub args: Vec<Box<Node>>,
    pub callee: Box<Node>,
}

impl Call {
    pub fn new(args: Vec<Box<Node>>, callee: Box<Node>) -> Box<Node> {
        Box::new(Node::Call(Call { args, callee }))
    }
}

#[derive(Debug)]
pub struct VarDecl {
    pub name: String,
    pub name_loc: (usize, usize),
    pub value: Box<Node>,
}

impl VarDecl {
    pub fn new(name: String, name_loc: (usize, usize), value: Box<Node>) -> Box<Node> {
        Box::new(Node::VarDecl(VarDecl {
            name,
            name_loc,
            value,
        }))
    }
}

#[derive(Debug)]
pub struct Grouping {
    pub expr: Box<Node>,
}

impl Grouping {
    pub fn new(expr: Box<Node>) -> Box<Node> {
        Box::new(Node::Grouping(Grouping { expr }))
    }
}

#[derive(Debug)]
pub struct Unary {
    pub op: UnaryOp,
    pub op_loc: (usize, usize),
    pub expr: Box<Node>,
}

impl Unary {
    pub fn new(op: UnaryOp, op_loc: (usize, usize), expr: Box<Node>) -> Box<Node> {
        Box::new(Node::Unary(Unary { op, op_loc, expr }))
    }
}

#[derive(Debug)]
pub struct Logical {
    pub lhs: Box<Node>,
    pub rhs: Box<Node>,
    pub op: LogicalOp,
}

impl Logical {
    pub fn new(lhs: Box<Node>, rhs: Box<Node>, op: LogicalOp) -> Box<Node> {
        Box::new(Node::Logical(Logical { lhs, rhs, op }))
    }
}

#[derive(Debug)]
pub struct Assign {
    pub name: String,
    pub name_loc: (usize, usize),
    pub value: Box<Node>,
}

impl Assign {
    pub fn new(name: String, name_loc: (usize, usize), value: Box<Node>) -> Box<Node> {
        Box::new(Node::Assign(Assign {
            name,
            name_loc,
            value,
        }))
    }
}

#[derive(Debug)]
pub struct For {
    pub name: String,
    pub name_loc: (usize, usize),
    pub target: Box<Node>,
    pub body: Box<Node>,
}

impl For {
    pub fn new(
        name: String,
        name_loc: (usize, usize),
        target: Box<Node>,
        body: Box<Node>,
    ) -> Box<Node> {
        Box::new(Node::For(For {
            name,
            name_loc,
            target,
            body,
        }))
    }
}

#[derive(Debug)]
pub struct If {
    pub condition: Box<Node>,
    pub then_block: Box<Node>,
    pub else_block: Box<Node>,
}

impl If {
    pub fn new(condition: Box<Node>, then_block: Box<Node>, else_block: Box<Node>) -> Box<Node> {
        Box::new(Node::If(If {
            condition,
            then_block,
            else_block,
        }))
    }
}

#[derive(Debug)]
pub struct Use {
    pub module: Box<Node>,
    pub item: String,
    pub loc: (usize, usize),
}

impl Use {
    pub fn new(module: Box<Node>, item: String, loc: (usize, usize)) -> Box<Node> {
        Box::new(Node::Use(Use { module, item, loc }))
    }
}

#[derive(Debug)]
pub struct Ret {
    pub value: Option<Box<Node>>,
    pub loc: (usize, usize),
}

impl Ret {
    pub fn new(value: Option<Box<Node>>, loc: (usize, usize)) -> Box<Node> {
        Box::new(Node::Ret(Ret { value, loc }))
    }
}

#[derive(Debug)]
pub struct Block {
    pub statements: Vec<Box<Node>>,
}

impl Block {
    pub fn new(statements: Vec<Box<Node>>) -> Box<Node> {
        Box::new(Node::Block(Block { statements }))
    }
}

#[derive(Debug)]
pub struct ExprStmt {
    pub expr: Box<Node>,
}

impl ExprStmt {
    pub fn new(expr: Box<Node>) -> Box<Node> {
        Box::new(Node::ExprStmt(ExprStmt { expr }))
    }
}
