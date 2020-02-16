use crate::chunk::Chunk;
use crate::scanner::TokenType::EOF;
use crate::scanner::{Scanner, Token, TokenType};
use crate::RoxError;

pub struct Parser {
    current: Token,
    previous: Token,
    had_error: bool,
    panic_mode: bool,
}

impl Parser {
    pub fn new() -> Self {
        Parser {
            current: Token::default(),
            previous: Token::default(),
            had_error: false,
            panic_mode: false,
        }
    }

    pub fn handle_error(&mut self, error: RoxError) {
        if !self.panic_mode {
            self.panic_mode = true;
            eprintln!("{}", error);
            self.had_error = true;
        }
    }
}

pub fn compile(source: &str) -> Option<Chunk> {
    let mut scanner = Scanner::new(source);
    let mut parser = Parser::new();

    advance(&mut parser, &mut scanner);
    expression();
    consume(&mut parser, &mut scanner, EOF, "Expect end of expression.").unwrap_or_else(|e| {
        parser.handle_error(e);
    });

    Some(Chunk::new())
}

fn advance(parser: &mut Parser, scanner: &mut Scanner) {
    parser.previous = parser.current.clone(); //Needed since String doesn't implement Copy

    loop {
        let token = scanner.scan_token();

        match token {
            Ok(t) => {
                parser.current = t;
                break;
            }
            Err(e) => {
                parser.handle_error(e);
            }
        }
    }
}

fn expression() {}

fn consume(
    parser: &mut Parser,
    scanner: &mut Scanner,
    token_type: TokenType,
    message: &str,
) -> Result<(), RoxError> {
    if parser.current.token_type == token_type {
        advance(parser, scanner);
        Ok(())
    } else {
        Err(RoxError::new(message, scanner.get_token(), scanner.line))
    }
}
