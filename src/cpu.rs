// TODO DOCUMENTATION

// MAJOR TODO IMPLEMENT MAIN MEMORY 

use std::io::prelude::*;
use std::io::BufReader;
use std::fmt;
use std::fs::OpenOptions;
use std::string::String;

///////////
//	PROGRAM STRUCT
///////////

pub struct Program {
	pub source: Vec<Instruction>,
	pub data: Vec<i16>,
	pub source_len: usize, 
}

impl Program {
	pub fn new(filename: String) -> Result<Program, &'static str> {
		let mut prog = Program { 
			source: Vec::new(),
			data: Vec::new(),
			source_len: 0,
		};
		let file = OpenOptions::new().read(true).open(filename);
		let mut reader = BufReader::new(file.unwrap());
		for line in reader.lines() {
			match Instruction::new(line.unwrap().trim()) {
				Ok(instr) => prog.source.push(instr),							 
				Err(error) => {
					return Err("errors in source file")
				}	   	
			}
		}
	
		prog.source_len = prog.source.len();	
		prog.data.extend_from_slice(&[0; 1000]);
		Ok(prog)
	}
}

impl Index<usize> for Program {
	type Output = i16;

	fn (index(&self, loc: usize) -> &i16 {
		if loc < self.source_len {
			
		}
	}

}

//////////
//	REGISTERS STRUCT
//////////

pub struct Registers {
	pub registers: Vec<i16>,
	pub pcreg: i16,
	pub ireg: Instruction,
}

impl Registers {
	// register[0] is a 0 register
	pub fn new() -> Registers {
		Registers { 
			registers: vec![0; 8],
			pcreg: 0,
			// TODO change to use updated "new" function" perhaps
			ireg: Instruction::new_from_binary(0b0000000000000000),
		}
	}
}

impl fmt::Display for Registers {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		println!("CURRENT CPUREGS STATE");
		for x in 0..8 {
			write!(f, "gpreg{} : {}\n", x, self.registers[x]);
		}
		write!(f, "pcreg: {}\n", self.pcreg);
		write!(f, "ireg: {}\n", self.ireg)
	}
}

//////////
// ENUMS FOR INSTRUCTION STRUCT
/////////

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone)]
enum Format {
	RRR,
	RRI,
	RI
}

#[derive(Debug)]
#[derive(Clone)]
enum Operation {
	ADD,
	ADDI,
	NAND,
	LUI,
	SW,
	LW,
	BEQ,
	JALR
}

//////////
//	INSTRUCTION STRUCT
//////////

#[derive(Clone)]
pub struct Instruction {
	binary_rep: u16,
	format: Format,
	op: Operation,
	reg_a: usize,
	reg_b: usize,
	reg_c: usize,
	s_immed: i8,
	u_immed: u16,
}

impl Instruction {
	// FOR CREATING AN INSTRUCTION FROM ASSEMBLY
	pub fn new(line: &str) -> Result<Instruction, &'static str> {
		let mut newin = Instruction { 
			binary_rep: 0, 
			format: Format::RRR,
			op: Operation::ADD,
			reg_a: 0,
			reg_b: 0,
			reg_c: 0,
			s_immed: 0,
			u_immed: 0
		};

