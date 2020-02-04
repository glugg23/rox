use crate::scanner::TokenType::*;
use crate::RoxError;
use std::fmt;
use std::fmt::{Display, Formatter};

macro_rules! two_char_token {
    ($scanner:ident, $token:expr, $first:path, $second:path) => {{
        let token_type = if $scanner.match_token($token) {
            $first
        } else {
            $second
        };

        return Ok(Token::new($scanner, token_type));
    }};
}

macro_rules! is_alpha {
    ($char:expr) => {
        $char.is_ascii_alphabetic() || $char == '_'
    };
}

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

    pub fn scan_token(&mut self) -> Result<Token, RoxError> {
        self.skip_whitespace();

        self.start = self.current;

        if self.is_at_end() {
            return Ok(Token::new(self, EOF));
        }

        let c = self.advance();

        if is_alpha!(c) {
            return Ok(self.identifier());
        }

        if c.is_ascii_digit() {
            return Ok(self.number());
        }

        match c {
            '(' => return Ok(Token::new(self, LeftParen)),
            ')' => return Ok(Token::new(self, RightParen)),
            '{' => return Ok(Token::new(self, LeftBrace)),
            '}' => return Ok(Token::new(self, RightBrace)),
            ';' => return Ok(Token::new(self, Semicolon)),
            ',' => return Ok(Token::new(self, Comma)),
            '.' => return Ok(Token::new(self, Dot)),
            '-' => return Ok(Token::new(self, Minus)),
            '+' => return Ok(Token::new(self, Plus)),
            '/' => return Ok(Token::new(self, Slash)),
            '*' => return Ok(Token::new(self, Star)),
            '!' => two_char_token!(self, '=', BangEqual, Bang),
            '=' => two_char_token!(self, '=', EqualEqual, Equal),
            '<' => two_char_token!(self, '=', LessEqual, Less),
            '>' => two_char_token!(self, '=', GreaterEqual, Greater),
            '"' => return self.string(),
            _ => (),
        }

        Err(RoxError::new("Unexpected character.", self.line))
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        self.source[self.current - 1]
    }

    fn match_token(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        if self.source[self.current] != expected {
            return false;
        }

        self.current += 1;
        true
    }

    fn string(&mut self) -> Result<Token, RoxError> {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            return Err(RoxError::new("Unterminated string.", self.line));
        }

        //Consume double quote
        self.advance();
        Ok(Token::new(self, RoxString))
    }

    fn number(&mut self) -> Token {
        while self.peek().is_ascii_digit() {
            self.advance();
        }

        //Look for fractional number
        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            //Consume dot
            self.advance();

            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }

        Token::new(self, Number)
    }

    fn identifier(&mut self) -> Token {
        while is_alpha!(self.peek()) || self.peek().is_ascii_digit() {
            self.advance();
        }

        let token_type = self.identifier_type();
        Token::new(self, token_type)
    }

    fn identifier_type(&mut self) -> TokenType {
        return match self.source[self.start] {
            'a' => self.check_keyword(1, "nd", And),
            'c' => self.check_keyword(1, "lass", Class),
            'e' => self.check_keyword(1, "lse", Else),
            'f' => {
                if self.current - self.start > 1 {
                    return match self.source[self.start + 1] {
                        'a' => self.check_keyword(2, "lse", False),
                        'o' => self.check_keyword(2, "r", For),
                        'u' => self.check_keyword(2, "n", Fun),
                        _ => Identifier,
                    };
                }

                return Identifier;
            }
            'i' => self.check_keyword(1, "f", If),
            'n' => self.check_keyword(1, "il", Nil),
            'o' => self.check_keyword(1, "r", Or),
            'p' => self.check_keyword(1, "rint", Print),
            'r' => self.check_keyword(1, "eturn", Return),
            's' => self.check_keyword(1, "uper", Super),
            't' => {
                if self.current - self.start > 1 {
                    return match self.source[self.start + 1] {
                        'h' => self.check_keyword(2, "is", This),
                        'r' => self.check_keyword(2, "ue", True),
                        _ => Identifier,
                    };
                }

                return Identifier;
            }
            'v' => self.check_keyword(1, "ar", Var),
            'w' => self.check_keyword(1, "hile", While),
            _ => Identifier,
        };
    }

    fn check_keyword(&self, start: usize, rest: &str, token_type: TokenType) -> TokenType {
        if self.current - self.start == start + rest.len() {
            let slice: String = self.source[self.start + start..self.start + start + rest.len()]
                .iter()
                .collect();
            if slice == rest.to_string() {
                return token_type;
            }
        }

        Identifier
    }

    fn skip_whitespace(&mut self) {
        loop {
            if self.is_at_end() {
                return;
            }

            let c = self.peek();
            match c {
                ' ' | '\r' | '\t' => {
                    self.advance();
                }
                '\n' => {
                    self.line += 1;
                    self.advance();
                }
                '/' => {
                    if self.peek_next() == '/' {
                        while self.peek() != '\n' && !self.is_at_end() {
                            self.advance();
                        }
                    } else {
                        return;
                    }
                }
                _ => return,
            }
        }
    }

    fn is_at_end(&self) -> bool {
        self.current == self.source.len() - 1
    }

    fn peek(&self) -> char {
        self.source[self.current]
    }

    fn peek_next(&self) -> char {
        if self.is_at_end() {
            '\0' //Maybe return None here
        } else {
            self.source[self.current + 1] //And Some(char) here
        }
    }
}

#[derive(Copy, Clone)]
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

impl Default for Token<'_> {
    fn default() -> Self {
        Token {
            token_type: EOF,
            lexeme: &[],
            line: 0
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
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
    RoxString,
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
}

impl Display for TokenType {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                LeftParen => "LeftParen",
                RightParen => "RightParen",
                LeftBrace => "LeftBrace",
                RightBrace => "RightBrace",
                Comma => "Comma",
                Dot => "Dot",
                Minus => "Minus",
                Plus => "Plus",
                Semicolon => "Semicolon",
                Slash => "Slash",
                Star => "Star",

                Bang => "Bang",
                BangEqual => "BangEqual",
                Equal => "Equal",
                EqualEqual => "EqualEqual",
                Greater => "Greater",
                GreaterEqual => "GreaterEqual",
                Less => "Less",
                LessEqual => "LessEqual",

                Identifier => "Identifier",
                RoxString => "String",
                Number => "Number",

                And => "And",
                Class => "Class",
                Else => "Else",
                False => "False",
                For => "For",
                Fun => "Fun",
                If => "If",
                Nil => "Nil",
                Or => "Or",
                Print => "Print",
                Return => "Return",
                Super => "Super",
                This => "This",
                True => "True",
                Var => "Var",
                While => "While",

                EOF => "EOF",
            }
        )
    }
}
