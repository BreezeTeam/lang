use crate::ast::*;
use crate::token::*;
use crate::ast::Precedence::*;
use nom::{
    branch::alt,
    bytes::complete::take,
    combinator::{map, opt, verify},
    error::{Error, ErrorKind},
    multi::many0,
    sequence::{preceded, terminated, tuple},
    Err, IResult,
};

/// 语言解析，生成 ast root node
mod program_parse {
    use super::*;

    /// 获取一个 token，并且进行检验的宏
    macro_rules! verify_token {
        ($vis:vis $function_name:ident,$token:expr) => {
            $vis fn $function_name(tokens:Tokens)->IResult<Tokens,Tokens>{
                verify(take(1usize),|t:&Tokens|t.tokens[0] == $token)(tokens)
            }
        };
    }

    /// take 一个 token,然后使用match匹配进行map转换的宏
    /// 将会返回第一个匹配的结果，然后对结果进行包装后返回
    macro_rules! map_token {
        ($vis:vis $function_name:ident->$outty:ty,( $( ($token:path=>$output:path) ),* )) => {
            $vis fn $function_name(input: Tokens) -> IResult<Tokens, $outty> {
                let (left, matches) = take(1usize)(input)?;
                if matches.tokens.is_empty() {
                    Err(Err::Error(Error::new(input, ErrorKind::Tag)))
                } else {
                    match matches.tokens[0].clone() {
                        $( $token(token) => Ok((left,$output(token))) ),*
                        ,_ => Err(Err::Error(Error::new(input, ErrorKind::Tag)))
                    }
                }
            }
        };
    }

    /// 将tuple中多解析子map通过闭包转换为对应的输出类型的宏
    macro_rules! map_parser {
         ($vis:vis $function_name:ident->$outty:ty,()=>$map_func:expr) => {
            $vis fn $function_name(input: Tokens) -> IResult<Tokens, $outty> {
                Ok((input, $map_func()))
            }
        };
        ($vis:vis $function_name:ident->$outty:ty,$parser:expr=>$map_func:expr) => {
            $vis fn $function_name(input: Tokens) -> IResult<Tokens, $outty> {
                map(tuple($parser),$map_func)(input)
            }
        };
    }

    /// 多解析子的alt宏
    macro_rules! parsers {
        ($vis:vis $function_name:ident->$outty:ty,$parsers:expr) => {
            $vis fn $function_name(input: Tokens) -> IResult<Tokens, $outty> {
                alt($parsers)(input)
            }
        };
    }

    verify_token!(verify_illegal, Token::Illegal);
    verify_token!(verify_eof, Token::EOF);
    verify_token!(verify_assign, Token::Assign);
    verify_token!(verify_if, Token::If);
    verify_token!(verify_else, Token::Else);
    verify_token!(verify_plus, Token::Plus);
    verify_token!(verify_minus, Token::Minus);
    verify_token!(verify_divide, Token::Divide);
    verify_token!(verify_multipy, Token::Multiply);
    verify_token!(verify_equal, Token::Equal);
    verify_token!(verify_not_equal, Token::NotEqual);
    verify_token!(verify_greater_than_equal, Token::GreaterThanEqual);
    verify_token!(verify_less_than_equal, Token::LessThanEqual);
    verify_token!(verify_greater_than, Token::GreaterThan);
    verify_token!(verify_less_than, Token::LessThan);
    verify_token!(verify_not, Token::Not);
    verify_token!(verify_function, Token::Function);
    verify_token!(verify_let, Token::Let);
    verify_token!(verify_return, Token::Return);
    verify_token!(verify_comma, Token::Comma);
    verify_token!(verify_colon, Token::Colon);
    verify_token!(verify_semicolon, Token::SemiColon);
    verify_token!(verify_lparen, Token::LParen);
    verify_token!(verify_rparen, Token::RParen);
    verify_token!(verify_lbrace, Token::LBrace);
    verify_token!(verify_rbrace, Token::RBrace);
    verify_token!(verify_lbracket, Token::LBracket);
    verify_token!(verify_rbracket, Token::RBracket);

    /// 将标识符 Token::Ident 转换为 Identifier struct
    map_token! {
        ident_parse->Identifier,
        (
            (Token::Ident => Identifier)
        )
    }
    /// 将 Literal Token 转换为 Literal Enum
    map_token! {
        literal_parse->Literal,
        (
            (Token::IntLiteral => Literal::IntLiteral),
            (Token::StringLiteral => Literal::StringLiteral),
            (Token::BoolLiteral => Literal::BoolLiteral)
        )
    }

    /// 表达式解析
    mod expr_parse {
        use super::*;
        use crate::ast;
        use std::cell::Cell;

        /// 解析 标识符列表
        mod ident_list_parse {
            use super::*;

            /// 空标识符列表解析
            map_parser! {
                empty_idents->Vec<Identifier>,
                ()=>||vec![]
            }

