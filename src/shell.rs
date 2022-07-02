use std::io::{self, Write};

pub fn welcome() {
    println!("emu6502 Emulator, Disassembler and Debugger");
}

pub fn prompt(buf: &mut String, msg: Option<&str>) -> usize {
    if msg.is_none() {
        print!("$(emu6502)> ");
    } else {
        print!("{}", msg.unwrap());
    }
    io::stdout().flush().expect("Can't write to the shell");
    io::stdin().read_line(buf).unwrap()
}

pub fn inp<'a>(inputs: &'a Vec<&str>, loc: usize) -> &'a str {
    if loc >= inputs.len() {
        ""
    } else {
        inputs[loc].trim()
    }
}
