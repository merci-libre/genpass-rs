use clap::{Args, Parser, Subcommand};
use clap_stdin::MaybeStdin;

#[derive(Clone, Debug, Parser)]

pub struct GenpassArgs {
    /// Commands
    #[command(subcommand)]
    pub generate: Generate,

    /// Prints debugging information.
    #[arg(long, short)]
    pub debug: bool,
}

#[derive(Clone, Debug, Subcommand)]
pub enum Generate {
    /// Generates a new string of specified length.
    String(StringArgs),
    /// Generates an integer of specified length.
    Integer(IntegerArgs),
    /// Generates an alphanumeric string.
    Alphanumeric(AlphaArgs),
    /// Estimates the strength of password.
    Estimate(EstimateArgs),
}

#[derive(Clone, Debug, Args)]
pub struct EstimateArgs {
    ///String to enumerate the strength of. Accepts STDIN. If no value or STDIN is provided,
    ///
    ///CTRL-D will send an EOF to kill the process. Additionally, it may help to run the command
    ///
    ///with a trailing `--` to negate any strings beginning with a `-`. i.e, `-2`.
    ///
    ///If you pipe in '-' alone, it will return an empty string due to this nature.
    #[clap(default_value = "-")]
    pub string: MaybeStdin<String>,
}

#[derive(Clone, Debug, Args)]
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

#[derive(Clone, Debug, Args)]
pub struct AlphaArgs {
    /// Length of the string
    #[arg(long, short)]
    pub length: u8,
}

#[derive(Clone, Debug, Args)]
pub struct IntegerArgs {
    /// Length of the string
    #[arg(long, short)]
    pub length: u8,
}
