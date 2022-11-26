mod lexer;

use lexer::parser::{Parser, ResolvedEntry};

use std::env;
use std::fs;

fn main() {
    let args = std::env::args();

    let data: ResolvedEntry = Parser::resolve_entry(args);

    Parser::parse(data);
}
