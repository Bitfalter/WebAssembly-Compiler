use crate::runtime::disassembler::parse_wasm;
use crate::runtime::error::RuntimeError;
use crate::runtime::reader::Reader;

mod disassembler;
mod error;
mod interpreter;
mod processor;
mod reader;
mod stack;

pub fn invoke_function(wasm: Vec<u8>, f_name: &str, params: &[i32]) -> Result<i32, RuntimeError> {
    let reader = Reader::new(wasm);
    let ast = parse_wasm(&reader)?;
    interpreter::invoke_function(&ast, f_name, params)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invoke_add_function() {
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

        let result = invoke_function(wasm, "add", &[1, 4]).unwrap();

        assert_eq!(5, result);
    }
}
