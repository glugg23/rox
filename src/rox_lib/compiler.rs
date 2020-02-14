use crate::chunk::Chunk;
use crate::scanner::TokenType::EOF;
use crate::scanner::{Scanner, Token, TokenType};

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
}

pub fn compile(source: &str) -> Option<Chunk> {
    let mut scanner = Scanner::new(source);
    let mut parser = Parser::new();

    advance(&mut parser, &mut scanner);
    expression();
    consume(EOF, "Expect end of expression.");

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
                if !parser.panic_mode {
                    parser.panic_mode = true;
                    eprintln!("{}", e);
                    parser.had_error = true;
                }
            }
        }
    }
}

fn expression() {}

fn consume(token_type: TokenType, message: &str) {}
