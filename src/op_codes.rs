use crate::ast::ValueType;

pub const MAGIC: &[u8] = &[0x00, 0x61, 0x73, 0x6d];
pub const VERSION: &[u8] = &[0x01, 0x00, 0x00, 0x00];

pub fn val_type(vt: &ValueType) -> u8 {
    match vt {
        ValueType::I32 => 0x7f,
        ValueType::I64 => 0x7e,
    }
}

pub mod section {
    pub const TYPE: u8 = 0x01;
    pub const CODE: u8 = 0x0a;
    pub const FUNC: u8 = 0x03;
    pub const EXPORT: u8 = 0x07;
}

pub mod var_instr {
    pub const LOCAL_GET: u8 = 0x20;
}

pub mod num_instr {
    pub const I32_ADD: u8 = 0x6a;
}

pub mod indices {
    pub const FUNC: u8 = 0x00;
}

pub mod control_flow {
    pub const FUNC: u8 = 0x60;
    pub const END: u8 = 0x0b;
}
