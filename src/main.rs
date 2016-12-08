mod cpu;
extern crate getopts;
use getopts::Options;
use std::option::Option;
use std::env;
use std::io;
use std::u16; 
use std::string::String;

// ideally, you would pass a config to set cycles required per instr per stage
// taking the easy way out right now and hardcoding it
static addc: [u8; 5]  = []; 
static addic: [u8; 5] = []; 
static nandc: [u8; 5] = []; 
static luic: [u8; 5]  = []; 
static swc: [u8; 5]   = []; 
static lwc: [u8; 5]   = []; 
static beqc: [u8; 5]  = []; 
static jalrc: [u8; 5] = []; 

///////////
// STAGE STRUCT
///////////

#[derive(Clone)]
pub struct Stage {
	instr: Option<cpu::Instruction>,
	ce: u8,
	cn: u8
}

impl Stage {
	pub fn set(&self, instr: &cpu::Instruction, stage) {
		self.instr = Some(instr.clone());
		self.ce = 0;
		match instr.op {
			cpu::Operation::ADD => self.cn = addc[stage],
			cpu::Operation::ADDI => self.cn = addic[stage],
			cpu::Operation::NAND => self.cn = nandc[stage],
			cpu::Operation::LUI => self.cn = luic[stage],
			cpu::Operation::SW => self.cn = swc[stage],
			cpu::Operation::LW => self.cn = lwc[stage],
			cpu::Operation::BEQC => self.cn = beqc[stage],
			cpu::Operation::JALRC => self.cn = jalrc[stage],
		}		
	}
}

//////////
// PIPELINE STRUCT
//////////

pub struct Pipeline {
	stages: [Stage; 5],
	cycle: u8
}

impl Pipeline {
	pub fn new() -> Pipeline {
		let mut stage = Stage {
			instr: None,
			ce: 0,
			cn: 0,
		};
		Pipeline {
			stages: [stage.clone(); 5],
			current_cycle: 0;
		}
	}

	// THIS FUNCTION IS WHAT "RUNS" THE PIPELINE
	// In terms of the approach, the idea is to "simulate" the work being done by the imaginary units
	// of the cpu by waiting a number of cycles corresponding to the stage and the instruction.
	// The result of a stage is effected (ie, register writing for WB)
	// only once the stage is completed
	// For a single cycle, stages are handled serially from the "top down", since for an instruction to
	// move in the next stage, the instruction in front of me must be ready to move on as well
	// The WB and ME (write back and memory access) stages are where real results are propagated,
	// ie. a memory location updated by SW at completion of ME stage, register update in 
	
	pub fn cycle(&self) {

		// HANDLE WB		
		match self.stages[4].instr {
			Some(instr) => {
				if self.stages[4].ce == self.stages[4].cn {
					// TODO EXECUTE REGISTER WRITE - POSSIBLY REWITE/REDESIGN CPU::EXECUTE
					self.instr = None; 
				} 
			}
			None => ()
		} 	

		// HANDLE ME
		match self.stages[3].instr {
			Some(instr) => {
				if self.stages[3].ce == self.stages[3].cn {
					// TODO DO MEMORY ACCESSES
				} 
				if self.stages[3].ce >= self.stage[3].cn  && self.stages[4].instr == None {
					self.stages[4].set(instr, 4);
					self.instr = None;
				}
			}
			None => ()
		}
 	
		// HANDLE EX
		match self.stages[2].instr {
			Some(instr) => {
				if self.stages[2].ce >= self.stage[2].cn  && self.stages[3].instr == None {
					self.stages[3].set(instr, 3);
					self.instr = None;
				}
			}
			None => ()
		}

		// HANDLE RR
		match self.stages[1].instr {
			Some(instr) => {
				if self.stages[1].ce >= self.stage[1].cn  && self.stages[2].instr == None {
					self.stages[2].set(instr, 2);
					self.instr = None;
				}
			}
			None => ()
		}

		// HANDLE IF		
		match self.stages[0].instr {
			Some(instr) => {
				if self.stages[0].ce >= self.stage[0].cn  && self.stages[1].instr == None {
					self.stages[1].set(instr, 1);
					self.instr = None;
				}
			}
			None => {
				// TODO GET NEXT INSTRUCTION	
			}
		}
		 
		self.cycle = self.cycle + 1;
		for stage in stages {
			stage.ce = stage.ce +1;
		}
				
	}
}

//////////
// MAIN
//////////

// TODO REVAMP TO USE PIPLINE
fn main() {
	let options: Vec<_> = env::args().collect();
		
	let mut prog = cpu::Program::new(options[1].clone()).unwrap(); 		
	
	let mut cpuregs = cpu::Registers::new(); 
	println!("{}", cpuregs);	
	
	loop {
		cpuregs.ireg = prog.source[cpuregs.pcreg as usize].clone();
		cpu::execute(&mut cpuregs, &mut prog); 
		println!("{}", cpuregs);
		cpuregs.pcreg = cpuregs.pcreg + 1; 
		if cpuregs.pcreg == prog.source_len as i16 {
			println!("reached end of source, halting");
			break;
		}
	}
}
