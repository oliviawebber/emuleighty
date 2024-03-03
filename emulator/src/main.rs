use oppsy::codes::MainInstructions;
use std::fs;
use std::env;
use deku::prelude::*;
use std::process::exit;

#[derive(Debug)]
struct State {
    a: u8,
    h: u8,
    l: u8,
}

struct Emulator {
    program: Vec<u8>,
    stack: [u8; 65536],
    pc: usize,
    state: State,
}

fn update_state(emulator: Emulator, instruction: MainInstructions) -> Emulator {
    match instruction {
        MainInstructions::NOP => {
            Emulator {
                program: emulator.program,
                stack: emulator.stack,
                pc: emulator.pc + 1,
                state: emulator.state,
            }
        },
        MainInstructions::LDBC(_) => todo!(),
        MainInstructions::INCBC => todo!(),
        MainInstructions::LDHL(val) => {
            let h = (val >> 8) as u8;
            let l = val as u8;
            let state = State {
                a: emulator.state.a,
                h,
                l,
            };
            Emulator {
                program: emulator.program,
                stack: emulator.stack,
                pc: emulator.pc + 3,
                state,
            }
        }
        MainInstructions::INCA => {
            let state = State {
                a: emulator.state.a + 1,
                h: emulator.state.h,
                l: emulator.state.l,
            };
            Emulator {
                program: emulator.program,
                stack: emulator.stack,
                pc: emulator.pc + 1,
                state,
            }
        }
        MainInstructions::LDA(val) => {
            let state = State {
                a: val,
                h: emulator.state.h,
                l: emulator.state.l,
            };
            Emulator {
                program: emulator.program,
                stack: emulator.stack,
                pc: emulator.pc + 2,
                state,
            }
        },
        MainInstructions::LDHLA => {
            let mut stack = emulator.stack;
            stack[emulator.get_hl_addr()] = emulator.state.a;
            Emulator {
                program: emulator.program,
                stack: stack,
                pc: emulator.pc + 1,
                state: emulator.state,
            }
        }
        MainInstructions::XORA => {
            let state = State {
                a: emulator.state.a ^ emulator.state.a,
                h: emulator.state.h,
                l: emulator.state.l,
            };
            Emulator {
                program: emulator.program,
                stack: emulator.stack,
                pc: emulator.pc + 1,
                state: state,
            }
        }
        MainInstructions::JPN(val) => {
            Emulator {
                program: emulator.program,
                stack: emulator.stack,
                pc: val as usize,
                state: emulator.state,
            }
        }
        MainInstructions::OUT(device) => {
            match device {
                1 => println!("OUT: {}", emulator.state.a),
                _ => todo!()
            }
            Emulator {
                program: emulator.program,
                stack: emulator.stack,
                pc: emulator.pc + 2,
                state: emulator.state,
            }
        },
    }
}

impl Emulator {
    pub fn new(program: Vec<u8>) -> Emulator {
        Emulator {
            program: program,
            stack: [0_u8; 65536],
            pc: 0,
            state: State {
                a: 0,
                h: 0,
                l: 0,
            },
        }
    }

    fn do_op(self) -> Emulator {
        let (_result, instruction) = MainInstructions::from_bytes((&self.program, 8*self.pc)).expect("Invalid Op Code");
        update_state(self, instruction)
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
