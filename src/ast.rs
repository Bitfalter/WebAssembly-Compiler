#[derive(Debug, PartialEq, Clone, Copy, Eq)]
pub enum ValueType {
    I32,
    I64,
}

pub type StackType = Vec<ValueType>;
pub type FuncType = (StackType, StackType);
pub type Type = FuncType;

#[derive(Debug, PartialEq, Clone, Eq)]
pub enum Instr {
    LocalGet(usize),
    I32Add,
}

#[derive(Debug, PartialEq, Clone, Eq)]
pub struct Func {
    pub f_type: i32,
    pub locals: Vec<ValueType>,
    pub body: Vec<Instr>,
}

#[derive(Debug, PartialEq, Clone, Eq)]
pub enum EDesc {
    FuncExport(usize),
}

#[derive(Debug, PartialEq, Clone, Eq)]
pub struct Export {
    pub name: String,
    pub e_desc: EDesc,
}

#[derive(Debug, PartialEq)]
pub struct Module {
    pub types: Vec<Type>,
    pub funcs: Vec<Func>,
    pub exports: Vec<Export>,
}
