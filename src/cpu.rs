use std::io::prelude::*;
use std::io::BufReader;
use std::fmt;
use std::fs::OpenOptions;
use std::option::Option;
use std::string::String;
use std::ops::Index;
use std::process::exit;

// this is decalared as a hacky way to get out of a lifetime problem later
static ZERO: i16 = 0;

// CYCLES PER STAGE PER INSTRUCTION
// ideally, you would be able pass a config to set cycles required per instr per stage
// taking the easy way out right now and hardcoding it
static addc:  [u8; 5] = [1, 1, 2, 1, 1]; 
static addic: [u8; 5] = [1, 1, 2, 1, 1]; 
static nandc: [u8; 5] = [1, 1, 2, 1, 1]; 
static luic:  [u8; 5] = [1, 1, 2, 1, 1]; 
static swc:   [u8; 5] = [1, 1, 2, 3, 1]; 
static lwc:   [u8; 5] = [1, 1, 2, 3, 1]; 
static beqc:  [u8; 5] = [1, 1, 1, 1, 1]; 
static jalrc: [u8; 5] = [1, 1, 1, 1, 1];
static haltc: [u8; 5] = [1, 1, 1, 1, 1];

///////////
//	PROGRAM STRUCT
//  represents a program as the source instructions and local memory as one struct
//  instructions start at address 0, data at source_len
//  instructions not readable or loadable, only memory
//  ie. sw 1,0,17 will store value from <Register>.registers[1] to address 17, if 
//  it is not an instruction space
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
		let mut count = 0;
		for line in reader.lines() {
			match Instruction::new(line.unwrap().trim()) {
				Ok(mut instr) => {
					instr.address = count;
					prog.source.push(instr);
					count = count + 1;							 
				}
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

//////////
//	REGISTERS STRUCT
//////////

pub struct Registers {
	pub registers: Vec<i16>,
	pub pcreg: i16,
}

impl Registers {
	// register[0] is a 0 register
	pub fn new() -> Registers {
		Registers { 
			registers: vec![0; 8],
			pcreg: 0,
		}
	}
}

impl fmt::Display for Registers {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		println!("CURRENT REGISTERS STATE");
		for x in 0..8 {
			write!(f, "gpreg{} : {}\n", x, self.registers[x]);
		}
		write!(f, "pcreg: {}\n", self.pcreg);
	}
}

//////////
// ENUMS FOR INSTRUCTION STRUCT
//////////

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone)]
enum Format {
	RRR,
	RRI,
	RI
}

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone)]
enum Operation {
	ADD,
	ADDI,
	NAND,
	LUI,
	SW,
	LW,
	BEQ,
	JALR,
	HALT
}

//////////
//	INSTRUCTION STRUCT
//////////

#[derive(Clone)]
pub struct Instruction {
	rep: String,
	format: Format,
	op: Operation,
	reg_a: usize,
	reg_b: usize,
	reg_c: usize,
	s_immed: i8,
	u_immed: u16,
	immed: i16,
	address: i16,
}

impl Instruction {
	// FOR CREATING AN INSTRUCTION FROM ASSEMBLY
	pub fn new(line: &str) -> Result<Instruction, &'static str> {
		let mut newin = Instruction { 
			rep: String::from(line), 
			format: Format::RRR,
			op: Operation::ADD,
			reg_a: 0,
			reg_b: 0,
			reg_c: 0,
			s_immed: 0,
			u_immed: 0,
			immed: 0,
			address: 0
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
				newin.op = Operation::NAND;
				newin.reg_a = usize::from_str_radix(fields_split[0], 10).unwrap();  	
				newin.reg_b = usize::from_str_radix(fields_split[1], 10).unwrap();
				newin.reg_c = usize::from_str_radix(fields_split[2], 10).unwrap();
			}
			"lui"    => {
				newin.format = Format::RI;
				newin.op = Operation::LUI;
				newin.reg_a = usize::from_str_radix(fields_split[0], 10).unwrap();  	
				newin.u_immed = u16::from_str_radix(fields_split[1], 10).unwrap(); 
			} 			
			"sw"     => { 			
				newin.format = Format::RRI;
				newin.op = Operation::SW;
				newin.reg_a = usize::from_str_radix(fields_split[0], 10).unwrap();  	
				newin.reg_b = usize::from_str_radix(fields_split[1], 10).unwrap();
				newin.s_immed = i8::from_str_radix(fields_split[2], 10).unwrap(); 
			}
			"lw"     => { 			
				newin.format = Format::RRI;
				newin.op = Operation::LW;
				newin.reg_a = usize::from_str_radix(fields_split[0], 10).unwrap();  	
				newin.reg_b = usize::from_str_radix(fields_split[1], 10).unwrap();
				newin.s_immed = i8::from_str_radix(fields_split[2], 10).unwrap(); 
			}
			"beq"    => { 			
				newin.format = Format::RRI;
				newin.op = Operation::BEQ;
				newin.reg_a = usize::from_str_radix(fields_split[0], 10).unwrap();  	
				newin.reg_b = usize::from_str_radix(fields_split[1], 10).unwrap();
				newin.s_immed = i8::from_str_radix(fields_split[2], 10).unwrap(); 
			}
			"jalr"   => { 			
				newin.format = Format::RRI;
				newin.op = Operation::JALR;
				newin.reg_a = usize::from_str_radix(fields_split[0], 10).unwrap();  	
				newin.reg_b = usize::from_str_radix(fields_split[1], 10).unwrap();
			}
			"nop"    => {
				// for a nop, replace with add 0,0,0
			} 			
			"halt"   => {
				newin.op = Operation::HALT;
			}			
			"lli"    => {
				// for a lli, replace with addi
				newin.format = Format::RRI;
				newin.op = Operation::ADDI;
				newin.reg_a = usize::from_str_radix(fields_split[0], 10).unwrap();  		
				newin.reg_b = usize::from_str_radix(fields_split[0], 10).unwrap();  	
				newin.s_immed = i8::from_str_radix(fields_split[1], 10).unwrap();
			}			
			_ => {
				// anything else? invalid operation, exit
				return Err("invalid operation")
			} 			
		}
		Ok(newin)		
	}	
}
	
