use crate::parser::token::bws;
use nom::branch::*;
use nom::bytes::complete::*;
use nom::character::complete::*;
use nom::combinator::*;
use nom::multi::*;
use nom::sequence::*;
use nom::IResult;

pub fn id(input: &str) -> IResult<&str, &str> {
    let additional_chars = "!#$%&′∗+−./:<=>?@∖^_`|~";
    let id_char = alt((alphanumeric1, is_a(additional_chars)));
    let id = recognize(pair(tag("$"), many1(id_char)));
    bws(id)(input)
}

pub fn u32(input: &str) -> IResult<&str, u32> {
    map(digit1, |d: &str| {
        d.parse().expect("Integer format not supported")
    })(input)
}

pub fn literal(input: &str) -> IResult<&str, String> {
    map(
        bws(delimited(char('"'), is_not("\""), char('"'))),
        |s: &str| s.to_string(),
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn id_parse() {
        assert_eq!(id("$valid_id%#! foo "), Ok(("foo ", "$valid_id%#!")));
        assert_eq!(id("  $valid_id%#! foo "), Ok(("foo ", "$valid_id%#!")));
        assert!(id("valid_id%#! foo ").is_err());
    }

    #[test]
    fn u32_parse() {
        assert_eq!(u32("12"), Ok(("", 12)));
    }

    #[test]
    fn literal_parse() {
        assert_eq!(
            literal("\"valid#+123\""),
            Ok(("", "valid#+123".to_string()))
        );

        assert!(literal("invalid").is_err());
    }
}
