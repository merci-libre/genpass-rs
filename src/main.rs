// modules
mod args;
mod stringgeneration;
mod zxcvbn;

use std::process::exit;

// calls to modules
use args::*;
use clap::Parser;

fn throwerrors(exitcode: u8) {
    match exitcode {
        1 => eprintln!(
            "Specified no valid encoding. See 'genpassrs string --help' for valid character types."
        ), // No character or invalid type error
        2 => eprintln!("Error: cannot parse an empty string."),
        _ => eprintln!("genpassrs failed to recognize this specific error. Weird..."),
    };
    exit(1);
}

fn main() {
    // parse the arguments for clap
    let args = GenpassArgs::parse();

    let mut result_string: String = String::new();
    // to make things a bit more readable.
    let debug = args.debug;
    let command = args.generate;
    //min and max value of inputted chars, enumerates spaces and encoding.
    let min: u8;
    let mut max: u8 = 0;

    // argument match cases
    match command {
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
            if debug {
                dbg!(min, max, StringArgs.length);
            }
        }
        Generate::Integer(IntegerArgs) => {
            result_string = stringgeneration::intgen(IntegerArgs.length, result_string);

            if debug {
                dbg!(IntegerArgs.length);
            }
        }
        Generate::Alphanumeric(AlphaArgs) => {
            // enumerate arguments
            let mut char_min: u8 = 48;
            let mut char_max: u8 = 123;

            if AlphaArgs.alphabet {
                char_min = 65;
            }
            if AlphaArgs.smallcase || AlphaArgs.upper {
                char_max = 90;
            }
            result_string =
                stringgeneration::alphanumeric(AlphaArgs.length, char_min, char_max, result_string);
            // manage letter cases.
            if AlphaArgs.upper && !AlphaArgs.smallcase {
                result_string = result_string.to_uppercase();
            }
            if AlphaArgs.smallcase && !AlphaArgs.upper {
                result_string = result_string.to_lowercase();
            }
            if debug {
                dbg!(char_min, char_max, AlphaArgs.length);
            }
        }
        Generate::Estimate(EstimateArgs) => {
            if EstimateArgs.string.is_empty() {
                throwerrors(2);
            }

            let score = zxcvbn::estimate(EstimateArgs.string.to_string());
            print!(
                "zxcvbn score for '{}': {}\n",
                EstimateArgs.string.to_string(),
                score
            );
            match score.into() {
                0 => println!(
                    "This password is extremely weak, as it would take at least 100 guesses to crack."
                ),
                1 => println!(
                    "This password is moderately weak, as it would take at least 100,000 guesses to crack."
                ),
                2 => println!(
                    "This password is slightly strong, as it would take at least 100,000,000 guesses to crack."
                ),
                3 => println!(
                    "This password is strong, as it would take at least 10,000,000,000 guesses to crack."
                ),
                4 => println!(
                    "This password is extremely strong, as it would take more than 10,000,000,000 guesses to crack."
                ),
                _ =>todo!()
            }
        }
    }

    //debugging information
    // string is outputted here.
    if result_string != "" {
        print!("{result_string}\n");
    }
}
