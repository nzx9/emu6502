use std::collections::HashMap;
pub struct Memory(HashMap<u16, u8>);

impl Memory {
    pub fn new() -> Memory {
        Memory(HashMap::new())
    }

    pub fn get(&self, addr: u16) -> u8 {
        *self.0.get(&addr).unwrap_or(&0x00)
    }
}
