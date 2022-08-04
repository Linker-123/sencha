#[derive(Debug, Clone)]
pub enum TokenKind {
    LeftBrace(usize, usize),
    RightBrace(usize, usize),
    LeftParen(usize, usize),
    RightParen(usize, usize),
    IntLiteral(String, usize, usize),
    FloatLiteral(String, usize, usize),
    StrLiteral(String, usize, usize),
    IdenLiteral(String, usize, usize),
    Func(usize, usize),
    Mod(usize, usize),
    Use(usize, usize),
    Ret(usize, usize),
    Arrow(usize, usize),
    Colon(usize, usize),
    ColonEq(usize, usize),
    Comma(usize, usize),
    Dot(usize, usize),
    Plus(usize, usize),
    Minus(usize, usize),
    Star(usize, usize),
    Slash(usize, usize),
    True(usize, usize),
    False(usize, usize),
    Bang(usize, usize),
    NewLine(usize, usize),
    Eof,
}

pub fn get_tok_loc(token: &TokenKind) -> (usize, usize) {
    match token {
        TokenKind::LeftBrace(a, b) => (*a, *b),
        TokenKind::RightBrace(a, b) => (*a, *b),
        TokenKind::LeftParen(a, b) => (*a, *b),
        TokenKind::RightParen(a, b) => (*a, *b),
        TokenKind::IntLiteral(_, a, b) => (*a, *b),
        TokenKind::FloatLiteral(_, a, b) => (*a, *b),
        TokenKind::StrLiteral(_, a, b) => (*a, *b),
        TokenKind::IdenLiteral(_, a, b) => (*a, *b),
        TokenKind::Func(a, b) => (*a, *b),
        TokenKind::Mod(a, b) => (*a, *b),
        TokenKind::Use(a, b) => (*a, *b),
        TokenKind::Ret(a, b) => (*a, *b),
        TokenKind::Arrow(a, b) => (*a, *b),
        TokenKind::Colon(a, b) => (*a, *b),
        TokenKind::ColonEq(a, b) => (*a, *b),
        TokenKind::Comma(a, b) => (*a, *b),
        TokenKind::Dot(a, b) => (*a, *b),
        TokenKind::Plus(a, b) => (*a, *b),
        TokenKind::Minus(a, b) => (*a, *b),
        TokenKind::Star(a, b) => (*a, *b),
        TokenKind::Slash(a, b) => (*a, *b),
        TokenKind::True(a, b) => (*a, *b),
        TokenKind::False(a, b) => (*a, *b),
        TokenKind::Bang(a, b) => (*a, *b),
        TokenKind::NewLine(a, b) => (*a, *b),
        TokenKind::Eof => panic!("Unsupported token"),
    }
}

fn is_digit(c: char) -> bool {
    c >= '0' && c <= '9'
}

fn is_digit_opt(c: Option<char>) -> bool {
    if let Some(c) = c {
        return c >= '0' && c <= '9';
    }
    false
}

