use crate::{
    ast::{
        Assign, Binary, BinaryOp, Block, ExprStmt, For, Logical, LogicalOp, Node, Ret, Unary,
        UnaryOp,
    },
    tokenizer::{get_tok_loc, TokenKind, Tokenizer},
};

macro_rules! matches {
    ($self: ident, $($tts:tt)*) => {
        if std::matches!($($tts)*) {
            $self.advance();
            true
        } else {
            false
        }
    };
}

macro_rules! consume {
    ($self: ident, $msg: expr, $($tts:tt)*) => {{
        if !matches!($self, $($tts)*) {
            panic!("{}", $msg);
        }
    }};
}

pub struct Parser<'a> {
    tokenizer: Tokenizer<'a>,
    current: TokenKind,
    pub declarations: Vec<Box<Node>>,
}

impl<'a> Parser<'a> {
    pub fn new(mut tokenizer: Tokenizer<'a>) -> Parser {
        let current = tokenizer.next().unwrap();
        Parser {
            tokenizer,
            current,
            declarations: Default::default(),
        }
    }

    pub fn parse(&mut self) {
        while !self.is_at_end() {
            let declaration = self.declaration();
            if let Some(decl) = declaration {
                self.declarations.push(decl);
            }
        }
    }

    fn declaration(&mut self) -> Option<Box<Node>> {
        self.statement()
    }

    fn statement(&mut self) -> Option<Box<Node>> {
        if matches!(self, self.current, TokenKind::ExprDelimiter(_, _)) {
            return None;
        }

        let loc = get_tok_loc(&self.current);
        if matches!(self, self.current, TokenKind::LeftBrace(_, _)) {
            return Some(Block::new(self.block()));
        }
        if matches!(self, self.current, TokenKind::Ret(_, _)) {
            return Some(self.ret_stmt(loc));
        }
        if matches!(self, self.current, TokenKind::For(_, _)) {
            return Some(self.for_stmt());
        }

        Some(self.expr_stmt())
    }

    fn ret_stmt(&mut self, loc: (usize, usize)) -> Box<Node> {
        let mut expr = None;
        if !std::matches!(self.current, TokenKind::ExprDelimiter(_, _)) {
            expr = Some(self.expr());
        }

        consume!(
            self,
            "Expected a ';' or a new line.",
            self.current,
            TokenKind::ExprDelimiter(_, _)
        );
        Ret::new(expr, loc)
    }

    fn for_stmt(&mut self) -> Box<Node> {
        let name;
        let name_loc;
        if let TokenKind::IdenLiteral(n, line, column) = &self.current {
            name = n.clone();
            name_loc = (*line, *column);
            self.advance();
        } else {
            let (line, column) = get_tok_loc(&self.current);
            panic!("Expected an identitfier at {}:{}", line, column)
        }

        {
            let (line, column) = get_tok_loc(&self.current);
            consume!(
                self,
                format!("Expected keyword 'in' at {}:{}", line, column),
                self.current,
                TokenKind::In(_, _)
            );
        }

        let target = self.expr();
        {
            let (line, column) = get_tok_loc(&self.current);
            consume!(
                self,
                format!("Expected a '{{' at {}:{}", line, column),
                self.current,
                TokenKind::LeftBrace(_, _)
            );
        }
        let body = self.block();
        For::new(name, name_loc, target, Block::new(body))
    }

    fn block(&mut self) -> Vec<Box<Node>> {
        let mut statements: Vec<Box<Node>> = Vec::with_capacity(10);
        while !std::matches!(self.current, TokenKind::RightBrace(_, _)) && !self.is_at_end() {
            let declaration = self.declaration();
            if let Some(decl) = declaration {
                statements.push(decl);
            }
        }

        consume!(
            self,
            "Expected an '}'",
            self.current,
            TokenKind::RightBrace(_, _)
        );

        statements
    }

    fn expr_stmt(&mut self) -> Box<Node> {
        let expr = self.expr();
        consume!(
            self,
            "Expected a ';' or a new line.",
            self.current,
            TokenKind::ExprDelimiter(_, _)
        );
        ExprStmt::new(expr)
    }

    fn expr(&mut self) -> Box<Node> {
        self.assignment()
    }

    fn assignment(&mut self) -> Box<Node> {
        let expr = self.or();
        if matches!(self, self.current, TokenKind::Equal(_, _)) {
            let value = self.assignment();

            match expr.as_ref() {
                Node::VarGet(name, line, column) => {
                    return Assign::new(name.to_string(), (*line, *column), value);
                }
                _ => panic!("Invalid target for assignment"),
            }
        }

        expr
    }

