use crate::chunk::{Chunk, OpCode};
use crate::scanner::TokenType::*;
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

    fn unary(&mut self, _scanner: &mut Scanner) {
        let operator_type = self.previous.token_type;

        self.parse_precedence(Precedence::Unary);

        match operator_type {
            Minus => self.emit_byte(OpCode::Negate as u8),
            _ => (),
        }
    }

    fn binary(&mut self, _scanner: &mut Scanner) {
        let operator_type = self.previous.token_type;

        self.parse_precedence(get_next_rule(operator_type).precedence);

        match operator_type {
            Plus => self.emit_byte(OpCode::Add as u8),
            Minus => self.emit_byte(OpCode::Subtract as u8),
            Star => self.emit_byte(OpCode::Multiple as u8),
            Slash => self.emit_byte(OpCode::Divide as u8),
            _ => (),
        }
    }

    fn parse_precedence(&mut self, precedence: Precedence) {}

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
        expression(self);
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
    expression(&mut parser);
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

fn expression(parser: &mut Parser) {
    parser.parse_precedence(Precedence::Assignment);
}

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

#[derive(PartialOrd, PartialEq, Copy, Clone)]
enum Precedence {
    None,
    Assignment, // =
    Or,         // or
    And,        // and
    Equality,   // == !=
    Comparison, // < > <= >=
    Term,       // + -
    Factor,     // * /
    Unary,      // ! -
    Call,       // . ()
    Primary,
}

type ParseFn = Option<fn(&mut Parser, &mut Scanner)>;

struct ParseRule {
    prefix: ParseFn,
    infix: ParseFn,
    precedence: Precedence,
}

fn get_rule(token_type: TokenType) -> &'static ParseRule {
    &RULES[token_type as usize]
}

fn get_next_rule(token_type: TokenType) -> &'static ParseRule {
    &RULES[(token_type as usize) + 1]
}

const RULES: &'static [ParseRule] = &[
    //LeftParen
    ParseRule {
        prefix: Some(|p, s| p.grouping(s)),
        infix: None,
        precedence: Precedence::None,
    },
    //RightParen
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    //LeftBrace
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    //RightBrace
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    //Comma
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    //Dot
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    //Minus
    ParseRule {
        prefix: Some(|p, s| p.unary(s)),
        infix: Some(|p, s| p.binary(s)),
        precedence: Precedence::Term,
    },
    //Plus
    ParseRule {
        prefix: None,
        infix: Some(|p, s| p.binary(s)),
        precedence: Precedence::Term,
    },
    //SemiColon
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    //Slash
    ParseRule {
        prefix: None,
        infix: Some(|p, s| p.binary(s)),
        precedence: Precedence::Factor,
    },
    //Star
    ParseRule {
        prefix: None,
        infix: Some(|p, s| p.binary(s)),
        precedence: Precedence::Factor,
    },
    //Bang
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    //BangEqual
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    //Equal
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    //EqualEqual
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    //Greater
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    //GreaterEqual
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    //Less
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    //LessEqual
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    //Identifier
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    //String
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    //Number
    ParseRule {
        prefix: Some(|p, _s| p.number()),
        infix: None,
        precedence: Precedence::None,
    },
    //And
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    //Class
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    //Else
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    //False
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    //For
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    //Fun
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    //If
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    //Nil
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    //Or
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    //Print
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    //Return
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    //Super
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    //This
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    //True
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    //Var
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    //While
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    //EOF
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
];
