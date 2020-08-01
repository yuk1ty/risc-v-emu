use risc_v_emu::cpu::Cpu;
use std::env;
use std::fs::File;
use std::io::prelude::Read;
use std::io::Result;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("The size of argument should be 2."); // TODO
    }

    let mut file = File::open(&args[1])?;
    let mut binary = Vec::new();
    file.read_to_end(&mut binary)?;

    let mut cpu = Cpu::new(binary);

    while cpu.pc < cpu.memory.len() as u64 {
        let inst = cpu.fetch();
        cpu.pc += 4;
        cpu.execute(inst)
    }

    cpu.dump_registers();

    Ok(())
}
