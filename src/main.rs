use std::env;
use std::fs;
use std::io::{self, Write};
use std::str::FromStr;
use risc_v_simulator::Cpu;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: cargo run -- <file_path.bin>");
        std::process::exit(1);
    }

    let filename = &args[1];

    let program = match fs::read(filename) {
        Ok(bytes) => bytes,
        Err(e) => {
            eprintln!("Failed to read file '{}': {}", filename, e);
            std::process::exit(1);
        }
    };

    println!("File '{}' loaded. Size: {} bytes.", filename, program.len());

    let mut cpu = Cpu::new(1024 * 4);
    cpu.load_program(&program);
    loop {
        println!("\nOptions: \n 1. step \n 2. show_register \n 3. show_memory \n 4. exit");
        print!("Select an option: ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Error reading input");
        let option: usize = usize::from_str(input.trim()).unwrap_or(0); 

        match option {
            1 => {
                cpu.step();
                println!("Executed 1 step.");
            }
            2 => {
                print!("Enter register number (0-31): ");
                io::stdout().flush().unwrap();
                
                input.clear();
                io::stdin().read_line(&mut input).expect("Error");
                let register: usize = usize::from_str(input.trim()).unwrap_or(99);
                
                if register < 32 {
                    let register_value = cpu.get_register(register);
                    println!("x{}: {} (0x{:08X})", register, register_value as i32, register_value);
                } else {
                    println!("Invalid register!");
                }
            }
            3 => {
                print!("Enter memory address: ");
                io::stdout().flush().unwrap();
                
                input.clear();
                io::stdin().read_line(&mut input).expect("Error");
                let memory_address: usize = usize::from_str(input.trim()).unwrap_or(usize::MAX);
                
                if memory_address < cpu.memory.len() {
                    let memory_value = cpu.get_memory_value(memory_address);
                    println!("Address {}: {} (0x{:02X})", memory_address, memory_value, memory_value);
                } else {
                    println!("Memory address out of bounds!");
                }
            }
            4 => {
                println!("Exiting emulator...");
                break;
            }
            _ => {
                println!("Incorrect option");
            }
        }
    }
}
