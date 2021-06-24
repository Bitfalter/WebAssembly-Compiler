use crate::ast::ValueType::*;
use crate::ast::{FuncType, ValueType};
use crate::parser::ctx::Ctx;
use crate::parser::token::{bws, ws};
use crate::parser::*;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::{map, opt, value};
use nom::multi::many0;
use nom::sequence::{preceded, tuple};
use nom::IResult;
use std::cell::RefCell;
use std::rc::Rc;

pub fn func_type<'a>(input: &'a str, ctx: &mut Rc<RefCell<Ctx>>) -> IResult<&'a str, FuncType> {
    #[derive(Clone)]
    enum PR {
        R(ValueType),
        P(ValueType, Option<String>),
    }

    let p = map(
        preceded(
            ws,
            token::pt(tuple((token::param, opt(values::id), types::value_type))),
        ),
        |p| PR::P(p.2, p.1.map(|id| id.to_string())),
    );

    let r = map(
        preceded(ws, token::pt(preceded(token::result, types::value_type))),
        PR::R,
    );

    let t = alt((p, r));
    let (input, many_t) = many0(t)(input)?;

    let results = many_t
        .iter()
        .filter_map(|t| match t {
            PR::R(r) => Some(*r),
            PR::P(_, _) => None,
        })
        .collect::<Vec<ValueType>>();

    let params = many_t
        .iter()
        .filter_map(|t| match t {
            PR::R(_) => None,
            PR::P(p, id) => {
                ctx.borrow_mut().insert_local_id(id);
                Some(*p)
            }
        })
        .collect::<Vec<ValueType>>();

    let ft = (params, results);
    Ok((input, ft))
}

pub fn value_type(input: &str) -> IResult<&str, ValueType> {
    let types = alt((value(I32, tag("i32")), value(I64, tag("i64"))));
    bws(types)(input)
}

pub fn type_use<'a>(input: &'a str, ctx: &mut Rc<RefCell<Ctx>>) -> IResult<&'a str, usize> {
    let mut ft = |i| func_type(i, ctx);
    let (input, ft) = ft(input)?;
    let index = ctx.borrow_mut().upsert_func_type(&ft);
    Ok((input, index))
}

pub enum Index {
    Idx(usize),
    Id(String),
}
pub fn index(input: &str) -> IResult<&str, Index> {
    let idx = map(values::u32, |u| Index::Idx(u as usize));
    let id = map(values::id, |id| Index::Id(id.to_string()));
    alt((idx, id))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn func_type_parse_1() {
        let mut ctx = Rc::new(RefCell::new(Ctx::new()));
        assert_eq!(
            func_type("(param $lhs i32)", &mut ctx),
            Ok(("", (vec![I32], vec![])))
        );
        assert_eq!(
            Ctx {
                locals: vec![Some("$lhs".to_string())],
                ..Ctx::new()
            },
            *ctx.borrow_mut()
        );
    }

    #[test]
    fn func_type_parse_2() {
        let mut ctx = Rc::new(RefCell::new(Ctx::new()));
        assert_eq!(
            func_type("(param $lhs i32) (param $rhs i32) ", &mut ctx),
            Ok((" ", (vec![I32, I32], vec![])))
        );
        assert_eq!(
            Ctx {
                locals: vec![Some("$lhs".to_string()), Some("$rhs".to_string())],
                ..Ctx::new()
            },
            *ctx.borrow_mut()
        );
    }

    #[test]
    fn func_type_parse_3() {
        let mut ctx = Rc::new(RefCell::new(Ctx::new()));
        assert_eq!(
            func_type("(xparam $lhs u32)", &mut ctx),
            Ok(("(xparam $lhs u32)", (vec![], vec![])))
        );
    }

    #[test]
    fn func_type_parse_4() {
        let mut ctx = Rc::new(RefCell::new(Ctx::new()));
        assert_eq!(
            func_type("param $lhs u32", &mut ctx),
            Ok(("param $lhs u32", (vec![], vec![])))
        );
    }

    #[test]
    fn func_type_parse_5() {
        let mut ctx = Rc::new(RefCell::new(Ctx::new()));
        assert_eq!(
            func_type("(param xlhs u32)", &mut ctx),
            Ok(("(param xlhs u32)", (vec![], vec![])))
        );
    }

    #[test]
    fn func_type_parse_6() {
        let mut ctx = Rc::new(RefCell::new(Ctx::new()));
        assert_eq!(
            func_type("(param $lhs i32)", &mut ctx),
            Ok(("", (vec![I32], vec![])))
        );
    }

    #[test]
    fn func_type_parse_7() {
        let mut ctx = Rc::new(RefCell::new(Ctx::new()));
        assert_eq!(
            func_type("(param $lhs i32) (param $rhs i32) (result i64)", &mut ctx),
            Ok(("", (vec![I32, I32], vec![I64])))
        );
    }

    #[test]
    fn func_type_parse_8() {
        let mut ctx = Rc::new(RefCell::new(Ctx::new()));
        assert_eq!(
            func_type("(param i32) (param i32) (result i64)", &mut ctx),
            Ok(("", (vec![I32, I32], vec![I64])))
        );
    }

    #[test]
    fn value_type_parse() {
        assert_eq!(value_type("i32"), Ok(("", I32)));
        assert_eq!(value_type("i64"), Ok(("", I64)));
        assert!(value_type("x32").is_err());
    }
}
