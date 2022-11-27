#![allow(unused_imports)]
#![allow(dead_code)]
mod lexer;

use lexer::parser::{Parser, ResolvedEntry};

use std::env;
use std::env::Args;
use std::fs;

fn main() {
    let args: Args = std::env::args();

    let data: ResolvedEntry = Parser::resolve_entry(args);

    Parser::parse(data);
}