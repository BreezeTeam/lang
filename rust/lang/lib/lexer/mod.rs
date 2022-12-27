#[warn(unused_imports)]
use crate::token::Token;
use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    character::complete::{alpha1, alphanumeric1, digit1, multispace0},
    combinator::{map, map_res, recognize},
    // error::Error,
    multi::many0,
    sequence::{delimited, pair},
    IResult,
};
use std::str;

// 对应Token 的解析子集合
mod token_lex {

    use super::*;
    /// tag 匹配宏
    macro_rules! pattern {
        ($vis:vis $function_name:ident,$tag_string:literal,$token:expr) => {
            $vis fn $function_name(input: &[u8]) -> IResult<&[u8], Token> {
                map(tag($tag_string), |_| $token)(input)
            }
        };
    }
    /// 多解析子的alt宏
    macro_rules! parsers {
        ($vis:vis $function_name:ident,$parsers:expr) => {
            $vis fn $function_name(input: &[u8]) -> IResult<&[u8], Token> {
                alt($parsers)(input)
            }
        };
    }

    // operators
    pattern! {equal_operator, "==", Token::Equal}
    pattern! {not_equal_operator, "!=", Token::NotEqual}
    pattern! {greater_equal_operator, ">=", Token::GreaterThanEqual}
    pattern! {lesser_equal_operator, "<=", Token::LessThanEqual}
    pattern! {assign_operator, "=", Token::Assign}
    pattern! {plus_operator, "+", Token::Plus}
    pattern! {minus_operator, "-", Token::Minus}
    pattern! {divide_operator, "/", Token::Divide}
    pattern! {multiply_operator, "*", Token::Multiply}
    pattern! {greater_operator, ">", Token::GreaterThan}
    pattern! {lesser_operator, "<", Token::LessThan}
    pattern! {not_operator, "!", Token::Not}

    // 创建一个 多解析子的 lex_operator
    parsers! {lex_operator,
        (
            equal_operator,
            not_equal_operator,
            greater_equal_operator,
            lesser_equal_operator,
            assign_operator,
            plus_operator,
            minus_operator,
            divide_operator,
            multiply_operator,
            greater_operator,
            lesser_operator,
            not_operator,
        )
    }

    // punctuations
    pattern! {comma_punctuation, ",", Token::Comma}
    pattern! {colon_punctuation, ":", Token::Colon}
    pattern! {semicolon_punctuation, ";", Token::SemiColon}
    pattern! {lparen_punctuation, "(", Token::LParen}
    pattern! {rparen_punctuation, ")", Token::RParen}
    pattern! {lbrace_punctuation, "{", Token::LBrace}
    pattern! {rbrace_punctuation, "}", Token::RBrace}
    pattern! {lbracket_punctuation, "[", Token::LBracket}
    pattern! {rbracket_punctuation, "]", Token::RBracket}

    // 创建一个用于解析 punctuations 的多匹配子 lex_punctuations
    parsers! {lex_punctuations,
        (   comma_punctuation,
            semicolon_punctuation,
            colon_punctuation,
            lparen_punctuation,
            rparen_punctuation,
            lbrace_punctuation,
            rbrace_punctuation,
            lbracket_punctuation,
            rbracket_punctuation,
        )
    }

    // keywords
    pattern! {let_keywords,"let",Token::Let}
    pattern! {function_keywords,"fn",Token::Function}
    pattern! {if_keywords,"if",Token::If}
    pattern! {else_keywords,"else",Token::Else}
    pattern! {return_keywords,"return",Token::Return}
    pattern! {true_keywords,"true",Token::BoolLiteral(true)}
    pattern! {false_keywords,"false",Token::BoolLiteral(false)}

    // 创建一个用于解析关键字的多匹配解析子
    parsers! {lex_keywords,
        (
            let_keywords,
            function_keywords,
            if_keywords,
            else_keywords,
            return_keywords,
            true_keywords,
            false_keywords,
        )
    }
    /// 用于解析string的辅助解析子集合
    mod string_lex {
        use super::*;
        use std::result::Result::*;
        fn pis(input: &[u8]) -> IResult<&[u8], Vec<u8>> {
            let (i1, c1) = take(1usize)(input)?;
            match c1 {
                b"\"" => Ok((input, vec![])),
                b"\\" => {
                    let (i2, c2) = take(1usize)(i1)?;
                    pis(i2).map(|(slice, done)| (slice, [&(c2.to_vec())[..], &done[..]].concat()))
                }
                c => pis(i1).map(|(slice, done)| (slice, [&(c.to_vec())[..], &done[..]].concat())),
            }
        }
        pub fn stringliteral(input: &[u8]) -> IResult<&[u8], String> {
            // 匹配包含在 \"和\" 之间的部分
            delimited(
                tag("\""),
                map_res(string_lex::pis, String::from_utf8),
                tag("\""),
            )(input)
        }
    }

