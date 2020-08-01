const REGISTER_SIZE: usize = 32;
const DEFAULT_MEMORY_SIZE: u64 = 1024 * 1024 * 128;

pub struct Cpu {
    /// 64-bit integer registers
    pub xregs: [u64; REGISTER_SIZE],
    /// program counter
    pub pc: u64,
    /// Memory to store executable instructions
    pub memory: Vec<u8>,
}

impl Cpu {
    /// Create CPU with initializing options.
    pub fn new(binary: Vec<u8>) -> Self {
        let mut xregs = [0; REGISTER_SIZE];
        xregs[2] = DEFAULT_MEMORY_SIZE;

        Self {
            xregs,
            pc: 0,
            memory: binary,
        }
    }

    pub fn fetch(&self) -> u32 {
        let index = self.pc as usize;
        (self.memory[index] as u32)
            | ((self.memory[index + 1] as u32) << 8)
            | ((self.memory[index + 2] as u32) << 16)
            | ((self.memory[index + 3] as u32) << 24)
    }

    pub fn execute(&mut self, inst: u32) {
        let opcode = inst & 0x0000007f;
        let rd = ((inst & 0x00000f80) >> 7) as usize;
        let rs1 = ((inst & 0x000f8000) >> 15) as usize;
        let rs2 = ((inst & 0x01f00000) >> 20) as usize;

        self.xregs[0] = 0;

        match opcode {
            0x13 => {
                let im = ((inst & 0xfff00000) as i32 as i64 >> 20) as u64;
                self.xregs[rd] = self.xregs[rs1].wrapping_add(im);
            }
            0x33 => {
                self.xregs[rd] = self.xregs[rs1].wrapping_add(self.xregs[rs2]);
            }
            _ => {
                unimplemented!("Not implemented yet");
            }
        }
    }

    pub fn dump_registers(&self) {
        let mut output = String::from("");
        for i in (0..32).step_by(4) {
            output = format!(
                "{}\n{}",
                output,
                format!(
                    "x{:02}={:>#18x} x{:02}={:>#18x} x{:02}={:>#18x} x{:02}={:>#18x}",
                    i,
                    self.xregs[i],
                    i + 1,
                    self.xregs[i + 1],
                    i + 2,
                    self.xregs[i + 2],
                    i + 3,
                    self.xregs[i + 3],
                )
            );
        }
        println!("{}", output);
    }
}
