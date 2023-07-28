use cali::parser::*;
use std::process;
use clap::Parser;

fn main() {
    let args = InputParser::parse();

    if let Err(e) = args.run() {
        println!("Application error: {e}");
        process::exit(1);
    }
}

