use std::collections::HashMap;

#[derive(Debug)]
pub struct Memory(HashMap<u16, u8>);

impl Memory {
    pub fn new() -> Memory {
        Memory(HashMap::new())
    }

    pub fn read(&self, addr: u16) -> u8 {
        *self.0.get(&addr).unwrap()
    }

    pub fn write(&mut self, addr: u16, val: u8) {
        self.0.insert(addr, val);
    }
}
