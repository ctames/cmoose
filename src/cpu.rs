use std::fmt;

pub struct Registers {
	registers: Vec<u16>,
	pcreg: u16,
	ireg: u16,
}

// TODO do I really need getters and setters? probably unnecessary code, information hiding not really important here
impl Registers {
	// register[0] is a 0 register
	pub fn new() -> Registers {
		Registers { 
			registers: vec![0; 8],
			pcreg: 0,
			ireg: 0,
		}
	}

	pub fn getreg(self, regnum: usize) -> u16 {
		// ! don't allow values outside of 0:8
		self.registers[regnum]
	}

	pub fn setreg(&mut self, regnum: usize, val: u16) { 
		// ! don't allow values outside of 1:8	
		self.registers[regnum] = val; 
	}
	
	pub fn get_pcreg(self) -> u16 {
		self.pcreg
	}

	pub fn set_pcreg(&mut self, val: u16) {
		self.pcreg = val;
	}
	
	pub fn get_ireg(self) -> u16 {
		self.ireg
	}

	pub fn set_ireg(&mut self, val: u16) {
		self.ireg = val;
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

#[derive(Debug)]
enum Format {
	RRR,
	RRI,
	RI,
}

#[derive(Debug)]
enum Operation {
	ADD,
	ADDI,
	NAND,
	LUI,
	SW,
	LW,
	BEQ,
	JALR,
}

pub struct Instruction {
	format: Format,
	op: Operation,
	regA: u8,
	regB: u8,
	regC: u8,
	immed: u16,
	s_immed: i8,
}

impl Instruction {
	// TODO finish
	pub fn new(binary: u16) -> Instruction {
		let mut new_instruction = Instruction { 
			format: Format::RRR,
			op: Operation::ADD,
			regA: 0,
			regB: 0,
			regC: 0,
			immed: 0,
			s_immed: 0
		};
		
		// get opcode to set operation/format
		match ((binary & 0b1110000000000000) >> 13) {
			0b000 => continue, 
			0b001 => {new_instruction.format = Format::RRI; new_instruction.op = Operation::ADDI},
			0b010 => new_instruction.op = Operation::NAND,
			0b011 => {new_instruction.format = Format::RI; new_instruction.op = Operation::LUI},
			0b100 => {new_instruction.format = Format::RRI; new_instruction.op = Operation::SW},
			0b101 => {new_instruction.format = Format::RRI; new_instruction.op = Operation::LW},
			0b110 => {new_instruction.format = Format::RRI; new_instruction.op = Operation::BEQ},
			0b111 => new_instruction.op = Operation::JALR,
		} 

		// get regA value
		new_instruction.regA = (binary & 0b0001110000000000) >> 10; 

		// if RRR or RRI type, get regB value
		if new_instruction.format == Format::RRR || new_instruction.format == Format::RRI {
			new_instruction.regB = (binary & 0b0000001110000000) >> 7;			
		} 
		
		// if RRR
		if new_instruction.format == Format::RRR {

		}

		// if RRI
		if new_instruction.format == Format::RRI {

		}
		
		// if RI
		if new_instruction.format == Format::RI {

		}
		
		new_instruction
	}

	// TODO getters and setters? if I decide to remove those for Registers, will leave them out here as well 

}	
	
// TODO implement
impl fmt::Display for Instruction {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "\n") 
	}
}