    /// String parsing
    fn lex_string(input: &[u8]) -> IResult<&[u8], Token> {
        map(string_lex::stringliteral, Token::StringLiteral)(input)
    }

    /// ident parsing
    fn lex_ident(input: &[u8]) -> IResult<&[u8], Token> {
        // ident以字母或者下划线开头
        // 然后是一个或者多个字母数字，下划线
        // let ident_parse = ;
        map_res(
            recognize(pair(
                alt((alpha1, tag("_"))),
                many0(alt((alphanumeric1, tag("_")))),
            )),
            |ident| {
                let byte_to_str = String::from_utf8(Vec::from(ident));
                byte_to_str.map(|astr| Token::Ident(astr))
            },
        )(input)
    }
    /// Integers parsing
    fn lex_integer(input: &[u8]) -> IResult<&[u8], Token> {
        map(
            map_res(map_res(digit1, str::from_utf8), str::FromStr::from_str),
            Token::IntLiteral,
        )(input)
    }

    /// Illegal parsing,当所有token都匹配失败时应用
    fn lex_illegal(input: &[u8]) -> IResult<&[u8], Token> {
        // 将匹配到的结果转换为Token
        map(take(1usize), |_| Token::Illegal)(input)
    }

    parsers! {lex_token,(
        lex_operator,
        lex_punctuations,
        lex_string,
        lex_keywords,
        lex_ident,
        lex_integer,
        lex_illegal,
    )}

    /// 匹配多个token
    pub fn lex_tokens(input: &[u8]) -> IResult<&[u8], Vec<Token>> {
        // 将解析器应用多次，并且返回Vec
        // 每个解析器，主要用于匹配由多个空白包围的token
        many0(delimited(multispace0, lex_token, multispace0))(input)
    }
}

