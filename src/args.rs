use clap::{Args, Parser, Subcommand};
use clap_stdin::MaybeStdin;

#[derive(Clone, Debug, Parser)]
#[clap(version)]

pub struct GenpassArgs {
    /// Commands
    #[command(subcommand)]
    pub commands: Commands,

    /// Prints debugging information
    #[arg(long, short)]
    pub debug: bool,
    /// Loops the program for infinite string generation
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

#[derive(Clone, Copy, Debug, Args)]
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

// Image Structs
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
    pub payload: MaybeStdin<String>,
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

#[derive(Debug, Clone, Copy)]
pub enum PasswordType {
    Alphanumeric(AlphaArgs),
    Numeric,
    Regular,
}

#[derive(Debug, Clone, Copy)]
pub struct Password {
    minimum_character: u8,
    maximum_character: u8,
    passtype: PasswordType,
}

impl Password {
    pub fn new() -> Password {
        //! creates a new password.
        Password {
            minimum_character: 0,
            maximum_character: 0,
            passtype: PasswordType::Regular,
        }
    }
    fn set_min(mut self, minimum: u8) -> Password {
        self.minimum_character = minimum;
        return self;
    }
    fn set_max(mut self, max: u8) -> Password {
        self.maximum_character = max;
        return self;
    }
    fn set_passwordtype(mut self, password_type: PasswordType) -> Password {
        self.passtype = password_type;
        return self;
    }

    pub fn password_type(self) -> PasswordType {
        self.passtype
    }
    pub fn min(self) -> u8 {
        self.minimum_character
    }
    pub fn max(self) -> u8 {
        self.maximum_character
    }
}

pub trait CheckArgs {
    fn check_arguments(self) -> Password;
    fn get_length(&self) -> u8;
}

impl CheckArgs for AlphaArgs {
    fn check_arguments(self) -> Password {
        let mut min: u8 = 48;
        let max: u8 = 122;

        if self.alphabet {
            min = 65;
        }

        let mut password_struct = Password::new();
        password_struct = password_struct.set_min(min);
        password_struct = password_struct.set_max(max);
        password_struct = password_struct.set_passwordtype(PasswordType::Alphanumeric(self));

        return password_struct;
    }
    fn get_length(&self) -> u8 {
        return self.length.to_owned();
    }
}

impl CheckArgs for IntegerArgs {
    fn check_arguments(self) -> Password {
        //! Checks to see if the user has asked for spaces to be generated
        //! in their password-- if so, then it returns

        let mut password_struct = Password::new();
        let zero = 48;
        let nine = 57;

        password_struct = password_struct.set_min(zero);
        password_struct = password_struct.set_max(nine);
        password_struct = password_struct.set_passwordtype(PasswordType::Numeric);

        return password_struct;
    }
    fn get_length(&self) -> u8 {
        return self.length.to_owned();
    }
}

impl CheckArgs for StringArgs {
    fn check_arguments(self) -> Password {
        //! Checks to see if the user has asked for spaces to be generated
        //! in their password-- if so, then it returns

        let mut password_struct = Password::new();
        let space = 32;
        let exclamation_point = 33;

        password_struct = match self.space {
            true => password_struct.set_min(space),
            false => password_struct.set_min(exclamation_point),
        };

        password_struct = match self.encoding.as_str() {
            "ext" | "extasc" => password_struct.set_max(255),
            "asc" | "ascii" => password_struct.set_max(126),
            _ => {
                eprintln!("Unknown encoding argument, setting to ASCII");
                password_struct.set_max(126)
            }
        };

        password_struct = password_struct.set_passwordtype(PasswordType::Regular);
        return password_struct;
    }
    fn get_length(&self) -> u8 {
        return self.length.to_owned();
    }
}

impl CheckArgs for NewArgs {
    fn check_arguments(self) -> Password {
        //! Checks to see if the user has asked for spaces to be generated
        //! in their password-- if so, then it returns

        let mut password_struct = Password::new();

        password_struct = match self.space {
            true => password_struct.set_min(32),
            false => password_struct.set_min(33),
        };

        password_struct = match self.encoding.as_str() {
            "ext" | "extasc" => password_struct.set_max(255),
            "asc" | "ascii" => password_struct.set_max(126),
            _ => {
                eprintln!("Unknown encoding argument, setting to ASCII");
                password_struct.set_max(126)
            }
        };
        password_struct = password_struct.set_passwordtype(PasswordType::Regular);
        return password_struct;
    }
    fn get_length(&self) -> u8 {
        return self.length.to_owned();
    }
}
