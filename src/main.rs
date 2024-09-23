// modules
mod args;
mod stringgeneration;

use std::process::exit;

// calls to modules
use args::*;
use clap::Parser;

fn throwerrors(exitcode: u8) {
    match exitcode {
        1 => eprintln!(
            "Specified no valid encoding. See 'genpassrs string --help' for valid character types."
        ), // No character or invalid type error
        _ => eprintln!("genpassrs failed to recognize this specific error. Weird..."),
    };
    exit(1);
}

fn main() {
    let args = GenpassArgs::parse();

    let mut result_string: String = String::new();
    let min: u8;
    let mut max: u8 = 0;

    // argument match cases
    match args.generate {
        Generate::String(StringArgs) => {
            let space = StringArgs.space;
            match space {
                true => min = 32,
                false => min = 33,
            }
            match StringArgs.encoding.as_str() {
                "ext" | "extasc" => max = 255,
                "asc" | "ascii" => max = 127,
                _ => throwerrors(1),
            }
            result_string = stringgeneration::generator(StringArgs.length, min, max, result_string);
            if args.debug {
                dbg!(min, max, StringArgs.length);
            }
        }
        Generate::Integer(IntegerArgs) => {
            result_string = stringgeneration::intgen(IntegerArgs.length, result_string);

            if args.debug {
                dbg!(IntegerArgs.length);
            }
        }
    }

    //debugging information
    // string is outputted here.
    print!("{result_string}\n");
}
