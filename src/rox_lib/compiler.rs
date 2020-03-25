use crate::chunk::{Chunk, OpCode};
use crate::debug::disassemble_chuck;
use crate::object::ObjectType;
use crate::scanner::TokenType::*;
use crate::scanner::{Scanner, Token, TokenType};
use crate::value::Value;
use crate::RoxError;
use std::ops::{AddAssign, SubAssign};
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

    pub fn check(&self, token_type: TokenType) -> bool {
        self.current.token_type == token_type
    }

    fn literal(&mut self) {
        match self.previous.token_type {
            False => self.emit_byte(OpCode::False as u8),
            Nil => self.emit_byte(OpCode::Nil as u8),
            True => self.emit_byte(OpCode::True as u8),
            _ => (),
        };
    }

    fn number(&mut self) {
        let value = f64::from_str(&self.previous.lexeme).unwrap();
        self.emit_constant(Value::Number(value));
    }

    fn string(&mut self) {
        self.emit_constant(Value::Object(ObjectType::String(Box::from(
            self.previous.lexeme.as_str()[1..self.previous.lexeme.len() - 1].to_owned(),
        ))));
    }

    fn variable(&mut self, scanner: &mut Scanner, compiler: &mut Compiler, can_assign: bool) {
        let name = self.previous.clone();
        self.named_variable(scanner, compiler, name, can_assign);
    }

    fn named_variable(
        &mut self,
        scanner: &mut Scanner,
        compiler: &mut Compiler,
        name: Token,
        can_assign: bool,
    ) {
        let get_op;
        let set_op;

        let mut arg = compiler.resolve_local(&name);

        if arg != None {
            get_op = OpCode::GetLocal;
            set_op = OpCode::SetLocal;
        } else {
            arg = Some(self.identifier_constant(name));
            get_op = OpCode::GetGlobal;
            set_op = OpCode::SetGlobal;
        }

        if can_assign && match_token(self, scanner, Equal) {
            expression(self, scanner, compiler);
            self.emit_bytes(set_op as u8, arg.unwrap());
        } else {
            self.emit_bytes(get_op as u8, arg.unwrap());
        }
    }

    fn unary(&mut self, scanner: &mut Scanner, compiler: &mut Compiler) {
        let operator_type = self.previous.token_type;

        self.parse_precedence(scanner, compiler, Precedence::Unary);

        match operator_type {
            Bang => self.emit_byte(OpCode::Not as u8),
            Minus => self.emit_byte(OpCode::Negate as u8),
            _ => (),
        }
    }

    fn binary(&mut self, scanner: &mut Scanner, compiler: &mut Compiler) {
        let operator_type = self.previous.token_type;

        self.parse_precedence(scanner, compiler, get_rule(operator_type).precedence.next());

        match operator_type {
            BangEqual => self.emit_bytes(OpCode::Equal as u8, OpCode::Not as u8),
            EqualEqual => self.emit_byte(OpCode::Equal as u8),
            Greater => self.emit_byte(OpCode::Greater as u8),
            GreaterEqual => self.emit_bytes(OpCode::Less as u8, OpCode::Not as u8),
            Less => self.emit_byte(OpCode::Less as u8),
            LessEqual => self.emit_bytes(OpCode::Greater as u8, OpCode::Not as u8),
            Plus => self.emit_byte(OpCode::Add as u8),
            Minus => self.emit_byte(OpCode::Subtract as u8),
            Star => self.emit_byte(OpCode::Multiple as u8),
            Slash => self.emit_byte(OpCode::Divide as u8),
            _ => (),
        }
    }

    fn parse_precedence(
        &mut self,
        scanner: &mut Scanner,
        compiler: &mut Compiler,
        precedence: Precedence,
    ) {
        advance(self, scanner);

        let prefix_rule = &get_rule(self.previous.token_type).prefix;

        let prefix_rule = match prefix_rule {
            Some(f) => f,
            None => {
                self.handle_error(RoxError::new(
                    "Expect expression.",
                    self.previous.lexeme.clone(),
                    self.previous.line,
                ));
                return;
            }
        };

        let can_assign = precedence <= Precedence::Assignment;
        prefix_rule(self, scanner, compiler, can_assign);

        while precedence <= get_rule(self.current.token_type).precedence {
            advance(self, scanner);
            let infix_rule = &get_rule(self.previous.token_type).infix.unwrap();
            infix_rule(self, scanner, compiler, can_assign);
        }

        if can_assign && match_token(self, scanner, Equal) {
            self.handle_error(RoxError::new(
                "Invalid assignment target.",
                self.previous.lexeme.clone(),
                self.previous.line,
            ));
        }
    }

    fn parse_variable(
        &mut self,
        scanner: &mut Scanner,
        compiler: &mut Compiler,
        error_message: &str,
    ) -> Option<u8> {
        consume(self, scanner, Identifier, error_message).unwrap_or_else(|e| {
            self.handle_error(e);
        });

        compiler.declare_variable(self).unwrap_or_else(|e| {
            self.handle_error(e);
        });

        if compiler.scope_depth > Depth::Global {
            None
        } else {
            let name = self.previous.clone();
            Some(self.identifier_constant(name))
        }
    }

    fn identifier_constant(&mut self, name: Token) -> u8 {
        self.make_constant(Value::Object(ObjectType::String(Box::new(name.lexeme))))
    }

    fn define_variable(&mut self, global: Option<u8>) {
        match global {
            Some(g) => self.emit_bytes(OpCode::DefineGlobal as u8, g),
            None => (), //No code needed at runtime for local variables
        }
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

    fn grouping(&mut self, scanner: &mut Scanner, compiler: &mut Compiler) {
        expression(self, scanner, compiler);
        consume(self, scanner, RightParen, "Expect ')' after expression.").unwrap_or_else(|e| {
            self.handle_error(e);
        });
    }

    pub fn end_compiler(&mut self) {
        self.emit_byte(OpCode::Return as u8);

        if cfg!(debug_assertions) {
            if !self.had_error {
                disassemble_chuck(&self.current_chunk, "code");
            }
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

pub struct Compiler {
    locals: Vec<Local>,
    scope_depth: Depth,
}

impl Compiler {
    pub fn new() -> Self {
        Compiler {
            locals: Vec::new(),
            scope_depth: Depth::Global,
        }
    }

    pub fn begin_scope(&mut self) {
        self.scope_depth += 1;
    }

    pub fn end_scope(&mut self, parser: &mut Parser) {
        self.scope_depth -= 1;

        //Would love to use self.locals.drain_filter() but it's nightly-only for now
        while self.locals.len() > 0 && self.locals[self.locals.len() - 1].depth > self.scope_depth {
            parser.emit_byte(OpCode::Pop as u8);
            self.locals.pop();
        }
    }

    pub fn declare_variable(&mut self, parser: &mut Parser) -> Result<(), RoxError> {
        //Global variables are implicitly declared.
        if self.scope_depth == Depth::Global {
            return Ok(());
        }

        let name = parser.previous.clone();

        for l in self.locals.iter().rev() {
            if l.depth != Depth::Uninitialised && l.depth < self.scope_depth {
                break;
            }

            if l.name.lexeme == name.lexeme {
                return Err(RoxError::new(
                    "Variable with this name already declared in this scope.",
                    name.lexeme.clone(),
                    name.line,
                ));
            }
        }

        self.add_local(name)
    }

    fn add_local(&mut self, name: Token) -> Result<(), RoxError> {
        return if self.locals.len() > std::u8::MAX as usize {
            Err(RoxError::new(
                "Too many local variables in function.",
                name.lexeme,
                name.line,
            ))
        } else {
            let local = Local {
                name,
                depth: self.scope_depth,
            };

            self.locals.push(local);
            Ok(())
        };
    }

    pub fn resolve_local(&self, name: &Token) -> Option<u8> {
        for (i, l) in self.locals.iter().enumerate().rev() {
            if l.name.lexeme == name.lexeme {
                return Some(i as u8);
            }
        }

        None
    }
}

struct Local {
    name: Token,
    depth: Depth,
}

#[derive(Copy, Clone, PartialOrd, PartialEq)]
enum Depth {
    Uninitialised,
    Global,
    Some(usize),
}

impl AddAssign<usize> for Depth {
    fn add_assign(&mut self, rhs: usize) {
        *self = match *self {
            Depth::Some(d) => Depth::Some(d + rhs),
            Depth::Global | Depth::Uninitialised => Depth::Some(rhs),
        }
    }
}

impl SubAssign<usize> for Depth {
    fn sub_assign(&mut self, rhs: usize) {
        *self = match *self {
            Depth::Some(d) if rhs >= d => Depth::Global,
            Depth::Some(d) => Depth::Some(d - rhs),
            Depth::Global | Depth::Uninitialised => Depth::Uninitialised,
        }
    }
}

pub fn compile(source: &str) -> Option<Chunk> {
    let mut scanner = Scanner::new(source);
    let mut parser = Parser::new();
    let mut compiler = Compiler::new();

    advance(&mut parser, &mut scanner);

    while !match_token(&mut parser, &mut scanner, EOF) {
        declaration(&mut parser, &mut scanner, &mut compiler);
    }

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

fn expression(parser: &mut Parser, scanner: &mut Scanner, compiler: &mut Compiler) {
    parser.parse_precedence(scanner, compiler, Precedence::Assignment);
}

fn block(parser: &mut Parser, scanner: &mut Scanner, compiler: &mut Compiler) {
    while !parser.check(RightBrace) && !parser.check(EOF) {
        declaration(parser, scanner, compiler);
    }

    consume(parser, scanner, RightBrace, "Expect '}' after block.").unwrap_or_else(|e| {
        parser.handle_error(e);
    });
}

fn declaration(parser: &mut Parser, scanner: &mut Scanner, compiler: &mut Compiler) {
    if match_token(parser, scanner, Var) {
        var_statement(parser, scanner, compiler);
    } else {
        statement(parser, scanner, compiler);
    }

    if parser.panic_mode {
        synchronise(parser, scanner);
    }
}

fn statement(parser: &mut Parser, scanner: &mut Scanner, compiler: &mut Compiler) {
    if match_token(parser, scanner, Print) {
        print_statement(parser, scanner, compiler);
    } else if match_token(parser, scanner, LeftBrace) {
        compiler.begin_scope();
        block(parser, scanner, compiler);
        compiler.end_scope(parser);
    } else {
        expression_statement(parser, scanner, compiler);
    }
}

fn var_statement(parser: &mut Parser, scanner: &mut Scanner, compiler: &mut Compiler) {
    let global = parser.parse_variable(scanner, compiler, "Expect variable name.");

    if match_token(parser, scanner, Equal) {
        expression(parser, scanner, compiler);
    } else {
        parser.emit_byte(OpCode::Nil as u8);
    }
    consume(
        parser,
        scanner,
        Semicolon,
        "Expect ';' variable declaration.",
    )
    .unwrap_or_else(|e| {
        parser.handle_error(e);
    });

    parser.define_variable(global);
}

fn print_statement(parser: &mut Parser, scanner: &mut Scanner, compiler: &mut Compiler) {
    expression(parser, scanner, compiler);
    consume(parser, scanner, Semicolon, "Expect ';' after value.").unwrap_or_else(|e| {
        parser.handle_error(e);
    });
    parser.emit_byte(OpCode::Print as u8);
}

fn expression_statement(parser: &mut Parser, scanner: &mut Scanner, compiler: &mut Compiler) {
    expression(parser, scanner, compiler);
    consume(parser, scanner, Semicolon, "Expect ';' after expression.").unwrap_or_else(|e| {
        parser.handle_error(e);
    });
    parser.emit_byte(OpCode::Pop as u8);
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

fn match_token(parser: &mut Parser, scanner: &mut Scanner, token_type: TokenType) -> bool {
    return if !parser.check(token_type) {
        false
    } else {
        advance(parser, scanner);
        true
    };
}

fn synchronise(parser: &mut Parser, scanner: &mut Scanner) {
    parser.panic_mode = false;

    while parser.current.token_type != EOF {
        if parser.previous.token_type == Semicolon {
            return;
        }

        match parser.current.token_type {
            Class | Fun | Var | For | If | While | Print | Return => return,
            _ => (),
        }

        advance(parser, scanner);
    }
}

#[derive(PartialOrd, PartialEq, Copy, Clone, Debug)]
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

impl Precedence {
    pub fn next(&self) -> Self {
        match self {
            Precedence::None => Precedence::Assignment,
            Precedence::Assignment => Precedence::Or,
            Precedence::Or => Precedence::And,
            Precedence::And => Precedence::Equality,
            Precedence::Equality => Precedence::Comparison,
            Precedence::Comparison => Precedence::Term,
            Precedence::Term => Precedence::Factor,
            Precedence::Factor => Precedence::Unary,
            Precedence::Unary => Precedence::Call,
            Precedence::Call => Precedence::Primary,
            Precedence::Primary => panic!("Can not get next precedence for Precedence::Primary"),
        }
    }
}

type ParseFn = Option<fn(&mut Parser, &mut Scanner, &mut Compiler, bool)>;

struct ParseRule {
    prefix: ParseFn,
    infix: ParseFn,
    precedence: Precedence,
}

fn get_rule(token_type: TokenType) -> &'static ParseRule {
    &RULES[token_type as usize]
}

const RULES: &'static [ParseRule] = &[
    //LeftParen
    ParseRule {
        prefix: Some(|p, s, c, _ca| p.grouping(s, c)),
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
        prefix: Some(|p, s, c, _ca| p.unary(s, c)),
        infix: Some(|p, s, c, _ca| p.binary(s, c)),
        precedence: Precedence::Term,
    },
    //Plus
    ParseRule {
        prefix: None,
        infix: Some(|p, s, c, _ca| p.binary(s, c)),
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
        infix: Some(|p, s, c, _ca| p.binary(s, c)),
        precedence: Precedence::Factor,
    },
    //Star
    ParseRule {
        prefix: None,
        infix: Some(|p, s, c, _ca| p.binary(s, c)),
        precedence: Precedence::Factor,
    },
    //Bang
    ParseRule {
        prefix: Some(|p, s, c, _ca| p.unary(s, c)),
        infix: None,
        precedence: Precedence::None,
    },
    //BangEqual
    ParseRule {
        prefix: None,
        infix: Some(|p, s, c, _ca| p.binary(s, c)),
        precedence: Precedence::Equality,
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
        infix: Some(|p, s, c, _ca| p.binary(s, c)),
        precedence: Precedence::Equality,
    },
    //Greater
    ParseRule {
        prefix: None,
        infix: Some(|p, s, c, _ca| p.binary(s, c)),
        precedence: Precedence::Comparison,
    },
    //GreaterEqual
    ParseRule {
        prefix: None,
        infix: Some(|p, s, c, _ca| p.binary(s, c)),
        precedence: Precedence::Comparison,
    },
    //Less
    ParseRule {
        prefix: None,
        infix: Some(|p, s, c, _ca| p.binary(s, c)),
        precedence: Precedence::Comparison,
    },
    //LessEqual
    ParseRule {
        prefix: None,
        infix: Some(|p, s, c, _ca| p.binary(s, c)),
        precedence: Precedence::Comparison,
    },
    //Identifier
    ParseRule {
        prefix: Some(|p, s, c, ca| p.variable(s, c, ca)),
        infix: None,
        precedence: Precedence::None,
    },
    //String
    ParseRule {
        prefix: Some(|p, _s, _c, _ca| p.string()),
        infix: None,
        precedence: Precedence::None,
    },
    //Number
    ParseRule {
        prefix: Some(|p, _s, _c, _ca| p.number()),
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
        prefix: Some(|p, _s, _c, _ca| p.literal()),
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
        prefix: Some(|p, _s, _c, _ca| p.literal()),
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
        prefix: Some(|p, _s, _c, _ca| p.literal()),
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compiler::Precedence::*;

    #[test]
    fn compiler_advance() {
        let mut parser = Parser::new();
        let mut scanner = Scanner::new("1");

        advance(&mut parser, &mut scanner);

        assert_eq!(parser.current.token_type, Number);
    }

    #[test]
    fn compiler_advance_with_error() {
        let mut parser = Parser::new();
        let mut scanner = Scanner::new("\"Hello World");

        advance(&mut parser, &mut scanner);

        assert!(parser.had_error);
        assert!(parser.panic_mode);
    }

    #[test]
    fn compiler_consume() {
        let mut parser = Parser::new();
        let mut scanner = Scanner::new("");
        parser.current = Token::new(&scanner, EOF);

        let result = consume(&mut parser, &mut scanner, EOF, "Error");

        assert!(result.is_ok());
    }

    #[test]
    fn compiler_consume_error() {
        let mut parser = Parser::new();
        let mut scanner = Scanner::new("");
        parser.current = Token::new(&scanner, Number);

        let result = consume(&mut parser, &mut scanner, EOF, "Error");

        assert!(result.is_err());
    }

    #[test]
    fn compiler_compile() {
        let result = compile("1 + 1;");

        assert!(result.is_some());
    }

    #[test]
    fn compiler_compile_with_error() {
        let result = compile("(-1");
        assert!(result.is_none());

        let result = compile("1 +");
        assert!(result.is_none());
    }

    #[test]
    fn precedence_next() {
        assert_eq!(None.next(), Assignment);
        assert_eq!(Assignment.next(), Precedence::Or);
        assert_eq!(Precedence::Or.next(), Precedence::And);
        assert_eq!(Precedence::And.next(), Equality);
        assert_eq!(Equality.next(), Comparison);
        assert_eq!(Comparison.next(), Term);
        assert_eq!(Term.next(), Factor);
        assert_eq!(Factor.next(), Unary);
        assert_eq!(Unary.next(), Call);
        assert_eq!(Call.next(), Primary);
    }

    #[test]
    #[should_panic(expected = "Can not get next precedence for Precedence::Primary")]
    fn precedence_next_error() {
        Primary.next();
    }

    #[test]
    fn parser_emit_byte() {
        let mut parser = Parser::new();

        parser.emit_byte(0);

        assert_eq!(parser.current_chunk.code[0], 0);
    }

    #[test]
    fn parser_emit_byte2() {
        let mut parser = Parser::new();

        parser.emit_bytes(0, 1);

        assert_eq!(parser.current_chunk.code[0], 0);
        assert_eq!(parser.current_chunk.code[1], 1);
    }

    #[test]
    fn parser_make_constant() {
        let mut parser = Parser::new();

        let result = parser.make_constant(Value::Number(1.0));

        assert_eq!(result, 0);
    }

    #[test]
    fn parser_make_constant_max_num() {
        let mut parser = Parser::new();
        parser.current_chunk.constants = vec![Value::Number(0.0); std::u8::MAX as usize + 1];

        parser.make_constant(Value::Number(1.0));

        assert!(parser.had_error);
    }

    #[test]
    fn parser_emit_constant() {
        let mut parser = Parser::new();

        parser.emit_constant(Value::Number(1.0));

        assert_eq!(parser.current_chunk.code[0], OpCode::Constant as u8);
        assert_eq!(parser.current_chunk.constants[0], Value::Number(1.0));
    }

    #[test]
    fn parser_end_compiler() {
        let mut parser = Parser::new();

        parser.end_compiler();

        assert_eq!(parser.current_chunk.code[0], OpCode::Return as u8);
    }
}