impl fmt::Display for Instruction {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{:?}", self.rep) 
	}
}

///////////
// STAGE STRUCT
///////////

#[derive(Clone)]
pub struct Stage {
    instr: Option<Instruction>,
    ce: u8,
    cn: u8
}

impl Stage {
    pub fn set(&self, instr: &Instruction, stage: usize) {
        self.instr = Some(instr.clone());
        self.ce = 0;
        match instr.op {
            Operation::ADD => self.cn = addc[stage],
            Operation::ADDI => self.cn = addic[stage],
            Operation::NAND => self.cn = nandc[stage],
            Operation::LUI => self.cn = luic[stage],
            Operation::SW => self.cn = swc[stage],
            Operation::LW => self.cn = lwc[stage],
            Operation::BEQ => self.cn = beqc[stage],
            Operation::JALR => self.cn = jalrc[stage],
			Operation::HALT => self.cn = haltc[stage],
		}
    }   
}

//////////
// PIPELINE STRUCT
//////////

pub struct Pipeline  {
    stages: [Stage; 5],
    regs: Registers, 
	prog: Program, 
	cycle: u8
}

impl Pipeline {
    pub fn new(filename: String) -> Pipeline {
        let mut stage = Stage {
            instr: None,
            ce: 0,
            cn: 0,
        };
        Pipeline {
            stages: [stage.clone(), stage.clone(), stage.clone(), stage.clone(), stage.clone()],
            regs: Registers::new(),
			prog: Program::new(filename.clone()).unwrap(),
			cycle: 0
        }
    }