            /// 标识符列表解析
            map_parser! {
                parse_idents->Vec<Identifier>,
                (
                    ident_parse,
                    many0(preceded(verify_comma, ident_parse))
                )=>|(ident, idents)| [&vec![ident][..], &idents[..]].concat()
            }

            /// 标识符列表解析
            parsers! {
                pub parse_ident_list->Vec<Identifier>,
                (
                    parse_idents,
                    empty_idents
                )
            }
        }

        /// 解析 表达式列表
        mod expr_list_parse {
            use super::*;

            /// 空表达式列表解析
            map_parser! {
                empty_exprs->Vec<Expr>,
                ()=>||vec![]
            }

            /// 表达式列表解析
            map_parser! {
                parse_exprs->Vec<Expr>,
                (
                    parse_expr,
                    many0(preceded(verify_comma, parse_expr))
                )=>|(expr, exprs)| [&vec![expr][..], &exprs[..]].concat()
            }

            /// 表达式列表解析
            parsers! {
                pub parse_expr_list->Vec<Expr>,
                (
                    parse_exprs,
                    empty_exprs
                )
            }
        }

        /// 解析 hashPair 列表
        mod pair_list_parse {
            use super::*;

            /// hash pair 解析
            map_parser! {
                parse_hash_pair->(Literal,Expr),
                (
                    literal_parse,
                    verify_colon,
                    parse_expr
                )=>|(l,_,e)|(l,e)
            }

            /// 空 hash pair 列表解析
            map_parser! {
                empty_pairs->Vec<(Literal, Expr)>,
                ()=>||vec![]
            }

            /// hash pair 列表解析
            map_parser! {
                parse_pairs->Vec<(Literal, Expr)>,
                (
                    parse_hash_pair,
                    many0(preceded(verify_comma, parse_hash_pair))
                )=>|(pair, pairs)| [&vec![pair][..], &pairs[..]].concat()
            }

            /// hashPair 列表解析
            parsers! {
                pub parse_pair_list->Vec<(Literal, Expr)>,
                (
                    parse_pairs,
                    empty_pairs
                )
            }
        }

        /// 解析 语句块
        /// 形如`{ stmt,stmt,... }`
        map_parser! {
            parse_block_stmt->BlockStatement,
            (
                verify_lbrace,
                many0(stmt_parse::parse_stmt),
                verify_rbrace
            )=>|(_,b,_)|b as BlockStatement
        }

        /// 解析 前缀语义表达式
        mod prefix_parse {
            use super::*;
            /// prefix plus 解析
            map_parser! {
                parse_prefix_plus->Expr,
                (
                    verify_plus,
                    parse_prefix,
                )=>|(_,e)|Expr::PrefixExpr(Prefix::Plus,Box::new(e))
            }
            /// prefix minus 解析
            map_parser! {
                parse_prefix_minus->Expr,
                (
                    verify_minus,
                    parse_prefix,
                )=>|(_,e)|Expr::PrefixExpr(Prefix::Minus,Box::new(e))
            }
            /// prefix plus 解析
            map_parser! {
                parse_prefix_not->Expr,
                (
                    verify_not,
                    parse_prefix,
                )=>|(_,e)|Expr::PrefixExpr(Prefix::Not,Box::new(e))
            }

            /// 前缀表达式解析
            /// 形如`[+/-/!]expr`
            parsers! {
                parse_prefix_expr->Expr,
                (
                    parse_prefix_plus,
                    parse_prefix_minus,
                    parse_prefix_not
                )
            }


            /// 解析 标识符表达式
            map_parser! {
                parse_ident_expr->Expr,
                (
                    ident_parse,
                )=>|(ident, )| Expr::IdentExpr(ident)
            }

            /// 解析 字面量表达式
            map_parser! {
                parse_literal_expr->Expr,
                (
                    literal_parse,
                )=>|(literal, )|Expr::LiteralExpr(literal)
            }

            /// 解析 paren表达式
            /// 形如 `(expr)`
            map_parser! {
                parse_paren_expr->Expr,
                (
                    verify_lparen,
                    parse_expr,
                    verify_rparen
                )=>|(_, expr, _)| expr
            }

            /// 解析 函数表达式
            /// 形如 `func ( [ident,ident,..] ) { stmt,stmt,... }`
            map_parser! {
                parse_func_expr->Expr,
                (
                    verify_function,
                    verify_lparen,
                    ident_list_parse::parse_ident_list,
                    verify_rparen,
                    parse_block_stmt,
                )=>|(_, _, p, _, b)| Expr::FnExpr {parameters: p,body: b}
            }

            /// 解析 数组表达式
            /// 形如 `[ [expr,expr,..] ]`
            map_parser! {
                parse_array_expr->Expr,
                (
                    verify_lbracket,
                    expr_list_parse::parse_expr_list,
                    verify_rbracket,
                )=>|(_, exprs, _)| Expr::ArrayExpr(exprs)
            }

