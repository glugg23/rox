use crate::chunk::Chunk;
use crate::scanner::TokenType::EOF;
use crate::scanner::{Scanner, TokenType, Token};
use crate::RoxError;

pub struct Parser<'a> {
    current: Token<'a>,
    previous: Token<'a>,
}

impl Parser<'_> {
    pub fn new() -> Self {
        Parser {
            current: Token::default(),
            previous: Token::default(),
        }
    }
}

pub fn compile(source: &str) -> Result<Chunk, RoxError> {
    let mut scanner = Scanner::new(source);
    let mut parser = Parser::new();

    advance(&mut parser, &mut scanner);
    expression();
    consume(EOF, "Expect end of expression.");

    Ok(Chunk::new())
}

fn advance<'a>(parser: &'a mut Parser<'a>, scanner: &'a mut Scanner) {
    parser.previous = parser.current;

    loop {
        let token = scanner.scan_token();

        match token {
            Ok(t) => {
                parser.current = t;
                break;
            }
            Err(e) => report_error(e),
        }
    }
}

fn expression() {}

fn consume(token_type: TokenType, message: &str) {}

fn report_error(error: RoxError) {

}
