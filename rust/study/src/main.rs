use String;
use anyhow::Result;
use nom::{
    bytes::complete::{tag, take, take_while_m_n},
    character::complete::{alpha1, alphanumeric1, digit1, multispace0},
    character::is_alphabetic,
    combinator::{map, map_res, recognize},
    error::Error,
    IResult,
    multi::many0,
    sequence::{delimited, pair, tuple, Tuple},
};

// 获取tag中的字符串
fn test_tag(input: &str) -> IResult<&str, &str> {
    let (input, matchs) = tag("#")(input)?;
    println!("input:{:?} matchs:{:?}", input, matchs);
    Ok((input, matchs))
}

// take指定数量的字符串
fn test_take() -> () {
    assert_eq!(take::<_, _, Error<_>>(1usize)("💙"), Ok(("", "💙")));
    assert_eq!(
        take::<_, _, Error<_>>(1usize)("💙".as_bytes()),
        Ok((b"\x9F\x92\x99".as_ref(), b"\xF0".as_ref()))
    );
}

// get digit
fn test_digit(input: &str) -> IResult<&str, &str> {
    let (input, matchs) = digit1(input)?;
    println!("input:{:?} matchs:{:?}", input, matchs);
    Ok((input, matchs))
}

// get alphabet
fn test_alpha(input: &str) -> IResult<&str, &str> {
    let (input, matchs) = alpha1(input)?;
    println!("input:{:?} matchs:{:?}", input, matchs);
    Ok((input, matchs))
}

// 0-9, a-z, A-Z
fn test_alphanumeric(input: &str) -> IResult<&str, &str> {
    let (input, matchs) = alphanumeric1(input)?;
    println!("input:{:?} matchs:{:?}", input, matchs);
    Ok((input, matchs))
}

// \n \r \t  \space
fn test_multispace(input: &str) -> IResult<&str, &str> {
    let (input, matchs) = multispace0(input)?;
    println!("input:{:?} matchs:{:?}", input, matchs);
    Ok((input, matchs))
}

// 获取与给定 字符串最长匹配 m<len<n 的输入片
// 是否匹配由cond确定
// 此处的cond是字符串
fn test_while(input: &[u8]) -> IResult<&[u8], &[u8]> {
    let (input, matchs) = take_while_m_n(3usize, 6usize, is_alphabetic)(input)?;
    println!("input:{:?} matchs:{:?}", String::from_utf8_lossy(input), String::from_utf8_lossy(matchs));
    Ok((input, matchs))
}


// 将解析器多次使用，并且返回vec
fn test_many(input: &str) -> IResult<&str, Vec<&str>> {
    let (input, matchs) = many0(tag("#"))(input)?;
    println!("input:{:?} matchs:{:?}", input, matchs);
    Ok((input, matchs))
}

// 对结果应用f
fn test_map(input: &str) -> IResult<&str, usize> {
    let mut parse = map(digit1, |s: &str| s.len());
    let (input, matchs) = parse(input)?;
    println!("input:{:?} matchs:{:?}", input, matchs);
    Ok((input, matchs))
}

// 对结果应用返回Result的f
fn test_map_res(input: &str) -> IResult<&str, u8> {
    let mut parse = map_res(digit1, |s: &str| s.parse::<u8>());
    let (input, matchs) = parse(input)?;
    println!("input:{:?} matchs:{:?}", input, matchs);
    Ok((input, matchs))
}

//逐个应用一个元组的解析器，并将它们的结果作为元组返回
fn test_tuple(input: &str) -> IResult<&str, ()> {
    let mut parse = tuple((tag("#"), alphanumeric1));
    let (input, matchs) = parse(input)?;
    println!("input:{:?} matchs:{:?}", input, matchs);
    Ok((input, ()))
}

// 匹配来自第一个解析器的对象，然后从 sep_parser 获取对象，然后匹配来自第二个解析器的另一个对象。
fn test_delimited(input: &str) -> IResult<&str, ()> {
    let mut parse = delimited(tag("def"), multispace0, alphanumeric1);
    let (input, matchs) = parse(input)?;
    println!("input:{:?} matchs:{:?}", input, matchs);
    Ok((input, ()))
}

// 如果子解析器成功，则将消耗的输入作为生成值返回
fn test_recognize(input: &str) -> IResult<&str, ()> {
    let mut parse = recognize(delimited(tag("def"), multispace0, alphanumeric1));
    let (input, matchs) = parse(input)?;
    println!("input:{:?} matchs:{:?}", input, matchs);
    Ok((input, ()))
}

// 从第一个解析器获取一个对象，然后从第二个解析器获取另一个对象。
fn test_pair(input: &str) -> IResult<&str, ()> {
    let mut parse = pair(tag("def"), multispace0);
    let (input, matchs) = parse(input)?;
    println!("input:{:?} matchs:{:?}", input, matchs);
    Ok((input, ()))
}

fn main() {
    test_tag("#2514DF");
    test_take();
    test_digit("2514DF");
    test_alpha("DF2514");
    test_alphanumeric("DF2514");
    test_while(b"FDF2514");
    test_multispace("\t\r\n   sasa");
    test_tuple("#DF2514");
    test_many("##ssasa");
    test_delimited("def\nfunc{}");
    test_pair("def\nfunc{}");
    test_recognize("def\nfunc{}");
    test_map("2514DF");
    test_map_res("123");
}
