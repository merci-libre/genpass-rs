use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]

pub struct GenpassArgs {
    /// Commands
    #[command(subcommand)]
    pub generate: Generate,

    /// Prints debugging information.
    #[arg(long, short)]
    pub debug: bool,
}

#[derive(Debug, Subcommand)]
pub enum Generate {
    /// Generates a new string of specified length.
    String(StringArgs),
    /// Generates an integer of specified length.
    Integer(IntegerArgs),
}

#[derive(Debug, Args)]
pub struct StringArgs {
    /// encoding for the characters used in the password. Valid arguments include: 'extasc, ascii'
    #[arg(long, short)]
    pub encoding: String,
    ///Produces spaces (char 32) in the password generated.
    #[arg(long, short)]
    pub space: bool,
    /// Length of the string
    #[arg(long, short)]
    pub length: u8,
}

#[derive(Debug, Args)]
pub struct IntegerArgs {
    /// Length of the string
    #[arg(long, short)]
    pub length: u8,
}
