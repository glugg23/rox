use crate::scanner::TokenType::*;
use crate::RoxError;

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

/*
TODO: Investigate using Peekable<Chars>, instead of Vec<char> for Scanner.source
Could use take_while(self, P) to implement advance(self)
Then return Cow::borrowed(str) instead of String since we never mutate the source code
*/

pub struct Scanner {
    source: Vec<char>,
    start: usize,
    current: usize,
    pub line: i32,
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

        Err(RoxError::new(
            "Unexpected character.",
            self.get_token(),
            self.line,
        ))
    }

    pub fn get_token(&self) -> String {
        self.source[self.start..self.current].iter().collect()
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        self.source[self.current - 1]
    }

    pub fn match_token(&mut self, expected: char) -> bool {
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
        while self.peek() != Some('"') && !self.is_at_end() {
            if self.peek() == Some('\n') {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            return Err(RoxError::new(
                "Unterminated string.",
                self.get_token(),
                self.line,
            ));
        }

        //Consume double quote
        self.advance();
        Ok(Token::new(self, RoxString))
    }

    fn number(&mut self) -> Token {
        while matches!(self.peek(), Some(c) if c.is_ascii_digit()) {
            self.advance();
        }

        //Look for fractional number
        if self.peek() == Some('.') && matches!(self.peek_next(), Some(c) if c.is_ascii_digit()) {
            //Consume dot
            self.advance();

            while matches!(self.peek(), Some(c) if c.is_ascii_digit()) {
                self.advance();
            }
        }

        Token::new(self, Number)
    }

    fn identifier(&mut self) -> Token {
        while matches!(self.peek(), Some(c) if is_alpha!(c) || c.is_ascii_digit()) {
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

            if let Some(c) = self.peek() {
                match c {
                    ' ' | '\r' | '\t' => {
                        self.advance();
                    }
                    '\n' => {
                        self.line += 1;
                        self.advance();
                    }
                    '/' => {
                        if self.peek_next() == Some('/') {
                            while self.peek() != Some('\n') && !self.is_at_end() {
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
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn peek(&self) -> Option<char> {
        self.source.get(self.current).map(|&c| c)
    }

    fn peek_next(&self) -> Option<char> {
        self.source.get(self.current + 1).map(|&c| c)
    }
}

#[derive(Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: i32,
}

impl Token {
    pub fn new(scanner: &Scanner, token_type: TokenType) -> Self {
        Token {
            token_type,
            lexeme: scanner.get_token(),
            line: scanner.line,
        }
    }
}

impl Default for Token {
    fn default() -> Self {
        Token {
            token_type: EOF,
            lexeme: String::new(),
            line: 0,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scanner_advance() {
        let mut scanner = Scanner::new("1");

        let result = scanner.advance();

        assert_eq!(result, '1');
    }

    #[test]
    fn scanner_get_token() {
        let mut scanner = Scanner::new("123");
        scanner.advance();
        scanner.advance();

        let result = scanner.get_token();

        assert_eq!(result, "12");
    }

    #[test]
    fn scanner_peek() {
        let scanner = Scanner::new("1");

        let result = scanner.peek();

        assert_eq!(result, Some('1'));
    }

    #[test]
    fn scanner_is_at_end() {
        let mut scanner = Scanner::new("1");
        assert_eq!(scanner.is_at_end(), false);

        scanner.advance();
        assert_eq!(scanner.is_at_end(), true);
    }

    #[test]
    fn scanner_peek_next() {
        let scanner = Scanner::new("12");

        let result = scanner.peek_next();

        assert_eq!(result, Some('2'));
    }

    #[test]
    fn scanner_peek_next_when_at_end() {
        let scanner = Scanner::new("1");

        let result = scanner.peek_next();

        assert_eq!(result, None);
    }

    #[test]
    fn scanner_skip_whitespace() {
        let mut scanner = Scanner::new(" \t\r1");

        scanner.skip_whitespace();

        assert_eq!(scanner.current, 3);
    }

    #[test]
    fn scanner_skip_whitespace_counts_newlines() {
        let mut scanner = Scanner::new("\n\n1");

        scanner.skip_whitespace();

        assert_eq!(scanner.current, 2);
        assert_eq!(scanner.line, 3);
    }

    #[test]
    fn scanner_skip_whitespace_ignores_comments() {
        let mut scanner = Scanner::new("//Hello world");

        scanner.skip_whitespace();

        assert_eq!(scanner.current, 13);
    }

    #[test]
    fn scanner_skip_whitespace_ignores_comments_ending_in_new_line() {
        let mut scanner = Scanner::new("//Hello world\n1");

        scanner.skip_whitespace();

        assert_eq!(scanner.current, 14);
        assert_eq!(scanner.line, 2);
    }

    #[test]
    fn scanner_skip_whitespace_does_not_skip_division() {
        let mut scanner = Scanner::new("/ 0");

        scanner.skip_whitespace();

        assert_eq!(scanner.current, 0);
    }

    #[test]
    fn scanner_number() {
        let mut scanner = Scanner::new("1");

        let result = scanner.number();

        assert_eq!(result.token_type, Number);
        assert_eq!(result.lexeme, "1");
    }

    #[test]
    fn scanner_fractional_number() {
        let mut scanner = Scanner::new("1.5");

        let result = scanner.number();

        assert_eq!(result.token_type, Number);
        assert_eq!(result.lexeme, "1.5");
    }

    #[test]
    fn scanner_number_ignores_dot_if_not_number() {
        let mut scanner = Scanner::new("1.half");

        let result = scanner.number();

        assert_eq!(result.token_type, Number);
        assert_eq!(result.lexeme, "1");
    }

    #[test]
    fn scanner_number_ignores_dot_if_nothing_after() {
        let mut scanner = Scanner::new("1.");

        let result = scanner.number();

        assert_eq!(result.token_type, Number);
        assert_eq!(result.lexeme, "1");
    }

    #[test]
    fn scanner_string() {
        let mut scanner = Scanner::new("\"Hello World\"");
        scanner.advance();

        let result = scanner.string().unwrap();

        assert_eq!(result.token_type, RoxString);
        assert_eq!(result.lexeme, "\"Hello World\"");
    }

    #[test]
    fn scanner_string_multiline() {
        let mut scanner = Scanner::new("\"Hello\nWorld\"");
        scanner.advance();

        let result = scanner.string().unwrap();

        assert_eq!(result.token_type, RoxString);
        assert_eq!(result.lexeme, "\"Hello\nWorld\"");
        assert_eq!(scanner.line, 2)
    }

    #[test]
    fn scanner_string_unterminated() {
        let mut scanner = Scanner::new("\"Hello World");
        scanner.advance();

        let result = scanner.string();

        assert!(result.is_err());
    }

    #[test]
    fn scanner_match_token() {
        let mut scanner = Scanner::new("1");

        let result = scanner.match_token('1');

        assert!(result);
    }

    #[test]
    fn scanner_match_token_not_expected() {
        let mut scanner = Scanner::new("1");

        let result = scanner.match_token('2');

        assert_eq!(result, false);
    }

    #[test]
    fn scanner_match_token_when_at_end() {
        let mut scanner = Scanner::new("");

        let result = scanner.match_token('1');

        assert_eq!(result, false);
    }

    #[test]
    fn scanner_identifier() {
        let mut scanner = Scanner::new("foo");

        let result = scanner.identifier();

        assert_eq!(result.token_type, Identifier);
        assert_eq!(result.lexeme, "foo");
    }

    #[test]
    fn scanner_identify_keywords() {
        let mut scanner = Scanner::new("and");
        let result = scanner.identifier();
        assert_eq!(result.token_type, And);

        let mut scanner = Scanner::new("class");
        let result = scanner.identifier();
        assert_eq!(result.token_type, Class);

        let mut scanner = Scanner::new("else");
        let result = scanner.identifier();
        assert_eq!(result.token_type, Else);

        let mut scanner = Scanner::new("false");
        let result = scanner.identifier();
        assert_eq!(result.token_type, False);

        let mut scanner = Scanner::new("for");
        let result = scanner.identifier();
        assert_eq!(result.token_type, For);

        let mut scanner = Scanner::new("fun");
        let result = scanner.identifier();
        assert_eq!(result.token_type, Fun);

        let mut scanner = Scanner::new("if");
        let result = scanner.identifier();
        assert_eq!(result.token_type, If);

        let mut scanner = Scanner::new("nil");
        let result = scanner.identifier();
        assert_eq!(result.token_type, Nil);

        let mut scanner = Scanner::new("or");
        let result = scanner.identifier();
        assert_eq!(result.token_type, Or);

        let mut scanner = Scanner::new("print");
        let result = scanner.identifier();
        assert_eq!(result.token_type, Print);

        let mut scanner = Scanner::new("return");
        let result = scanner.identifier();
        assert_eq!(result.token_type, Return);

        let mut scanner = Scanner::new("super");
        let result = scanner.identifier();
        assert_eq!(result.token_type, Super);

        let mut scanner = Scanner::new("this");
        let result = scanner.identifier();
        assert_eq!(result.token_type, This);

        let mut scanner = Scanner::new("true");
        let result = scanner.identifier();
        assert_eq!(result.token_type, True);

        let mut scanner = Scanner::new("var");
        let result = scanner.identifier();
        assert_eq!(result.token_type, Var);

        let mut scanner = Scanner::new("while");
        let result = scanner.identifier();
        assert_eq!(result.token_type, While);
    }

    #[test]
    fn scanner_scan_identifier() {
        let mut scanner = Scanner::new("bar");

        let result = scanner.scan_token().unwrap();

        assert_eq!(result.token_type, Identifier);
        assert_eq!(result.lexeme, "bar");
    }

    #[test]
    fn scanner_scan_with_whitespace() {
        let mut scanner = Scanner::new("    baz");

        let result = scanner.scan_token().unwrap();

        assert_eq!(result.token_type, Identifier);
        assert_eq!(result.lexeme, "baz");
    }

    #[test]
    fn scanner_scan_number() {
        let mut scanner = Scanner::new("42");

        let result = scanner.scan_token().unwrap();

        assert_eq!(result.token_type, Number);
        assert_eq!(result.lexeme, "42");
    }

    #[test]
    fn scanner_scan_string() {
        let mut scanner = Scanner::new("\"Hello World\"");

        let result = scanner.scan_token().unwrap();

        assert_eq!(result.token_type, RoxString);
        assert_eq!(result.lexeme, "\"Hello World\"");
    }

    #[test]
    fn scanner_scan_at_end() {
        let mut scanner = Scanner::new("");

        let result = scanner.scan_token().unwrap();

        assert_eq!(result.token_type, EOF);
    }

    #[test]
    fn scanner_scan_unexpected_character() {
        let mut scanner = Scanner::new("🤔");

        let result = scanner.scan_token();

        assert!(result.is_err());
    }

    #[test]
    fn scanner_scan_one_character_tokens() {
        let mut scanner = Scanner::new("(");
        let result = scanner.scan_token().unwrap();
        assert_eq!(result.token_type, LeftParen);

        let mut scanner = Scanner::new(")");
        let result = scanner.scan_token().unwrap();
        assert_eq!(result.token_type, RightParen);

        let mut scanner = Scanner::new("{");
        let result = scanner.scan_token().unwrap();
        assert_eq!(result.token_type, LeftBrace);

        let mut scanner = Scanner::new("}");
        let result = scanner.scan_token().unwrap();
        assert_eq!(result.token_type, RightBrace);

        let mut scanner = Scanner::new(";");
        let result = scanner.scan_token().unwrap();
        assert_eq!(result.token_type, Semicolon);

        let mut scanner = Scanner::new(",");
        let result = scanner.scan_token().unwrap();
        assert_eq!(result.token_type, Comma);

        let mut scanner = Scanner::new(".");
        let result = scanner.scan_token().unwrap();
        assert_eq!(result.token_type, Dot);

        let mut scanner = Scanner::new("-");
        let result = scanner.scan_token().unwrap();
        assert_eq!(result.token_type, Minus);

        let mut scanner = Scanner::new("+");
        let result = scanner.scan_token().unwrap();
        assert_eq!(result.token_type, Plus);

        let mut scanner = Scanner::new("/");
        let result = scanner.scan_token().unwrap();
        assert_eq!(result.token_type, Slash);

        let mut scanner = Scanner::new("*");
        let result = scanner.scan_token().unwrap();
        assert_eq!(result.token_type, Star);

        let mut scanner = Scanner::new("!");
        let result = scanner.scan_token().unwrap();
        assert_eq!(result.token_type, Bang);

        let mut scanner = Scanner::new("=");
        let result = scanner.scan_token().unwrap();
        assert_eq!(result.token_type, Equal);

        let mut scanner = Scanner::new("<");
        let result = scanner.scan_token().unwrap();
        assert_eq!(result.token_type, Less);

        let mut scanner = Scanner::new(">");
        let result = scanner.scan_token().unwrap();
        assert_eq!(result.token_type, Greater);
    }

    #[test]
    fn scanner_scan_two_character_tokens() {
        let mut scanner = Scanner::new("!=");
        let result = scanner.scan_token().unwrap();
        assert_eq!(result.token_type, BangEqual);

        let mut scanner = Scanner::new("==");
        let result = scanner.scan_token().unwrap();
        assert_eq!(result.token_type, EqualEqual);

        let mut scanner = Scanner::new("<=");
        let result = scanner.scan_token().unwrap();
        assert_eq!(result.token_type, LessEqual);

        let mut scanner = Scanner::new(">=");
        let result = scanner.scan_token().unwrap();
        assert_eq!(result.token_type, GreaterEqual);
    }
}
