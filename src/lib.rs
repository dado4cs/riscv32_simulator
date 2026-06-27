use wasm_bindgen::prelude::*;
    const ABI_NAMES: [&str; 32] = [
    "zero", "ra", "sp", "gp", "tp", "t0", "t1", "t2",
    "s0", "s1", "a0", "a1", "a2", "a3", "a4", "a5",
    "a6", "a7", "s2", "s3", "s4", "s5", "s6", "s7",
    "s8", "s9", "s10", "s11", "t3", "t4", "t5", "t6"
    ];

#[wasm_bindgen]
pub struct Cpu {
    pub pc: u32,
    registers: [u32; 32],
    memory: Vec<u8>,
    pub is_halted: bool,
}

#[wasm_bindgen]
impl Cpu {
    #[wasm_bindgen(constructor)]
    pub fn new(memory_size: usize) -> Self {
        let mut cpu = Cpu {
            pc: 0,
            registers: [0; 32],
            memory: vec![0; memory_size],
            is_halted: false,
        };
        cpu.registers[0] = 0;
        cpu.registers[2] = memory_size as u32;
        cpu
    }
    pub fn load_program(&mut self, program: &[u8]) {
        for (i, &byte) in program.iter().enumerate() {
            if i < self.memory.len() {
                self.memory[i] = byte;
            }
        }
    }

    pub fn step(&mut self) -> Option<String> {
        if self.is_halted {return None;}

        let instruction = self.fetch();
        self.decode_and_execute(instruction)
    }

    fn fetch(&self) -> u32 {
        let pc = self.pc as usize;
        let b0 = self.memory[pc] as u32;
        let b1 = self.memory[pc + 1] as u32;
        let b2 = self.memory[pc + 2] as u32;
        let b3 = self.memory[pc + 3] as u32;

        b3 << 24 | b2 << 16 | b1 << 8 | b0
    }

