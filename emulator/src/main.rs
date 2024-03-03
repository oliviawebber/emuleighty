use oppsy::codes::MainInstructions;
use std::fs;
use std::env;
use std::io::Read;
use deku::prelude::*;
use std::process::exit;

#[derive(Debug)]
struct State {
    a: u8,
}

struct Emulator {
    program: Vec<u8>,
    pc: usize,
    state: State,
}

fn update_state(emulator: Emulator, instruction: MainInstructions) -> Emulator {
    match instruction {
        MainInstructions::NOP => {
            Emulator {
                program: emulator.program,
                pc: emulator.pc + 1,
                state: emulator.state,
            }
        },
        MainInstructions::LDBC(_) => todo!(),
        MainInstructions::INCBC => todo!(),
        MainInstructions::INCA => {
            let state = State {
                a: emulator.state.a + 1,
            };
            Emulator {
                program: emulator.program,
                pc: emulator.pc + 1,
                state,
            }
        }
        MainInstructions::LDA(val) => {
            let state = State {
                a: val,
            };
            Emulator {
                program: emulator.program,
                pc: emulator.pc + 2,
                state,
            }
        },
        MainInstructions::OUT(device) => {
            match device {
                1 => println!("OUT: {}", emulator.state.a),
                _ => todo!()
            }
            Emulator {
                program: emulator.program,
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
            pc: 0,
            state: State {
                a: 0
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
