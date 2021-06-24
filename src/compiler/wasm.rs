use crate::ast::{EDesc, Export, Func, Instr, Module, Type};
use crate::compiler::leb128::from_u32;
use crate::op_codes::*;

fn encode_type_section(ast: &Module) -> Vec<u8> {
    fn encode_type(t: &Type) -> Vec<u8> {
        vec![
            vec![control_flow::FUNC],
            vec![t.0.len() as u8],
            t.0.iter().map(val_type).collect::<Vec<u8>>(),
            vec![t.1.len() as u8],
            t.1.iter().map(val_type).collect::<Vec<u8>>(),
        ]
        .concat()
    }

    let body: Vec<u8> = ast
        .types
        .iter()
        .map(|t| encode_type(t))
        .collect::<Vec<Vec<u8>>>()
        .concat();

    vec![
        vec![section::TYPE],
        from_u32((body.len() + 1) as u32),
        from_u32(ast.types.len() as u32),
        body,
    ]
    .concat()
}

fn encode_func_section(ast: &Module) -> Vec<u8> {
    if ast.funcs.is_empty() {
        vec![]
    } else {
        let body = ast
            .funcs
            .iter()
            .map(|f| f.f_type as u8)
            .collect::<Vec<u8>>();
        vec![
            vec![section::FUNC],
            from_u32((body.len() + 1) as u32),
            from_u32(ast.funcs.len() as u32),
            body,
        ]
        .concat()
    }
}

fn encode_export_section(ast: &Module) -> Vec<u8> {
    fn encode_export(export: &Export) -> Vec<u8> {
        vec![
            from_u32(export.name.len() as u32),
            export.name.as_bytes().to_vec(),
            match export.e_desc {
                EDesc::FuncExport(_) => vec![indices::FUNC],
            },
            match export.e_desc {
                EDesc::FuncExport(idx) => vec![idx as u8],
            },
        ]
        .concat()
    }

    if ast.exports.is_empty() {
        vec![]
    } else {
        let body = ast
            .exports
            .iter()
            .map(encode_export)
            .collect::<Vec<Vec<u8>>>()
            .concat();
        vec![
            vec![section::EXPORT],
            from_u32((body.len() + 1) as u32),
            from_u32(ast.exports.len() as u32),
            body,
        ]
        .concat()
    }
}

fn encode_code_section(ast: &Module) -> Vec<u8> {
    fn encode_func(func: &Func) -> Vec<u8> {
        fn encode_instr(instr: &Instr) -> Vec<u8> {
            match instr {
                Instr::LocalGet(idx) => vec![var_instr::LOCAL_GET, (*idx as u8)],
                Instr::I32Add => vec![num_instr::I32_ADD],
            }
        }

        let body = vec![
            vec![func.locals.len() as u8], // local decl count
            func.body
                .iter()
                .map(encode_instr)
                .collect::<Vec<Vec<u8>>>()
                .concat(),
            vec![control_flow::END],
        ]
        .concat();

        vec![from_u32(body.len() as u32), body].concat()
    }

    if ast.funcs.is_empty() {
        vec![]
    } else {
        let body = vec![
            vec![ast.funcs.len() as u8],
            ast.funcs
                .iter()
                .map(encode_func)
                .collect::<Vec<Vec<u8>>>()
                .concat(),
        ]
        .concat();
        vec![vec![section::CODE], from_u32((body.len()) as u32), body].concat()
    }
}

pub fn compile(ast: &Module) -> Vec<u8> {
    vec![
        MAGIC,
        VERSION,
        &encode_type_section(ast),
        &encode_func_section(ast),
        &encode_export_section(ast),
        &encode_code_section(ast),
    ]
    .concat()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::EDesc::FuncExport;
    use crate::ast::Instr::{I32Add, LocalGet};
    use crate::ast::ValueType::*;
    use crate::ast::*;

    #[test]
    fn compile_module_with_add_function() {
        let ast = Module {
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

        let wasm = [
            // wasm magic
            0x00, // \0
            0x61, // a
            0x73, // s
            0x6d, // m
            // wasm version
            0x01, // 1
            0x00, // 0
            0x00, // 0
            0x00, // 0
            // section "Type" (1)
            0x01, // section code
            0x07, // section size
            0x01, // num types
            // type 0
            0x60, // func
            0x02, // num params
            0x7f, // i32
            0x7f, // i32
            0x01, // num results
            0x7f, // i32
            // section "Function" (3)
            0x03, // section code
            0x02, // section size
            0x01, // num functions
            0x00, // function 0 signature index
            // section "Export" (7)
            0x07, // section export
            0x07, // section size
            0x01, // num exports
            0x03, // string length
            // "add" export name
            0x61, // a
            0x64, // d
            0x64, // d
            0x00, // 0
            // export kind
            0x00, // export func index
            // section "Code" (10)
            0x0a, // section code
            0x09, // section size
            0x01, // num function
            // function body 0
            0x07, // func body size
            0x00, // local decl count
            0x20, // local.get
            0x00, // local index
            0x20, // local.get
            0x01, // local index
            0x6a, // i32.add
            0x0b, // end
        ];

        assert_eq!(compile(&ast), wasm);
    }
}
