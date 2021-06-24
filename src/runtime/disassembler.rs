use crate::ast::*;
use crate::op_codes::*;
use crate::runtime::error::RuntimeError;
use crate::runtime::reader::Reader;

fn check_header(wasm: &Reader) -> Result<(), RuntimeError> {
    if wasm.len() < 8 {
        return Err(RuntimeError::ModuleToShort);
    }

    if wasm.bytes(4) != *b"\0asm" {
        return Err(RuntimeError::WrongMagicHeader);
    }

    if wasm.dword() != 1 {
        return Err(RuntimeError::WrongVersionHeader);
    }

    Ok(())
}

fn parse_type_section(wasm: &Reader) -> Result<Vec<Type>, RuntimeError> {
    if wasm.byte() != section::TYPE {
        return Err(RuntimeError::InvalidSectionCode);
    }
    let _size = wasm.byte();
    let num_types = wasm.byte();
    let mut types = vec![];

    fn parse_valuetype(wasm: &Reader) -> Result<ValueType, RuntimeError> {
        match wasm.byte() {
            0x7f => Ok(ValueType::I32),
            0x7e => Ok(ValueType::I64),
            _ => Err(RuntimeError::InvalidValueType),
        }
    }

    for _ in 0..num_types {
        let _func = wasm.byte();

        // parse params
        let mut params = vec![];
        for _ in 0..wasm.byte() {
            params.push(parse_valuetype(wasm)?);
        }

        // parse results
        let mut results = vec![];
        for _ in 0..wasm.byte() {
            results.push(parse_valuetype(wasm)?);
        }

        types.push((params, results));
    }

    Ok(types)
}

fn parse_func_section(wasm: &Reader) -> Result<Vec<i32>, RuntimeError> {
    if wasm.byte() != section::FUNC {
        return Err(RuntimeError::InvalidSectionCode);
    }

    let _size = wasm.byte();
    let num = wasm.byte();
    let mut f_types = vec![];

    for _ in 0..num {
        f_types.push(wasm.byte() as i32)
    }

    Ok(f_types)
}

fn parse_export_section(wasm: &Reader) -> Result<Vec<Export>, RuntimeError> {
    if wasm.byte() != section::EXPORT {
        return Err(RuntimeError::InvalidSectionCode);
    }

    let _size = wasm.byte();
    let num = wasm.byte();
    let mut exports = vec![];

    for _ in 0..num {
        let length = wasm.byte();
        let name = match std::str::from_utf8(wasm.bytes(length.into())) {
            Ok(n) => n.to_string(),
            Err(_) => return Err(RuntimeError::InvalidExportName),
        };
        let _zero = wasm.byte();
        let e_desc = match wasm.byte() {
            0x00 => EDesc::FuncExport(0),
            _ => return Err(RuntimeError::InvalidExportType),
        };

        exports.push(Export { name, e_desc })
    }

    Ok(exports)
}

pub fn parse_code_section(wasm: &Reader) -> Result<Vec<(StackType, Vec<Instr>)>, RuntimeError> {
    if wasm.byte() != section::CODE {
        return Err(RuntimeError::InvalidSectionCode);
    };

    let _size = wasm.byte();
    let num = wasm.byte();
    let mut code = vec![];

    for _ in 0..num {
        let _size = wasm.byte();
        let num_locals = wasm.byte() as i32;
        let mut locals = vec![];
        let mut instrs = vec![];

        for _ in 0..num_locals {
            let vt = match wasm.byte() {
                0x7f => ValueType::I32,
                0x7e => ValueType::I64,
                _ => return Err(RuntimeError::InvalidValueType),
            };
            locals.push(vt);
        }

        loop {
            let instr = match wasm.byte() {
                0x20 => Instr::LocalGet(wasm.byte() as usize),
                0x6a => Instr::I32Add,
                0x0b => break,
                _ => return Err(RuntimeError::InvalidInstruction),
            };

            instrs.push(instr);
        }

        code.push((locals, instrs));
    }

    Ok(code)
}

pub fn parse_wasm(wasm: &Reader) -> Result<Module, RuntimeError> {
    check_header(wasm)?;
    let types = parse_type_section(wasm)?;
    let funcs = parse_func_section(wasm)?;
    let exports = parse_export_section(wasm)?;
    let code = parse_code_section(wasm)?;

    let join_code_func = || {
        funcs
            .iter()
            .enumerate()
            .map(|(i, f)| Func {
                f_type: *f,
                locals: code[i].0.clone(),
                body: code[i].1.clone(),
            })
            .collect::<Vec<Func>>()
    };

    Ok(Module {
        types,
        exports,
        funcs: join_code_func(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_code_section_test() {
        let wasm = vec![
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
        let reader = Reader::new(wasm);

        let (locals, instructions) = parse_code_section(&reader).unwrap()[0].clone();

        assert_eq!(Vec::<ValueType>::new(), locals);
        assert_eq!(
            vec![Instr::LocalGet(0), Instr::LocalGet(1), Instr::I32Add],
            instructions
        );
    }

    #[test]
    fn parse_export_section_test() {
        let wasm = vec![
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
        ];
        let reader = Reader::new(wasm);

        let result = parse_export_section(&reader).unwrap();

        assert_eq!(
            vec![Export {
                name: "add".to_string(),
                e_desc: EDesc::FuncExport(0)
            }],
            result
        );
    }

    #[test]
    fn parse_func_section_test() {
        let wasm = vec![
            0x03, // section code
            0x02, // section size
            0x01, // num functions
            0x00, // function 0 signature index
        ];
        let reader = Reader::new(wasm);

        let result = parse_func_section(&reader).unwrap();

        assert_eq!(vec![0], result);
    }

    #[test]
    fn parse_wasm_test() {
        let wasm = vec![
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
        let reader = Reader::new(wasm);

        let result = parse_wasm(&reader).unwrap();

        assert_eq!(
            Module {
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
            },
            result
        );
    }

    #[test]
    fn check_header_test() {
        let wasm = vec![
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
        ];
        let reader = Reader::new(wasm);

        assert!(check_header(&reader).is_ok());
    }

    #[test]
    fn parse_type_section_test() {
        let wasm = vec![
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
        ];
        let reader = Reader::new(wasm);

        let types = parse_type_section(&reader).unwrap();

        assert_eq!(
            types,
            vec![(vec![ValueType::I32, ValueType::I32], vec![ValueType::I32])]
        );
    }
}
