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
use std::path::{Path, PathBuf};

/*
@import Builtins/Python

@config Buitlins/Python {
    .bin_path as String := "/usr/bin/python3"
}
*/

pub struct ParsedLine {
    expression: String,
    keyword: Token,
    keyword_ctx: String,
    value_type: Option<Token>,
}

pub struct ResolvedEntry {
    pub file_name: String,
    pub full_path: String,
    pub parent_directory: String,
}

pub struct Parser {}

impl Parser {
    ///
    /// Parses command-line arguments to
    /// search for the entry point file.
    /// 
    pub fn resolve_entry(cli_args: env::Args) -> ResolvedEntry {
        let mut path: PathBuf = PathBuf::new();

        for arg in cli_args {
            if !arg.contains(".xbs") {
                continue;
            }

            if Path::new(&arg).exists() {
                path = fs::canonicalize(PathBuf::from(arg)).unwrap();
                break;
            }
        }

        ResolvedEntry {
            full_path: path
                        .to_str()
                        .expect("[Lexer::Parser]: Failed to convert PathBuf to &str! (INTERNAL_ERROR)")
                        .to_string(),

            file_name: path
                        .file_name()
                        .expect("[Lexer::Parser]: Failed to obtain file name! (INTERNAL_ERROR)")
                        .to_str()
                        .expect("[Lexer::Parser]: Failed to convert file name's &OsStr to &str! (INTERNAL_ERROR)")
                        .to_string(),

            parent_directory: path
                            .parent()
                                .expect("[Lexer::Parser]: Failed to obtain parent directory! (INTERNAL_ERROR)")
                                .to_str()
                                .expect("[Lexer::Parser]: Failed to convert parent directory's &OsStr to &str! (INTERNAL_ERROR)")
                                .to_string(),
        }
    }

    pub fn parse(entry: ResolvedEntry) -> () {
        let contents = fs::read_to_string(entry.full_path).unwrap();

        println!("{contents}");
    }
}
