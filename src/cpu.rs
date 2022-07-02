use std::ops::Add;

// use crate::addrmod::AddrMod;
use crate::addrmod::AddrMod;
use crate::flags::Flags;
use crate::memory::Memory;
use crate::{assembler, check_bit_one, get_bit};

#[derive(Debug)]
pub(crate) struct CPU {
    pub a: u8,
    pub x: u8,
    pub y: u8,
    pub pc: u16,
    pub sp: u16,
    pub flags: Flags,
    pub memory: Memory,
    pub cyc: u32,
    pub rom: Vec<u8>,
}

impl CPU {
    pub fn new(rom: &Vec<u8>) -> CPU {
        CPU {
            a: 0x00,
            x: 0x00,
            y: 0x00,
            pc: 0x00,
            sp: 0x00,
            flags: Flags::new(),
            memory: Memory::new(),
            cyc: 0x00,
            rom: rom.clone(),
        }
    }

    pub fn reset(&mut self) {
        self.a = 0x00;
        self.x = 0x00;
        self.y = 0x00;
        self.pc = 0x00;
        self.sp = 0x00;
        self.flags = Flags::new();
        self.memory = Memory::new();
        self.cyc = 0x00;
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

    pub fn unimpl() {
        println!("unimpl");
    }

    pub fn update_pc(&mut self, pc: u16) {
        self.pc = pc;
    }

    pub fn mem_read(&mut self, addr_mod: AddrMod, operands: &Vec<u8>) -> Option<u8> {
        match addr_mod {
            AddrMod::Immediate => Some(operands[0]),
            AddrMod::ZeroPage => Some(self.memory.read(operands[0] as u16)),
            AddrMod::ZeroPageX => Some(self.memory.read((operands[0] + self.x) as u16)),
            AddrMod::ZeroPageY => Some(self.memory.read((operands[0] + self.y) as u16)),
            AddrMod::Absolute => Some(self.memory.read((operands[0] | operands[1]) as u16)),
            AddrMod::AbsoluteX => Some(
                self.memory
                    .read((operands[0] | operands[1]) as u16 + self.x as u16),
            ),
            AddrMod::AbsoluteY => Some(
                self.memory
                    .read((operands[0] | operands[1]) as u16 + self.y as u16),
            ),
            AddrMod::Indirect => Some(
                self.memory
                    .read(self.memory.read(operands[0] as u16) as u16),
            ),
            AddrMod::IndirectX => Some(
                self.memory
                    .read((self.memory.read(operands[0] as u16) as u16 + self.x as u16) as u16),
            ),
            AddrMod::IndirectY => Some(
                self.memory
                    .read((self.memory.read(operands[0] as u16) as u16 + self.y as u16) as u16),
            ),
            _ => None,
        }
    }

    pub fn mem_write(&mut self, addr_mod: AddrMod, operands: &Vec<u8>, val: u8) {
        match addr_mod {
            AddrMod::ZeroPage => self.memory.write(operands[0] as u16, val),
            AddrMod::ZeroPageX => self.memory.write((operands[0] + self.x) as u16, val),
            AddrMod::ZeroPageY => self.memory.write((operands[0] + self.y) as u16, val),
            AddrMod::Absolute => self.memory.write((operands[0] | operands[1]) as u16, val),
            AddrMod::AbsoluteX => self
                .memory
                .write((operands[0] | operands[1]) as u16 + self.x as u16, val),
            AddrMod::AbsoluteY => self
                .memory
                .write((operands[0] | operands[1]) as u16 + self.y as u16, val),
            AddrMod::Indirect => self
                .memory
                .write(self.memory.read(operands[0] as u16) as u16, val),
            AddrMod::IndirectX => self.memory.write(
                self.memory.read(operands[0] as u16) as u16 + self.x as u16,
                val,
            ),
            AddrMod::IndirectY => self.memory.write(
                self.memory.read(operands[0] as u16) as u16 + self.y as u16,
                val,
            ),
            _ => (),
        }
    }

    pub fn unwrap_operands(operands: &Vec<u8>) -> Option<(u8, Option<u8>)> {
        if operands.len() == 1 {
            return Some((operands[0], None));
        } else if operands.len() == 2 {
            return Some((operands[0], Some(operands[1])));
        } else {
            return None;
        }
    }

    pub fn step(&mut self) {
        if self.pc >= self.rom.len() as u16 {
            println!("PC (0x{:0>2x}) out of bounds", self.pc);
            return;
        }
        let (instr, next) = assembler::next_instruction(&self.rom, self.pc);

        match instr.opcode.as_str() {
            "ADC" => {
                // Add with carry
                let val = self.mem_read(instr.addr_mod, &instr.operands);
                if !val.is_none() {
                    let b7 = get_bit!(self.a, 7);

                    match self.a.checked_add(val.unwrap()) {
                        Some(v) => {
                            self.a += val.unwrap();
                            self.flags.trig_c_if(false);
                        }
                        None => {
                            self.a = 0xff;
                            self.flags.trig_c_if(true);
                        }
                    };

                    self.flags.trig_z_if(self.a == 0);
                    self.flags.trig_v_if(get_bit!(self.a, 7) != b7);
                    self.flags.trig_n_if(check_bit_one!(self.a, 7));
                }
            }
            "STA" => {
                // Memory <- Accumulator
                self.mem_write(instr.addr_mod, &instr.operands, self.a);
            }
            "STX" => {
                // Memory <- X
                self.mem_write(instr.addr_mod, &instr.operands, self.x);
            }
            "STY" => {
                // Memory <- Y
                self.mem_write(instr.addr_mod, &instr.operands, self.y);
            }
            "" => {}

            _ => println!("Invalid instruction"),
        }

        self.update_pc(next);
    }

    pub fn run(&mut self) {
        loop {
            self.step();
            if self.pc == self.rom.len() as u16 {
                break;
            }
        }
    }

    pub fn show_state(&self) {
        self.show_accu();
        self.show_x();
        self.show_y();
        self.show_pc();
        self.show_sp();
        println!("-----------------------");
        self.show_flags();
        self.show_memory();
        self.show_cyc();
    }

    pub fn show_flags(&self) {
        println!("{:?}", self.flags);
    }

    pub fn show_accu(&self) {
        println!("{:14} :=> 0x{:0>2x}", "Accumulator", self.a);
    }

    pub fn show_x(&self) {
        println!("{:14} :=> 0x{:0>2x}", "Register X", self.x);
    }

    pub fn show_y(&self) {
        println!("{:14} :=> 0x{:0>2x}", "Register Y", self.y);
    }

    pub fn show_pc(&self) {
        println!("{:14} :=> 0x{:0>2x}", "Prog. Counter", self.pc);
    }

    pub fn show_sp(&self) {
        println!("{:14} :=> 0x{:0>2x}", "Stack Pointer", self.sp);
    }

    pub fn show_memory(&self) {
        println!("{:?}", self.memory);
    }

    pub fn show_cyc(&self) {
        println!("Cyc: {:?}", self.cyc);
    }
}
