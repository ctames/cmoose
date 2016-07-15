mod cpu;
use std::io;

// TODO input loop allowing binary instructions to be typed in and executed
fn main() {
	let mut cpuregs = cpu::Registers::new(); 
	println!("{}", cpuregs);	
	
	let mut input = String::new();
	for {
		try!(io::stdin().read_line(&mut input)); 
	}
}
