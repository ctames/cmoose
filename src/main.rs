mod cpu;
use std::env;
use std::io;
use std::process::exit;

//////////
// MAIN
// Take in a source file, parse in to a Program
// Create a set of registers
// Pass both of these to pipeline constructor
// Run the program through the pipeline, prompting user to continue or quit
//////////

fn main() {
	let options: Vec<_> = env::args().collect();
	let mut pipeline = cpu::Pipeline::new(options[1].clone());
	let mut input = String::new();
	loop {
		pipeline.cycle();
		println!("{}\n\n\n", pipeline);
		io::stdin().read_line(&mut input);
		match input.trim() {
			"" => (),
			_  => exit(1)
		}		
	}	
}
