use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    // Keywords
    Use,
    New,
    Sprite,
    Stage,
    Implements,
    Code,
    On,
    Let,
    Set,
    Change,
    If,
    Else,
    Fn,
    True,
    False,

    // Types
    Int,
    Float,
    Bool,
    String,

    // Identifiers and literals
    Identifier(String),
    StringLiteral(String),
    IntLiteral(i64),
    FloatLiteral(f64),

    // Symbols
    LBrace,     // {
    RBrace,     // }
    LParen,     // (
    RParen,     // )
    LBracket,   // [
    RBracket,   // ]
    Colon,      // :
    ColonColon, // ::
    Semicolon,  // ;
    Comma,      // ,
    Dot,        // .
    Equals,     // =
    Plus,       // +
    Minus,      // -
    Star,       // *
    Slash,      // /
    Percent,    // %
    Bang,       // !
    Lt,         // <
    Gt,         // >
    LtEq,       // <=
    GtEq,       // >=
    EqEq,       // ==
    BangEq,     // !=
    AndAnd,     // &&
    OrOr,       // ||

    // Special
    Eof,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub line: usize,
    pub column: usize,
}

impl Token {
    pub fn new(kind: TokenKind, line: usize, column: usize) -> Self {
        Token { kind, line, column }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} at {}:{}", self.kind, self.line, self.column)
    }
}

pub struct Lexer {
    source: Vec<char>,
    pos: usize,
    line: usize,
    column: usize,
}

impl Lexer {
    pub fn new(source: &str) -> Self {
        Lexer {
            source: source.chars().collect(),
            pos: 0,
            line: 1,
            column: 1,
        }
    }

    fn current(&self) -> Option<char> {
        self.source.get(self.pos).copied()
    }

    fn peek(&self) -> Option<char> {
        self.source.get(self.pos + 1).copied()
    }

