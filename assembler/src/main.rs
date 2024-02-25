use oppsy::codes::MainInstructions;
use deku::prelude::*;

fn main() {
    let prog = vec![MainInstructions::LDBC(5), MainInstructions::INCBC, MainInstructions::INCBC];
    let prog: Vec<_> = prog.iter().flat_map(|s| s.to_bytes().unwrap()).collect();
    dbg!(prog);
}
