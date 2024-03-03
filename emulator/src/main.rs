use deku::prelude::*;
use oppsy::codes::MainInstructions;
use std::env;
use std::fs;
use std::process::exit;

#[derive(Debug)]
struct State {
    a: u8,
    h: u8,
    l: u8,
}

struct Emulator {
    program: Vec<u8>,
    heap: [u8; 65536],
    stack: Vec<u16>,
    pc: usize,
    state: State,
}



impl Emulator {
    pub fn new(program: Vec<u8>) -> Emulator {
        Emulator {
            program: program,
            heap: [0_u8; 65536],
            stack: Vec::new(),
            pc: 0,
            state: State { a: 0, h: 0, l: 0 },
        }
    }

    fn do_op(self) -> Emulator {
        let (_result, instruction) =
            MainInstructions::from_bytes((&self.program, 8 * self.pc)).expect("Invalid Op Code");
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
            }
            MainInstructions::LDHLA => {
                self.heap[self.get_hl_addr()] = self.state.a;
                1
            },
            MainInstructions::LDAHL => {
                self.state.a = self.heap[self.get_hl_addr()];
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
            MainInstructions::CALL(addr) => {
                self.stack.push(addr);
                self.pc = addr as usize;
                0
            },
            MainInstructions::OUT(device) => {
                match device {
                    1 => println!("OUT: {}", self.state.a),
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
            }
        };
        self
    }

    fn get_state(&self) -> () {
        println!("PC: {}", self.pc);
        println!("STATE:\n {:?}", self.state);
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
    let file = &args[1];
    let prog = fs::read(file).expect("Should be able to read file!");
    let mut emulator = Emulator::new(prog);
    loop {
        println!("---");
        emulator = emulator.do_op();
        emulator.get_state();
    }
}