    fn advance(&mut self) -> Option<char> {
        let ch = self.current();
        if let Some(c) = ch {
            self.pos += 1;
            if c == '\n' {
                self.line += 1;
                self.column = 1;
            } else {
                self.column += 1;
            }
        }
        ch
    }

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.current() {
            if ch.is_whitespace() {
                self.advance();
            } else if ch == '/' && self.peek() == Some('/') {
                // Single-line comment
                while let Some(c) = self.current() {
                    if c == '\n' {
                        break;
                    }
                    self.advance();
                }
            } else if ch == '/' && self.peek() == Some('*') {
                // Multi-line comment
                self.advance(); // skip /
                self.advance(); // skip *
                while let Some(c) = self.current() {
                    if c == '*' && self.peek() == Some('/') {
                        self.advance(); // skip *
                        self.advance(); // skip /
                        break;
                    }
                    self.advance();
                }
            } else {
                break;
            }
        }
    }

    fn read_string(&mut self) -> Result<String, String> {
        let start_line = self.line;
        self.advance(); // skip opening quote

        let mut result = String::new();
        loop {
            match self.current() {
                Some('"') => {
                    self.advance();
                    return Ok(result);
                }
                Some('\\') => {
                    self.advance();
                    match self.current() {
                        Some('n') => result.push('\n'),
                        Some('t') => result.push('\t'),
                        Some('r') => result.push('\r'),
                        Some('\\') => result.push('\\'),
                        Some('"') => result.push('"'),
                        Some(c) => return Err(format!("Invalid escape sequence: \\{}", c)),
                        None => {
                            return Err(format!(
                                "Unterminated string starting at line {}",
                                start_line
                            ))
                        }
                    }
                    self.advance();
                }
                Some(c) => {
                    result.push(c);
                    self.advance();
                }
                None => {
                    return Err(format!(
                        "Unterminated string starting at line {}",
                        start_line
                    ))
                }
            }
        }
    }

    fn read_number(&mut self) -> TokenKind {
        let mut num_str = String::new();
        let mut is_float = false;

        while let Some(ch) = self.current() {
            if ch.is_ascii_digit() {
                num_str.push(ch);
                self.advance();
            } else if ch == '.' && !is_float {
                if let Some(next) = self.peek() {
                    if next.is_ascii_digit() {
                        is_float = true;
                        num_str.push(ch);
                        self.advance();
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        if is_float {
            TokenKind::FloatLiteral(num_str.parse().unwrap())
        } else {
            TokenKind::IntLiteral(num_str.parse().unwrap())
        }
    }

    fn read_identifier(&mut self) -> String {
        let mut ident = String::new();
        while let Some(ch) = self.current() {
            if ch.is_alphanumeric() || ch == '_' {
                ident.push(ch);
                self.advance();
            } else {
                break;
            }
        }
        ident
    }

    fn keyword_or_ident(&self, ident: &str) -> TokenKind {
        match ident {
            "use" => TokenKind::Use,
            "new" => TokenKind::New,
            "Sprite" => TokenKind::Sprite,
            "Stage" => TokenKind::Stage,
            "implements" => TokenKind::Implements,
            "Code" => TokenKind::Code,
            "on" => TokenKind::On,
            "let" => TokenKind::Let,
            "set" => TokenKind::Set,
            "change" => TokenKind::Change,
            "if" => TokenKind::If,
            "else" => TokenKind::Else,
            "fn" => TokenKind::Fn,
            "true" => TokenKind::True,
            "false" => TokenKind::False,
            "int" => TokenKind::Int,
            "float" => TokenKind::Float,
            "bool" => TokenKind::Bool,
            "string" => TokenKind::String,
            _ => TokenKind::Identifier(ident.to_string()),
        }
    }

    pub fn next_token(&mut self) -> Result<Token, String> {
        self.skip_whitespace();

        let line = self.line;
        let column = self.column;

        let kind = match self.current() {
            None => TokenKind::Eof,
            Some(ch) => match ch {
                '{' => {
                    self.advance();
                    TokenKind::LBrace
                }
                '}' => {
                    self.advance();
                    TokenKind::RBrace
                }
                '(' => {
                    self.advance();
                    TokenKind::LParen
                }
                ')' => {
                    self.advance();
                    TokenKind::RParen
                }
                '[' => {
                    self.advance();
                    TokenKind::LBracket
                }
                ']' => {
                    self.advance();
                    TokenKind::RBracket
                }
                ':' => {
                    self.advance();
                    if self.current() == Some(':') {
                        self.advance();
                        TokenKind::ColonColon
                    } else {
                        TokenKind::Colon
                    }
                }
                ';' => {
                    self.advance();
                    TokenKind::Semicolon
                }
                ',' => {
                    self.advance();
                    TokenKind::Comma
                }
                '.' => {
                    self.advance();
                    TokenKind::Dot
                }
                '+' => {
                    self.advance();
                    TokenKind::Plus
                }
                '-' => {
                    self.advance();
                    TokenKind::Minus
                }
                '*' => {
                    self.advance();
                    TokenKind::Star
                }
                '/' => {
                    self.advance();
                    TokenKind::Slash
                }
                '%' => {
                    self.advance();
                    TokenKind::Percent
                }
                '=' => {
                    self.advance();
                    if self.current() == Some('=') {
                        self.advance();
                        TokenKind::EqEq
                    } else {
                        TokenKind::Equals
                    }
                }
                '!' => {
                    self.advance();
                    if self.current() == Some('=') {
                        self.advance();
                        TokenKind::BangEq
                    } else {
                        TokenKind::Bang
                    }
                }
                '<' => {
                    self.advance();
                    if self.current() == Some('=') {
                        self.advance();
                        TokenKind::LtEq
                    } else {
                        TokenKind::Lt
                    }
                }
                '>' => {
                    self.advance();
                    if self.current() == Some('=') {
                        self.advance();
                        TokenKind::GtEq
                    } else {
                        TokenKind::Gt
                    }
                }
                '&' => {
                    self.advance();
                    if self.current() == Some('&') {
                        self.advance();
                        TokenKind::AndAnd
                    } else {
                        return Err(format!("Unexpected character '&' at {}:{}", line, column));
                    }
                }
                '|' => {
                    self.advance();
                    if self.current() == Some('|') {
                        self.advance();
                        TokenKind::OrOr
                    } else {
                        return Err(format!("Unexpected character '|' at {}:{}", line, column));
                    }
                }
                '"' => {
                    let s = self.read_string()?;
                    TokenKind::StringLiteral(s)
                }
                _ if ch.is_ascii_digit() => self.read_number(),
                _ if ch.is_alphabetic() || ch == '_' => {
                    let ident = self.read_identifier();
                    self.keyword_or_ident(&ident)
                }
                _ => {
                    return Err(format!(
                        "Unexpected character '{}' at {}:{}",
                        ch, line, column
                    ))
                }
            },
        };

        Ok(Token::new(kind, line, column))
    }
}

pub fn tokenize(source: &str) -> Result<Vec<Token>, String> {
    let mut lexer = Lexer::new(source);
    let mut tokens = Vec::new();

    loop {
        let token = lexer.next_token()?;
        let is_eof = token.kind == TokenKind::Eof;
        tokens.push(token);
        if is_eof {
            break;
        }
    }

    Ok(tokens)
}
