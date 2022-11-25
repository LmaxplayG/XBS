//
//  # Lexer module
//
//  Parses source files and passes
//  the extracted tokens to the validator
//  for validation steps.
//

use super::token::{DataType, Token};
use super::validator::LexerValidator;

use std::env;
use std::fs;
use std::path::Path;

pub fn parse_example() {
    let file_name = "example.xbs";

    let mut cwd = env::current_dir().ok().unwrap();

    while (cwd
        .to_str()
        .unwrap()
        .split(if env::consts::OS == "windows" {
            '\\'
        } else {
            '/'
        })
        .last()
        .unwrap()
        != "XBS")
    {
        cwd.pop();
    }

    cwd.push("example.xbs");

    // println!("Complete path: {}", cwd.to_str().unwrap());

    let contents = fs::read_to_string(cwd).expect("Failed to read contents of Example.xbs!");

    println!("Contents: \n{contents}");
}
