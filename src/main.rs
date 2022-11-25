mod lexer;

use lexer::parser;

fn main() {
    println!("Hello, world!");

    parser::parse_example();
}