    fn or(&mut self) -> Box<Node> {
        let mut expr = self.and();
        loop {
            let lop;

            if matches!(self, self.current, TokenKind::Or(_, _)) {
                lop = LogicalOp::Or;
            } else {
                break;
            }

            let right = self.and();
            expr = Logical::new(expr, right, lop);
        }
        expr
    }

    fn and(&mut self) -> Box<Node> {
        let mut expr = self.equality();
        loop {
            let lop;

            if matches!(self, self.current, TokenKind::And(_, _)) {
                lop = LogicalOp::And;
            } else {
                break;
            }

            let right = self.equality();
            expr = Logical::new(expr, right, lop);
        }
        expr
    }

    fn equality(&mut self) -> Box<Node> {
        let mut expr = self.comparison();
        loop {
            let bop;

            if matches!(self, self.current, TokenKind::NotEqual(_, _)) {
                bop = BinaryOp::NotEqual;
            } else if matches!(self, self.current, TokenKind::EqualEqual(_, _)) {
                bop = BinaryOp::Equal;
            } else {
                break;
            }

            let right = self.comparison();
            expr = Binary::new(expr, right, bop);
        }
        expr
    }

    fn comparison(&mut self) -> Box<Node> {
        let mut expr = self.term();
        loop {
            let bop;

            if matches!(self, self.current, TokenKind::Greater(_, _)) {
                bop = BinaryOp::Greater;
            } else if matches!(self, self.current, TokenKind::GreaterEq(_, _)) {
                bop = BinaryOp::GreaterEq;
            } else if matches!(self, self.current, TokenKind::Less(_, _)) {
                bop = BinaryOp::Less;
            } else if matches!(self, self.current, TokenKind::LessEq(_, _)) {
                bop = BinaryOp::LessEq;
            } else {
                break;
            }

            let right = self.term();
            expr = Binary::new(expr, right, bop);
        }
        expr
    }

    fn term(&mut self) -> Box<Node> {
        let mut expr = self.factor();
        loop {
            let bop;

            if matches!(self, self.current, TokenKind::Plus(_, _)) {
                bop = BinaryOp::Add;
            } else if matches!(self, self.current, TokenKind::Minus(_, _)) {
                bop = BinaryOp::Sub;
            } else {
                break;
            }

            let right = self.factor();
            expr = Binary::new(expr, right, bop);
        }
        expr
    }

    fn factor(&mut self) -> Box<Node> {
        let mut expr = self.unary();
        loop {
            let bop;

            if matches!(self, self.current, TokenKind::Slash(_, _)) {
                bop = BinaryOp::Div;
            } else if matches!(self, self.current, TokenKind::Star(_, _)) {
                bop = BinaryOp::Mul;
            } else {
                break;
            }

            let right = self.unary();
            expr = Binary::new(expr, right, bop);
        }
        expr
    }

    fn unary(&mut self) -> Box<Node> {
        let mut uop = UnaryOp::None;
        let mut loc = (0, 0);

        if matches!(self, self.current, TokenKind::Bang(_, _)) {
            uop = UnaryOp::Not;
            loc = get_tok_loc(&self.current);
        } else if matches!(self, self.current, TokenKind::Minus(_, _)) {
            uop = UnaryOp::Negate;
            loc = get_tok_loc(&self.current);
        }

        if uop != UnaryOp::None {
            let expr = self.unary();
            return Unary::new(uop, loc, expr);
        }

        self.primary()
    }

    fn primary(&mut self) -> Box<Node> {
        let tok = self.current.clone();
        let node = match tok {
            TokenKind::True(line, column) => Node::BoolLiteral(true, line, column),
            TokenKind::False(line, column) => Node::BoolLiteral(false, line, column),
            TokenKind::IntLiteral(integer, line, column) => match integer.parse::<i32>() {
                Ok(n) => Node::Signed32(n, line, column),
                Err(e) => panic!("Couldn't parse i32 {} at {}:{}", e, line, column),
            },
            TokenKind::FloatLiteral(float, line, column) => match float.parse::<f64>() {
                Ok(n) => Node::F64(n, line, column),
                Err(e) => panic!("Couldn't parse f64 {} at {}:{}", e, line, column),
            },
            TokenKind::StrLiteral(string, line, column) => {
                Node::StringLiteral(string, line, column)
            }
            TokenKind::IdenLiteral(ident, line, column) => Node::VarGet(ident, line, column),
            TokenKind::LeftParen(_, _) => {
                todo!();
            }
            _ => {
                panic!("Unexpected token: {:#?}", tok);
            }
        };

        self.advance();
        Box::new(node)
    }

    fn advance(&mut self) {
        self.current = self.tokenizer.next().unwrap_or_else(|| TokenKind::Eof);
    }

    fn is_at_end(&mut self) -> bool {
        std::matches!(self.current, TokenKind::Eof)
    }
}