/// Lexer 词法解析器
pub struct Lexer;
impl Lexer {
    /// 词法分析入口,利用匹配子进行词法分析，最后map Result后添加 `Token::EOF`
    pub fn lexing(bytes: &[u8]) -> IResult<&[u8], Vec<Token>> {
        token_lex::lex_tokens(bytes)
            .map(|(slice, result)| (slice, [&result[..], &vec![Token::EOF][..]].concat()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer1() {
        let input = &b"=+(){},;"[..];
        let (_, result) = Lexer::lexing(input).unwrap();

        let expected_results = vec![
            Token::Assign,
            Token::Plus,
            Token::LParen,
            Token::RParen,
            Token::LBrace,
            Token::RBrace,
            Token::Comma,
            Token::SemiColon,
            Token::EOF,
        ];

        assert_eq!(result, expected_results);
    }

    #[test]
    fn test_lexer2() {
        let input = "let five = 5;\
             let ten = 10;\
             let add = fn(x, y) {\
                 x + y;\
             };\
             let result = add(five, ten);"
            .as_bytes();

        let (_, result) = Lexer::lexing(input).unwrap();

        let expected_results = vec![
            Token::Let,
            Token::Ident("five".to_owned()),
            Token::Assign,
            Token::IntLiteral(5),
            Token::SemiColon,
            Token::Let,
            Token::Ident("ten".to_owned()),
            Token::Assign,
            Token::IntLiteral(10),
            Token::SemiColon,
            Token::Let,
            Token::Ident("add".to_owned()),
            Token::Assign,
            Token::Function,
            Token::LParen,
            Token::Ident("x".to_owned()),
            Token::Comma,
            Token::Ident("y".to_owned()),
            Token::RParen,
            Token::LBrace,
            Token::Ident("x".to_owned()),
            Token::Plus,
            Token::Ident("y".to_owned()),
            Token::SemiColon,
            Token::RBrace,
            Token::SemiColon,
            Token::Let,
            Token::Ident("result".to_owned()),
            Token::Assign,
            Token::Ident("add".to_owned()),
            Token::LParen,
            Token::Ident("five".to_owned()),
            Token::Comma,
            Token::Ident("ten".to_owned()),
            Token::RParen,
            Token::SemiColon,
            Token::EOF,
        ];

        assert_eq!(result, expected_results);
    }

    #[test]
    fn test_lexer3() {
        let input = "if (a == 10) {\
                return a;\
             } else if (a != 20) {\
                return !a;\
            } else if (a > 20) {\
                return -30 / 40 * 50;\
            } else if (a < 30) {\
                return true;\
            }\
            return false;\
            "
        .as_bytes();

        let (_, result) = Lexer::lexing(input).unwrap();

        let expected_results = vec![
            Token::If,
            Token::LParen,
            Token::Ident("a".to_owned()),
            Token::Equal,
            Token::IntLiteral(10),
            Token::RParen,
            Token::LBrace,
            Token::Return,
            Token::Ident("a".to_owned()),
            Token::SemiColon,
            Token::RBrace,
            Token::Else,
            Token::If,
            Token::LParen,
            Token::Ident("a".to_owned()),
            Token::NotEqual,
            Token::IntLiteral(20),
            Token::RParen,
            Token::LBrace,
            Token::Return,
            Token::Not,
            Token::Ident("a".to_owned()),
            Token::SemiColon,
            Token::RBrace,
            Token::Else,
            Token::If,
            Token::LParen,
            Token::Ident("a".to_owned()),
            Token::GreaterThan,
            Token::IntLiteral(20),
            Token::RParen,
            Token::LBrace,
            Token::Return,
            Token::Minus,
            Token::IntLiteral(30),
            Token::Divide,
            Token::IntLiteral(40),
            Token::Multiply,
            Token::IntLiteral(50),
            Token::SemiColon,
            Token::RBrace,
            Token::Else,
            Token::If,
            Token::LParen,
            Token::Ident("a".to_owned()),
            Token::LessThan,
            Token::IntLiteral(30),
            Token::RParen,
            Token::LBrace,
            Token::Return,
            Token::BoolLiteral(true),
            Token::SemiColon,
            Token::RBrace,
            Token::Return,
            Token::BoolLiteral(false),
            Token::SemiColon,
            Token::EOF,
        ];

        assert_eq!(result, expected_results);
    }

    #[test]
    fn string_literals() {
        let (_, result) = Lexer::lexing(&b"\"foobar\""[..]).unwrap();
        assert_eq!(
            result,
            vec![Token::StringLiteral("foobar".to_owned()), Token::EOF]
        );

        let (_, result) = Lexer::lexing(&b"\"foo bar\""[..]).unwrap();
        assert_eq!(
            result,
            vec![Token::StringLiteral("foo bar".to_owned()), Token::EOF]
        );

        let (_, result) = Lexer::lexing(&b"\"foo\nbar\""[..]).unwrap();
        assert_eq!(
            result,
            vec![Token::StringLiteral("foo\nbar".to_owned()), Token::EOF]
        );

        let (_, result) = Lexer::lexing(&b"\"foo\tbar\""[..]).unwrap();
        assert_eq!(
            result,
            vec![Token::StringLiteral("foo\tbar".to_owned()), Token::EOF]
        );

        let (_, result) = Lexer::lexing(&b"\"foo\\\"bar\""[..]).unwrap();
        assert_eq!(
            result,
            vec![Token::StringLiteral("foo\"bar".to_owned()), Token::EOF]
        );

        let (_, result) =
            Lexer::lexing(&b"\"foo\\\"bar with \xf0\x9f\x92\x96 emojis\""[..]).unwrap();
        assert_eq!(
            result,
            vec![
                Token::StringLiteral("foo\"bar with 💖 emojis".to_owned()),
                Token::EOF
            ]
        );
    }

    #[test]
    fn id_with_numbers() {
        let (_, result) = Lexer::lexing(&b"hello2 hel301oo120"[..]).unwrap();
        let expected = vec![
            Token::Ident("hello2".to_owned()),
            Token::Ident("hel301oo120".to_owned()),
            Token::EOF,
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn array_tokens() {
        let (_, result) = Lexer::lexing(&b"[1, 2];"[..]).unwrap();
        let expected = vec![
            Token::LBracket,
            Token::IntLiteral(1),
            Token::Comma,
            Token::IntLiteral(2),
            Token::RBracket,
            Token::SemiColon,
            Token::EOF,
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn hash_tokens() {
        let (_, result) = Lexer::lexing(&b"{\"hello\": \"world\"}"[..]).unwrap();
        let expected = vec![
            Token::LBrace,
            Token::StringLiteral("hello".to_owned()),
            Token::Colon,
            Token::StringLiteral("world".to_owned()),
            Token::RBrace,
            Token::EOF,
        ];
        assert_eq!(result, expected);
    }
}
