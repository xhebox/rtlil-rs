// Copyright (c) 2020 xhe

use rtlil::{dumper::Dumper, lexer::Lexer, parser::Parser, syntax::Visit};
use std::env;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        2 => {
            let input = fs::read_to_string(&args[1])?;
            let lx = Lexer::new(input.chars());
            let mut pr = Parser::new();
            let mut res = pr.parse(lx)?;
            res.visit(&mut Dumper::new())?;
        }
        _ => println!("main [input]"),
    }
    Ok(())
}
