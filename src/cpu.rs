use crate::addrmod::AddrMod;
use crate::flags::Flags;
use crate::memory::Memory;

struct CPU {
    pub a: u8,
    pub x: u8,
    pub y: u8,
    pub pc: u16,
    pub sp: u16,
    pub flags: Flags,
    pub memory: Memory,
    pub cyc: u32,
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            a: 0x00,
            x: 0x00,
            y: 0x00,
            pc: 0x00,
            sp: 0x00,
            flags: Flags::new(),
            memory: Memory::new(),
            cyc: 0x00,
        }
    }

    pub fn add(&self, a: u8, b: u8) -> u8 {
        a + b
    }

    pub fn sub(&self, a: u8, b: u8) -> u8 {
        a - b
    }

    pub fn mul(&self, a: u8, b: u8) -> u8 {
        a * b
    }

    pub fn div(&self, a: u8, b: u8) -> u8 {
        a / b
    }

    pub fn check_bit(val: u8, bit: u8) -> bool {
        val & (1 << bit) != 0
    }

    // pub fn get(&self, addrmod: AddrMod, addr: Option<u16>) -> Option<u8> {
    //     match addrmod {
    //         AddrMod::None => None,
    //         AddrMod::Implied => None,
    //         AddrMod::ZeroPage => Some(self.memory.get(addr.unwrap() as u16)),
    //         AddrMod::Immediate => Some(self.memory.get(addr.unwrap() as u16)),
    //         AddrMod::Indirect => None,
    //         AddrMod::Absolute => None,
    //     }
    // }

    pub fn nxt(&mut self, rom: &Vec<u8>) {
        match rom[self.pc as usize] {
            0x18 => {
                self.pc += 1;
                self.cyc += 7;
            }
            _ => println!("Invalid"),
        };
    }
}
