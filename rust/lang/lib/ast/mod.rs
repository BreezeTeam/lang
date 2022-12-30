use crate::token::Token;

/// Program is ast root Node
pub type Program = Vec<Stmt>;

/// 块语句
pub type BlockStatement = Vec<Stmt>;

/// statement 语句对象
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Stmt {
    // let 语句
    LetStmt(Identifier, Expr),
    // return 语句
    ReturnStmt(Expr),
    // 表达式语句
    ExprStmt(Expr),
}

/// 表达式
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
    // 标识符表达式
    IdentExpr(Identifier),
    // 字面量表达式
    LiteralExpr(Literal),
    // 前缀表达式
    PrefixExpr(Prefix, Box<Expr>),
    // if else 语句表达式
    IfExpr {
        // condition
        cond: Box<Expr>,
        // condition为true的语句块
        consequence: BlockStatement,
        // condition为false的语句块
        alternative: Option<BlockStatement>,
    },
    // 函数表达式
    FnExpr {
        // 函数参数
        parameters: Vec<Identifier>,
        // 函数体
        body: BlockStatement,
    },
    // Array Literal
    ArrayExpr(Vec<Expr>),
    // HashMap Literal
    HashExpr(Vec<(Literal, Expr)>),
    // 中缀表达式
    InfixExpr(Infix, Box<Expr>, Box<Expr>),
    // 调用表达式
    CallExpr {
        // 函数体
        function: Box<Expr>,
        // 函数调用入参
        arguments: Vec<Expr>,
    },
    // 索引表达式
    IndexExpr {
        // 被索引体
        left: Box<Expr>,

        // 索引值
        index: Box<Expr>,
    },
}

/// 前缀类型
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Prefix {
    // +
    Plus,
    // -
    Minus,
    // !
    Not,
}

/// 中缀类型
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Infix {
    // +
    Plus,
    // -
    Minus,
    // /
    Divide,
    // *
    Multiply,
    // ==
    Equal,
    // !=
    NotEqual,
    // >=
    GreaterThanEqual,
    // <=
    LessThanEqual,
    // >
    GreaterThan,
    // <
    LessThan,
}

/// 基本字面量类型
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Literal {
    IntLiteral(i64),
    BoolLiteral(bool),
    StringLiteral(String),
}

/// 标识符
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Identifier(pub String);

/// 优先级定义
#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub enum Precedence {
    PLowest,
    // == !=
    PEquals,
    // > <
    PCompare,
    // + -
    PSum,
    //  * /
    PProduct,
    // -X !X
    PPrefix,
    // g(X)
    PCall,
    // a[Index]
    PIndex,
}

/// 优先级解析
pub fn precedences(t: &Token) -> (Precedence, Option<Infix>) {
    match *t {
        Token::Equal => (Precedence::PEquals, Some(Infix::Equal)),
        Token::NotEqual => (Precedence::PEquals, Some(Infix::NotEqual)),
        Token::LessThanEqual => (Precedence::PCompare, Some(Infix::LessThanEqual)),
        Token::GreaterThanEqual => (Precedence::PCompare, Some(Infix::GreaterThanEqual)),
        Token::LessThan => (Precedence::PCompare, Some(Infix::LessThan)),
        Token::GreaterThan => (Precedence::PCompare, Some(Infix::GreaterThan)),
        Token::Plus => (Precedence::PSum, Some(Infix::Plus)),
        Token::Minus => (Precedence::PSum, Some(Infix::Minus)),
        Token::Multiply => (Precedence::PProduct, Some(Infix::Multiply)),
        Token::Divide => (Precedence::PProduct, Some(Infix::Divide)),

        Token::LParen => (Precedence::PCall, None),
        Token::LBracket => (Precedence::PIndex, None),
        _ => (Precedence::PLowest, None),
    }
}