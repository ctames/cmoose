use std::fmt;

pub struct Registers {
	registers: Vec<u16>,
	pcreg: u16,
	ireg: u16,
}

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
		for x in 0..8 {
			write!(f, "{} : {}\n", x, self.registers[x]);
		}
		write!(f, "pcreg: {}\n", self.pcreg);
		write!(f, "ireg: {}\n", self.ireg)
	}
}
