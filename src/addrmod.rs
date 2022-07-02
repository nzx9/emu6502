#[derive(Debug, PartialEq)]
pub enum AddrMod {
    None,
    Implied,
    Accumulator,
    Immediate,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Relative,
    IndirectX,
    IndirectY,
    Indirect,
    Absolute,
    AbsoluteX,
    AbsoluteY,
}

impl AddrMod {
    pub fn bytes(&self) -> usize {
        match self {
            AddrMod::None => 0,
            AddrMod::Implied => 1,
            AddrMod::Accumulator => 1,
            AddrMod::Immediate => 2,
            AddrMod::ZeroPage => 2,
            AddrMod::ZeroPageX => 2,
            AddrMod::ZeroPageY => 2,
            AddrMod::Relative => 2,
            AddrMod::IndirectX => 2,
            AddrMod::IndirectY => 2,
            AddrMod::Indirect => 3,
            AddrMod::Absolute => 3,
            AddrMod::AbsoluteX => 3,
            AddrMod::AbsoluteY => 3,
        }
    }

    pub fn indicator(&self) -> String {
        match self {
            AddrMod::None => "".to_string(),
            AddrMod::Implied => "".to_string(),
            AddrMod::Accumulator => "".to_string(),
            AddrMod::Immediate => "#".to_string(),
            AddrMod::ZeroPage => "".to_string(),
            AddrMod::ZeroPageX => "".to_string(),
            AddrMod::ZeroPageY => "".to_string(),
            AddrMod::IndirectX => "".to_string(),
            AddrMod::IndirectY => "".to_string(),
            AddrMod::Indirect => "".to_string(),
            AddrMod::Relative => "".to_string(),
            AddrMod::Absolute => "".to_string(),
            AddrMod::AbsoluteX => "".to_string(),
            AddrMod::AbsoluteY => "".to_string(),
        }
    }
}
