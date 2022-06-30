use crate::addrmod::AddrMod;
use crate::flags::Flags;
use crate::opcat::OpCat;

#[derive(Debug)]
pub struct Instruction {
    pub opcode: String,
    pub operands: Vec<u8>,
    pub op_cat: OpCat,
    pub addr_mod: AddrMod,
    pub desc: String,
    pub aflags: Flags,
}

impl Instruction {
    pub fn new(
        opcode: String,
        operands: Vec<u8>,

        op_cat: OpCat,
        addr_mod: AddrMod,
        desc: String,
        aflags: Flags,
    ) -> Instruction {
        Instruction {
            opcode,
            operands,
            op_cat,
            addr_mod,
            desc,
            aflags,
        }
    }

    pub fn ukn() -> Instruction {
        Instruction::new(
            "UKN".to_string(),
            vec![],
            OpCat::Unimpl,
            AddrMod::None,
            "UNKNOWN".to_string(),
            Flags::new(),
        )
    }

    pub fn lda(operands: Vec<u8>, addr_mod: AddrMod) -> Instruction {
        Instruction::new(
            "LDA".to_string(),
            operands,
            OpCat::LoadStore,
            addr_mod,
            "Load Accumulator".to_string(),
            Flags::trigger(vec!['N', 'Z']),
        )
    }

    pub fn ldx(operands: Vec<u8>, addr_mod: AddrMod) -> Instruction {
        Instruction::new(
            "LDX".to_string(),
            operands,
            OpCat::LoadStore,
            addr_mod,
            "Load X Register".to_string(),
            Flags::trigger(vec!['N', 'Z']),
        )
    }

    pub fn ldy(operands: Vec<u8>, addr_mod: AddrMod) -> Instruction {
        Instruction::new(
            "LDY".to_string(),
            operands,
            OpCat::LoadStore,
            addr_mod,
            "Load Y Register".to_string(),
            Flags::trigger(vec!['N', 'Z']),
        )
    }

    pub fn sta(operands: Vec<u8>, addr_mod: AddrMod) -> Instruction {
        Instruction::new(
            "STA".to_string(),
            operands,
            OpCat::LoadStore,
            addr_mod,
            "Store Accumulator".to_string(),
            Flags::new(),
        )
    }

    pub fn stx(operands: Vec<u8>, addr_mod: AddrMod) -> Instruction {
        Instruction::new(
            "STX".to_string(),
            operands,
            OpCat::LoadStore,
            addr_mod,
            "Store X Register".to_string(),
            Flags::new(),
        )
    }

    pub fn sty(operands: Vec<u8>, addr_mod: AddrMod) -> Instruction {
        Instruction::new(
            "STY".to_string(),
            operands,
            OpCat::LoadStore,
            addr_mod,
            "Store Y Register".to_string(),
            Flags::new(),
        )
    }

    pub fn tax() -> Instruction {
        Instruction::new(
            "TAX".to_string(),
            vec![],
            OpCat::Register,
            AddrMod::Implied,
            "Transfer Accumulator to X".to_string(),
            Flags::trigger(vec!['N', 'Z']),
        )
    }

    pub fn tay() -> Instruction {
        Instruction::new(
            "TAY".to_string(),
            vec![],
            OpCat::Register,
            AddrMod::Implied,
            "Transfer Accumulator to Y".to_string(),
            Flags::trigger(vec!['N', 'Z']),
        )
    }

    pub fn txa() -> Instruction {
        Instruction::new(
            "TXA".to_string(),
            vec![],
            OpCat::Register,
            AddrMod::Implied,
            "Transfer X to Accumulator".to_string(),
            Flags::trigger(vec!['N', 'Z']),
        )
    }

    pub fn tya() -> Instruction {
        Instruction::new(
            "TYA".to_string(),
            vec![],
            OpCat::Register,
            AddrMod::Implied,
            "Transfer Y to Accumulator".to_string(),
            Flags::trigger(vec!['N', 'Z']),
        )
    }

    pub fn tsx() -> Instruction {
        Instruction::new(
            "TSX".to_string(),
            vec![],
            OpCat::Stack,
            AddrMod::Implied,
            "Transfer Stack Pointer to X".to_string(),
            Flags::trigger(vec!['N', 'Z']),
        )
    }