		let mut line_split: Vec<&str> = line.split(" ").collect();
		let mut fields_split: Vec<&str> = vec![];
		if line_split.len() == 2 {
			fields_split = line_split[1].split(",").collect();
		}
		match line_split[0] {
			"add"    => {
				newin.reg_a = usize::from_str_radix(fields_split[0], 10).unwrap();  	
				newin.reg_b = usize::from_str_radix(fields_split[1], 10).unwrap();
				newin.reg_c = usize::from_str_radix(fields_split[2], 10).unwrap();
			}			
			"addi"   => {
				newin.format = Format::RRI;
				newin.op = Operation::ADDI;
				newin.reg_a = usize::from_str_radix(fields_split[0], 10).unwrap();  	
				newin.reg_b = usize::from_str_radix(fields_split[1], 10).unwrap();
				newin.s_immed = i8::from_str_radix(fields_split[2], 10).unwrap(); 
			}			
			"nand"   => { 			
				newin.reg_a = usize::from_str_radix(fields_split[0], 10).unwrap();  	
				newin.reg_b = usize::from_str_radix(fields_split[1], 10).unwrap();
				newin.reg_c = usize::from_str_radix(fields_split[2], 10).unwrap();
			}
			"lui"    => {
				newin.reg_a = usize::from_str_radix(fields_split[0], 10).unwrap();  	
				newin.u_immed = u16::from_str_radix(fields_split[1], 10).unwrap(); 
			} 			
			/*
			"sw"     => 			
			"lw"     => 			
			"beq"    => 			
			"jalr"   => 			
			"nop"    => 			
			"halt"   => 			
			"lli"    => 			
			"movi"   => 			
			".fill"  => 			
			".space" =>
			*/	
			_ => {
				return Err("invalid operation")
			} 			
		}
		Ok(newin)		
	}	

	// FOR CREATING AN INSTRUCTION FROM BINARY
	// NOT CURRENTLY USED IN ACTUAL OPERATION (PARSING A SOURCE FILE)
	// DOES NOT SUPPORT DIRECTIVES OR SPECIAL OPERATIONS
	pub fn new_from_binary(binary: u16) -> Instruction {
		let mut new_instruction = Instruction { 
			binary_rep: binary, 
			format: Format::RRR,
			op: Operation::ADD,
			reg_a: 0,
			reg_b: 0,
			reg_c: 0,
			s_immed: 0,
			u_immed: 0
		};
		
		// get opcode to set operation/format
		match (binary & 0b1110000000000000) >> 13 {
			0b000 => {}, 
			0b001 => {new_instruction.format = Format::RRI; new_instruction.op = Operation::ADDI},
			0b010 => new_instruction.op = Operation::NAND,
			0b011 => {new_instruction.format = Format::RI; new_instruction.op = Operation::LUI},
			0b100 => {new_instruction.format = Format::RRI; new_instruction.op = Operation::SW},
			0b101 => {new_instruction.format = Format::RRI; new_instruction.op = Operation::LW},
			0b110 => {new_instruction.format = Format::RRI; new_instruction.op = Operation::BEQ},
			0b111 => new_instruction.op = Operation::JALR,
			_ => println!("error: couldn't determine instruction format"),
		} 

		// get regA value
		new_instruction.reg_a = ((binary & 0b0001110000000000) >> 10) as usize; 

		// if RRR or RRI type, get regB value
		if new_instruction.format == Format::RRR || new_instruction.format == Format::RRI {
			new_instruction.reg_b = ((binary & 0b0000001110000000) >> 7) as usize;			
		} 
		
		// if RRR
		if new_instruction.format == Format::RRR {
			new_instruction.reg_c = (binary & 0b0000000000000111) as usize;
		}

		// if RRI
		if new_instruction.format == Format::RRI {
			new_instruction.s_immed = (binary & 0b0000000001111111) as i8;
		}
		
		// if RI
		if new_instruction.format == Format::RI {
			new_instruction.u_immed = (binary & 0b0000001111111111) as u16;
		}
		
		new_instruction
	}
}	
	
impl fmt::Display for Instruction {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "format: {:?}\n op: {:?}\n reg_a: {}\n reg_b: {}\n reg_c: {}\n s_immed: {}\n u_immed: {}\n",
			self.format, self.op, self.reg_a, self.reg_b, self.reg_c, self.s_immed, self.u_immed) 
	}
}

// TODO finish
///////////
// EXECUTE INSTRUCTION FUNCTION
//////////

pub fn execute(cpuregs: &mut Registers) {
	let ref instr = cpuregs.ireg; 
	let ref mut regs = cpuregs.registers; 
	match instr.op {
		Operation::ADD => {
			if instr.reg_a != 0 {
				regs[instr.reg_a] = regs[instr.reg_b] + regs[instr.reg_c];	
			}
		},
		Operation::ADDI => {
			if instr.reg_a != 0 {
				regs[instr.reg_a] = regs[instr.reg_b] + instr.s_immed as i16;
			}
		},
		Operation::NAND => {
			if instr.reg_a != 0 {
				regs[instr.reg_a] =  !(regs[instr.reg_b] & regs[instr.reg_c]);
			}
		},
		Operation::LUI => {
			if instr.reg_a != 0 {
				regs[instr.reg_a] = (regs[instr.reg_b] << 6) as i16;
			}
		},
		Operation::SW => {
			// TODO implement after Main Memory is implemented
		},
		Operation::LW => {
			// TODO implement after Main Memory is implemented
		},
		Operation::BEQ => {
			// TODO implement after Main Memory is implemented
		},
		Operation::JALR => {
			// TODO implement after Main Memory is implemented
		},
	}
}
