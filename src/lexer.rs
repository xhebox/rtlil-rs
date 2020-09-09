// Copyright (c) 2020 xhe

//! Lexer for RTLIL files.

use super::syntax::*;
use std::fmt;

pub type Error = anyhow::Error;

#[derive(Debug, Clone)]
pub struct Location {
    offset: usize,
    line: u32,
    column: u32,
}

impl Default for Location {
    fn default() -> Self {
        Self {
            offset: 0,
            line: 1,
            column: 1,
        }
    }
}

impl Location {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn bump(&mut self) {
        self.offset += 1;
        self.column += 1;
    }

    pub fn bump_line(&mut self) {
        self.line += 1;
        self.column = 1;
    }
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "line {} column {} (offset {})",
            self.line, self.column, self.offset
        )
    }
}

#[derive(Debug, Clone)]
pub enum Token {
    Ident(String),
    Literal(String),
    Int(i64),
    Signal(Signal),
    LBracket,
    RBracket,
    LBrace,
    RBrace,
    Comma,
    Colon,

    Autoidx,
    Module,
    Attribute,
    Parameter,
    Signed,
    Real,
    Wire,
    Memory,
    Width,
    Upto,
    Offset,
    Size,
    Input,
    Output,
    Inout,
    Cell,
    Connect,
    Switch,
    Case,
    Assign,
    Sync,
    Low,
    High,
    Posedge,
    Negedge,
    Edge,
    Always,
    Global,
    Init,
    Update,
    Process,
    End,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// A lexer for RTLIL files.
pub struct Lexer<I> {
    input: I,
    peek: [Option<char>; 2],
    loc: Location,
}

impl<I: Iterator<Item = char>> Lexer<I> {
    /// Create a new lexer.
    pub fn new(input: I) -> Self {
        let mut lexer = Self {
            input,
            peek: [None, None],
            loc: Location::new(),
        };
        lexer.bump();
        lexer.bump();
        lexer
    }

