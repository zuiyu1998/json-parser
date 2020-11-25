use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::complete::{escaped, tag, take_till1, take_while_m_n},
    character::complete::multispace0,
    combinator::{map, peek},
    error::context,
    multi::separated_list0,
    number::complete::double,
    sequence::{delimited, preceded, separated_pair},
    IResult,
};

#[derive(Debug)]
pub enum JsonValue {
    Str(String),
    Boolean(bool),
    Null,
    Num(f64),
    Array(Vec<JsonValue>),
    Object(HashMap<String, JsonValue>),
}

//解析数字
fn num(i: &str) -> IResult<&str, JsonValue> {
    context("num", map(double, |s| JsonValue::Num(s)))(i)
}

//解析str
fn string(i: &str) -> IResult<&str, JsonValue> {
    context(
        "string",
        map(
            alt((tag("\"\""), delimited(tag("\""), parse_str, tag("\"")))),
            |s| JsonValue::Str(String::from(s)),
        ),
    )(i)
}

//解析含有转义字符的字符串
fn parse_str(i: &str) -> IResult<&str, &str> {
    context("parse_str", escaped(normal, '\\', escapable))(i)
}

//解析正常字符
fn normal(i: &str) -> IResult<&str, &str> {
    take_till1(|c: char| c == '\\' || c == '"' || c.is_ascii_control())(i)
}

//解析转义字符后的内容
fn escapable(i: &str) -> IResult<&str, &str> {
    context(
        "escapable",
        alt((
            tag("\""),
            tag("\\"),
            tag("/"),
            tag("b"),
            tag("f"),
            tag("n"),
            tag("r"),
            tag("t"),
            parse_hex,
        )),
    )(i)
}

//解析unicode字符
fn parse_hex(i: &str) -> IResult<&str, &str> {
    context(
        "parse_hex",
        preceded(
            peek(tag("u")),
            take_while_m_n(5, 5, |c: char| c.is_ascii_hexdigit() || c == 'u'),
        ),
    )(i)
}

//解析value
fn value(i: &str) -> IResult<&str, JsonValue> {
    context(
        "value",
        delimited(
            multispace0,
            alt((string, num, boolean, null, array, object)),
            multispace0,
        ),
    )(i)
}

//解析布尔
fn boolean(i: &str) -> IResult<&str, JsonValue> {
    let parse_true = nom::combinator::value(true, tag("true"));
    let parse_false = nom::combinator::value(false, tag("false"));

    context(
        "boolean",
        map(alt((parse_true, parse_false)), |b| JsonValue::Boolean(b)),
    )(i)
}

//解析null
fn null(i: &str) -> IResult<&str, JsonValue> {
    context("null", map(tag("null"), |_| JsonValue::Null))(i)
}

//解析array
fn array(i: &str) -> IResult<&str, JsonValue> {
    context(
        "array",
        map(
            delimited(
                tag("["),
                separated_list0(tag(","), delimited(multispace0, value, multispace0)),
                tag("]"),
            ),
            |res| JsonValue::Array(res),
        ),
    )(i)
}

fn object(i: &str) -> IResult<&str, JsonValue> {
    context(
        "object",
        map(
            delimited(
                tag("{"),
                map(
                    separated_list0(tag(","), separated_pair(key, tag(":"), value)),
                    |vec: Vec<(&str, JsonValue)>| {
                        vec.into_iter().map(|(k, v)| (String::from(k), v)).collect()
                    },
                ),
                tag("}"),
            ),
            |map: HashMap<String, JsonValue>| JsonValue::Object(map),
        ),
    )(i)
}

fn key(i: &str) -> IResult<&str, &str> {
    context(
        "key",
        alt((tag("\"\""), delimited(tag("\""), parse_str, tag("\"")))),
    )(i)
}

pub fn json(i: &str) -> IResult<&str, JsonValue> {
    context(
        "json",
        delimited(multispace0, alt((object, array)), multispace0),
    )(i)
}
