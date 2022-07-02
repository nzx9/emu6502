use super::instructions::Instruction;
use crate::addrmod::AddrMod;
use std::fs;

pub fn read_rom(filepath: &str) -> Vec<u8> {
    let rom = fs::read(filepath).expect("Can't read rom");
    rom
}

pub fn next_instruction(rom: &Vec<u8>, pc: u16) -> (Instruction, u16) {
    let mut _pc = pc as usize;

    let instruction = match rom[_pc] {
        // ADC
        0x69 => Instruction::adc(vec![rom[_pc + 1]], AddrMod::Immediate),
        0x65 => Instruction::adc(vec![rom[_pc + 1]], AddrMod::ZeroPage),
        0x75 => Instruction::adc(vec![rom[_pc + 1]], AddrMod::ZeroPageX),
        0x6D => Instruction::adc(vec![rom[_pc + 1], rom[_pc + 2]], AddrMod::Absolute),
        0x7D => Instruction::adc(vec![rom[_pc + 1], rom[_pc + 2]], AddrMod::AbsoluteX),
        0x79 => Instruction::adc(vec![rom[_pc + 1], rom[_pc + 2]], AddrMod::AbsoluteX),
        0x61 => Instruction::adc(vec![rom[_pc + 1]], AddrMod::IndirectX),
        0x71 => Instruction::adc(vec![rom[_pc + 1]], AddrMod::IndirectY),

        // AND
        0x29 => Instruction::and(vec![rom[_pc + 1]], AddrMod::Immediate),
        0x25 => Instruction::and(vec![rom[_pc + 1]], AddrMod::ZeroPage),
        0x35 => Instruction::and(vec![rom[_pc + 1]], AddrMod::ZeroPageX),
        0x2D => Instruction::and(vec![rom[_pc + 1], rom[_pc + 2]], AddrMod::Absolute),
        0x3D => Instruction::and(vec![rom[_pc + 1], rom[_pc + 2]], AddrMod::AbsoluteX),
        0x39 => Instruction::and(vec![rom[_pc + 1], rom[_pc + 2]], AddrMod::AbsoluteY),
        0x21 => Instruction::and(vec![rom[_pc + 1]], AddrMod::IndirectX),
        0x31 => Instruction::and(vec![rom[_pc + 1]], AddrMod::IndirectY),

        // ASL
        0x0A => Instruction::asl(vec![], AddrMod::Accumulator),
        0x06 => Instruction::asl(vec![rom[_pc + 1]], AddrMod::ZeroPage),
        0x16 => Instruction::asl(vec![rom[_pc + 1]], AddrMod::ZeroPageX),
        0x0E => Instruction::asl(vec![rom[_pc + 1], rom[_pc + 2]], AddrMod::Absolute),
        0x1E => Instruction::asl(vec![rom[_pc + 1], rom[_pc + 2]], AddrMod::AbsoluteX),

        // BCC
        0x90 => Instruction::bcc(vec![rom[_pc + 1]]),

        // BSC
        0xB0 => Instruction::bcs(vec![rom[_pc + 1]]),

        // BEQ
        0xF0 => Instruction::beq(vec![rom[_pc + 1]]),

        // BIT
        0x24 => Instruction::bit(vec![rom[_pc + 1]], AddrMod::ZeroPage),
        0x2C => Instruction::bit(vec![rom[_pc + 1], rom[_pc + 2]], AddrMod::Absolute),

        // BMI
        0x30 => Instruction::bmi(vec![rom[_pc + 1]]),

        // BNE
        0xD0 => Instruction::bne(vec![rom[_pc + 1]]),

        // BPL
        0x10 => Instruction::bpl(vec![rom[_pc + 1]]),

        // BRK
        0x00 => Instruction::brk(),

        // BVC
        0x50 => Instruction::bvc(vec![rom[_pc + 1]]),

        // BVS
        0x70 => Instruction::bvs(vec![rom[_pc + 1]]),

        // CLC
        0x18 => Instruction::clc(),

        // CLD
        0xD8 => Instruction::cld(),

        // CLI
        0x58 => Instruction::cli(),

        // CLV
        0xB8 => Instruction::clv(),

        // CMP
        0xC9 => Instruction::cmp(vec![rom[_pc + 1]], AddrMod::Immediate),
        0xC5 => Instruction::cmp(vec![rom[_pc + 1]], AddrMod::ZeroPage),
        0xD5 => Instruction::cmp(vec![rom[_pc + 1]], AddrMod::ZeroPageX),
        0xCD => Instruction::cmp(vec![rom[_pc + 1], rom[_pc + 2]], AddrMod::Absolute),
        0xDD => Instruction::cmp(vec![rom[_pc + 1], rom[_pc + 2]], AddrMod::AbsoluteX),
        0xD9 => Instruction::cmp(vec![rom[_pc + 1], rom[_pc + 2]], AddrMod::AbsoluteY),
        0xC1 => Instruction::cmp(vec![rom[_pc + 1]], AddrMod::IndirectX),
        0xD1 => Instruction::cmp(vec![rom[_pc + 1]], AddrMod::IndirectY),

        // CPX
        0xE0 => Instruction::cpx(vec![rom[_pc + 1]], AddrMod::Immediate),
        0xE4 => Instruction::cpx(vec![rom[_pc + 1]], AddrMod::ZeroPage),
        0xEC => Instruction::cpx(vec![rom[_pc + 1], rom[_pc + 2]], AddrMod::Absolute),

        // CPY
        0xC0 => Instruction::cpy(vec![rom[_pc + 1]], AddrMod::Immediate),
        0xC4 => Instruction::cpy(vec![rom[_pc + 1]], AddrMod::ZeroPage),
        0xCC => Instruction::cpy(vec![rom[_pc + 1], rom[_pc + 2]], AddrMod::Absolute),

        // DEC
        0xC6 => Instruction::dec(vec![rom[_pc + 1]], AddrMod::ZeroPage),
        0xD6 => Instruction::dec(vec![rom[_pc + 1]], AddrMod::ZeroPageX),
        0xCE => Instruction::dec(vec![rom[_pc + 1], rom[_pc + 2]], AddrMod::Absolute),
        0xDE => Instruction::dec(vec![rom[_pc + 1], rom[_pc + 2]], AddrMod::AbsoluteX),

        // DEX
        0xCA => Instruction::dex(),

        // DEY
        0x88 => Instruction::dey(),

        // EOR
        0x49 => Instruction::eor(vec![rom[_pc + 1]], AddrMod::Immediate),
        0x45 => Instruction::eor(vec![rom[_pc + 1]], AddrMod::ZeroPage),
        0x55 => Instruction::eor(vec![rom[_pc + 1]], AddrMod::ZeroPageX),
        0x4D => Instruction::eor(vec![rom[_pc + 1], rom[_pc + 2]], AddrMod::Absolute),
        0x5D => Instruction::eor(vec![rom[_pc + 1], rom[_pc + 2]], AddrMod::AbsoluteX),
        0x59 => Instruction::eor(vec![rom[_pc + 1], rom[_pc + 2]], AddrMod::AbsoluteY),
        0x41 => Instruction::eor(vec![rom[_pc + 1]], AddrMod::IndirectX),
        0x51 => Instruction::eor(vec![rom[_pc + 1]], AddrMod::IndirectY),

        // INC
        0xE6 => Instruction::inc(vec![rom[_pc + 1]], AddrMod::ZeroPage),
        0xF6 => Instruction::inc(vec![rom[_pc + 1]], AddrMod::ZeroPageX),
        0xEE => Instruction::inc(vec![rom[_pc + 1], rom[_pc + 2]], AddrMod::Absolute),
        0xFE => Instruction::inc(vec![rom[_pc + 1], rom[_pc + 2]], AddrMod::AbsoluteX),

        // INX
        0xE8 => Instruction::inx(),

        // INY
        0xC8 => Instruction::iny(),

        // JMP
        0x4C => Instruction::jmp(vec![rom[_pc + 1], rom[_pc + 2]], AddrMod::Absolute),
        0x6C => Instruction::jmp(vec![rom[_pc + 1], rom[_pc + 2]], AddrMod::Indirect),

        // JSR
        0x20 => Instruction::jsr(vec![rom[_pc + 1], rom[_pc + 2]]),

        // LDA
        0xA9 => Instruction::lda(vec![rom[_pc + 1]], AddrMod::Immediate),
        0xA5 => Instruction::lda(vec![rom[_pc + 1]], AddrMod::ZeroPage),
        0xB5 => Instruction::lda(vec![rom[_pc + 1]], AddrMod::ZeroPageX),
        0xAD => Instruction::lda(vec![rom[_pc + 1], rom[_pc + 2]], AddrMod::Absolute),
        0xBD => Instruction::lda(vec![rom[_pc + 1], rom[_pc + 2]], AddrMod::AbsoluteX),
        0xB9 => Instruction::lda(vec![rom[_pc + 1], rom[_pc + 2]], AddrMod::AbsoluteY),
        0xA1 => Instruction::lda(vec![rom[_pc + 1]], AddrMod::IndirectX),
        0xB1 => Instruction::lda(vec![rom[_pc + 1]], AddrMod::IndirectY),

        // LDX
        0xA2 => Instruction::ldx(vec![rom[_pc + 1]], AddrMod::Immediate),
        0xA6 => Instruction::ldx(vec![rom[_pc + 1]], AddrMod::ZeroPage),
        0xB6 => Instruction::ldx(vec![rom[_pc + 1]], AddrMod::ZeroPageY),
        0xAE => Instruction::ldx(vec![rom[_pc + 1], rom[_pc + 2]], AddrMod::Absolute),
        0xBE => Instruction::ldx(vec![rom[_pc + 1], rom[_pc + 2]], AddrMod::AbsoluteY),

        // LDY
        0xA0 => Instruction::ldy(vec![rom[_pc + 1]], AddrMod::Immediate),
        0xA4 => Instruction::ldy(vec![rom[_pc + 1]], AddrMod::ZeroPage),
        0xB4 => Instruction::ldy(vec![rom[_pc + 1]], AddrMod::ZeroPageX),
        0xAC => Instruction::ldy(vec![rom[_pc + 1], rom[_pc + 2]], AddrMod::Absolute),
        0xBC => Instruction::ldy(vec![rom[_pc + 1], rom[_pc + 2]], AddrMod::AbsoluteX),

        // LSR
        0x4A => Instruction::lsr(vec![rom[_pc + 1]], AddrMod::Accumulator),
        0x46 => Instruction::lsr(vec![rom[_pc + 1]], AddrMod::ZeroPage),
        0x56 => Instruction::lsr(vec![rom[_pc + 1]], AddrMod::ZeroPageX),
        0x4E => Instruction::lsr(vec![rom[_pc + 1], rom[_pc + 2]], AddrMod::Absolute),
        0x5E => Instruction::lsr(vec![rom[_pc + 1], rom[_pc + 2]], AddrMod::AbsoluteX),

        // NOP
        0xEA => Instruction::nop(),

        // ORA
        0x09 => Instruction::ora(vec![rom[_pc + 1]], AddrMod::Immediate),
        0x05 => Instruction::ora(vec![rom[_pc + 1]], AddrMod::ZeroPage),
        0x15 => Instruction::ora(vec![rom[_pc + 1]], AddrMod::ZeroPageX),
        0x0D => Instruction::ora(vec![rom[_pc + 1], rom[_pc + 2]], AddrMod::Absolute),
        0x1D => Instruction::ora(vec![rom[_pc + 1], rom[_pc + 2]], AddrMod::AbsoluteX),
        0x19 => Instruction::ora(vec![rom[_pc + 1], rom[_pc + 2]], AddrMod::AbsoluteY),
        0x01 => Instruction::ora(vec![rom[_pc + 1]], AddrMod::IndirectX),
        0x11 => Instruction::ora(vec![rom[_pc + 1]], AddrMod::IndirectY),

        // PHA
        0x48 => Instruction::pha(),

        // PHP
        0x08 => Instruction::php(),

        // PLA
        0x68 => Instruction::pla(),

        // PLP
        0x28 => Instruction::plp(),

        // ROL
        0x2A => Instruction::rol(vec![rom[_pc + 1]], AddrMod::Accumulator),
        0x26 => Instruction::rol(vec![rom[_pc + 1]], AddrMod::ZeroPage),
        0x36 => Instruction::rol(vec![rom[_pc + 1]], AddrMod::ZeroPageX),
        0x2E => Instruction::rol(vec![rom[_pc + 1], rom[_pc + 2]], AddrMod::Absolute),
        0x3E => Instruction::rol(vec![rom[_pc + 1], rom[_pc + 2]], AddrMod::AbsoluteX),

        // ROR
        0x6A => Instruction::ror(vec![rom[_pc + 1]], AddrMod::Accumulator),
        0x66 => Instruction::ror(vec![rom[_pc + 1]], AddrMod::ZeroPage),
        0x76 => Instruction::ror(vec![rom[_pc + 1]], AddrMod::ZeroPageX),
        0x6E => Instruction::ror(vec![rom[_pc + 1], rom[_pc + 2]], AddrMod::Absolute),
        0x7E => Instruction::ror(vec![rom[_pc + 1], rom[_pc + 2]], AddrMod::AbsoluteX),

        // RTI
        0x40 => Instruction::rti(),

        // RTS
        0x60 => Instruction::rts(),

        // SBC
        0xE9 => Instruction::sbc(vec![rom[_pc + 1]], AddrMod::Immediate),
        0xE5 => Instruction::sbc(vec![rom[_pc + 1]], AddrMod::ZeroPage),
        0xF5 => Instruction::sbc(vec![rom[_pc + 1]], AddrMod::ZeroPageX),
        0xED => Instruction::sbc(vec![rom[_pc + 1], rom[_pc + 2]], AddrMod::Absolute),
        0xFD => Instruction::sbc(vec![rom[_pc + 1], rom[_pc + 2]], AddrMod::AbsoluteX),
        0xF9 => Instruction::sbc(vec![rom[_pc + 1], rom[_pc + 2]], AddrMod::AbsoluteY),
        0xE1 => Instruction::sbc(vec![rom[_pc + 1]], AddrMod::IndirectX),
        0xF1 => Instruction::sbc(vec![rom[_pc + 1]], AddrMod::IndirectY),

        // SEC
        0x38 => Instruction::sec(),

        // SED
        0xF8 => Instruction::sed(),

        // SEI
        0x78 => Instruction::sei(),

        // STA
        0x85 => Instruction::sta(vec![rom[_pc + 1]], AddrMod::ZeroPage),
        0x95 => Instruction::sta(vec![rom[_pc + 1]], AddrMod::ZeroPageX),
        0x8D => Instruction::sta(vec![rom[_pc + 1], rom[_pc + 2]], AddrMod::Absolute),
        0x9D => Instruction::sta(vec![rom[_pc + 1], rom[_pc + 2]], AddrMod::AbsoluteX),
        0x99 => Instruction::sta(vec![rom[_pc + 1], rom[_pc + 2]], AddrMod::AbsoluteY),
        0x81 => Instruction::sta(vec![rom[_pc + 1]], AddrMod::IndirectX),
        0x91 => Instruction::sta(vec![rom[_pc + 1]], AddrMod::IndirectY),

        // STX
        0x86 => Instruction::stx(vec![rom[_pc + 1]], AddrMod::ZeroPage),
        0x96 => Instruction::stx(vec![rom[_pc + 1]], AddrMod::ZeroPageY),
        0x8E => Instruction::stx(vec![rom[_pc + 1], rom[_pc + 2]], AddrMod::Absolute),

        // STY
        0x84 => Instruction::sty(vec![rom[_pc + 1]], AddrMod::ZeroPage),
        0x94 => Instruction::sty(vec![rom[_pc + 1]], AddrMod::ZeroPageX),
        0x8C => Instruction::sty(vec![rom[_pc + 1], rom[_pc + 2]], AddrMod::Absolute),

        // TAX
        0xAA => Instruction::tax(),

        // TAY
        0xA8 => Instruction::tay(),

        // TSX
        0xBA => Instruction::tsx(),

        // TXA
        0x8A => Instruction::txa(),

        // TXS
        0x9A => Instruction::txs(),

        // TYA
        0x98 => Instruction::tya(),

        // UKN
        _ => Instruction::ukn(),
    };

    let instruction_size = instruction.addr_mod.bytes();
    (instruction, (_pc + instruction_size) as u16)
}

pub fn disassemble(rom: &Vec<u8>, show_line_number: bool) -> String {
    let mut pc = 0;
    let mut dis_asm = String::new();
    let mut counter: u32 = 0;

    while (pc as usize) < rom.len() {
        let result = crate::assembler::next_instruction(&rom, pc);
        pc = result.1;
        if show_line_number {
            counter += 1;
            dis_asm.push_str(&format!("{:>3}: ", counter));
        }
        dis_asm.push_str(&fmt_dasm(result.0));
        dis_asm.push_str("\n");
    }
    dis_asm
}

pub fn fmt_dasm(instruction: Instruction) -> String {
    let opcode = instruction.opcode;
    let operands = instruction.operands;
    let mut code = opcode.to_string();
    let addr_mod = instruction.addr_mod;

    if addr_mod == AddrMod::Absolute
        || addr_mod == AddrMod::AbsoluteX
        || addr_mod == AddrMod::AbsoluteY
    {
        code.push_str(&format!(
            " {}${:02x}{:02x}",
            addr_mod.indicator(),
            operands[1],
            operands[0]
        ));
    } else {
        for operand in operands {
            code.push_str(&format!(" {}${:0>2x}", addr_mod.indicator(), operand));
        }
    }

    code
}