            /// 解析 hashmap表达式
            /// 形如 `{ [pair,pair,..] }`
            map_parser! {
                parse_hash_expr->Expr,
                (
                    verify_lbrace,
                    pair_list_parse::parse_pair_list,
                    verify_rbrace,
                )=>|(_, pairs, _)| Expr::HashExpr(pairs)
            }

            /// 解析 if_else表达式
            /// 形如 `if ( expr ) { stmt,stmt,... } [else { stmt,stmt,... } ]`
            map_parser! {
                parse_if_expr->Expr,
                (
                    verify_if,
                    verify_lparen,
                    parse_expr,
                    verify_rparen,
                    parse_block_stmt,
                    opt(preceded(verify_else, parse_block_stmt)),
                )=>|(_, _, expr, _, c, a)| Expr::IfExpr {
                    cond: Box::new(expr),
                    consequence: c,
                    alternative: a,
                }
            }

            /// 解析 具有prefix语义的表达式
            parsers! {
                pub parse_prefix->Expr,
                (
                    parse_ident_expr,
                    parse_literal_expr,
                    parse_prefix_expr,
                    parse_if_expr,
                    parse_func_expr,
                    parse_array_expr,
                    parse_hash_expr,
                    parse_paren_expr,
                )
            }
        }

        /// 解析 中缀语义表达式
        mod infix_parse {
            use nom::combinator::success;
            use super::*;
            /// 解析 调用表达式
            /// 形如 `left ( [expr,expr,...] )`
            /// 解析时，只解析 left 后面部分，left传递给返回的闭包
            map_parser! {
                pub parse_call_expr->impl FnOnce(Expr) -> Expr,
                (
                    verify_lparen,
                    expr_list_parse::parse_expr_list,
                    verify_rparen
                )=>|(_, arguments, _)| {
                    |function:Expr| {
                        Expr::CallExpr {
                            function: Box::new(function),
                            arguments: arguments
                        }
                    }
                }
            }

            /// 解析 索引表达式
            /// 形如 `left [ expr ]`
            /// 解析时，只解析 left 后面部分，left传递给返回的闭包
            map_parser! {
                pub parse_index_expr->impl FnOnce(Expr) -> Expr,
                (
                    verify_lbracket,
                    parse_expr,
                    verify_rbracket
                )=>|(_, index, _)| {
                    |left:Expr| {
                        Expr::IndexExpr {
                            left: Box::new(left),
                            index: Box::new(index),
                        }
                    }
                }
            }

            /// 解析 中缀表达式
            /// 匹配中缀操作符，将其映射为优先级以及Infix::Option
            /// 然后再及解析剩余部分得到 right
            /// 最后包装为一个 fn ，输入 left 返回 Expr::InfixExpr
            pub fn parse_infix_expr(input: Tokens) -> IResult<Tokens, impl FnOnce(Expr) -> Expr> {
                let (tokens, (token_precedence, token_opt)) = map(alt((
                    verify_equal,
                    verify_not_equal,
                    verify_less_than_equal,
                    verify_greater_than_equal,
                    verify_less_than,
                    verify_greater_than,
                    verify_plus,
                    verify_minus,
                    verify_multipy,
                    verify_divide,
                )), |next| precedences(&next.tokens[0]))(input).unwrap();
                if tokens.tokens.is_empty() {
                    Err(Err::Error(Error::new(input, ErrorKind::Tag)))
                } else {
                    let (tokens, right) = precedence_parse_expr(tokens, token_precedence).unwrap();
                    Ok((tokens, |left: Expr| Expr::InfixExpr(token_opt.unwrap(), Box::new(left), Box::new(right))))
                }
            }

            /// 解析 具有infix语义的表达式
            pub fn parse_infix(input: Tokens, precedence: Precedence, left: Expr) -> IResult<Tokens, Expr> {
                let (tokens, next) = take(1usize)(input)?;
                if next.tokens.is_empty() {
                    Ok((tokens, left))
                } else {
                    match precedences(&next.tokens[0]).0 {
                        Precedence::PCall if precedence < Precedence::PCall => {
                            let (tokens, expression) = parse_call_expr(input)?;
                            parse_infix(tokens, precedence, expression(left))
                        }
                        Precedence::PIndex if precedence < Precedence::PIndex => {
                            let (tokens, expression) = parse_index_expr(input)?;
                            parse_infix(tokens, precedence, expression(left))
                        }
                        ref next_precedence if precedence < *next_precedence => {
                            let (tokens, expression) = parse_infix_expr(input)?;
                            parse_infix(tokens, precedence, expression(left))
                        }
                        _ => {
                            Ok((input, left))
                        }
                    }
                }
            }
        }

