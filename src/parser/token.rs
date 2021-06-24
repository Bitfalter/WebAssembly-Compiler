use nom::error::ParseError;
use nom::{
    bytes::complete::*, character::complete::*, sequence::delimited, AsChar, IResult, InputIter,
    InputTakeAtPosition, Parser, Slice,
};
use std::ops::RangeFrom;

pub fn pt<I, O, E: ParseError<I>, G>(inner: G) -> impl FnMut(I) -> IResult<I, O, E>
where
    G: Parser<I, O, E>,
    I: Slice<RangeFrom<usize>> + InputIter,
    <I as InputIter>::Item: AsChar,
{
    delimited(char('('), inner, char(')'))
}

pub fn bws<I, O, E: ParseError<I>, G>(inner: G) -> impl FnMut(I) -> IResult<I, O, E>
where
    G: Parser<I, O, E>,
    I: InputTakeAtPosition,
    <I as InputTakeAtPosition>::Item: AsChar + Clone,
{
    delimited(multispace0, inner, multispace0)
}

pub fn ws(input: &str) -> IResult<&str, &str> {
    multispace0(input)
}

pub fn func(input: &str) -> IResult<&str, &str> {
    bws(tag("func"))(input)
}

pub fn param(input: &str) -> IResult<&str, &str> {
    bws(tag("param"))(input)
}

pub fn result(input: &str) -> IResult<&str, &str> {
    bws(tag("result"))(input)
}

pub fn export(input: &str) -> IResult<&str, &str> {
    bws(tag("export"))(input)
}

pub fn module(input: &str) -> IResult<&str, &str> {
    bws(tag("module"))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pt_parse() {
        let (_input, x) = pt(delimited(ws, tag("hello"), ws))("( hello )").unwrap();
        assert_eq!(x, "hello");
    }

    #[test]
    fn func_parse() {
        assert_eq!(func("func"), Ok(("", "func")));
        assert_eq!(func("func foobar"), Ok(("foobar", "func")));
        assert!(func("notfunc").is_err());
    }

    #[test]
    fn ws_parse() {
        assert_eq!(ws("  foo"), Ok(("foo", "  ")));
        assert_eq!(ws(" \nfoo"), Ok(("foo", " \n")));
        assert_eq!(ws("foo"), Ok(("foo", "")));
    }

    #[test]
    fn param_parse() {
        assert_eq!(param("param"), Ok(("", "param")));
        assert_eq!(param("param123"), Ok(("123", "param")));
    }

    #[test]
    fn result_parse() {
        assert_eq!(result("result"), Ok(("", "result")));
        assert_eq!(result("result123"), Ok(("123", "result")));
    }

    #[test]
    fn export_parse() {
        assert_eq!(export(" export "), Ok(("", "export")));
        assert!(export("noexport").is_err());
    }

    #[test]
    fn module_parse() {
        assert_eq!(module(" module "), Ok(("", "module")));
        assert!(module("nomodule").is_err());
    }
}
