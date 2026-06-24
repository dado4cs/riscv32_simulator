use std::env;
use std::fs;
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
        cpu.step();
    }
}
