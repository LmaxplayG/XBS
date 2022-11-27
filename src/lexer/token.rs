//
//  # Lexer module
//
//  Used internally to describe a source
//  file's contents, and describe tokens
//  which will be extracted by the LexerParser
//  and validated by the LexerValidator.
//

use std::fmt::Debug;

#[derive(Debug, PartialEq)]
pub enum DataType {
    String(String),
    Int(i64),
    Float(f64),
    Character(char),
    Boolean,
    Struct,
    Opaque
}

#[derive(Debug, PartialEq)]
pub enum EqType {
    ///
    /// `x == 3`
    ///
    Equal,

    ///
    /// `x != 3`
    ///
    NotEqual,

    ///
    /// `x > 3`
    /// 
    GreaterThan,

    ///
    /// `x < 3`
    /// 
    LessThan,

    ///
    /// `x >= 3`
    /// 
    GreaterThanEqual,
    
    ///
    /// `x <= 3`
    /// 
    LessThanEqual,
}

#[derive(Debug, PartialEq)]
pub enum Token {
    ///
    /// Represents a string literal.
    ///
    String(String),

    ///
    /// Represents an integer literal.
    ///
    Int(i64),

    ///
    /// Represents a float literal.
    ///
    Float(f64),

    ///
    /// Represents a character literal.
    ///
    Character(char),

    ///
    /// Represents a boolean literal.
    ///
    Boolean,

    ///
    /// Represents a typed array.
    ///
    Array(DataType),

    ///
    /// Mathematical binary addition operator.
    /// 
    Add,

    ///
    /// Mathematical subtraction operator.
    /// 
    Subtract,

    ///
    /// Mathematical binary multiplication operator.
    /// 
    Multiply,

    ///
    /// Mathematical division operator.
    /// 
    Divide,

    ///
    /// Modulus operator.
    /// 
    Mod,

    ///
    /// Equality<?> operation.
    ///
    ValueAssertion(EqType),

    ///
    /// "Define" operator for values.
    /// 
    /// Example:
    /// ```groovy
    ///     @let x := 2;
    /// ```
    /// 
    Define,

    ///
    /// Function parameters to be
    /// piped to a function call.
    /// 
    FunctionParams(DataType),

    ///
    /// Function **type* for external imports.
    ///
    FunctionType,

    ///
    /// Keyword for declaring a
    /// C-like struct.
    ///
    Struct,

    ///
    /// Opaque data type.
    ///
    Opaque,

    ///
    /// Keyword for declaring a local variable
    /// relative to the current scope.
    ///
    Let,

    ///
    ///  Keyword to declare a new function.
    ///
    Function,

    ///
    ///  Keyword for importing built-in
    ///  or local XBS libraries.
    ///
    Import,

    ///
    /// Keyword for importing external
    /// (OS-relative) libraries.
    ///
    /// Example:
    ///
    /// ```groovy
    /// @extern("inttypes.h") <- {
    ///     UInt32 as Opaque := "uint32_t"
    /// }
    /// ```
    ///
    Extern,

    ///
    /// Keyword for importing modules
    /// from stdlib.
    ///
    /// Example:
    ///
    /// ```groovy
    /// @let fizzbuzz: Tuple[Number, String[]] := @call(@std(.fizzbuzz)) <- (0, 255)
    /// ```
    Std,

    ///
    /// Keyword for executing a function.
    ///
    Call,

    ///
    /// Executes the specified function that is meant
    /// to be for CLI-only commands.
    ///
    /// A CLI call will return the status code,
    /// as well as the raw text traceback if something
    /// goes wrong.
    ///
    CliCall,

    ///
    /// Keyword for creating a plugin.
    ///
    Plugin,
}