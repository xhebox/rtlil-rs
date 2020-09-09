use super::grammar::DesignParser;
use super::lexer::Lexer;
use super::syntax::{Const, Design};
use anyhow::Result;
use getset::*;
use std::collections::HashMap;

#[derive(Debug, Default, Getters, MutGetters)]
#[get = "pub"]
#[get_mut = "pub"]
pub struct Parser {
    attrs: HashMap<String, Const>,
}

impl Parser {
    pub fn new() -> Self {
        Self { ..Self::default() }
    }

    pub fn parse<I>(&mut self, i: Lexer<I>) -> Result<Design>
    where
        I: Iterator<Item = char>,
    {
        Ok(DesignParser::new().parse(self, i)?)
    }
}
