// modules
mod args;
mod stringgeneration;

use std::process::exit;

// calls to modules
use args::GenpassArgs;
use clap::Parser;

fn throwerrors(exitcode: u8) {
    match exitcode {
        1 => eprintln!(
            "Specified no valid encoding. See 'genpassrs --help' for valid character types."
        ), // No character or invalid type error
        2 => eprintln!(
            "Error, {} larger than {}, exiting gracefully.",
            GenpassArgs::parse().length,
            std::u8::MAX
        ),
        _ => eprintln!("genpassrs failed to recognize this specific error. Weird..."),
    };
    exit(1);
}
fn main() {
    let args = GenpassArgs::parse();
    let mut string: String = String::new();
    let min: u8;
    let mut max: u8 = 0;

    match args.space {
        true => min = 32,
        false => min = 33,
    };
    match args.encoding.as_str() {
        "extasc" | "extascii" => max = 255,
        "ascii" | "asc" => max = 127,
        _ => throwerrors(1),
    };

    if args.length > std::u8::MAX {
        throwerrors(2);
    }

    // string is outputted here.
    string = stringgeneration::generator(args.length, min, max, string);
    //debugging information
    if args.debug {
        dbg!(min, max, args.space);
    }
    print!("{string}\n");

    // generate the random characters.
}
