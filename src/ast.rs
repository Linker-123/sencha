#[derive(Debug)]
pub struct FunctionArg {
    pub name: String,
    pub name_loc: (usize, usize),
    pub dtype: String,
    pub size: usize,
}

impl FunctionArg {
    pub fn new(name: String, name_loc: (usize, usize), dtype: String) -> FunctionArg {
        FunctionArg {
            name,
            name_loc,
            dtype,
            size: 0,
        }
    }
}

#[derive(Debug)]
pub enum Node {
    Number(String, usize, usize, usize),
    Float(String, usize, usize, usize),
    StringLiteral(String, usize, usize),
    BoolLiteral(bool, usize, usize, usize),
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
    pub lhs: Box<Node>,
    pub rhs: Box<Node>,
    pub op: BinaryOp,
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
    pub body: Box<Node>,
    pub ret_type: Option<String>,
    pub ret_size: usize,
}

impl Function {
    pub fn new(
        name: String,
        loc: (usize, usize),
        args: Vec<FunctionArg>,
        body: Box<Node>,
        ret_type: Option<String>,
    ) -> Box<Node> {
        Box::new(Node::Function(Function {
            name,
            loc,
            args,
            body,
            ret_type,
            ret_size: 0,
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
    pub dtype: Option<String>,
    pub dtype_size: usize,
    pub value: Box<Node>,
}

impl VarDecl {
    pub fn new(
        name: String,
        name_loc: (usize, usize),
        dtype: Option<String>,
        value: Box<Node>,
    ) -> Box<Node> {
        Box::new(Node::VarDecl(VarDecl {
            name,
            name_loc,
            dtype,
            value,
            dtype_size: 0,
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
    pub else_block: Option<Box<Node>>,
}

impl If {
    pub fn new(
        condition: Box<Node>,
        then_block: Box<Node>,
        else_block: Option<Box<Node>>,
    ) -> Box<Node> {
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
