//
//  # Lexer module
//
//  Validates tokens that the Lexer
//  extracted from the source file.
//

use super::token::{Token, DataType};

pub struct ErrorContext {
    line_number: u64,
    line_content: String
}

///
/// Enum describing the type
/// of error that occurred
/// during lex validation.
///
#[allow(dead_code)]
pub enum ErrorType {
    ///
    /// Syntax error
    ///
    /// Example:
    ///
    /// ```groovy
    /// @plugin @print "hi" // Error: Expected plugin name declaration, found "@print"
    ///
    /// ```
    ///
    UnexpectedToken,

    ///
    ///
    ///
    InvalidName,

    ///
    /// A reference was not found that matched
    /// any:
    ///
    ///  - Keyword
    ///  - Local variable in the scope
    ///  - Module object
    ///  - Function declaration
    ///  - Plugin object
    ///
    ReferenceNotFound(String),

    ///
    /// No errors were found for this lex.
    ///
    None
}

#[allow(dead_code)]
pub struct Error {
    ///
    /// Defines the context the error is in
    ///
    context: ErrorContext,

    ///
    /// The type of error
    ///
    error_type: ErrorType,

    ///
    /// Extra data pertaining to the
    /// error context.
    ///
    data: [String]
}

#[allow(dead_code)]
pub struct LexerValidator {

}

impl LexerValidator {
    ///
    /// Validates the internal usage
    /// of tokens.
    ///
    /// Semantics, and syntax-wise.
    ///
    // pub fn validate_usage<'a>(tokens: &'a [Token]) -> Error {
    //     // NYE (NOT YET IMPLEMENTED)
    //     return 
    // }
    pub fn PLACEHOLDER() {}
}