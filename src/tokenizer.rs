#[derive(Debug)]
pub enum TokenKind {
    LeftBrace(usize, usize),
    RightBrace(usize, usize),
    LeftParen(usize, usize),
    RightParen(usize, usize),
    NumLiteral(String, usize, usize),
    StrLiteral(String, usize, usize),
    // IdenLiteral(String, usize, usize),
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

    // fn consume(&mut self, expected: char) -> bool {
    //     if self.is_at_end() {
    //         return false;
    //     }

    //     let c = self.source.chars().nth(self.current);
    //     if let Some(c) = c {
    //         if c != expected {
    //             return false;
    //         }
    //     }

    //     self.current += 1;
    //     self.column += 1;
    //     true
    // }

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
                '\n' => {
                    self.line += 1;
                    self.column = 0;
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

        if self.peek() == Some('.') && is_digit_opt(self.peek_next()) {
            self.advance();
            while is_digit_opt(self.peek()) {
                self.advance();
            }
        }

        return TokenKind::NumLiteral(
            self.source
                .chars()
                .skip(self.start)
                .take(self.current - self.start)
                .collect(),
            self.line,
            self.column,
        );
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

        if is_digit(c) {
            return Some(self.number());
        }

        match c {
            '(' => Some(TokenKind::LeftParen(self.line, self.current)),
            ')' => Some(TokenKind::RightParen(self.line, self.current)),
            '{' => Some(TokenKind::LeftBrace(self.line, self.current)),
            '}' => Some(TokenKind::RightBrace(self.line, self.current)),
            '"' => Some(self.string()),
            _ => None,
        }
    }
}
