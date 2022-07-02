#[macro_export]
macro_rules! disassemble {
    ($rom:expr) => {
        assembler::disassemble($rom, true)
    };
}

#[macro_export]
macro_rules! check_bit_one {
    ($val:expr, $bit: expr) => {
        $val & (1 << $bit) != 0
    };
}

#[macro_export]
macro_rules! get_bit {
    ($val:expr, $bit: expr) => {
        $val & (1 << $bit)
    };
}