        /// 带优先级的表达式解析
        fn precedence_parse_expr(input: Tokens, precedence: Precedence) -> IResult<Tokens, Expr> {
            let (input, expression) = prefix_parse::parse_prefix(input)?;
            let (tokens, expr) = infix_parse::parse_infix(input, precedence, expression)?;
            Ok((tokens, expr))
        }

        /// 对于带优先级的表达式解析的包装
        pub fn parse_expr(token: Tokens) -> IResult<Tokens, Expr> {
            precedence_parse_expr(token, Precedence::PLowest)
        }
    }

    /// stmt parse
    /// 匹配语句
    mod stmt_parse {
        use super::*;
        use expr_parse::*;

        /// let stmt parse
        /// 形如 `let ident = expr [;]`
        map_parser! {
            parse_let_stmt->Stmt,
            (
                verify_let,
                ident_parse,
                verify_assign,
                parse_expr,
                opt(verify_semicolon),
            )=>|(_, ident, _, expr, _)| Stmt::LetStmt(ident, expr)
        }

        /// return stmt parse
        /// 形如 `return expr [;]`
        map_parser! {
            parse_return_stmt->Stmt,
            (
                verify_return,
                parse_expr,
                opt(verify_semicolon)
            )=>|(_, expr, _)| Stmt::ReturnStmt(expr)
        }

        /// 多种语句表达式的匹配
        /// 形如 `expr [;]`
        map_parser! {
            parse_expr_stmt->Stmt,
            (
                parse_expr,
                opt(verify_semicolon)
            )=>|(expr, _)| Stmt::ExprStmt(expr)
        }

        /// 匹配单个stmt
        /// 依次匹配三种 stmt即 LetStmt，ReturnStmt,ExprStmt
        parsers! {
            pub parse_stmt->Stmt,
            (
                parse_let_stmt,
                parse_return_stmt,
                parse_expr_stmt,
            )
        }
    }

    /// 匹配多个stmt，并且最后匹配一个 token::EOF
    pub fn parse_program(input: Tokens) -> IResult<Tokens, Program> {
        // terminated:匹配第一个和第二个，并且丢弃第二个的匹配值
        // 同时由于第一个是many0匹配，也就是会匹配多个语句
        terminated(many0(stmt_parse::parse_stmt), verify_eof)(input)
    }
}

/// Parser 语法解析器
pub struct Parser;

impl Parser {
    /// 语法解析器入口，根据多个匹配子进行语法解析
    pub fn parsing(tokens: Tokens) -> IResult<Tokens, Program> {
        program_parse::parse_program(tokens)
    }
}


#[cfg(test)]
mod tests {
    use std::borrow::Borrow;
    use super::*;
    use crate::lexer::*;

    fn assert_input_with_program(input: &[u8], expected_results: Program) {
        let (_, r) = Lexer::lexing(input).unwrap();
        let tokens = Tokens::new(&r);
        let (_, result) = Parser::parsing(tokens).unwrap();
        assert_eq!(result, expected_results);
    }

    fn compare_inputs(input: &[u8], input2: &[u8]) {
        let (_, r) = Lexer::lexing(input).unwrap();
        let tokens = Tokens::new(&r);
        let (_, result) = Parser::parsing(tokens).unwrap();

        let (_, r) = Lexer::lexing(input2).unwrap();
        let tokens = Tokens::new(&r);
        let (_, expected_results) = Parser::parsing(tokens).unwrap();

        assert_eq!(result, expected_results);
    }

    #[test]
    fn empty() {
        assert_input_with_program(&b""[..], vec![]);
    }

    #[test]
    fn let_statements() {
        let input = "let x = 5;\
             let y = 10;\
             let foobar = 838383;\
             let boo = true;\
            "
            .as_bytes();

        let program: Program = vec![
            Stmt::LetStmt(Identifier("x".to_owned()), Expr::LiteralExpr(Literal::IntLiteral(5))),
            Stmt::LetStmt(
                Identifier("y".to_owned()),
                Expr::LiteralExpr(Literal::IntLiteral(10)),
            ),
            Stmt::LetStmt(
                Identifier("foobar".to_owned()),
                Expr::LiteralExpr(Literal::IntLiteral(838383)),
            ),
            Stmt::LetStmt(
                Identifier("boo".to_owned()),
                Expr::LiteralExpr(Literal::BoolLiteral(true)),
            ),
        ];

        assert_input_with_program(input, program);
    }

    #[test]
    fn return_statements() {
        let input = "return 5;\
             return 10;\
             return 838383;\
             return true;\
            "
            .as_bytes();

        let program: Program = vec![
            Stmt::ReturnStmt(Expr::LiteralExpr(Literal::IntLiteral(5))),
            Stmt::ReturnStmt(Expr::LiteralExpr(Literal::IntLiteral(10))),
            Stmt::ReturnStmt(Expr::LiteralExpr(Literal::IntLiteral(838383))),
            Stmt::ReturnStmt(Expr::LiteralExpr(Literal::BoolLiteral(true))),
        ];

        assert_input_with_program(input, program);
    }