    pub fn txs() -> Instruction {
        Instruction::new(
            "TXS".to_string(),
            vec![],
            OpCat::Stack,
            AddrMod::Implied,
            "Transfer X to Stack Pointer".to_string(),
            Flags::new(),
        )
    }

    pub fn pha() -> Instruction {
        Instruction::new(
            "PHA".to_string(),
            vec![],
            OpCat::Stack,
            AddrMod::Implied,
            "Push Accumulator on Stack".to_string(),
            Flags::new(),
        )
    }

    pub fn php() -> Instruction {
        Instruction::new(
            "PHP".to_string(),
            vec![],
            OpCat::Stack,
            AddrMod::Implied,
            "Push Processor Status on Stack".to_string(),
            Flags::new(),
        )
    }

    pub fn pla() -> Instruction {
        Instruction::new(
            "PLA".to_string(),
            vec![],
            OpCat::Stack,
            AddrMod::Implied,
            "Pull Accumulator from Stack".to_string(),
            Flags::trigger(vec!['N', 'Z']),
        )
    }

    pub fn plp() -> Instruction {
        Instruction::new(
            "PLP".to_string(),
            vec![],
            OpCat::Stack,
            AddrMod::Implied,
            "Pull Processor Status from Stack".to_string(),
            Flags::trigger_all(),
        )
    }

    pub fn and(operands: Vec<u8>, addr_mod: AddrMod) -> Instruction {
        Instruction::new(
            "AND".to_string(),
            operands,
            OpCat::Logical,
            addr_mod,
            "Logical AND".to_string(),
            Flags::trigger(vec!['N', 'Z']),
        )
    }

    pub fn eor(operands: Vec<u8>, addr_mod: AddrMod) -> Instruction {
        Instruction::new(
            "EOR".to_string(),
            operands,
            OpCat::Logical,
            addr_mod,
            "Logical Exclusive OR".to_string(),
            Flags::trigger(vec!['N', 'Z']),
        )
    }

    pub fn ora(operands: Vec<u8>, addr_mod: AddrMod) -> Instruction {
        Instruction::new(
            "ORA".to_string(),
            operands,
            OpCat::Logical,
            addr_mod,
            "Logical Inclusive OR".to_string(),
            Flags::trigger(vec!['N', 'Z']),
        )
    }

    pub fn bit(operands: Vec<u8>, addr_mod: AddrMod) -> Instruction {
        Instruction::new(
            "BIT".to_string(),
            operands,
            OpCat::Logical,
            addr_mod,
            "Bit Test".to_string(),
            Flags::trigger(vec!['N', 'V', 'Z']),
        )
    }

    pub fn adc(operands: Vec<u8>, addr_mod: AddrMod) -> Instruction {
        Instruction::new(
            "ADC".to_string(),
            operands,
            OpCat::Arithmetic,
            addr_mod,
            "Add with Carry".to_string(),
            Flags::trigger(vec!['N', 'V', 'Z', 'C']),
        )
    }

    pub fn sbc(operands: Vec<u8>, addr_mod: AddrMod) -> Instruction {
        Instruction::new(
            "SBC".to_string(),
            operands,
            OpCat::Arithmetic,
            addr_mod,
            "Subtract with Carry".to_string(),
            Flags::trigger(vec!['N', 'V', 'Z', 'C']),
        )
    }

    pub fn cmp(operands: Vec<u8>, addr_mod: AddrMod) -> Instruction {
        Instruction::new(
            "CMP".to_string(),
            operands,
            OpCat::Arithmetic,
            addr_mod,
            "Compare Accumulator".to_string(),
            Flags::trigger(vec!['N', 'Z', 'C']),
        )
    }

    pub fn cpx(operands: Vec<u8>, addr_mod: AddrMod) -> Instruction {
        Instruction::new(
            "CPX".to_string(),
            operands,
            OpCat::Arithmetic,
            addr_mod,
            "Compare X Register".to_string(),
            Flags::trigger(vec!['N', 'Z', 'C']),
        )
    }

    pub fn cpy(operands: Vec<u8>, addr_mod: AddrMod) -> Instruction {
        Instruction::new(
            "CPY".to_string(),
            operands,
            OpCat::Arithmetic,
            addr_mod,
            "Compare Y Register".to_string(),
            Flags::trigger(vec!['N', 'Z', 'C']),
        )
    }

    pub fn inc(operands: Vec<u8>, addr_mod: AddrMod) -> Instruction {
        Instruction::new(
            "INC".to_string(),
            operands,
            OpCat::IncDec,
            addr_mod,
            "Increment Memory Location".to_string(),
            Flags::trigger(vec!['N', 'Z']),
        )
    }

