use std::fs::{read_to_string, File};
use std::io::{Read, Write};

mod ast;
mod compiler;
mod op_codes;
mod parser;
mod runtime;

fn main() {
    // Parse the "add.wat" file with the WASM text representation.
    let wat = read_to_string("./add.wat").expect("Failed to read wat file.");
    let ast = parser::parse(&wat);

    // Compile the WASM text representation to WASM binary code and save the
    // compiled module in the file "add.wasm"
    let wasm = compiler::compile(&ast);
    let mut file = File::create("add.wasm").expect("Failed to create wasm file.");
    file.write_all(&wasm).expect("Failed to write wasm file.");

    // Read the compiled WASM module "add.wasm" and execute the function "add" from it.
    let mut wasm = vec![];
    File::open("add.wasm")
        .unwrap()
        .read_to_end(&mut wasm)
        .unwrap();
    let result = runtime::invoke_function(wasm, "add", &[5, 6]).unwrap();

    println!("5 + 6 = {}", result);
}
