use crate::ast::Module;

mod ctx;
mod instr;
mod module;
mod token;
mod types;
mod values;

pub fn parse(wat: &str) -> Module {
    let (_, ast) = module::module(wat).expect("Ups, something went wrong!");
    ast
}
