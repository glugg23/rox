use crate::chunk::{Chunk, OpCode};
use crate::scanner::TokenType::{RightParen, EOF};
use crate::scanner::{Scanner, Token, TokenType};
use crate::{RoxError, Value};
use std::str::FromStr;

pub struct Parser {
    current: Token,
    previous: Token,
    current_chunk: Chunk,
    had_error: bool,
    panic_mode: bool,
}

impl Parser {
    pub fn new() -> Self {
        Parser {
            current: Token::default(),
            previous: Token::default(),
            current_chunk: Chunk::new(),
            had_error: false,
            panic_mode: false,
        }
    }

    pub fn emit_byte(&mut self, byte: u8) {
        self.current_chunk.write(byte, self.previous.line);
    }

    pub fn emit_bytes(&mut self, byte1: u8, byte2: u8) {
        self.emit_byte(byte1);
        self.emit_byte(byte2);
    }

    fn number(&mut self) {
        let value = f64::from_str(&self.previous.lexeme).unwrap(); //TODO: Don't use unwrap here
        self.emit_constant(value);
    }

    fn emit_constant(&mut self, value: Value) {
        let constant = self.make_constant(value);
        self.emit_bytes(OpCode::Constant as u8, constant);
    }

    fn make_constant(&mut self, value: Value) -> u8 {
        let constant = self.current_chunk.add_constant(value);

        return if constant > std::u8::MAX as usize {
            self.handle_error(RoxError::new(
                "Too many constants in one chunk.",
                self.previous.lexeme.clone(),
                self.previous.line,
            ));
            0
        } else {
            constant as u8
        };
    }

    fn grouping(&mut self, scanner: &mut Scanner) {
        expression();
        consume(self, scanner, RightParen, "Expect ')' after expression.").unwrap_or_else(|e| {
            self.handle_error(e);
        });
    }

    pub fn end_compiler(&mut self) {
        self.emit_byte(OpCode::Return as u8);
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

    parser.end_compiler();

    return if !parser.had_error {
        Some(parser.current_chunk)
    } else {
        None
    };
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