    pub fn inx() -> Instruction {
        Instruction::new(
            "INX".to_string(),
            vec![],
            OpCat::IncDec,
            AddrMod::Implied,
            "Increment X Register".to_string(),
            Flags::trigger(vec!['N', 'Z']),
        )
    }

    pub fn iny() -> Instruction {
        Instruction::new(
            "INY".to_string(),
            vec![],
            OpCat::IncDec,
            AddrMod::Implied,
            "Increment Y Register".to_string(),
            Flags::trigger(vec!['N', 'Z']),
        )
    }

    pub fn dec(operands: Vec<u8>, addr_mod: AddrMod) -> Instruction {
        Instruction::new(
            "DEC".to_string(),
            operands,
            OpCat::IncDec,
            addr_mod,
            "Decrement Memory Location".to_string(),
            Flags::trigger(vec!['N', 'Z']),
        )
    }

    pub fn dex() -> Instruction {
        Instruction::new(
            "DEX".to_string(),
            vec![],
            OpCat::IncDec,
            AddrMod::Implied,
            "Decrement X Register".to_string(),
            Flags::trigger(vec!['N', 'Z']),
        )
    }

    pub fn dey() -> Instruction {
        Instruction::new(
            "DEY".to_string(),
            vec![],
            OpCat::IncDec,
            AddrMod::Implied,
            "Decrement Y Register".to_string(),
            Flags::trigger(vec!['N', 'Z']),
        )
    }

    pub fn asl(operands: Vec<u8>, addr_mod: AddrMod) -> Instruction {
        Instruction::new(
            "ASL".to_string(),
            operands,
            OpCat::Shifts,
            addr_mod,
            "Arithmetic Shift Left".to_string(),
            Flags::trigger(vec!['N', 'Z', 'C']),
        )
    }

    pub fn lsr(operands: Vec<u8>, addr_mod: AddrMod) -> Instruction {
        Instruction::new(
            "LSR".to_string(),
            operands,
            OpCat::Shifts,
            addr_mod,
            "Logical Shift Right".to_string(),
            Flags::trigger(vec!['N', 'Z', 'C']),
        )
    }

    pub fn rol(operands: Vec<u8>, addr_mod: AddrMod) -> Instruction {
        Instruction::new(
            "ROL".to_string(),
            operands,
            OpCat::Shifts,
            addr_mod,
            "Rotate Left".to_string(),
            Flags::trigger(vec!['N', 'Z', 'C']),
        )
    }

    pub fn ror(operands: Vec<u8>, addr_mod: AddrMod) -> Instruction {
        Instruction::new(
            "ROR".to_string(),
            operands,
            OpCat::Shifts,
            addr_mod,
            "Rotate Right".to_string(),
            Flags::trigger(vec!['N', 'Z', 'C']),
        )
    }

    pub fn jmp(operands: Vec<u8>, addr_mod: AddrMod) -> Instruction {
        Instruction::new(
            "JMP".to_string(),
            operands,
            OpCat::JumpCall,
            addr_mod,
            "Jump to New Location".to_string(),
            Flags::new(),
        )
    }

    pub fn jsr(operands: Vec<u8>) -> Instruction {
        Instruction::new(
            "JSR".to_string(),
            operands,
            OpCat::JumpCall,
            AddrMod::Absolute,
            "Jump to New Location Saving Return Address".to_string(),
            Flags::new(),
        )
    }

    pub fn rts() -> Instruction {
        Instruction::new(
            "RTS".to_string(),
            vec![],
            OpCat::JumpCall,
            AddrMod::Implied,
            "Return from Subroutine".to_string(),
            Flags::new(),
        )
    }

    pub fn bcc(operands: Vec<u8>) -> Instruction {
        Instruction::new(
            "BCC".to_string(),
            operands,
            OpCat::Branch,
            AddrMod::Relative,
            "Branch if Carry Clear".to_string(),
            Flags::new(),
        )
    }

    pub fn bcs(operands: Vec<u8>) -> Instruction {
        Instruction::new(
            "BCS".to_string(),
            operands,
            OpCat::Branch,
            AddrMod::Relative,
            "Branch if Carry Set".to_string(),
            Flags::new(),
        )
    }