    fn decode_and_execute(&mut self, instruction: u32) -> Option<String> {
        let op = instruction & 0x7F;
        let rd = ((instruction >> 7) & 0x1F) as usize;
        let rs1 = ((instruction >> 15) & 0x1F) as usize;
        let rs2 = ((instruction >> 20) & 0x1F) as usize;
        let funct3 = ((instruction >> 12) & 0x7) as u8;
        let funct7 = ((instruction >> 25) & 0x7F) as u8;
        let mut next_pc = self.pc + 4;
        let mut log_message: Option<String> = None;

        match op {
            // load-instructions
            3 => {
                let imm = ((instruction as i32) >> 20) as u32;
                let target = self.registers[rs1].wrapping_add(imm) as usize;
                match funct3 {
                    // lb
                    0b000 => self.registers[rd] = self.memory_load(target,0),
                    // lh
                    0b001 => self.registers[rd] = self.memory_load(target,1),
                    // lw
                    0b010 => self.registers[rd] = self.memory_load(target,2),
                    // lbu
                    0b100 => self.registers[rd] = self.memory_load(target,3),
                    // lhu
                    0b101 => self.registers[rd] = self.memory_load(target,4),
                    _ => {todo!("Unknown instruction")}

                }

            },
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
            // auipc
            23 => {
                let imm = instruction & 0xFFFFF000;
                self.registers[rd] = imm + self.pc;
            }
            // S-type
            35 => {
                let imm11_5 = instruction & 0xFE000000;
                let imm4_0 = (instruction << 13) & 0x01F00000;
                let imm_ns = (imm11_5 | imm4_0) as i32;
                let imm = (imm_ns >> 20) as u32;
                let target = self.registers[rs1].wrapping_add(imm) as usize;
                match funct3 {
                    // sb
                    0b000 => self.memory_save(target, self.registers[rs2],0),
                    // sh
                    0b001 => self.memory_save(target, self.registers[rs2], 1),
                    // sw
                    0b010 => self.memory_save(target, self.registers[rs2], 2),
                    _ => {todo!("Unknown instruction")}
                }
            }
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
            // lui
            55 => {
                let imm = instruction & 0xFFFFF000;
                self.registers[rd] = imm;
            }
            // B-type
            99 => {
                let immb12 = instruction & 0x80000000;
                let immb11 = (instruction << 23) & 0x40000000;
                let immb10_5 = (instruction >> 1) & 0x3F000000;
                let immb4_1 = (instruction << 12) & 0x00F00000;
                let imm_ns = (immb12 | immb11 | immb10_5 | immb4_1) as i32;
                let imm = (imm_ns >> 19) as u32;
                match funct3 {
                    0b000 => if self.registers[rs1] == self.registers[rs2] {
                        next_pc = self.pc.wrapping_add(imm);
                    },
                    0b001 => if self.registers[rs1] != self.registers[rs2] {
                        next_pc = self.pc.wrapping_add(imm);
                    },
                    0b100 => if (self.registers[rs1] as i32) < (self.registers[rs2] as i32) {
                        next_pc = self.pc.wrapping_add(imm);
                    },
                    0b101 => if (self.registers[rs1] as i32) >= (self.registers[rs2] as i32) {
                        next_pc = self.pc.wrapping_add(imm);
                    },
                    0b110 => if self.registers[rs1] < self.registers[rs2] {
                        next_pc = self.pc.wrapping_add(imm);
                    }
                    0b111 => if self.registers[rs1] >= self.registers[rs2] {
                        next_pc = self.pc.wrapping_add(imm);
                    }
                    _ => {todo!("Unknown instruction")}

                }
            }
            // jalr
            103 => if funct3 == 0b000{
                let imm = ((instruction as i32) >> 20) as u32;
                next_pc = self.registers[rs1].wrapping_add(imm) & 0xFFFFFFFE;
                self.registers[rd] = self.pc + 4;
            } else {
                todo!("Unknown instruction")
            },
            // jal
            111 => {
                let imm20 = instruction & 0x80000000;
                let imm19_12 = (instruction << 11) & 0x7F800000;
                let imm11 = (instruction << 2) & 0x00400000;
                let imm10_1 = (instruction >> 9) & 0x003FF000;
                let imm_ns = (imm20 | imm19_12 | imm11 | imm10_1) as i32;
                let imm = (imm_ns >> 11) as u32;
                next_pc = self.pc.wrapping_add(imm);
                self.registers[rd] = self.pc + 4;
            }
            // priviliged instructions
            115 => {
                let imm = ((instruction as i32) >> 20) as u32;
                match funct3 {
                    0b000 => match imm {
                        // ecall
                        0 => match self.registers[17]{
                            // print integer
                            1 => {
                                log_message = Some(format!("Console: {}", self.registers[10] as i32));
                            }
                            // exit program
                            93 => {
                                let exit_code = self.registers[10];
                                self.is_halted = true;
                                log_message = Some(format!("Program exited with code: {}", exit_code));
                            }
                            _ => {}
                        }
                        // ebreak
                        1 => {todo!("ebreak")}
                        _ => {}
                    }
                    // csrrw
                    0b001 => {todo!("csrrw")}
                    // csrrs
                    0b010 => {todo!("csrrs")}
                    // csrrc
                    0b011 => {todo!("csrrc")}
                    // csrrwi
                    0b101 => {todo!("csrrwi")}
                    // csrrsi
                    0b110 => {todo!("csrrsi")}
                    // csrrci
                    0b111 => {todo!("csrrci")}
                    _ => {}
                }
            }
            _ => {todo!("Unknown instruction")}
        }
        self.registers[0] = 0;
        self.pc = next_pc;
        log_message
    }


    fn memory_save(&mut self, address: usize, value: u32, save_type: u8) {
        if address + 3 >= self.memory.len() {
            self.is_halted = true;
            return;
        }

        match save_type {
            0 => self.memory[address] = (value & 0xFF) as u8,
            1 => {
                let b0 = (value & 0xFF) as u8;
                let b1 = ((value & 0xFF00) >> 8)  as u8;
                self.memory[address] = b0;
                self.memory[address+1] = b1;
            }
            2 => {
                let b0 = (value & 0xFF) as u8;
                let b1 = ((value & 0xFF00) >> 8)  as u8;
                let b2 = ((value & 0xFF0000) >> 16)  as u8;
                let b3 = ((value & 0xFF000000) >> 24)  as u8;
                self.memory[address] = b0;
                self.memory[address+1] = b1;
                self.memory[address+2] = b2;
                self.memory[address+3] = b3;
            }
            _ => {}
        }
    }

    fn memory_load(&self, address: usize, load_type: u8) -> u32 {
        if address + 3 >= self.memory.len() {
            return 0;
        }
        match load_type {
            0 => {
                let b0 = (self.memory[address] as u32) << 24;
                ((b0 as i32) >> 24) as u32
            }
            1 => {
                let b0 = self.memory[address] as u32;
                let b1 = self.memory[address + 1] as u32;
                let value = (((b1 << 8) | b0) << 16) as i32;
                (value >> 16) as u32
            }
            2 => {
                let b0 = self.memory[address] as u32;
                let b1 = self.memory[address + 1] as u32;
                let b2 = self.memory[address + 2] as u32;
                let b3 = self.memory[address + 3] as u32;
                (b3 << 24) | (b2 << 16) | (b1 << 8) | b0

            }
            3 => {
                self.memory[address] as u32
            }
            4 => {
                let b0 = self.memory[address] as u32;
                let b1 = self.memory[address + 1] as u32;
                (b1 << 8) | b0
            }
            _ => {unreachable!()}
        }
    }
    pub fn get_register(&self, register: usize) -> u32 {
        self.registers[register]
    }
    pub fn get_memory_value(&self, address: usize) -> u8{
        self.memory[address]
    }



