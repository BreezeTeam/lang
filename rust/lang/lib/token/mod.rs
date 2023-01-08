use std::iter::Enumerate;
use std::ops::{Range, RangeFrom};
use std::ptr::eq;
use std::slice::Iter;

use nom::{InputIter, InputLength, InputTake, Needed, Slice};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Token {
    //非法字符
    Illegal,
    // 文件结尾
    EOF,

    // identifier and literals
    // identifier
    Ident(String),
    // string literals
    StringLiteral(String),
    // integer literals
    IntLiteral(i64),
    // boolean literals，True/False
    BoolLiteral(bool),

    // operators
    // ==
    Equal,
    // !=
    NotEqual,
    // >=
    GreaterThanEqual,
    // <=
    LessThanEqual,
    // =
    Assign,
    // +
    Plus,
    // -
    Minus,
    // /
    Divide,
    // *
    Multiply,
    // >
    GreaterThan,
    // <
    LessThan,
    // !
    Not,

    // reserved words
    // func
    Function,
    // let
    Let,
    // return
    Return,
    // if
    If,
    // else
    Else,
    // true
    TRUE,
    // false
    FALSE,

    // punctuations
    // delimiters
    // ,
    Comma,
    // :
    Colon,
    // ;
    SemiColon,
    // (
    LParen,
    // )
    RParen,
    // {
    LBrace,
    // }
    RBrace,
    // [
    LBracket,
    // ]
    RBracket,
}

/// 由于在使用nom时，需要实现 InputLength,InputTake,InputIter 这三个trait
/// 因此对 Vec[token] 进行包装
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Tokens<'a> {
    pub tokens: &'a [Token], // 不需要所有权
}

impl<'a> Slice<RangeFrom<usize>> for Tokens<'a> {
    fn slice(&self, range: RangeFrom<usize>) -> Self {
        self.slice(range.start..self.tokens.len() - 0usize)
    }
}

impl<'a> Slice<Range<usize>> for Tokens<'a> {
    fn slice(&self, range: Range<usize>) -> Self {
        Tokens {
            tokens: self.tokens.slice(range.clone()),
        }
    }
}

impl<'a> Tokens<'a> {
    pub fn new(tokens: &'a [Token]) -> Self {
        Tokens { tokens }
    }
}

impl<'a> InputLength for Tokens<'a> {
    fn input_len(&self) -> usize {
        self.tokens.len()
    }
}

impl<'a> InputTake for Tokens<'a> {
    fn take(&self, count: usize) -> Self {
        Tokens {
            tokens: &self.tokens[0..count],
        }
    }
    fn take_split(&self, count: usize) -> (Self, Self) {
        let (prefix, suffix) = self.tokens.split_at(count);
        (Tokens { tokens: suffix }, Tokens { tokens: prefix })
    }
}


impl<'a> InputIter for Tokens<'a> {
    type Item = &'a Token;
    type Iter = Enumerate<Iter<'a, Token>>;
    type IterElem = Iter<'a, Token>;

    // 返回元素及其偏移量迭代器，即enumerate
    fn iter_indices(&self) -> Self::Iter {
        self.tokens.iter().enumerate()
    }

    // 返回元素迭代器，即iter
    fn iter_elements(&self) -> Self::IterElem {
        self.tokens.iter()
    }

    // 查找元素的位置
    fn position<P>(&self, predicate: P) -> Option<usize> where P: Fn(Self::Item) -> bool {
        self.tokens.iter().position(predicate)
    }

    // 从元素在流中的位置获取偏移量
    fn slice_index(&self, count: usize) -> Result<usize, Needed> {
        if self.tokens.len() >= count {
            Ok(count)
        } else {
            Err(Needed::Unknown)
        }
    }
}