    pub fn beq(operands: Vec<u8>) -> Instruction {
        Instruction::new(
            "BEQ".to_string(),
            operands,
            OpCat::Branch,
            AddrMod::Relative,
            "Branch if Zero Set".to_string(),
            Flags::new(),
        )
    }

    pub fn bmi(operands: Vec<u8>) -> Instruction {
        Instruction::new(
            "BMI".to_string(),
            operands,
            OpCat::Branch,
            AddrMod::Relative,
            "Branch if Negative Set".to_string(),
            Flags::new(),
        )
    }

    pub fn bne(operands: Vec<u8>) -> Instruction {
        Instruction::new(
            "BNE".to_string(),
            operands,
            OpCat::Branch,
            AddrMod::Relative,
            "Branch if Zero Clear".to_string(),
            Flags::new(),
        )
    }

    pub fn bpl(operands: Vec<u8>) -> Instruction {
        Instruction::new(
            "BPL".to_string(),
            operands,
            OpCat::Branch,
            AddrMod::Relative,
            "Branch if Negative Clear".to_string(),
            Flags::new(),
        )
    }

    pub fn bvc(operands: Vec<u8>) -> Instruction {
        Instruction::new(
            "BVC".to_string(),
            operands,
            OpCat::Branch,
            AddrMod::Relative,
            "Branch if Overflow Clear".to_string(),
            Flags::new(),
        )
    }

    pub fn bvs(operands: Vec<u8>) -> Instruction {
        Instruction::new(
            "BVS".to_string(),
            operands,
            OpCat::Branch,
            AddrMod::Relative,
            "Branch if Overflow Set".to_string(),
            Flags::new(),
        )
    }

    pub fn clc() -> Instruction {
        Instruction::new(
            "CLC".to_string(),
            vec![],
            OpCat::StatusCtrl,
            AddrMod::Implied,
            "Clear Carry Flag".to_string(),
            Flags::trigger(vec!['C']),
        )
    }

    pub fn cld() -> Instruction {
        Instruction::new(
            "CLD".to_string(),
            vec![],
            OpCat::StatusCtrl,
            AddrMod::Implied,
            "Clear Decimal Mode".to_string(),
            Flags::trigger(vec!['D']),
        )
    }

    pub fn cli() -> Instruction {
        Instruction::new(
            "CLI".to_string(),
            vec![],
            OpCat::StatusCtrl,
            AddrMod::Implied,
            "Clear Interrupt Disable Bit".to_string(),
            Flags::trigger(vec!['I']),
        )
    }

    pub fn clv() -> Instruction {
        Instruction::new(
            "CLV".to_string(),
            vec![],
            OpCat::StatusCtrl,
            AddrMod::Implied,
            "Clear Overflow Flag".to_string(),
            Flags::trigger(vec!['V']),
        )
    }

    pub fn sec() -> Instruction {
        Instruction::new(
            "SEC".to_string(),
            vec![],
            OpCat::StatusCtrl,
            AddrMod::Implied,
            "Set Carry Flag".to_string(),
            Flags::trigger(vec!['C']),
        )
    }

    pub fn sed() -> Instruction {
        Instruction::new(
            "SED".to_string(),
            vec![],
            OpCat::StatusCtrl,
            AddrMod::Implied,
            "Set Decimal Mode".to_string(),
            Flags::trigger(vec!['D']),
        )
    }

    pub fn sei() -> Instruction {
        Instruction::new(
            "SEI".to_string(),
            vec![],
            OpCat::StatusCtrl,
            AddrMod::Implied,
            "Set Interrupt Disable".to_string(),
            Flags::trigger(vec!['I']),
        )
    }

    pub fn brk() -> Instruction {
        Instruction::new(
            "BRK".to_string(),
            vec![],
            OpCat::StatusCtrl,
            AddrMod::Implied,
            "Force an Interrupt".to_string(),
            Flags::trigger(vec!['B']),
        )
    }

    pub fn nop() -> Instruction {
        Instruction::new(
            "NOP".to_string(),
            vec![],
            OpCat::StatusCtrl,
            AddrMod::Implied,
            "No Operation".to_string(),
            Flags::new(),
        )
    }

    pub fn rti() -> Instruction {
        Instruction::new(
            "RTI".to_string(),
            vec![],
            OpCat::StatusCtrl,
            AddrMod::Implied,
            "Return from Interrupt".to_string(),
            Flags::trigger_all(),
        )
    }
}