    pub fn disassemble_all(&self) -> Vec<String> {
        let mut instructions = Vec::new();
        let mut current_addr = 0;

        while current_addr + 3 < self.memory.len() {
            let b0 = self.memory[current_addr] as u32;
            let b1 = self.memory[current_addr + 1] as u32;
            let b2 = self.memory[current_addr + 2] as u32;
            let b3 = self.memory[current_addr + 3] as u32;
            let instruction = b3 << 24 | b2 << 16 | b1 << 8 | b0;
            let mnemonic = if instruction == 0 {
                String::from("nop")
            } else {
                self.decode_single(instruction)
            };
            instructions.push(format!("0x{:04X}: {}", current_addr, mnemonic));
            current_addr += 4;
        }

        instructions}

    fn decode_single(&self, instruction: u32) -> String {
        let op = instruction & 0x7F;
        let rd = ((instruction >> 7) & 0x1F) as usize;
        let rs1 = ((instruction >> 15) & 0x1F) as usize;
        let rs2 = ((instruction >> 20) & 0x1F) as usize;
        let funct3 = ((instruction >> 12) & 0x7) as u8;
        let funct7 = ((instruction >> 25) & 0x7F) as u8;

        match op {
            3 => {
                let imm = (instruction as i32) >> 20;
                match funct3 {
                    0b000 => format!("lb {}, {}({})", ABI_NAMES[rd], imm, ABI_NAMES[rs1]),
                    0b001 => format!("lh {}, {}({})", ABI_NAMES[rd], imm, ABI_NAMES[rs1]),
                    0b010 => format!("lw {}, {}({})", ABI_NAMES[rd], imm, ABI_NAMES[rs1]),
                    0b100 => format!("lbu {}, {}({})", ABI_NAMES[rd], imm, ABI_NAMES[rs1]),
                    0b101 => format!("lhu {}, {}({})", ABI_NAMES[rd], imm, ABI_NAMES[rs1]),
                    _ => String::from("unknown load"),
                }
            },
            19 => {
                let imm = (instruction as i32) >> 20;
                match funct3 {
                    0b000 => format!("addi {}, {}, {}", ABI_NAMES[rd], ABI_NAMES[rs1], imm),
                    0b001 => format!("slli {}, {}, {}", ABI_NAMES[rd], ABI_NAMES[rs1], imm & 0x1F),
                    0b010 => format!("slti {}, {}, {}", ABI_NAMES[rd], ABI_NAMES[rs1], imm),
                    0b011 => format!("sltiu {}, {}, {}", ABI_NAMES[rd], ABI_NAMES[rs1], imm),
                    0b100 => format!("xori {}, {}, {}", ABI_NAMES[rd], ABI_NAMES[rs1], imm),
                    0b101 => match funct7 {
                        0x0 => format!("srli {}, {}, {}", ABI_NAMES[rd], ABI_NAMES[rs1], imm & 0x1F),
                        0x20 => format!("srai {}, {}, {}", ABI_NAMES[rd], ABI_NAMES[rs1], imm & 0x1F),
                        _ => String::from("unknown shift"),
                    },
                    0b110 => format!("ori {}, {}, {}", ABI_NAMES[rd], ABI_NAMES[rs1], imm),
                    0b111 => format!("andi {}, {}, {}", ABI_NAMES[rd], ABI_NAMES[rs1], imm),
                    _ => String::from("unknown I-type"),
                }
            },
            23 => {
                let imm = (instruction & 0xFFFFF000) >> 12;
                format!("auipc {}, 0x{:X}", ABI_NAMES[rd], imm)
            },
            35 => {
                let imm11_5 = instruction & 0xFE000000;
                let imm4_0 = (instruction << 13) & 0x01F00000;
                let imm_ns = (imm11_5 | imm4_0) as i32;
                let imm = imm_ns >> 20;
                match funct3 {
                    0b000 => format!("sb {}, {}({})", ABI_NAMES[rs2], imm, ABI_NAMES[rs1]),
                    0b001 => format!("sh {}, {}({})", ABI_NAMES[rs2], imm, ABI_NAMES[rs1]),
                    0b010 => format!("sw {}, {}({})", ABI_NAMES[rs2], imm, ABI_NAMES[rs1]),
                    _ => String::from("unknown S-type"),
                }
            },
            51 => {
                match funct3 {
                    0b000 => match funct7 {
                        0x0 => format!("add {}, {}, {}", ABI_NAMES[rd], ABI_NAMES[rs1], ABI_NAMES[rs2]),
                        0x20 => format!("sub {}, {}, {}", ABI_NAMES[rd], ABI_NAMES[rs1], ABI_NAMES[rs2]),
                        _ => String::from("unknown R-type add/sub"),
                    },
                    0b001 => format!("sll {}, {}, {}", ABI_NAMES[rd], ABI_NAMES[rs1], ABI_NAMES[rs2]),
                    0b010 => format!("slt {}, {}, {}", ABI_NAMES[rd], ABI_NAMES[rs1], ABI_NAMES[rs2]),
                    0b011 => format!("sltu {}, {}, {}", ABI_NAMES[rd], ABI_NAMES[rs1], ABI_NAMES[rs2]),
                    0b100 => format!("xor {}, {}, {}", ABI_NAMES[rd], ABI_NAMES[rs1], ABI_NAMES[rs2]),
                    0b101 => match funct7 {
                        0x0 => format!("srl {}, {}, {}", ABI_NAMES[rd], ABI_NAMES[rs1], ABI_NAMES[rs2]),
                        0x20 => format!("sra {}, {}, {}", ABI_NAMES[rd], ABI_NAMES[rs1], ABI_NAMES[rs2]),
                        _ => String::from("unknown R-type shift"),
                    },
                    0b110 => format!("or {}, {}, {}", ABI_NAMES[rd], ABI_NAMES[rs1], ABI_NAMES[rs2]),
                    0b111 => format!("and {}, {}, {}", ABI_NAMES[rd], ABI_NAMES[rs1], ABI_NAMES[rs2]),
                    _ => String::from("unknown R-type"),
                }
            },
            55 => {
                let imm = (instruction & 0xFFFFF000) >> 12;
                format!("lui {}, 0x{:X}", ABI_NAMES[rd], imm)
            },
            99 => {
                let immb12 = instruction & 0x80000000;
                let immb11 = (instruction << 23) & 0x40000000;
                let immb10_5 = (instruction >> 1) & 0x3F000000;
                let immb4_1 = (instruction << 12) & 0x00F00000;
                let imm_ns = (immb12 | immb11 | immb10_5 | immb4_1) as i32;
                let imm = imm_ns >> 19;
                match funct3 {
                    0b000 => format!("beq {}, {}, {}", ABI_NAMES[rs1], ABI_NAMES[rs2], imm),
                    0b001 => format!("bne {}, {}, {}", ABI_NAMES[rs1], ABI_NAMES[rs2], imm),
                    0b100 => format!("blt {}, {}, {}", ABI_NAMES[rs1], ABI_NAMES[rs2], imm),
                    0b101 => format!("bge {}, {}, {}", ABI_NAMES[rs1], ABI_NAMES[rs2], imm),
                    0b110 => format!("bltu {}, {}, {}", ABI_NAMES[rs1], ABI_NAMES[rs2], imm),
                    0b111 => format!("bgeu {}, {}, {}", ABI_NAMES[rs1], ABI_NAMES[rs2], imm),
                    _ => String::from("unknown B-type"),
                }
            },
            103 => {
                if funct3 == 0b000 {
                    let imm = (instruction as i32) >> 20;
                    format!("jalr {}, {}({})", ABI_NAMES[rd], imm, ABI_NAMES[rs1])
                } else {
                    String::from("unknown jalr")
                }
            },
            111 => {
                let imm20 = instruction & 0x80000000;
                let imm19_12 = (instruction << 11) & 0x7F800000;
                let imm11 = (instruction << 2) & 0x00400000;
                let imm10_1 = (instruction >> 9) & 0x003FF000;
                let imm_ns = (imm20 | imm19_12 | imm11 | imm10_1) as i32;
                let imm = imm_ns >> 11;
                format!("jal {}, {}", ABI_NAMES[rd], imm)
            },
            115 => {
                let imm = (instruction as i32) >> 20;
                match funct3 {
                    0b000 => match imm {
                        0 => String::from("ecall"),
                        1 => String::from("ebreak"),
                        _ => String::from("unknown system"),
                    },
                    0b001 => String::from("csrrw"),
                    0b010 => String::from("csrrs"),
                    0b011 => String::from("csrrc"),
                    0b101 => String::from("csrrwi"),
                    0b110 => String::from("csrrsi"),
                    0b111 => String::from("csrrci"),
                    _ => String::from("unknown CSR"),
                }
            },
            _ => format!("Unknown Opcode: 0x{:X}", op),
        }
    }
}

