use clap::{Args, Parser, Subcommand};
use clap_stdin::MaybeStdin;

#[derive(Clone, Debug, Parser)]
#[clap(version)]

pub struct GenpassArgs {
    /// Commands
    #[command(subcommand)]
    pub commands: Commands,

    /// Prints debugging information.
    #[arg(long, short)]
    pub debug: bool,
    /// Loops the program (currently only works for String command)
    #[arg(long, short)]
    pub r#loop: bool,
}

#[derive(Clone, Debug, Subcommand)]
pub enum Commands {
    /// Generates a new string of specified length.
    String(StringArgs),
    /// Generates an integer of specified length.
    Integer(IntegerArgs),
    /// Generates an alphanumeric string.
    Alphanumeric(AlphaArgs),
    /// Estimates the strength of password.
    Estimate(EstimateArgs),
    /// Use Steganography to store strings into PNGs or JPEGs. Acceptable formats: ([.png], [.jpg], [.jpeg])
    Steg(StoreArgs),
}
// Generation Arguments

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
    #[arg(long, short, default_value = "ascii")]
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
    /// Print only letters
    #[arg(long, short)]
    pub alphabet: bool,
    /// Print only lowercase letters
    #[arg(long, short)]
    pub smallcase: bool,
    /// Print only uppercase letters
    #[arg(long, short)]
    pub upper: bool,
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

// database arguments
#[derive(Clone, Debug, Args)]
pub struct StoreArgs {
    #[command(subcommand)]
    /// Use Steganography to store strings into PNGs or JPEGs. Acceptable formats: ([.png], [.jpg], [.jpeg])
    pub store: ImageCommands,
}
#[derive(Clone, Debug, Subcommand)]
pub enum ImageCommands {
    /// Generates a string using the default 'String Command' and stores the result into an image.
    Generate(NewArgs),
    /// Read and decrypt stored password from an image
    Read(ReadArgs),
    /// Write an existing password to an image.
    Embed(ExistingArgs),
}

#[derive(Clone, Debug, Args)]
pub struct NewArgs {
    /// encoding for the characters used in the password. Valid arguments include: 'extasc, ascii'
    #[arg(long, short, default_value = "ascii")]
    pub encoding: String,
    ///Produces spaces (char 32) in the password generated.
    #[arg(long, short)]
    pub space: bool,
    /// Length of the string. Can only be up to 240 characters for 'asc' and 120 for 'extasc'.
    #[arg(long, short)]
    pub length: u8,
    /// Use this option to embed the message into the image without any encryption.
    ///
    /// (DANGEROUS FOR STORING PASSWORDS!)
    #[arg(long, short)]
    pub unencrypted: bool,
    /// name of the output file
    #[arg(long, short, default_value = "")]
    pub output: String,
    /// Name of the input image file to encrypt the password into.
    pub name: String,
}
#[derive(Clone, Debug, Args)]
pub struct ExistingArgs {
    /// String to encode into image.
    #[arg(long, short)]
    pub payload: String,
    /// Use this option to embed the message into the image without any encryption.
    ///
    /// (DANGEROUS FOR STORING PASSWORDS!)
    #[arg(long, short)]
    pub unencrypted: bool,
    #[arg(long, short, default_value = "")]
    pub output: String,
    /// Image to modify.
    pub name: String,
}
#[derive(Clone, Debug, Args)]
pub struct ReadArgs {
    /// Name of the image file to read and decrypt.
    pub name: String,
}