fn is_alpha(c: char) -> bool {
    (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_'
}

fn is_alpha_opt(c: Option<char>) -> bool {
    if let Some(c) = c {
        return (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_';
    }
    false
}

pub struct Tokenizer<'a> {
    current: usize,
    start: usize,
    line: usize,
    column: usize,
    source: &'a String,
}

impl<'a> Tokenizer<'a> {
    pub fn new(source: &'a String) -> Tokenizer {
        Tokenizer {
            current: 0,
            start: 0,
            line: 1,
            column: 1,
            source,
        }
    }

    fn is_at_end(&mut self) -> bool {
        if self.current == self.source.len() {
            true
        } else {
            false
        }
    }

    fn advance(&mut self) -> Option<char> {
        self.current += 1;
        self.column += 1;
        self.source.chars().nth(self.current - 1)
    }

    fn matches(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        let c = self.source.chars().nth(self.current);
        if let Some(c) = c {
            if c != expected {
                return false;
            }
        }

        self.current += 1;
        self.column += 1;
        true
    }

    fn peek(&mut self) -> Option<char> {
        self.source.chars().nth(self.current)
    }

    fn peek_next(&mut self) -> Option<char> {
        if self.is_at_end() {
            return None;
        }
        self.source.chars().nth(self.current + 1)
    }

    fn skip_whitespace(&mut self) {
        loop {
            let c = match self.peek() {
                Some(c) => c,
                None => return,
            };
            match c {
                ' ' | '\r' | '\t' => {
                    self.advance();
                }
                '/' => {
                    if self.peek_next() == Some('/') {
                        while self.peek() != Some('\n') && !self.is_at_end() {
                            self.advance();
                        }
                    }
                }
                _ => return,
            }
        }
    }

    fn string(&mut self) -> TokenKind {
        while self.peek() != Some('"') && !self.is_at_end() {
            if self.peek() == Some('\n') {
                self.line += 1;
                self.column = 0;
            }
            self.advance();
        }

        if self.is_at_end() {
            panic!("Unterminated string at {}:{}", self.line, self.current);
        }

        self.advance();
        TokenKind::StrLiteral(
            self.source
                .chars()
                .skip(self.start + 1)
                .take(self.current - self.start - 2)
                .collect(),
            self.line,
            self.column,
        )
    }

    fn number(&mut self) -> TokenKind {
        while is_digit_opt(self.peek()) {
            self.advance();
        }

        let mut is_float = false;
        if self.peek() == Some('.') && is_digit_opt(self.peek_next()) {
            self.advance();
            while is_digit_opt(self.peek()) {
                self.advance();
            }
            is_float = true;
        }

        let raw = self
            .source
            .chars()
            .skip(self.start)
            .take(self.current - self.start)
            .collect();
        if is_float {
            TokenKind::FloatLiteral(raw, self.line, self.column)
        } else {
            TokenKind::IntLiteral(raw, self.line, self.column)
        }
    }

    fn identifier(&mut self) -> TokenKind {
        while is_alpha_opt(self.peek()) || is_digit_opt(self.peek()) {
            self.advance();
        }

        let identifier = self
            .source
            .chars()
            .skip(self.start)
            .take(self.current - self.start)
            .collect::<String>();
        match identifier.as_str() {
            "func" => return TokenKind::Func(self.line, self.column),
            "mod" => return TokenKind::Mod(self.line, self.column),
            "use" => return TokenKind::Use(self.line, self.column),
            "ret" => return TokenKind::Ret(self.line, self.column),
            "true" => return TokenKind::True(self.line, self.column),
            "false" => return TokenKind::False(self.line, self.column),
            _ => (),
        }

        TokenKind::IdenLiteral(identifier, self.line, self.column)
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = TokenKind;

    fn next(&mut self) -> Option<Self::Item> {
        self.skip_whitespace();
        self.start = self.current;
        if self.is_at_end() {
            return None;
        }

        let c = match self.advance() {
            Some(c) => c,
            None => return None,
        };

        if is_alpha(c) {
            return Some(self.identifier());
        }
        if is_digit(c) {
            return Some(self.number());
        }

        match c {
            '(' => Some(TokenKind::LeftParen(self.line, self.column)),
            ')' => Some(TokenKind::RightParen(self.line, self.column)),
            '{' => Some(TokenKind::LeftBrace(self.line, self.column)),
            '}' => Some(TokenKind::RightBrace(self.line, self.column)),
            ':' => Some(if self.matches('=') {
                TokenKind::ColonEq(self.line, self.column)
            } else {
                TokenKind::Colon(self.line, self.column)
            }),
            '.' => Some(TokenKind::Dot(self.line, self.column)),
            ',' => Some(TokenKind::Comma(self.line, self.column)),
            '+' => Some(TokenKind::Plus(self.line, self.column)),
            '-' => Some(if self.matches('>') {
                TokenKind::Arrow(self.line, self.column)
            } else {
                TokenKind::Minus(self.line, self.column)
            }),
            '*' => Some(TokenKind::Star(self.line, self.column)),
            '/' => Some(TokenKind::Slash(self.line, self.column)),
            '"' => Some(self.string()),
            '!' => Some(TokenKind::Bang(self.line, self.column)),
            '\n' => {
                self.line += 1;
                self.column = 0;
                Some(TokenKind::NewLine(self.line, self.column))
            }
            _ => None,
        }
    }
}
