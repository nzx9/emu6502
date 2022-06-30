mod addrmod;
mod assembler;
mod cpu;
mod flags;
mod instructions;
mod memory;
mod opcat;

fn main() {
    let rom = assembler::read_rom("6502_bin_examples/test.bin");

    assembler::disassemble(&rom)
}
