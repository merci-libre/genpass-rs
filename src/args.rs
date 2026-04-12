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

#[derive(Clone, Copy, Debug, Args, PartialEq)]
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

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PasswordType {
    Alphanumeric(AlphaArgs),
    Numeric,
    Regular,
}

#[derive(Debug, Clone, Copy)]
#[doc(hidden)]
/// Password Type that gets produced that gets passed over to
/// the password generator inside of crate::modules::
pub struct Password {
    /// Minimum UTF-8 character generated.
    minimum_character: u8,
    /// Maximum UTF-8 character generated.
    maximum_character: u8,
    /// Password type for the password dynamically selected from the type
    /// passed from the CLI argument.
    passtype: PasswordType,
}

impl Password {
    pub fn new() -> Password {
        //! creates a new Password type. All functions are
        //! accessible through functional setters.
        Password {
            minimum_character: 0,
            maximum_character: 0,
            passtype: PasswordType::Regular,
        }
    }

    /* These functions are only used inside of this module for dynamically creating passwords
     * based on the PasswordType enum. It does all of the heavy lifting for the password generator
     * by setting all of the values that it needs in order to properly generate passwords.
     */

    fn set_min(mut self, minimum: u8) -> Password {
        //! Sets the minimum UTF-8 character for the
        //! password generator.
        self.minimum_character = minimum;
        return self;
    }
    fn set_max(mut self, max: u8) -> Password {
        //! sets the maximum UTF-8 character for the
        //! password generator.
        self.maximum_character = max;
        return self;
    }
    fn set_passwordtype(mut self, password_type: PasswordType) -> Password {
        //! sets the password types
        self.passtype = password_type;
        return self;
    }

    /*
     * These functions are usable by any of the modules that import password. Typically used to just
     * retrieve information for the password type.
     */

    pub fn password_type(self) -> PasswordType {
        //! Returns the password type.
        self.passtype
    }
    pub fn min(self) -> u8 {
        //! Return the minimum character allowed for generation.
        self.minimum_character
    }
    pub fn max(self) -> u8 {
        //! Return the maximum character allowed for generation.
        self.maximum_character
    }
}

pub trait CheckArgs {
    /// Trait used to automatically parse the subcommand passed to fill out the password type.
    fn check_arguments(self) -> Password;
    fn get_length(&self) -> u8;
}

impl CheckArgs for AlphaArgs {
    fn check_arguments(self) -> Password {
        //! Sets the password traits automatically for alphabetical arguments.
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
        //! Returns the password's length from the overlaying struct.
        //! Used for tests since this trait can be used as a generic.
        self.length
    }
}

impl CheckArgs for IntegerArgs {
    fn check_arguments(self) -> Password {
        //! Sets the password traits automatically for
        //! Integer arguments.

        let mut password_struct = Password::new();
        let zero = 48;
        let nine = 57;

        password_struct = password_struct.set_min(zero);
        password_struct = password_struct.set_max(nine);
        password_struct = password_struct.set_passwordtype(PasswordType::Numeric);

        return password_struct;
    }
    fn get_length(&self) -> u8 {
        //! Returns the password's length from the overlaying struct.
        //! Used for tests since this trait can be used as a generic.
        self.length
    }
}

/* Regular printable UTF-8 Strings with no restrictions */

fn create_arguments_for_strings(use_space: bool, encoding: String) -> Password {
    //! Common function for handling String arguments for both steganography
    //! and regular string outputs:
    let mut password_struct = Password::new();
    let space = 32;
    let exclamation_point = 33;

    password_struct = match use_space {
        true => password_struct.set_min(space),
        false => password_struct.set_min(exclamation_point),
    };
    password_struct = match encoding.as_str() {
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

impl CheckArgs for StringArgs {
    fn check_arguments(self) -> Password {
        //! Sets the password traits automatically for
        //! `string` arguments.
        create_arguments_for_strings(self.space, self.encoding)
    }
    fn get_length(&self) -> u8 {
        //! Returns the password's length from the overlaying struct.
        //! Used for tests since this trait can be used as a generic.
        self.length
    }
}

impl CheckArgs for NewArgs {
    fn check_arguments(self) -> Password {
        //! Sets the password traits automatically for
        //! Steganographic arguments. Will be replaced
        //! in future versions
        create_arguments_for_strings(self.space, self.encoding)
    }
    fn get_length(&self) -> u8 {
        //! Returns the password's length from the overlaying struct.
        //! Used for tests since this trait can be used as a generic.
        self.length
    }
}

mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_check_args() {
        //! tests the CheckArgs trait for each type of generation arg for both StringArgs
        //! and NewArgs
        let args = StringArgs {
            encoding: String::from("ascii"),
            space: false,
            length: 30,
        };

        let p = args.check_arguments();

        let args = NewArgs {
            space: false,
            length: 30,
            unencrypted: true,
            encoding: String::from("ascii"),
            output: String::from("none"),
            name: String::from("none"),
        };
        let q = args.check_arguments();

        assert_eq!(p.passtype, q.passtype);
    }
    #[test]
    fn test_get_length() {
        //! tests the length modifier that every password has.
        //! used for generic implementations, and tests.
        let args = StringArgs {
            encoding: String::from("ascii"),
            space: false,
            length: 30,
        };

        assert_eq!(args.get_length(), args.length)
    }
}
