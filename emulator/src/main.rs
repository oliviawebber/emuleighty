use deku::prelude::*;
use oppsy::codes::MainInstructions;
use std::env;
use std::fs;
use std::process::exit;
use std::io::{stdin, Read};

#[derive(Debug)]
struct State {
    a: u8,
    h: u8,
    l: u8,
}

#[derive(Debug)]
struct Flags {
    z: bool,
}

struct Emulator {
    memory: [u8; 65536],
    stack: Vec<u16>,
    pc: usize,
    state: State,
    flags: Flags,
}



impl Emulator {
    pub fn new(program: Vec<u8>) -> Emulator {
        let mut memory = [0u8; 65536];
        memory[..program.len()].copy_from_slice(&program);
        Emulator {
            memory: memory,
            stack: Vec::new(),
            pc: 0,
            state: State { a: 0, h: 0, l: 0 },
            flags: Flags { z: false }
        }
    }

    fn do_op(self, debug: bool) -> Emulator {
        let (_result, instruction) =
            MainInstructions::from_bytes((&self.memory, 8 * self.pc)).expect("Invalid Op Code");
        if debug {
            println!("{:?} {:X?}", instruction, instruction.to_bytes().unwrap());
        }
        self.update_state(instruction)
    }

    fn update_state(mut self, instruction: MainInstructions) -> Emulator {
        self.pc += match instruction {
            MainInstructions::NOP => {
                1
            },
            MainInstructions::LDBC(_) => todo!(),
            MainInstructions::INCBC => todo!(),
            MainInstructions::LDHL(val) => {
                self.state.h = (val >> 8) as u8;
                self.state.l = val as u8;
                3
            },
            MainInstructions::INCHL => {
                let val = (self.get_hl_addr() + 1) as u16;
                self.state.h = (val >> 8) as u8;
                self.state.l = val as u8;
                1
            }
            MainInstructions::INCA => {
                self.state.a += 1;
                1
            }
            MainInstructions::LDA(val) => {
                self.state.a = val;
                2
            },
            MainInstructions::HALT => {
                exit(0)
            }
            MainInstructions::LDHLA => {
                self.memory[self.get_hl_addr()] = self.state.a;
                1
            },
            MainInstructions::LDAHL => {
                self.state.a = self.memory[self.get_hl_addr()];
                1
            },
            MainInstructions::XORA => {
                self.state.a ^= self.state.a;
                1
            }
            MainInstructions::JPN(val) => {
                self.pc = val as usize;
                0
            },
            MainInstructions::RETZ => {
                if self.flags.z {
                    let val = self.stack.pop().expect("No stack value.");
                    self.pc = val as usize;
                    0
                } else {
                    1
                }
            }
            MainInstructions::CALL(addr) => {
                self.stack.push(self.pc as u16 + 3);
                self.pc = addr as usize;
                0
            },
            MainInstructions::OUT(device) => {
                match device {
                    1 => {
                        let out_char = char::from_u32(self.state.a as u32).unwrap();
                        print!("{}", out_char)
                    },
                    _ => todo!(),
                }
                2
            },
            MainInstructions::POPHL => {
                let val = self.stack.pop().expect("No value on stack!");
                self.state.h = (val >> 8) as u8;
                self.state.l = val as u8;
                1
            },
            MainInstructions::PUSHHL => {
                let hl = self.get_hl_addr() as u16;
                self.stack.push(hl);
                1
            },
            MainInstructions::CPN(val) => {
                let cmp = self.state.a.wrapping_sub(val);
                if cmp == 0 {
                    self.flags.z = true;
                }
                else {
                    self.flags.z = false;
                }
                2
            }
        };
        self
    }

    fn get_state(&self) -> () {
        println!("PC: {}", self.pc);
        println!("FLAGS:\n {:?}", self.flags);
        println!("STATE:\n {:?}", self.state);
        println!("STACK:\n {:X?}", self.stack);
    }

    fn get_hl_addr(&self) -> usize {
        u16::from_le_bytes([self.state.l, self.state.h]) as usize
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        exit(1);
    }
    let debug = args.len() == 3;
    let file = &args[1];
    let prog = fs::read(file).expect("Should be able to read file!");
    let mut emulator = Emulator::new(prog);
    loop {
        if debug {
            println!("---");
            emulator.get_state();
            stdin().read(&mut [0]).unwrap();
        }
        emulator = emulator.do_op(debug);
    }
}
