// modules
mod args;
mod mime;
mod steganographic;
mod stringgeneration;
mod zxcvbn;

use std::{path::Path, process::exit};

// calls to modules
use args::*; // we can safely glob this
use clap::Parser;

struct Information {
    name: String,
    version: String,
    author: String,
    contact: String,
}
/*
 To make things a bit more readable, I rewrote most of my comments inside of the program,
 because I completely forgot what most things did in here. I realized that most of my comments
 that I had originally made were kinda sh*t so I'm going to best explain what my thought
 process was when creating this program.

 The main file just does all the argument and error handling for the majority of the program.
 It comprises of 2 primary functions:

 - main() // matches command line arguments to their specified modules. Provides both parameters,
             among other things.


 There will be comments mostly explaining how or why things are set not only in this file, but in
 stringgeneration.rs, db.rs, and whichever modules that may or may not come up.

 Additionally, I will be creating better documentation on my website, permitted that I have the
 time to do so.

*/

// rewrite this as a method.. use Result Types.

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // parse the arguments for clap
    let args = GenpassArgs::parse();
    let info: Information = Information {
        name: String::from("genpass-rs"),
        version: String::from("1.4.0"),
        author: String::from("Westwardfishdme/Finch"),
        contact: String::from("westwardfishme@gmail.com"),
    };
    eprintln!(
        "{} v.{} \n- Developed by: {}\n- Contact: {}\n",
        info.name, info.version, info.author, info.contact
    );

    let mut result_string: String = String::new();
    let debug = args.debug;
    let command = args.commands;
    let lp = args.r#loop;
    /*
     *
     * */
    let min: u8;
    let mut max: u8 = 0;

    // argument match cases
    match command {
        /* String Command */
        Commands::String(StringArgs) => {
            // main stuff
            let space = StringArgs.space;
            match space {
                true => min = 32,
                false => min = 33,
            }
            match StringArgs.encoding.as_str() {
                "ext" | "extasc" => max = 255,
                "asc" | "ascii" => max = 127,
                _ => {
                    eprintln!("Unknown encoding argument, setting to ASCII");
                    max = 127
                }
            }
            result_string =
                stringgeneration::generator(StringArgs.length, min, max, result_string, debug);

            if debug {
                dbg!(min, max, StringArgs.length);
                println!("[String Vector]:\n{:#x?}", result_string.as_bytes());
                println!("Vector Size: {}", result_string.as_bytes().len());
            }

            // loops the program inside of here. can be refactored into a function (TODO)
            match lp {
                true => loop {
                    result_string = String::new();
                    result_string = stringgeneration::generator(
                        StringArgs.length,
                        min,
                        max,
                        result_string,
                        debug,
                    );
                    if debug {
                        dbg!(min, max, StringArgs.length);
                        println!("[String Vector]:\n{:#x?}", result_string.as_bytes());
                        println!("Vector Size: {}", result_string.as_bytes().len());
                    }
                    println!("\n{}", result_string);
                },
                false => (),
            }
        }
        /* Integer Command */
        Commands::Integer(IntegerArgs) => {
            result_string = stringgeneration::intgen(IntegerArgs.length, result_string);

            if debug {
                dbg!(IntegerArgs.length);
            }

            match lp {
                true => loop {
                    result_string = String::new();
                    result_string = stringgeneration::intgen(IntegerArgs.length, result_string);
                    if debug {
                        dbg!(IntegerArgs.length);
                        println!("[String Vector]:\n{:#x?}", result_string.as_bytes());
                    }
                    println!("\n{}", result_string);
                },
                false => (),
            }
        }
        /* Alphanumeric Command */
        Commands::Alphanumeric(AlphaArgs) => {
            /*
             * By default, the starting character will be '0' if no and encompass all letters from
             * 'A-Z, a-z'. However, if the user specifies that they want only a select case
             * (i.e, uppercase or lowercase generation ONLY), the program will only generate the
             * utf-8 codes 'A-Z'. This was done to reduce the frequency of the same character being
             * generated.
             */

            let mut min: u8 = 48;
            let mut max: u8 = 123;

            if AlphaArgs.alphabet {
                min = 65;
            }
            if AlphaArgs.smallcase || AlphaArgs.upper {
                max = 90;
            }
            /*
              We store the result string first in the case that the user only had specified
              uppercase or lowercase letter generation, otherwise the program just finishes.
            */
            result_string =
                stringgeneration::alphanumeric(AlphaArgs.length, min, max, result_string);
            // manage letter cases.
            if AlphaArgs.upper && !AlphaArgs.smallcase {
                result_string = result_string.to_uppercase();
            }
            if AlphaArgs.smallcase && !AlphaArgs.upper {
                result_string = result_string.to_lowercase();
            }
            match lp {
                true => loop {
                    result_string = String::new();
                    result_string =
                        stringgeneration::alphanumeric(AlphaArgs.length, min, max, result_string);
                    if debug {
                        dbg!(min, max, AlphaArgs.length);
                        println!("[String Vector]:\n{:#x?}", result_string.as_bytes());
                    }
                    if AlphaArgs.upper && !AlphaArgs.smallcase {
                        result_string = result_string.to_uppercase();
                    }
                    if AlphaArgs.smallcase && !AlphaArgs.upper {
                        result_string = result_string.to_lowercase();
                    }
                    println!("\n{}", result_string);
                },
                false => (),
            }
            if debug {
                dbg!(min, max, AlphaArgs.length);
            }
        }

        /* Estimate Command */
        Commands::Estimate(EstimateArgs) => {
            if EstimateArgs.string.is_empty() {
                eprintln!("cannot parse an empty string");
                exit(1)
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

        /* Steganographic Commands */
        Commands::Steg(StoreArgs) => {
            let subcommand = StoreArgs.store;
            match subcommand {
                // NewArgs is the command parser for Generate, originally named New, so don't fret
                // about the ARGs parser being named differently than the actual command name.
                ImageCommands::Generate(NewArgs) => {
                    let space = NewArgs.space;
                    let min = match space {
                        true => 32,
                        false => 33,
                    };
                    let max = match NewArgs.encoding.as_str() {
                        "ext" | "extasc" => 255,
                        "asc" | "ascii" => 127,
                        _ => {
                            eprintln!("Unknown encoding argument, setting to ASCII");
                            127
                        }
                    };
                    if max == 255 && NewArgs.length > 120 {
                        // error 6: max length for extended ascii/utf-8 cannot exceed 120
                        // characters
                        eprintln! ("Error: Using the `extasc` command with `generate` generate strings longer than 120 characters.\n
                                    This is due to certain technological limitations with the steganography crate.");
                        exit(1)
                    }
                    result_string =
                        stringgeneration::generator(NewArgs.length, min, max, result_string, debug);
                    // switch to magic bytes
                    if Path::new(&NewArgs.name).exists() {
                        let filepath = String::from(&NewArgs.name);
                        mime::check_magic(filepath)?;
                        {
                            steganographic::store(
                                NewArgs.name,
                                NewArgs.output,
                                result_string.clone(),
                                NewArgs.unencrypted,
                            )?;
                        }
                    } else {
                        eprintln!("file does not exist");
                        exit(1)
                    }
                }
                ImageCommands::Read(ReadArgs) => {
                    let filepath = String::from(&ReadArgs.name);
                    mime::check_magic(filepath)?;
                    match Path::new(&ReadArgs.name).exists() {
                        //switch to magic bytes by v2
                        true => match steganographic::extract_raw_unencrypted(&ReadArgs.name) {
                            Ok(_e) => (),
                            Err(_) => steganographic::extract(&ReadArgs.name),
                        },
                        false => {
                            eprintln!("file does not exist");
                            exit(1)
                        }
                    }
                }
                ImageCommands::Embed(ExistingArgs) => {
                    if Path::new(&ExistingArgs.name).exists() {
                        let filepath = String::from(&ExistingArgs.name);
                        mime::check_magic(filepath)?;
                        match steganographic::store(
                            ExistingArgs.name,
                            ExistingArgs.output,
                            ExistingArgs.payload,
                            ExistingArgs.unencrypted,
                            // See documentation for how this function works.
                        ) {
                            Ok(_) => (),
                            Err(e) => eprintln!("{} experienced an error: {e}", info.name),
                        }
                    }
                }
            }
        }
    }
    Ok(print!("{result_string}\n"))
}
