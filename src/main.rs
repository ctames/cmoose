mod cpu;
use std::env;
use std::io;
use std::u16; 
use std::string::String;

fn main() {

	let options: Vec<_> = env::args().collect();
	
	if options.len() != 2 {
		println!("invalid arguments - proper usage: ./cmoose inputfile");		
	}
	
	let prog = cpu::Program::new(options[1].clone()).unwrap(); 		
	
	let mut cpuregs = cpu::Registers::new(); 
	println!("{}", cpuregs);	
	
	loop {
		cpuregs.ireg = prog.source[cpuregs.pcreg as usize].clone();
		cpu::execute(&mut cpuregs); 
		println!("{}", cpuregs);
		cpuregs.pcreg = cpuregs.pcreg + 1; 
		if cpuregs.pcreg == prog.source.len() as i16 {
			println!("reached end of source, halting");
			break;
		}
	}
}
