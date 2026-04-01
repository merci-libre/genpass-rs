// modules
mod args;
mod modules;

// standard library
use std::{error::Error, path::Path, process::exit};

// calls to modules
use args::{CheckArgs, Commands, GenpassArgs, ImageCommands, Password, PasswordType};
use clap::Parser;
use modules::{passwords, steganographic};
use std::env;

// package info
const BIN_NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");
const AUTHOR: &str = env!("CARGO_PKG_AUTHORS");

fn enumerate_image_subs(subcommands: ImageCommands, debug: bool) -> Result<(), Box<dyn Error>> {
    match subcommands {
        ImageCommands::Generate(new_steganographic_image_options) => {
            let arguments = new_steganographic_image_options
                .to_owned()
                .check_arguments();
            let result_string = passwords::generator::generate(
                arguments,
                new_steganographic_image_options.length,
                debug,
            )
            .get_password()
            .to_owned();

            if !Path::new(&new_steganographic_image_options.name).exists() {
                eprintln!("file does not exist");
                exit(1)
            }
            let filepath = String::from(&new_steganographic_image_options.name);
            // check the file's magic bytes.
            steganographic::mime::check_magic(&filepath)?;
            {
                steganographic::image::store(
                    new_steganographic_image_options.name,
                    new_steganographic_image_options.output,
                    &result_string,
                    new_steganographic_image_options.unencrypted,
                )?;
            }
            println!("{result_string}");
        }

        ImageCommands::Read(steganographic_read_options) => {
            let filepath = String::from(&steganographic_read_options.name);
            // check the file's magic bytes.
            steganographic::mime::check_magic(&filepath)?;

            if !Path::new(&filepath).exists() {
                eprintln!("file does not exist");
                exit(1)
            }
            // First tries to extract an encrypted password-- otherwise fails.
            match steganographic::image::extract_raw_unencrypted(&steganographic_read_options.name)
            {
                Ok(password) => println!("{password}"),
                Err(_) => {
                    modules::steganographic::image::extract(&steganographic_read_options.name)
                }
            }
        }

        ImageCommands::Embed(existing_args) => {
            let filepath = String::from(&existing_args.name);
            if Path::new(&filepath).exists() {
                let filepath = String::from(&existing_args.name);
                steganographic::mime::check_magic(&filepath)?;

                match steganographic::image::store(
                    existing_args.name,
                    existing_args.output,
                    &existing_args.payload.to_string(),
                    existing_args.unencrypted,
                    // See documentation for how this function works.
                ) {
                    Ok(_) => (),
                    Err(e) => eprintln!("{} experienced an error: {e}", BIN_NAME),
                }
            }
        }
    }
    Ok(())
}

fn do_forever(arguments: Password, length: u8, debug: bool) {
    loop {
        let password_info = passwords::generator::generate(arguments, length, debug);
        match arguments.password_type() {
            PasswordType::Alphanumeric(alphabetic_password) => {
                let mut password = password_info.get_password().to_owned();
                if alphabetic_password.upper && !alphabetic_password.smallcase {
                    password = password.to_uppercase();
                }
                if alphabetic_password.smallcase && !alphabetic_password.upper {
                    password = password.to_lowercase();
                }
                println!("\n{}", password);
            }
            _ => {
                let password = password_info.get_password().to_owned();
                println!("\n{}", password);
            }
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // parse the arguments for clap
    let args = GenpassArgs::parse();
    eprintln!("{} v.{} \n- Developed by: {}\n", BIN_NAME, VERSION, AUTHOR);

    let debug = args.debug;
    let command = args.commands;
    let pass_loop = args.r#loop;

    let mut arguments: Password = Password::new();
    let mut length: u8 = 0;

    // options for set alphbetic
    let mut uppercase = false;
    let mut lowercase = false;

    // argument match cases, need to refactor to reinforce DRY principles?
    match command {
        /* String Command */
        Commands::String(regular_password_options) => {
            arguments = regular_password_options.to_owned().check_arguments();
            length = regular_password_options.length;
            if pass_loop {
                do_forever(arguments, length, debug);
            }
        }

        /* Integer Command */
        Commands::Integer(integer_password) => {
            arguments = integer_password.to_owned().check_arguments();
            length = integer_password.length;

            if pass_loop {
                do_forever(arguments, length, debug);
            }
        }
        /* Alphanumeric Command */
        Commands::Alphanumeric(alphabetic_password) => {
            /*
              We store the result string first in the case that the user only had specified
              uppercase or lowercase letter generation, otherwise the program just finishes.
            */
            arguments = alphabetic_password.to_owned().check_arguments();
            length = alphabetic_password.length;

            // manage letter cases.
            if alphabetic_password.upper && !alphabetic_password.smallcase {
                uppercase = true;
            }
            if alphabetic_password.smallcase && !alphabetic_password.upper {
                lowercase = true;
            }

            if pass_loop {
                do_forever(arguments, length, debug);
            }
        }

        /* Estimate Command */
        Commands::Estimate(estimate_args) => {
            if estimate_args.string.is_empty() {
                eprintln!("cannot parse an empty string");
                exit(1)
            }
            let password = estimate_args.string.to_string();
            modules::passwords::zxcvbn::estimate(password);
        }

        /* Steganographic Commands */
        Commands::Steg(steganographic_arguments) => {
            let subcommand = steganographic_arguments.store;
            enumerate_image_subs(subcommand, debug)?;
            exit(0);
        }
    }

    // actually print the string
    let mut result_string = passwords::generator::generate(arguments, length, debug)
        .get_password()
        .to_owned();
    if uppercase {
        result_string = result_string.to_uppercase();
    }
    if lowercase {
        result_string = result_string.to_lowercase();
    }
    Ok(println!("{result_string}"))
}