    #[test]
    fn some_statements() {
        let input = "let x = 5;\
             return 10;\
             15;\
             let y = 20;\
             return false;\
            "
            .as_bytes();

        let program: Program = vec![
            Stmt::LetStmt(Identifier("x".to_owned()), Expr::LiteralExpr(Literal::IntLiteral(5))),
            Stmt::ReturnStmt(Expr::LiteralExpr(Literal::IntLiteral(10))),
            Stmt::ExprStmt(Expr::LiteralExpr(Literal::IntLiteral(15))),
            Stmt::LetStmt(
                Identifier("y".to_owned()),
                Expr::LiteralExpr(Literal::IntLiteral(20)),
            ),
            Stmt::ReturnStmt(Expr::LiteralExpr(Literal::BoolLiteral(false))),
        ];

        assert_input_with_program(input, program);
    }

    #[test]
    fn test_identifier() {
        let input = "foobar;\
             foobar\
            "
            .as_bytes();

        let program: Program = vec![
            Stmt::ExprStmt(Expr::IdentExpr(Identifier("foobar".to_owned()))),
            Stmt::ExprStmt(Expr::IdentExpr(Identifier("foobar".to_owned()))),
        ];

        assert_input_with_program(input, program);
    }

    #[test]
    fn prefix_expr() {
        let input = "-foobar;\
             +10\
             !true\
            "
            .as_bytes();

        let program: Program = vec![
            Stmt::ExprStmt(Expr::PrefixExpr(
                Prefix::Minus,
                Box::new(Expr::IdentExpr(Identifier("foobar".to_owned()))),
            )),
            Stmt::ExprStmt(Expr::PrefixExpr(
                Prefix::Plus,
                Box::new(Expr::LiteralExpr(Literal::IntLiteral(10))),
            )),
            Stmt::ExprStmt(Expr::PrefixExpr(
                Prefix::Not,
                Box::new(Expr::LiteralExpr(Literal::BoolLiteral(true))),
            )),
        ];

        assert_input_with_program(input, program);
    }

    #[test]
    fn prefix_expr2() {
        let input = "-(foobar);\
             (+(10));\
             (((!true)));\
            "
            .as_bytes();

        let program: Program = vec![
            Stmt::ExprStmt(Expr::PrefixExpr(
                Prefix::Minus,
                Box::new(Expr::IdentExpr(Identifier("foobar".to_owned()))),
            )),
            Stmt::ExprStmt(Expr::PrefixExpr(
                Prefix::Plus,
                Box::new(Expr::LiteralExpr(Literal::IntLiteral(10))),
            )),
            Stmt::ExprStmt(Expr::PrefixExpr(
                Prefix::Not,
                Box::new(Expr::LiteralExpr(Literal::BoolLiteral(true))),
            )),
        ];

        assert_input_with_program(input, program);
    }

    #[test]
    fn infix_expr() {
        let input = "10 + 20".as_bytes();

        let program: Program = vec![Stmt::ExprStmt(Expr::InfixExpr(
            Infix::Plus,
            Box::new(Expr::LiteralExpr(Literal::IntLiteral(10))),
            Box::new(Expr::LiteralExpr(Literal::IntLiteral(20))),
        ))];

        assert_input_with_program(input, program);

        let input = "10 * 20".as_bytes();

        let program: Program = vec![Stmt::ExprStmt(Expr::InfixExpr(
            Infix::Multiply,
            Box::new(Expr::LiteralExpr(Literal::IntLiteral(10))),
            Box::new(Expr::LiteralExpr(Literal::IntLiteral(20))),
        ))];

        assert_input_with_program(input, program);

        let input = "10 + 5 / -20 - (x + x)".as_bytes();

        let input2 = "10 + (5 / (-20)) - (x + x)".as_bytes();

        compare_inputs(input, input2);

        let input = "10 + 5 / -20 - (x + x)".as_bytes();

        let program: Program = vec![Stmt::ExprStmt(Expr::InfixExpr(
            Infix::Minus,
            Box::new(Expr::InfixExpr(
                Infix::Plus,
                Box::new(Expr::LiteralExpr(Literal::IntLiteral(10))),
                Box::new(Expr::InfixExpr(
                    Infix::Divide,
                    Box::new(Expr::LiteralExpr(Literal::IntLiteral(5))),
                    Box::new(Expr::PrefixExpr(
                        Prefix::Minus,
                        Box::new(Expr::LiteralExpr(Literal::IntLiteral(20))),
                    )),
                )),
            )),
            Box::new(Expr::InfixExpr(
                Infix::Plus,
                Box::new(Expr::IdentExpr(Identifier("x".to_owned()))),
                Box::new(Expr::IdentExpr(Identifier("x".to_owned()))),
            )),
        ))];

        assert_input_with_program(input, program);
    }

