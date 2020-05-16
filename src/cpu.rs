const REGISTER_SIZE: usize = 32;
const DEFAULT_MEMORY_SIZE: u64 = 1024 * 1024 * 128;

pub struct Cpu {
    /// 64-bit integer registers
    xregs: [u64; REGISTER_SIZE],
    /// program counter
    pc: u64,
    /// Memory to store executable instructions
    memory: Vec<u8>,
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
}
