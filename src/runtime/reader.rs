use std::{cell::Cell, convert::TryInto};

pub struct Reader {
    data: Vec<u8>,
    pos: Cell<usize>,
}

impl Reader {
    pub fn new(data: Vec<u8>) -> Self {
        Self {
            data,
            pos: Cell::new(0),
        }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn dword(&self) -> u32 {
        let prev = self.pos.replace(self.pos.get() + 4);
        u32::from_le_bytes(self.data[prev..self.pos.get()].try_into().unwrap())
    }

    pub fn bytes(&self, num: usize) -> &[u8] {
        let prev = self.pos.replace(self.pos.get() + num);
        &self.data[prev..self.pos.get()]
    }

    pub fn byte(&self) -> u8 {
        let prev = self.pos.replace(self.pos.get() + 1);
        self.data[prev]
    }
}