    #[test]
    fn op_precedence() {
        let input = "!-a".as_bytes();

        let input2 = "(!(-a))".as_bytes();

        compare_inputs(input, input2);

        let input = "a + b + c".as_bytes();

        let input2 = "((a + b) + c)".as_bytes();

        compare_inputs(input, input2);

        let input = "a + b - c".as_bytes();

        let input2 = "((a + b) - c)".as_bytes();

        compare_inputs(input, input2);

        let input = "a * b * c".as_bytes();

        let input2 = "((a * b) * c)".as_bytes();

        compare_inputs(input, input2);

        let input = "a * b / c".as_bytes();

        let input2 = "((a * b) / c)".as_bytes();

        compare_inputs(input, input2);

        let input = "a + b / c".as_bytes();

        let input2 = "(a + (b / c))".as_bytes();

        compare_inputs(input, input2);

        let input = "a + b * c + d / e - f".as_bytes();

        let input2 = "(((a + (b * c)) + (d / e)) - f)".as_bytes();

        compare_inputs(input, input2);

        let input = "3 + 4; -5 * 5".as_bytes();

        let input2 = "(3 + 4);((-5) * 5)".as_bytes();

        compare_inputs(input, input2);

        let input = "5 > 4 == 3 < 4".as_bytes();

        let input2 = "((5 > 4) == (3 < 4))".as_bytes();

        compare_inputs(input, input2);

        let input = "5 < 4 != 3 > 4".as_bytes();

        let input2 = "((5 < 4) != (3 > 4))".as_bytes();

        compare_inputs(input, input2);

        let input = "3 + 4 * 5 == 3 * 1 + 4 * 5".as_bytes();

        let input2 = "((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))".as_bytes();

        compare_inputs(input, input2);
    }

    #[test]
    fn if_expr() {
        let input = "if (x < y) { x }".as_bytes();

        let program: Program = vec![Stmt::ExprStmt(Expr::IfExpr {
            cond: Box::new(Expr::InfixExpr(
                Infix::LessThan,
                Box::new(Expr::IdentExpr(Identifier("x".to_owned()))),
                Box::new(Expr::IdentExpr(Identifier("y".to_owned()))),
            )),
            consequence: vec![Stmt::ExprStmt(Expr::IdentExpr(Identifier("x".to_owned())))],
            alternative: None,
        })];

        assert_input_with_program(input, program);

        let input = "if (x < y) { x } else { y }".as_bytes();

        let program: Program = vec![Stmt::ExprStmt(Expr::IfExpr {
            cond: Box::new(Expr::InfixExpr(
                Infix::LessThan,
                Box::new(Expr::IdentExpr(Identifier("x".to_owned()))),
                Box::new(Expr::IdentExpr(Identifier("y".to_owned()))),
            )),
            consequence: vec![Stmt::ExprStmt(Expr::IdentExpr(Identifier("x".to_owned())))],
            alternative: Some(vec![Stmt::ExprStmt(Expr::IdentExpr(Identifier("y".to_owned())))]),
        })];

        assert_input_with_program(input, program);
    }

    #[test]
    fn function_expr() {
        let input = "fn() {\
                return foobar + barfoo;\
            }\
            "
            .as_bytes();

        let program: Program = vec![Stmt::ExprStmt(Expr::FnExpr {
            parameters: vec![],
            body: vec![Stmt::ReturnStmt(Expr::InfixExpr(
                Infix::Plus,
                Box::new(Expr::IdentExpr(Identifier("foobar".to_owned()))),
                Box::new(Expr::IdentExpr(Identifier("barfoo".to_owned()))),
            ))],
        })];

        assert_input_with_program(input, program);

        let input = "fn(x, y) {\
                return x + y;\
            }\
            "
            .as_bytes();

        let program: Program = vec![Stmt::ExprStmt(Expr::FnExpr {
            parameters: vec![Identifier("x".to_owned()), Identifier("y".to_owned())],
            body: vec![Stmt::ReturnStmt(Expr::InfixExpr(
                Infix::Plus,
                Box::new(Expr::IdentExpr(Identifier("x".to_owned()))),
                Box::new(Expr::IdentExpr(Identifier("y".to_owned()))),
            ))],
        })];

        assert_input_with_program(input, program);

        let input = "fn() {
                return fn (x, y, z, zz) { return x >= y; };
             }
            "
            .as_bytes();

