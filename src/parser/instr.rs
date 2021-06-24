use crate::ast::Instr;
use crate::ast::Instr::I32Add;
use crate::parser::ctx::Ctx;
use crate::parser::token::bws;
use crate::parser::types::index;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::map;
use nom::multi::many1;
use nom::sequence::preceded;
use nom::IResult;
use std::cell::RefCell;
use std::rc::Rc;

fn local_get<'a>(input: &'a str, ctx: &Rc<RefCell<Ctx>>) -> IResult<&'a str, Instr> {
    let local_get = bws(tag("local.get"));
    let (input, i) = preceded(local_get, index)(input)?;
    let i = ctx.borrow().get_local_idx(&i);
    Ok((input, Instr::LocalGet(i)))
}

fn i32_add(input: &str) -> IResult<&str, Instr> {
    map(bws(tag("i32.add")), |_| I32Add)(input)
}

pub fn instrs<'a>(input: &'a str, ctx: &mut Rc<RefCell<Ctx>>) -> IResult<&'a str, Vec<Instr>> {
    let lg = |i| local_get(i, ctx);
    let instruction = alt((lg, i32_add));
    many1(bws(instruction))(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::Instr::LocalGet;

    #[test]
    fn local_get_parse() {
        let ctx = Rc::new(RefCell::new(Ctx {
            locals: vec![Some("$lhs".to_string())],
            ..Ctx::new()
        }));
        assert_eq!(local_get("local.get 1", &ctx), Ok(("", Instr::LocalGet(1))));
        assert_eq!(
            local_get("local.get $lhs", &ctx),
            Ok(("", Instr::LocalGet(0)))
        );
    }

    #[test]
    fn i32_add_parse() {
        assert_eq!(i32_add(" i32.add "), Ok(("", I32Add)));
        assert!(i32_add("local.get").is_err());
    }

    #[test]
    fn instrs_parse() {
        let mut ctx = Rc::new(RefCell::new(Ctx::new()));
        assert_eq!(
            instrs(
                "local.get 1\
                i32.add\
                local.get 2",
                &mut ctx
            ),
            Ok(("", vec![LocalGet(1), I32Add, LocalGet(2)]))
        );
    }
}
