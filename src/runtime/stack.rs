use std::cell::Cell;

pub struct Stack {
    stack: Cell<Vec<u8>>,
}

impl Stack {
    pub fn new() -> Self {
        Self {
            stack: Cell::new(Vec::<u8>::new()),
        }
    }

    pub fn push<T: Stackable>(&mut self, arg: T) {
        let mut bytes = arg.to_bytes();
        self.stack.get_mut().append(&mut bytes);
    }

    pub fn pop<T: Stackable>(&mut self) -> T {
        use std::convert::TryInto;
        let bytes = self
            .stack
            .get_mut()
            .iter()
            .rev()
            .take(T::byte_size())
            .copied()
            .collect::<Vec<u8>>();
        let value = bytes.as_slice().try_into().unwrap();
        self.stack.get_mut().truncate(T::byte_size());
        T::from_bytes(value)
    }
}

pub trait Stackable {
    fn to_bytes(&self) -> Vec<u8>;
    fn from_bytes(stack: &[u8; 4]) -> Self;
    fn byte_size() -> usize;
}

impl Stackable for i32 {
    fn to_bytes(&self) -> Vec<u8> {
        self.to_ne_bytes().to_vec()
    }

    fn from_bytes(stack: &[u8; 4]) -> Self {
        i32::from_ne_bytes(*stack)
    }

    fn byte_size() -> usize {
        4
    }
}
