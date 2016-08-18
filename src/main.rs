mod cpu;
use std::io;
use std::u16; 
use std::string::String;

/******
Currently, the program is merely an input loop allowing binary instructions to be typed in and executed
either an instruction, "quit", or "status" can be input
instructions are executed, and then the state of the machine is output
"quit", obviously, exits the program
"status" outputs machine state 
******/
fn main() {
	let mut cpuregs = cpu::Registers::new(); 
	println!("{}", cpuregs);	
	
	loop {
		let mut input = String::new();
		match io::stdin().read_line(&mut input) {
			Ok(n) => {
				match input.trim() {
					"quit" => break,
					"status" => println!("{}", cpuregs),
					_ => {
						match u16::from_str_radix(&input.trim(), 2) {
							Ok(binary_instr) => {
								let instr: cpu::Instruction = cpu::Instruction::new(binary_instr);
								cpuregs.ireg = instr;
								cpu::execute(&mut cpuregs); 
								println!("{}", cpuregs);
							},
							Err(error) => println!("error: {}", error), 
						}
					},
				}
			},
			Err(error) => println!("error: {}", error),
		} 
	}
}
