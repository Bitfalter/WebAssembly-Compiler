use crate::ast::{Export, Func, FuncType, Type};
use crate::parser::types::Index;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Field<T> {
    pub ids: Vec<Option<String>>,
    pub list: Vec<T>,
}

impl<T> Field<T> {
    pub fn new() -> Self {
        Self {
            ids: Vec::new(),
            list: Vec::new(),
        }
    }

    pub fn add(&mut self, id: Option<String>, item: T) {
        self.add_item(item);
        self.add_id(id);
    }

    pub fn add_item(&mut self, item: T) {
        self.list.push(item);
    }

    pub fn add_id(&mut self, id: Option<String>) {
        self.ids.push(id)
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Ctx {
    pub locals: Vec<Option<String>>,
    pub types: Field<Type>,
    pub funcs: Field<Func>,
    pub exports: Field<Export>,
}

impl Ctx {
    pub fn new() -> Self {
        Self {
            locals: Vec::new(),
            types: Field::new(),
            funcs: Field::new(),
            exports: Field::new(),
        }
    }

    pub fn get_func_idx(&self, idx: &Index) -> usize {
        match idx {
            Index::Idx(i) => *i,
            Index::Id(id) => self
                .funcs
                .ids
                .iter()
                .position(|i| i == &Some(id.to_owned()))
                .expect("Func id has to exists"),
        }
    }

    pub fn insert_local_id(&mut self, id: &Option<String>) {
        if id.is_some() && self.locals.contains(id) {
            panic!("Local identifiers have to be unique in the scope of the function")
        } else {
            self.locals.push((*id).clone());
        }
    }

    pub fn get_local_idx(&self, index: &Index) -> usize {
        match index {
            Index::Idx(i) => *i,
            Index::Id(id) => self
                .locals
                .iter()
                .position(|x| x == &Some(id.clone()))
                .expect("Identifier not found"),
        }
    }

    pub fn insert_func_id(&mut self, id: Option<String>) -> usize {
        self.funcs.add_id(id);
        self.funcs.ids.len() - 1
    }

    pub fn insert_id_func_type(&mut self, id: Option<String>, t: &FuncType) {
        self.types.add(id, (*t).clone());
    }

    pub fn get_idx_from_func_type(&self, ft: &FuncType) -> Option<usize> {
        self.types.list.iter().position(|t| t == ft)
    }

    pub fn insert_func_type_get_idx(&mut self, ft: &FuncType) -> usize {
        self.insert_id_func_type(None, ft);
        self.get_idx_from_func_type(ft).expect("Type has to exist")
    }

    pub fn upsert_func_type(&mut self, ft: &FuncType) -> usize {
        match self.get_idx_from_func_type(ft) {
            None => self.insert_func_type_get_idx(ft),
            Some(i) => i,
        }
    }

    pub fn insert_func(&mut self, func: &Func) {
        self.funcs.add_item((*func).clone());
    }

    pub fn insert_export(&mut self, id: &Option<String>, export: &Export) {
        self.exports.add((*id).clone(), (*export).clone());
    }
}
