use clap::Parser;

#[derive(Parser)]

pub struct GenpassArgs {
    /// Bytesize of the string
    #[arg(long, short)]
    pub length: u8,
    /// encoding for the characters used in the password. Valid arguments include: 'exasc, ascii'
    #[arg(long, short)]
    pub encoding: String,
    ///Produces Spaces in the password generated.
    #[arg(long, short)]
    pub space: bool,
    /// Prints debugging information.
    #[arg(long, short)]
    pub debug: bool,
}