    /// Advance the lexer to the next character.
    fn bump(&mut self) {
        self.peek[0] = self.peek[1];
        self.peek[1] = self.input.next();
        self.loc.bump();
    }
}

impl<I: Iterator<Item = char>> Iterator for Lexer<I> {
    type Item = Result<(Location, Token, Location), Error>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let loc = self.loc.clone();
            match (self.peek[0], self.peek[1]) {
                // skip single-line comment
                (Some('#'), _) => {
                    while self.peek[0].map_or(false, |c| c != '\n' && c != '\r') {
                        self.bump();
                    }
                    continue;
                }
                // skip whitespace
                (Some(c), _) if c == ' ' || c == '\t' => {
                    self.bump();
                    continue;
                }
                (Some(c), d) if c == '\r' || c == '\n' => {
                    self.bump();
                    self.loc.bump_line();
                    if c == '\r' && d.map_or(false, |d| d == '\n') {
                        self.bump();
                    }
                    continue
                }
                (Some('['), _) => {
                    self.bump();
                    return Some(Ok((loc, Token::LBracket, self.loc.clone())));
                }
                (Some(']'), _) => {
                    self.bump();
                    return Some(Ok((loc, Token::RBracket, self.loc.clone())));
                }
                (Some('{'), _) => {
                    self.bump();
                    return Some(Ok((loc, Token::LBrace, self.loc.clone())));
                }
                (Some('}'), _) => {
                    self.bump();
                    return Some(Ok((loc, Token::RBrace, self.loc.clone())));
                }
                (Some(','), _) => {
                    self.bump();
                    return Some(Ok((loc, Token::Comma, self.loc.clone())));
                }
                (Some(':'), _) => {
                    self.bump();
                    return Some(Ok((loc, Token::Colon, self.loc.clone())));
                }
                // Literal
                (Some('"'), _) => {
                    self.bump();

                    let mut v = String::new();
                    while self.peek[0].map_or(false, |c| c != '"') {
                        let mut c = self.peek[0].unwrap();
                        if c == '\\' {
                            self.bump();
                            if self.peek[0].is_none() {
                                panic!("syntax error: {}: expect esaped character", self.loc);
                            }
                            c = self.peek[0].unwrap()
                        }
                        v.push(c);
                        self.bump();
                    }

                    self.bump();
                    return Some(Ok((loc, Token::Literal(v), self.loc.clone())));
                }
                // Identifier
                (Some(c), _) if c == '$' || c == '\\' => {
                    let mut v = String::new();

                    v.push(c);
                    self.bump();
                    while self.peek[0].map_or(false, |c| !c.is_ascii_whitespace()) {
                        v.push(self.peek[0].unwrap());
                        self.bump();
                    }

                    return Some(Ok((loc, Token::Ident(v), self.loc.clone())));
                }
                (Some(c), _) if c == '.' => {
                    let mut v = String::new();

                    v.push(c);
                    self.bump();
                    while self.peek[0].map_or(false, |c| c.is_ascii_digit()) {
                        v.push(self.peek[0].unwrap());
                        self.bump();
                    }

                    return Some(Ok((loc, Token::Ident(v), self.loc.clone())));
                }
                // Keyword
                (Some(c), _) if c.is_ascii_alphabetic() => {
                    let mut v = String::new();

                    while self.peek[0].map_or(false, |c| c.is_ascii_alphabetic()) {
                        v.push(self.peek[0].unwrap());
                        self.bump();
                    }

                    return match v.as_str() {
                        "autoidx" => Some(Ok((loc, Token::Autoidx, self.loc.clone()))),
                        "module" => Some(Ok((loc, Token::Module, self.loc.clone()))),
                        "attribute" => Some(Ok((loc, Token::Attribute, self.loc.clone()))),
                        "parameter" => Some(Ok((loc, Token::Parameter, self.loc.clone()))),
                        "signed" => Some(Ok((loc, Token::Signed, self.loc.clone()))),
                        "real" => Some(Ok((loc, Token::Real, self.loc.clone()))),
                        "wire" => Some(Ok((loc, Token::Wire, self.loc.clone()))),
                        "memory" => Some(Ok((loc, Token::Memory, self.loc.clone()))),
                        "width" => Some(Ok((loc, Token::Width, self.loc.clone()))),
                        "upto" => Some(Ok((loc, Token::Upto, self.loc.clone()))),
                        "offset" => Some(Ok((loc, Token::Offset, self.loc.clone()))),
                        "size" => Some(Ok((loc, Token::Size, self.loc.clone()))),
                        "input" => Some(Ok((loc, Token::Input, self.loc.clone()))),
                        "output" => Some(Ok((loc, Token::Output, self.loc.clone()))),
                        "inout" => Some(Ok((loc, Token::Inout, self.loc.clone()))),
                        "cell" => Some(Ok((loc, Token::Cell, self.loc.clone()))),
                        "connect" => Some(Ok((loc, Token::Connect, self.loc.clone()))),
                        "switch" => Some(Ok((loc, Token::Switch, self.loc.clone()))),
                        "case" => Some(Ok((loc, Token::Case, self.loc.clone()))),
                        "assign" => Some(Ok((loc, Token::Assign, self.loc.clone()))),
                        "sync" => Some(Ok((loc, Token::Sync, self.loc.clone()))),
                        "low" => Some(Ok((loc, Token::Low, self.loc.clone()))),
                        "high" => Some(Ok((loc, Token::High, self.loc.clone()))),
                        "posedge" => Some(Ok((loc, Token::Posedge, self.loc.clone()))),
                        "negedge" => Some(Ok((loc, Token::Negedge, self.loc.clone()))),
                        "edge" => Some(Ok((loc, Token::Edge, self.loc.clone()))),
                        "always" => Some(Ok((loc, Token::Always, self.loc.clone()))),
                        "global" => Some(Ok((loc, Token::Global, self.loc.clone()))),
                        "init" => Some(Ok((loc, Token::Init, self.loc.clone()))),
                        "update" => Some(Ok((loc, Token::Update, self.loc.clone()))),
                        "process" => Some(Ok((loc, Token::Process, self.loc.clone()))),
                        "end" => Some(Ok((loc, Token::End, self.loc.clone()))),
                        _ => {
                            panic!(
                                "syntax error: {}: except a keyword, but get {}",
                                self.loc, v,
                            );
                        }
                    };
                }
                // Integer or Signal
                (Some(c), _) if c.is_ascii_digit() || c == '-' => {
                    let mut v = String::new();

                    if c == '-' {
                        v.push(c);
                        self.bump();
                    }

                    while self.peek[0].map_or(false, |c| c.is_ascii_digit()) {
                        v.push(self.peek[0].unwrap());
                        self.bump();
                    }

                    let num = v.parse::<i64>();
                    if num.is_err() {
                        panic!("syntax error: {}: {}", self.loc, num.unwrap_err(),);
                    }
                    let num = num.unwrap();

                    if c == '-' || self.peek[0].map_or(false, |c| c != '\'') {
                        return Some(Ok((loc, Token::Int(num), self.loc.clone())));
                    }

                    // it is signal vector
                    self.bump();

                    let mut r = Vec::with_capacity(num as usize);

                    loop {
                        let s = self.peek[0];
                        if s.is_none() {
                            break;
                        }

                        let c = match s.unwrap() {
                            '0' => State::S0,
                            '1' => State::S1,
                            'x' => State::Sx,
                            'z' => State::Sz,
                            'm' => State::Sm,
                            '-' => State::Sa,
                            _ => break,
                        };

                        r.push(c);
                        self.bump();
                    }

                    return Some(Ok((
                        loc,
                        Token::Signal(Signal::new(num, r)),
                        self.loc.clone(),
                    )));
                }
                // End of file.
                (None, _) => return None,
                (Some(c), _) => panic!("syntax error: {}: unexpected \"{}\"", self.loc, c),
            }
        }
    }
}
