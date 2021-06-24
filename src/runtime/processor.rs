use crate::ast::{Func, Instr};
use crate::runtime::stack::Stack;

pub struct Processor {
    stack: Stack,
}

impl Processor {
    pub fn new() -> Self {
        Self {
            stack: Stack::new(),
        }
    }

    pub fn execute_func(&mut self, func: &Func, params: &[i32]) {
        for instr in &func.body {
            match instr {
                Instr::LocalGet(i) => {
                    self.stack.push(params[*i]);
                }
                Instr::I32Add => {
                    let a: i32 = self.stack.pop();
                    let b: i32 = self.stack.pop();
                    let result = a + b;
                    self.stack.push(result);
                }
            }
        }
    }

    pub fn get_result(&mut self) -> i32 {
        self.stack.pop::<i32>()
    }
}
