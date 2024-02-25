use std::io::Write;

use deku::prelude::*;
use oppsy::codes::MainInstructions;
use patharg::{InputArg, OutputArg};

use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Input file, use '-' for stdin
    #[arg(value_parser)]
    input: InputArg,

    /// Output file '-' for stdout
    #[arg(long, short, value_parser, default_value = "a.out")]
    output: OutputArg,

    /// Whether to overwrite existing output file
    #[arg(long, value_parser, default_value = "false")]
    overwrite: bool,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let prog = vec![
        MainInstructions::LDBC(5),
        MainInstructions::INCBC,
        MainInstructions::INCBC,
    ];
    let prog: Vec<_> = prog.iter().flat_map(|s| s.to_bytes().unwrap()).collect();
    let mut f = args.output.create()?;
    f.write_all(&prog)?;
    Ok(())
}
