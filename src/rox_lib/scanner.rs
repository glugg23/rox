use std::fmt::{Display, Error, Formatter};

pub struct Scanner {
    source: Vec<char>,
    start: usize,
    current: usize,
    line: i32,
}

impl Scanner {
    pub fn new(source: &str) -> Self {
        Scanner {
            source: source.chars().collect(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_token(&mut self) -> Token {
        self.start = self.current;

        if self.is_at_end() {
            return Token::new(self, TokenType::EOF);
        }

        Token::new(self, TokenType::Error)
    }

    fn is_at_end(&self) -> bool {
        self.current == self.source.len()
    }
}

pub struct Token<'a> {
    pub token_type: TokenType,
    pub lexeme: &'a [char],
    pub line: i32,
}

impl<'a> Token<'a> {
    pub fn new(scanner: &'a Scanner, token_type: TokenType) -> Self {
        Token {
            token_type,
            lexeme: &scanner.source[scanner.start..scanner.current],
            line: scanner.line,
        }
    }
}

#[derive(PartialEq)]
pub enum TokenType {
    //Single-character tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    //One or two character tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    //Literals
    Identifier,
    String,
    Number,

    //Keywords
    And,
    Class,
    Else,
    False,
    For,
    Fun,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    //Other
    EOF,
    Error,
}

impl Display for TokenType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(
            f,
            "{}",
            match self {
                TokenType::LeftParen => "LeftParen",
                TokenType::RightParen => "RightParen",
                TokenType::LeftBrace => "LeftBrace",
                TokenType::RightBrace => "RightBrace",
                TokenType::Comma => "Comma",
                TokenType::Dot => "Dot",
                TokenType::Minus => "Minus",
                TokenType::Plus => "Plus",
                TokenType::Semicolon => "Semicolon",
                TokenType::Slash => "Slash",
                TokenType::Star => "Star",

                TokenType::Bang => "Bang",
                TokenType::BangEqual => "BangEqual",
                TokenType::Equal => "Equal",
                TokenType::EqualEqual => "EqualEqual",
                TokenType::Greater => "Greater",
                TokenType::GreaterEqual => "GreaterEqual",
                TokenType::Less => "Less",
                TokenType::LessEqual => "LessEqual",

                TokenType::Identifier => "Identifier",
                TokenType::String => "String",
                TokenType::Number => "Number",

                TokenType::And => "And",
                TokenType::Class => "Class",
                TokenType::Else => "Else",
                TokenType::False => "False",
                TokenType::For => "For",
                TokenType::Fun => "Fun",
                TokenType::If => "If",
                TokenType::Nil => "Nil",
                TokenType::Or => "Or",
                TokenType::Print => "Print",
                TokenType::Return => "Return",
                TokenType::Super => "Super",
                TokenType::This => "This",
                TokenType::True => "True",
                TokenType::Var => "Var",
                TokenType::While => "While",

                TokenType::EOF => "EOF",
                TokenType::Error => "Error",
            }
        )
    }
}
