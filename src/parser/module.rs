use crate::ast::EDesc::FuncExport;
use crate::ast::*;
use crate::parser::ctx::Ctx;
use crate::parser::token::{bws, ws};
use crate::parser::{instr, token, types, values};
use nom::branch::alt;
use nom::combinator::map;
use nom::multi::many0;
use nom::sequence::{preceded, tuple};
use nom::IResult;
use std::cell::RefCell;
use std::rc::Rc;

fn func<'a>(input: &'a str, ctx: &mut Rc<RefCell<Ctx>>) -> IResult<&'a str, Func> {
    fn inner<'a>(input: &'a str, ctx: &mut Rc<RefCell<Ctx>>) -> IResult<&'a str, Func> {
        let (input, id) = preceded(token::func, values::id)(input)?;
        ctx.borrow_mut().insert_func_id(Some(id.to_string()));
        let (input, f_type) = types::type_use(input, ctx)?;
        let (input, instrs) = instr::instrs(input, ctx)?;

        let f = Func {
            f_type: f_type as i32,
            locals: vec![],
            body: instrs,
        };

        Ok((input, f))
    }

    let in_pt = |i| inner(i, ctx);
    let (input, func) = token::pt(in_pt)(input)?;
    ctx.borrow_mut().insert_func(&func);

    Ok((input, func))
}

fn export<'a>(input: &'a str, ctx: &mut Rc<RefCell<Ctx>>) -> IResult<&'a str, Export> {
    let index = token::pt(preceded(token::func, types::index));
    let mut exp = token::pt(preceded(token::export, tuple((values::literal, index))));
    let (input, (lit, idx)) = exp(input)?;

    let export = Export {
        name: lit.clone(),
        e_desc: FuncExport(ctx.borrow().get_func_idx(&idx)),
    };

    ctx.borrow_mut().insert_export(&Some(lit), &export);

    Ok((input, export))
}

pub fn module(input: &str) -> IResult<&str, Module> {
    let ctx = Rc::new(RefCell::new(Ctx::new()));
    let func_ctx = |i| func(i, &mut ctx.clone());
    let export_ctx = |i| export(i, &mut ctx.clone());
    let mod_field = bws(many0(bws(alt((
        map(func_ctx, |_| ()),
        map(export_ctx, |_| ()),
    )))));
    let _ = preceded(ws, token::pt(preceded(token::module, mod_field)))(input)?;

    let module = Module {
        types: ctx.borrow().types.list.clone(),
        funcs: ctx.borrow().funcs.list.clone(),
        exports: ctx.borrow().exports.list.clone(),
    };

    Ok(("", module))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::Instr::*;
    use crate::ast::ValueType::I32;
    use crate::parser::ctx::Field;

    #[test]
    fn func_parse() {
        let mut ctx = Rc::new(RefCell::new(Ctx::new()));
        let wat = "(func $add (param $lhs i32) (param $rhs i32) (result i32)
              local.get $lhs
              local.get $rhs
              i32.add)";

        let expected = Func {
            f_type: 0,
            locals: vec![],
            body: vec![LocalGet(0), LocalGet(1), I32Add],
        };

        assert_eq!(func(wat, &mut ctx), Ok(("", expected.clone())));
        assert_eq!(
            ctx,
            Rc::new(RefCell::new(Ctx {
                locals: vec![Some("$lhs".to_string()), Some("$rhs".to_string())],
                types: Field {
                    ids: vec![None],
                    list: vec![(vec![I32, I32], vec![I32])],
                },
                funcs: Field {
                    ids: vec![Some("$add".to_string())],
                    list: vec![expected]
                },
                exports: Field::new()
            }))
        )
    }

    #[test]
    fn export_parse() {
        let mut ctx = Rc::new(RefCell::new(Ctx {
            funcs: Field {
                ids: vec![Some("$add".to_string())],
                list: vec![],
            },
            ..Ctx::new()
        }));
        let expected = Export {
            name: "add".to_string(),
            e_desc: FuncExport(0),
        };
        assert_eq!(
            export("(export \"add\" (func $add))", &mut ctx),
            Ok(("", expected))
        );
        assert_eq!(
            ctx,
            Rc::new(RefCell::new(Ctx {
                locals: vec![],
                types: Field::new(),
                funcs: Field {
                    ids: vec![Some("$add".to_string())],
                    list: vec![]
                },
                exports: Field {
                    ids: vec![Some("add".to_string())],
                    list: vec![Export {
                        name: "add".to_string(),
                        e_desc: EDesc::FuncExport(0)
                    }]
                }
            }))
        )
    }

    #[test]
    fn module_parse() {
        let wat = "(module
                (func $add (param $lhs i32) (param $rhs i32) (result i32)
                  local.get $lhs
                  local.get $rhs
                  i32.add)
                (export \"add\" (func $add))
            )";

        let expected = Module {
            types: vec![(vec![I32, I32], vec![I32])],
            funcs: vec![Func {
                f_type: 0,
                locals: vec![],
                body: vec![LocalGet(0), LocalGet(1), I32Add],
            }],
            exports: vec![Export {
                name: "add".to_string(),
                e_desc: FuncExport(0),
            }],
        };

        assert_eq!(module(wat), Ok(("", expected)));
    }
}
