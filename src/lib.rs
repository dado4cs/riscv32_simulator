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
        let pc = self.pc as usize;
        let b0 = self.memory[pc] as u32;
        let b1 = self.memory[pc + 1] as u32;
        let b2 = self.memory[pc + 2] as u32;
        let b3 = self.memory[pc + 3] as u32;

        b3 << 24 | b2 << 16 | b1 << 8 | b0
    }

    fn decode_and_execute(&mut self, instruction: u32) {
        let op = instruction & 0x7F;
        let rd = ((instruction >> 7) & 0x1F) as usize;
        let rs1 = ((instruction >> 15) & 0x1F) as usize;
        let rs2 = ((instruction >> 20) & 0x1F) as usize;
        let funct3 = ((instruction >> 12) & 0x7) as u8;
        let funct7 = ((instruction >> 25) & 0x7F) as u8;

        match op {
            3 => {}
            // I-type
            19 => {
                let imm = ((instruction as i32) >> 20) as u32;
                match funct3 {
                    // addi
                    0b000 => self.registers[rd] = self.registers[rs1].wrapping_add(imm),
                    // slli
                    0b001 => self.registers[rd] = self.registers[rs1] << (imm & 0x1F),
                    // slti
                    0b010 => self.registers[rd] = ((self.registers[rs1] as i32) < (imm as i32)) as u32,
                    // sltiu
                    0b011 => self.registers[rd] = (self.registers[rs1] < imm) as u32,
                    // xori
                    0b100 => self.registers[rd] = self.registers[rs1] ^ imm,
                    0b101 => match funct7 {
                        //srli
                        0x0 => self.registers[rd] = self.registers[rs1] >> (imm & 0x1F),
                        // srai
                        0x20 => self.registers[rd] = ((self.registers[rs1] as i32) >> (imm & 0x1F)) as u32,
                        _ => {todo!("Unknown instruction")}
                    }
                    // ori
                    0b110 => self.registers[rd] = self.registers[rs1] | imm,
                    // andi
                    0b111 => self.registers[rd] = self.registers[rs1] & imm,
                    _ => {todo!("Unknown instruction")}
                }
            }
            23 => {}
            35 => {}
            // R-type
            51 => match funct3 {
                0b000 => match funct7 {
                    // add
                    0x0 => self.registers[rd] = self.registers[rs1].wrapping_add(self.registers[rs2]),
                    // sub
                    0x20 => self.registers[rd] = self.registers[rs1].wrapping_sub(self.registers[rs2]),

                    _ => {todo!("Unknown instruction")}
                },
                // sll
                0b001 => self.registers[rd] = self.registers[rs1] << (self.registers[rs2] & 0x1F),
                // slt
                0b010 => self.registers[rd] = ((self.registers[rs1] as i32)< (self.registers[rs2] as i32)) as u32,
                // sltu
                0b011 => self.registers[rd] = (self.registers[rs1] < self.registers[rs2]) as u32,
                // xor
                0b100 => self.registers[rd] = self.registers[rs1] ^ self.registers[rs2],
                0b101 => match funct7 {
                    // srl
                    0x0 => self.registers[rd] = self.registers[rs1] >> (self.registers[rs2] & 0x1F),
                    // sra
                    0x20 => self.registers[rd] = ((self.registers[rs1] as i32) >> (self.registers[rs2] & 0x1F)) as u32,
                    _ => {todo!("Unknown instruction")}
                    }
                // or
                0b110 => self.registers[rd] = self.registers[rs1] | self.registers[rs2],
                // and
                0b111 => self.registers[rd] = self.registers[rs1] & self.registers[rs2],
                _ => {todo!("Unknown instruction")}
                },  
            99 => {}
            103 => {}
            111 => {}
            _ => {todo!("Unknown instruction")}
        }
        self.registers[0] = 0;
    }

    fn memory_save(&mut self, address: u32, value: u32, save_type: u8) {
        todo!("save");
    }

    fn memory_load(&self, address: u32, load_type: u8) -> u32 {
        todo!("load");
    }
}
