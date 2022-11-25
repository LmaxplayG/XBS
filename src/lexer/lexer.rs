#[derive(Debug)]
pub enum Token {
    //
    //  Literals
    //
    String(String),
    Int(i64),
    Float(f64),
    Character(Char),
    
    //
    //  Core types
    //
    Boolean,    ///     Boolean data type.
    Array,      ///     Array data type.
    Struct,     ///     C-struct like data type.
    Let,        ///     Variable relative to the current scope.
    
    //
    //  Function declarations
    //
    Function,   ///     Function declaration keyword
    FunctionT,  ///     Function _type_ for external imports

    //
    //  Opaque data type
    //
    Opaque,     ///     Opaque data type.

    //
    //  Library/module imports 
    //
    Import,     ///     Import a built-in (or project-relative) module.
    Extern,     ///     External library module (from the OS).
    Std,        ///     Standard library module.

    //
    //  Function calls
    //
    Call,       ///     Calls the specified function.
    CliCall,    ///     Calls the specified function that is meant to be CLI-only. (Such as CLI commands)
}