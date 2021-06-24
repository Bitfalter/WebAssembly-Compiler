use crate::ast::*;
use crate::runtime::error::RuntimeError;
use crate::runtime::error::RuntimeError::ExportNotFound;
use crate::runtime::processor::Processor;

pub fn invoke_function(ast: &Module, func: &str, params: &[i32]) -> Result<i32, RuntimeError> {
    let export = match ast.exports.iter().find(|e| e.name == func) {
        None => return Err(ExportNotFound),
        Some(e) => e,
    };

    let EDesc::FuncExport(f_index) = export.e_desc;
    let func = &ast.funcs[f_index];
    let f_type = &ast.types[func.f_type as usize];

    if f_type.0.len() != params.len() {
        return Err(RuntimeError::InvalidArgNumber);
    };

    let mut processor = Processor::new();
    processor.execute_func(&func, params);

    Ok(processor.get_result())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invoke_function_test() {
        let ast = Module {
            types: vec![(vec![ValueType::I32, ValueType::I32], vec![ValueType::I32])],
            funcs: vec![Func {
                f_type: 0,
                locals: vec![],
                body: vec![Instr::LocalGet(0), Instr::LocalGet(1), Instr::I32Add],
            }],
            exports: vec![Export {
                name: "add".to_string(),
                e_desc: EDesc::FuncExport(0),
            }],
        };

        let result = invoke_function(&ast, "add", &[5, 6]).unwrap();

        assert_eq!(11, result);
    }
}