    // THIS FUNCTION IS WHAT "RUNS" THE PIPELINE
    // In terms of the approach, the idea is to "simulate" the work being done by the imaginary units
    // of the cpu by waiting a number of cycles corresponding to the stage and the instruction.
    // The result of a stage is effected (ie, register writing for WB)
    // only once the stage is completed
    // For a single cycle, stages are handled serially from the "top down", since for an instruction to
    // move in the next stage, the instruction in front of me must be ready to move on as well
    // The RR, WB, and ME (register erad, write back and memory access) stages are where real results are propagated,
    // ie. a memory location updated by SW at completion of ME stage, register updates in (everything but SW, JALR, BEQ)
	// WB, and BEQ, JALR in RR (decode) 
	pub fn cycle(&self) {
	    // HANDLE WB        
        if self.stages[4].instr.is_some() {
            if self.stages[4].ce == self.stages[4].cn {
                let instr = self.stages[4].instr.unwrap();
                match instr.op {
					Operation::ADD => execute(self.instr, self.regs, self.prog),
					Operation::ADDI => execute(self.instr, self.regs, self.prog),
					Operation::NAND => execute(self.instr, self.regs, self.prog),
					Operation::LUI => execute(self.instr, self.regs, self.prog),
					Operation::LW => execute(self.instr, self.regs, self.prog),
					Operation::HALT => execute(self.instr, self.regs, self.prog),
					_ => ()
				}
				self.stages[4].instr = None;
            }
        }

        // HANDLE ME
        if self.stages[3].instr.is_some() {
            if self.stages[3].ce == self.stages[3].cn {
                let instr = self.stages[3].instr.unwrap();
				match instr.op {
					Operation::SW => execute(self.instr, self.reg, self.prog),
					_ => ()
				}
			}
            if self.stages[3].ce >= self.stage[3].cn  && self.stages[4].instr.is_none() {
                let instr = self.stages[3].instr.unwrap();
                self.stages[4].set(instr, 4);
                self.stages[3].instr = None;
            }
        }

        // HANDLE EX
        // funnily enough, for our purposes, nothing is really done here. just fake computation latency
		if self.stages[2].instr.is_some() {
            if self.stages[2].ce >= self.stage[2].cn  && self.stages[3].instr.is_none() {
                let instr = self.stages[2].instr.unwrap();
                self.stages[3].set(instr, 3);
                self.stages[2].instr = None;
            }
        }

        // HANDLE RR
        // THIS STAGE HANDLES JUMPS AND BRANCHES
		if self.stages[1].instr.is_some() {
            if self.stage[1].ce == self.stages[1].cn {
                let instr = self.stages[1].instr.unwrap();
				match instr.op {
					Operation::BEQ | Operation::JALR => {
						execute(self.instr, self.reg, self.prog);
						self.stages[0].instr = Some(Instruction::new("add 0,0,0"));
					}
					_ => ()
				}
			}
			if self.stages[1].ce >= self.stage[1].cn  && self.stages[2].instr.is_none() {
                let instr = self.stages[1].instr.unwrap();
                self.stages[2].set(instr, 2);
                self.stages[1].instr = None;
            }
        }
        
		// HANDLE IF
		if self.stages[0].instr.is_some() {
            if self.stages[0].ce >= self.stage[0].cn  && self.stages[1].instr.is_none() {
                let instr = self.stages[0].instr.unwrap();
                self.stages[1].set(instr, 1);
				if self.regs.pcreg < self.prog.source_len {
					self.stages[0].instr = Some(self.prog.source[self.regs.pcreg]);		
					self.regs.pcreg = self.regs.pcreg + 1;
				}
			}
		} 
		else {
			if self.regs.pcreg < self.prog.source_len {
				self.stages[0].instr = Some(self.prog.source[self.reg.pcreg]);		
				self.regs.pcreg = self.regs.pcreg + 1;
			}
		}

        self.cycle = self.cycle + 1;
        for stage in self.stages {
            self.stage.ce = self.stage.ce + 1;
        }
    }
}

impl fmt::Display for Pipeline {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let default = Instruction::new("add 0,0,0");
		write!(f, "STATE AFTER CYCLE: {:?}\n ", self.cycle); 
		write!(f, "IF: {:?} - cycles {:?} out of {:?}\n", self.stages[0].instr.unwrap_or(default), self.stages[0].ce, self.stages[0].cn);
		write!(f, "ID: {:?} - cycles {:?} out of {:?}\n", self.stages[1].instr.unwrap_or(default), self.stages[1].ce, self.stages[1].cn);
		write!(f, "EX: {:?} - cycles {:?} out of {:?}\n", self.stages[2].instr.unwrap_or(default), self.stages[2].ce, self.stages[2].cn);
		write!(f, "ME: {:?} - cycles {:?} out of {:?}\n", self.stages[3].instr.unwrap_or(default), self.stages[3].ce, self.stages[3].cn);
		write!(f, "Wb: {:?} - cycles {:?} out of {:?}\n", self.stages[4].instr.unwrap_or(default), self.stages[4].ce, self.stages[4].cn);
		write!(f, "{:?}", self.regs)
	}
}

///////////
// EXECUTE INSTRUCTION FUNCTION
//////////

pub fn execute(instr: &Instruction, cpuregs: &mut Registers, prog: &mut Program) { 
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
				regs[instr.reg_a] = (instr.u_immed << 6) as i16;
			}
		},
		Operation::SW => {
			// can only store words in prog.memory
			// if address is within the source, does nothing
			let address = regs[instr.reg_b] + instr.s_immed as i16; 
			if address >= prog.source_len as i16 {
				prog.data[address as usize - prog.source_len] = regs[instr.reg_a];
			}
		},
		Operation::LW => {
			// can only load words from memory or fill instructions
			// loads a 0 if trying to load from a normal instruction in prog.source
			let address = regs[instr.reg_b] + instr.s_immed as i16; 
			if instr.reg_a != 0 {
				regs[instr.reg_a] = prog[address as usize];	
			}
		},
		Operation::BEQ => {
			if regs[instr.reg_a] == regs[instr.reg_b] {
				cpuregs.pcreg = instr.address + 1 + instr.s_immed as i16;				
			}
		},
		Operation::JALR => {
			if instr.reg_a != 0 {
				regs[instr.reg_a] = instr.address + 1;
			}
			cpuregs.pcreg = instr.address+1;
		},
		Operation::HALT => {
			println!("HALT has reached WB stage - exiting!");
			exit(0);			
		}
	}
}
