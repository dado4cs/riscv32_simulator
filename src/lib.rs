pub struct Cpu {
    pub pc: u32,
    pub registers: [u32; 32],
    pub memory: Vec<u8>,
}

impl Cpu {
    pub fn new(memory_size: usize) -> Self {
        let mut cpu = Cpu {
            pc: 0,
            registers: [0; 32],
            memory: vec![0; memory_size],
        };
        cpu.registers[0] = 0;
        cpu
    }
    pub fn load_program(&mut self, program: &[u8]) {
        for (i, &byte) in program.iter().enumerate() {
            if i < self.memory.len() {
                self.memory[i] = byte;
            }
        }
    }

    pub fn step(&mut self) {
        let instruction = self.fetch();
        self.decode_and_execute(instruction);
    }

    fn fetch(&self) -> u32 {
        todo!("fetch");
    }

    fn decode_and_execute(&mut self, instruction: u32) {
        todo!("decode");
    }

    fn memory_save(&mut self, address: u32, value: u32, save_type: u8) {
        todo!("save");
    }

    fn memory_load(&self, address: u32, load_type: u8) -> u32 {
        todo!("load");
    }
}