        let program: Program = vec![Stmt::ExprStmt(Expr::FnExpr {
            parameters: vec![],
            body: vec![Stmt::ReturnStmt(Expr::FnExpr {
                parameters: vec![
                    Identifier("x".to_owned()),
                    Identifier("y".to_owned()),
                    Identifier("z".to_owned()),
                    Identifier("zz".to_owned()),
                ],
                body: vec![Stmt::ReturnStmt(Expr::InfixExpr(
                    Infix::GreaterThanEqual,
                    Box::new(Expr::IdentExpr(Identifier("x".to_owned()))),
                    Box::new(Expr::IdentExpr(Identifier("y".to_owned()))),
                ))],
            })],
        })];

        assert_input_with_program(input, program);
    }

    #[test]
    fn function_call_expr() {
        let input = "add(2, 3);\
             add(a, b, 1, 2 * 3, other(4 + 5), add(6, 7 * 8));\
             fn(a, b) { return a + b; }(1, 2);\
            "
            .as_bytes();

        let program: Program = vec![
            Stmt::ExprStmt(Expr::CallExpr {
                function: Box::new(Expr::IdentExpr(Identifier("add".to_owned()))),
                arguments: vec![
                    Expr::LiteralExpr(Literal::IntLiteral(2)),
                    Expr::LiteralExpr(Literal::IntLiteral(3)),
                ],
            }),
            Stmt::ExprStmt(Expr::CallExpr {
                function: Box::new(Expr::IdentExpr(Identifier("add".to_owned()))),
                arguments: vec![
                    Expr::IdentExpr(Identifier("a".to_owned())),
                    Expr::IdentExpr(Identifier("b".to_owned())),
                    Expr::LiteralExpr(Literal::IntLiteral(1)),
                    Expr::InfixExpr(
                        Infix::Multiply,
                        Box::new(Expr::LiteralExpr(Literal::IntLiteral(2))),
                        Box::new(Expr::LiteralExpr(Literal::IntLiteral(3))),
                    ),
                    Expr::CallExpr {
                        function: Box::new(Expr::IdentExpr(Identifier("other".to_owned()))),
                        arguments: vec![Expr::InfixExpr(
                            Infix::Plus,
                            Box::new(Expr::LiteralExpr(Literal::IntLiteral(4))),
                            Box::new(Expr::LiteralExpr(Literal::IntLiteral(5))),
                        )],
                    },
                    Expr::CallExpr {
                        function: Box::new(Expr::IdentExpr(Identifier("add".to_owned()))),
                        arguments: vec![
                            Expr::LiteralExpr(Literal::IntLiteral(6)),
                            Expr::InfixExpr(
                                Infix::Multiply,
                                Box::new(Expr::LiteralExpr(Literal::IntLiteral(7))),
                                Box::new(Expr::LiteralExpr(Literal::IntLiteral(8))),
                            ),
                        ],
                    },
                ],
            }),
            Stmt::ExprStmt(Expr::CallExpr {
                function: Box::new(Expr::FnExpr {
                    parameters: vec![Identifier("a".to_owned()), Identifier("b".to_owned())],
                    body: vec![Stmt::ReturnStmt(Expr::InfixExpr(
                        Infix::Plus,
                        Box::new(Expr::IdentExpr(Identifier("a".to_owned()))),
                        Box::new(Expr::IdentExpr(Identifier("b".to_owned()))),
                    ))],
                }),
                arguments: vec![
                    Expr::LiteralExpr(Literal::IntLiteral(1)),
                    Expr::LiteralExpr(Literal::IntLiteral(2)),
                ],
            }),
        ];

        assert_input_with_program(input, program);
    }

    #[test]
    fn strings() {
        let input = &b"\"foobar\""[..];

        let program: Program = vec![Stmt::ExprStmt(Expr::LiteralExpr(Literal::StringLiteral(
            "foobar".to_owned(),
        )))];

        assert_input_with_program(input, program);

        let input = &b"\"foo bar\""[..];

        let program: Program = vec![Stmt::ExprStmt(Expr::LiteralExpr(Literal::StringLiteral(
            "foo bar".to_owned(),
        )))];

        assert_input_with_program(input, program);

        let input = &b"\"foo\nbar\""[..];

        let program: Program = vec![Stmt::ExprStmt(Expr::LiteralExpr(Literal::StringLiteral(
            "foo\nbar".to_owned(),
        )))];

        assert_input_with_program(input, program);

        let input = &b"\"foo\tbar\""[..];

        let program: Program = vec![Stmt::ExprStmt(Expr::LiteralExpr(Literal::StringLiteral(
            "foo\tbar".to_owned(),
        )))];

        assert_input_with_program(input, program);

        let input = &b"\"foo\\\"bar\""[..];

        let program: Program = vec![Stmt::ExprStmt(Expr::LiteralExpr(Literal::StringLiteral(
            "foo\"bar".to_owned(),
        )))];

        assert_input_with_program(input, program);
    }

    #[test]
    fn arrays() {
        let input = &b"[1, 2 * 2, 3 + 3]"[..];

        let program: Program = vec![Stmt::ExprStmt(Expr::ArrayExpr(vec![
            Expr::LiteralExpr(Literal::IntLiteral(1)),
            Expr::InfixExpr(
                Infix::Multiply,
                Box::new(Expr::LiteralExpr(Literal::IntLiteral(2))),
                Box::new(Expr::LiteralExpr(Literal::IntLiteral(2))),
            ),
            Expr::InfixExpr(
                Infix::Plus,
                Box::new(Expr::LiteralExpr(Literal::IntLiteral(3))),
                Box::new(Expr::LiteralExpr(Literal::IntLiteral(3))),
            ),
        ]))];

        assert_input_with_program(input, program);

        let input = &b"myArray[1 + 1]"[..];

        let program: Program = vec![Stmt::ExprStmt(Expr::IndexExpr {
            left: Box::new(Expr::IdentExpr(Identifier("myArray".to_owned()))),
            index: Box::new(Expr::InfixExpr(
                Infix::Plus,
                Box::new(Expr::LiteralExpr(Literal::IntLiteral(1))),
                Box::new(Expr::LiteralExpr(Literal::IntLiteral(1))),
            )),
        })];

        assert_input_with_program(input, program);
    }

    #[test]
    fn array_precedence() {
        let input = "a * [1, 2, 3, 4][b * c] * d".as_bytes();

        let input2 = "((a * ([1, 2, 3, 4][b * c])) * d)".as_bytes();

        compare_inputs(input, input2);

        let input = "add(a * b[2], b[1], 2 * [1, 2][1])".as_bytes();

        let input2 = "add((a * (b[2])), (b[1]), (2 * ([1, 2][1])))".as_bytes();

        compare_inputs(input, input2);
    }

    #[test]
    fn hash() {
        let input = &b"{}"[..];

        let program: Program = vec![Stmt::ExprStmt(Expr::HashExpr(vec![]))];

        assert_input_with_program(input, program);

        let input = &b"{\"one\": 1, \"two\": 2, \"three\": 3}"[..];

        let program: Program = vec![Stmt::ExprStmt(Expr::HashExpr(vec![
            (
                Literal::StringLiteral("one".to_owned()),
                Expr::LiteralExpr(Literal::IntLiteral(1)),
            ),
            (
                Literal::StringLiteral("two".to_owned()),
                Expr::LiteralExpr(Literal::IntLiteral(2)),
            ),
            (
                Literal::StringLiteral("three".to_owned()),
                Expr::LiteralExpr(Literal::IntLiteral(3)),
            ),
        ]))];

        assert_input_with_program(input, program);

        let input = &b"{4: 1, 5: 2, 6: 3}"[..];

        let program: Program = vec![Stmt::ExprStmt(Expr::HashExpr(vec![
            (
                Literal::IntLiteral(4),
                Expr::LiteralExpr(Literal::IntLiteral(1)),
            ),
            (
                Literal::IntLiteral(5),
                Expr::LiteralExpr(Literal::IntLiteral(2)),
            ),
            (
                Literal::IntLiteral(6),
                Expr::LiteralExpr(Literal::IntLiteral(3)),
            ),
        ]))];

        assert_input_with_program(input, program);

        let input = &b"{true: 1, false: 2}"[..];

        let program: Program = vec![Stmt::ExprStmt(Expr::HashExpr(vec![
            (
                Literal::BoolLiteral(true),
                Expr::LiteralExpr(Literal::IntLiteral(1)),
            ),
            (
                Literal::BoolLiteral(false),
                Expr::LiteralExpr(Literal::IntLiteral(2)),
            ),
        ]))];

        assert_input_with_program(input, program);

        let input = &b"{\"one\": 0 + 1, \"two\": 10 - 8, \"three\": 15/5}"[..];

        let program: Program = vec![Stmt::ExprStmt(Expr::HashExpr(vec![
            (
                Literal::StringLiteral("one".to_owned()),
                Expr::InfixExpr(
                    Infix::Plus,
                    Box::new(Expr::LiteralExpr(Literal::IntLiteral(0))),
                    Box::new(Expr::LiteralExpr(Literal::IntLiteral(1))),
                ),
            ),
            (
                Literal::StringLiteral("two".to_owned()),
                Expr::InfixExpr(
                    Infix::Minus,
                    Box::new(Expr::LiteralExpr(Literal::IntLiteral(10))),
                    Box::new(Expr::LiteralExpr(Literal::IntLiteral(8))),
                ),
            ),
            (
                Literal::StringLiteral("three".to_owned()),
                Expr::InfixExpr(
                    Infix::Divide,
                    Box::new(Expr::LiteralExpr(Literal::IntLiteral(15))),
                    Box::new(Expr::LiteralExpr(Literal::IntLiteral(5))),
                ),
            ),
        ]))];

        assert_input_with_program(input, program);
    }
}
