#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Token {
    Illegal, //非法字符
    EOF,     // 文件结尾

    // identifier and literals
    Ident(String),         // identifier
    StringLiteral(String), // string literals
    IntLiteral(i64),       // integer literals
    BoolLiteral(bool),     // boolean literals，True/False

    // operators
    Equal,            // ==
    NotEqual,         // !=
    GreaterThanEqual, // >=
    LessThanEqual,    // <=
    Assign,           // =
    Plus,             // +
    Minus,            // -
    Divide,           // /
    Multiply,         // *
    GreaterThan,      // >
    LessThan,         // <
    Not,              // !

    // reserved words
    Function, // func
    Let,      // let
    Return,   // return
    If,       // if
    Else,     // else
    TRUE,     // true
    FALSE,    // false

    // punctuations
    // delimiters
    Comma,     // ,
    Colon,     // :
    SemiColon, // ;

    LParen,   // (
    RParen,   // )
    LBrace,   // {
    RBrace,   // }
    LBracket, // [
    RBracket, // ]
}
