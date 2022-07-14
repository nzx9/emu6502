mod addrmod;
mod assembler;
mod cpu;
mod flags;
mod instructions;
mod macros;
mod memory;
mod opcat;
mod shell;

use std::env;

fn main() {
    // if let Some(filepath) = env::args().nth(1) {
    //     let rom = assembler::read_rom(&filepath);

    let rom = vec![105, 255, 105, 10, 0x85, 10];
    shell::welcome();

    let mut cpu = cpu::CPU::new(&rom);

    loop {
        let mut input = String::new();
        shell::prompt(&mut input, None);

        let inp: Vec<&str> = input.split(" ").collect();

        match shell::inp(&inp, 0) {
            "exit" => break,
            "disassemble" => {
                let disasm = assembler::disassemble(&rom, true);
                println!("{}", disasm);
            }
            "run" => {
                cpu.run();
            }
            "step" => {
                cpu.step();
            }
            "rest" => cpu.reset(),
            "show" => match shell::inp(&inp, 1) {
                "accu" => cpu.show_accu(),
                "flags" => cpu.show_flags(),
                "" => cpu.show_state(),
                _ => println!("Invalid Command"),
            },
            "settings" => {}
            _ => {
                println!("Invalid Command, try again");
            }
        }
    }
    // } else {
    //     println!("No argument provided");
    // }
}
