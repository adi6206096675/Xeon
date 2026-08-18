#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Import, From, Component, Return, Let, Identifier(String), StringLiteral(String), NumberLiteral(f64),
    OpenParen, CloseParen, OpenBrace, CloseBrace, OpenBracket, CloseBracket,
    AngleOpen, AngleClose, Slash, Assign, Arrow, Comma, Semicolon, Eof, Illegal(char),
}

// NEW: Couples the token with its exact location in the file
#[derive(Debug, Clone)]
pub struct TokenSpan {
    pub token: Token,
    pub line: usize,
    pub col: usize,
}

#[derive(Clone)]
pub struct Lexer<'a> {
    input: std::iter::Peekable<std::str::Chars<'a>>,
    pub line: usize,
    pub col: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self { 
            input: source.chars().peekable(),
            line: 1,
            col: 1,
        }
    }

    // NEW: Safely advances the iterator while updating spatial coordinates
    fn next_char(&mut self) -> Option<char> {
        let ch = self.input.next();
        if let Some(c) = ch {
            if c == '\n' {
                self.line += 1;
                self.col = 1;
            } else {
                self.col += 1;
            }
        }
        ch
    }

    pub fn peek_token(&self) -> TokenSpan {
        let mut cloned = self.clone();
        cloned.next_token()
    }

    fn skip_whitespace(&mut self) {
        while let Some(&c) = self.input.peek() {
            if c.is_whitespace() { self.next_char(); } 
            else { break; }
        }
    }

    fn read_identifier(&mut self, first: char) -> String {
        let mut ident = String::from(first);
        while let Some(&c) = self.input.peek() {
            if c.is_alphanumeric() || c == '_' { ident.push(self.next_char().unwrap()); } 
            else { break; }
        }
        ident
    }

    pub fn next_token(&mut self) -> TokenSpan {
        self.skip_whitespace();
        let start_line = self.line;
        let start_col = self.col;

        let ch = match self.next_char() {
            Some(c) => c,
            None => return TokenSpan { token: Token::Eof, line: start_line, col: start_col },
        };

        let token = match ch {
            '(' => Token::OpenParen, ')' => Token::CloseParen,
            '{' => Token::OpenBrace, '}' => Token::CloseBrace,
            '[' => Token::OpenBracket, ']' => Token::CloseBracket,
            '<' => Token::AngleOpen, '>' => Token::AngleClose,
            '/' => Token::Slash,
            ',' => Token::Comma, ';' => Token::Semicolon,
            '=' => {
                if let Some(&'>') = self.input.peek() {
                    self.next_char();
                    Token::Arrow
                } else { Token::Assign }
            }
            '"' => {
                let mut string = String::new();
                while let Some(c) = self.next_char() {
                    if c == '"' { break; }
                    string.push(c);
                }
                Token::StringLiteral(string)
            }
            _ if ch.is_alphabetic() => {
                let ident = self.read_identifier(ch);
                match ident.as_str() {
                    "import" => Token::Import, "from" => Token::From,
                    "component" => Token::Component, "return" => Token::Return,
                    "let" => Token::Let, _ => Token::Identifier(ident),
                }
            }
            _ if ch.is_numeric() => Token::NumberLiteral(0.0),
            _ => Token::Illegal(ch),
        };

        TokenSpan { token, line: start_line, col: start_col }
    }
}