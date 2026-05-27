use console;
use rand::Rng;
use std::{io, thread, time};

use crate::args::{Password, PasswordType};

/* As of v.1.0.5, you don't need to uncomment any code for debugging purposes.*/

fn testing(password_info: &GeneratorDetails) {
    //! prints out debugging information if the user requests.
    let t = console::Term::stderr();
    let what_to_print = format!(
        "\nCurrently Generated String:\n{}\nfinal_bytesize={}\ntarget_bytesize={}\ntrue length={}\nmax_bytes={}\npossible_characters:\n{:?}",
        password_info.get_password(),
        password_info.get_final_bytesize(),
        password_info.get_target_bytesize(),
        password_info.len(),
        password_info.get_max_byte_size(),
        password_info.get_possible_characters(),
    );
    t.write_line(what_to_print.as_str()).unwrap();
    thread::sleep(time::Duration::from_secs(2));

    // to make terminal output look nice:
    t.clear_last_lines(6).unwrap();
    io::Write::flush(&mut io::stdout()).expect("failed to flush output");
}

#[derive(Clone)]
pub struct GeneratorDetails {
    possible_characters: Vec<char>,
    password: String,
    target_bytesize: u16,
    max_bytes: u16,
    final_bytesize: u16,
    true_length: u8,
}

impl GeneratorDetails {
    fn new(
        target: u16,
        character_list: Vec<char>,
        true_length: u8,
        final_generation_size: u16,
        max_size: u16,
        string: String,
    ) -> GeneratorDetails {
        //! Creates a new Details struct for the Generated passwords.
        GeneratorDetails {
            possible_characters: character_list,
            password: string,
            target_bytesize: target,
            max_bytes: max_size,
            final_bytesize: final_generation_size,
            true_length: true_length,
        }
    }
    fn get_possible_characters(&self) -> &[char] {
        self.possible_characters.as_slice()
    }

    pub fn get_password(&self) -> &String {
        //! Gets the actual password and returns it
        //! as a reference.
        &self.password
    }
    fn get_target_bytesize(&self) -> u16 {
        //! Gets the target bytesize that the password generation engine automatically
        //! set when ran.
        self.target_bytesize
    }
    fn get_max_byte_size(&self) -> u16 {
        //! gets the maximum size bytes the generator set at it's start.
        self.max_bytes
    }
    fn get_final_bytesize(&self) -> u16 {
        //! gets the final bytesize after creating the string.
        self.final_bytesize
    }
    pub fn len(&self) -> u8 {
        //! gets the actual length of the string.
        self.true_length
    }
}

#[derive(Debug, PartialEq)]
enum GenerationType {
    Ascii,
    ExtAscii,
}

fn generate_password(
    length: u8,
    character_list: Vec<char>,
    generation_type: GenerationType,
) -> GeneratorDetails {
    //! The actual password engine that genpass-rs uses, returns the generator
    //! details which is used in the `test()` modules and from versions 2.0.0+ on will be the
    //! defacto method of storing passwords.
    let mut bytesize: u16 = 0;
    let mut target_bytesize: u16 = length.into();
    let max_size: u16 = target_bytesize * 2;
    let mut truecount: u8 = 0;

    let mut random = rand::rng();
    let mut bytes = String::new();

    match generation_type {
        GenerationType::Ascii => {
            for _ in 0..length {
                let random_index: usize = random.random_range(0..character_list.len());
                let selected_character: &char = character_list.get(random_index).unwrap();
                bytes.push(*selected_character);
                bytesize += 1;
            }
        }
        GenerationType::ExtAscii => {
            // This is what was originally used for generating characters originally--
            // it works extremely well for Extended ascii, and works generally
            while bytesize != target_bytesize {
                let random_index: usize = random.random_range(0..character_list.len());
                let selected_character: &char = character_list.get(random_index).unwrap();

                bytes.push(*selected_character);

                // valid for utf-8 smaller than 255 as char is limited by this.
                if *selected_character as u8 > 128 && target_bytesize < max_size {
                    target_bytesize += 1;
                    bytesize += 2;
                } else {
                    bytesize += 1;
                }
                truecount += 1;
            }
        }
    }
    let final_password: String = bytes;
    if generation_type == GenerationType::Ascii {
        truecount = final_password.len() as u8;
    }
    GeneratorDetails::new(
        target_bytesize,
        character_list,
        truecount,
        bytesize,
        max_size,
        final_password,
    )
}

#[doc(hidden)]
pub fn generate(password_options: Password, length: u8, debug: bool) -> GeneratorDetails {
    //! Creates the password from a given password_options returns the GeneratorDetails struct.
    //! in order to access the generated password, calling the `.get_password()` will return a
    //! reference to the string.

    // generate character list to pull from.
    let mut valid_charlist: Vec<char> = Vec::new();

    let mut char_min = password_options.min();
    let char_max = password_options.max();
    let mut gentype = GenerationType::Ascii;

    match password_options.password_type() {
        PasswordType::Regular => {
            // accidentally added the delete character-- 126 is the actual max for basic ascii :/
            // this should fix issue #7
            for i in char_min..=126 {
                valid_charlist.push(i as char);
            }

            if char_max == 255 {
                // I used hex values for ease of reading. It's not true extended ascii--
                // but printable utf-8 up to value 255.

                // These ranges essentially remove all non-printable
                // characters from being pushed to our character list.
                let char_ranges: [std::ops::RangeInclusive<u8>; 4] =
                    [0xa1..=0xac, 0xae..=0xb3, 0xb5..=0xb7, 0xb9..=0xff];
                for x in char_ranges {
                    for i in x {
                        valid_charlist.push(i as char);
                    }
                }
                gentype = GenerationType::ExtAscii
            }
        }
        PasswordType::Alphanumeric(string_args) => {
            if char_min == 48 {
                let numbers: std::ops::RangeInclusive<u8> = char_min..=57;
                for i in numbers {
                    valid_charlist.push(i as char);
                }
                char_min = 65;
            }

            // ascii char sequences
            let capital_range = char_min..=90;
            let lowercase_range = 97..=char_max;

            if string_args.upper || string_args.smallcase {
                // capital range only, since adding the lowercase can create predictable strings.
                for i in capital_range {
                    valid_charlist.push(i as char);
                }
            } else {
                // capital + lowercase  characters
                for i in capital_range {
                    valid_charlist.push(i as char);
                }
                for i in lowercase_range {
                    valid_charlist.push(i as char);
                }
            }
        }
        PasswordType::Numeric => {
            for i in char_min..=char_max {
                valid_charlist.push(i as char);
            }
        }
    }
    let password_details = generate_password(length, valid_charlist, gentype);
    if debug {
        // if modifying any of the code above, change from a comment.

        //dbg!(&password_options, &valid_charlist);
        testing(&password_details);
    }

    password_details
}

// tests
mod tests {
    #[allow(dead_code)]
    // changed the test count to be more reasonable.
    const TESTCOUNT: std::ops::Range<usize> = 0..1000;

    use super::GeneratorDetails;
    use crate::args::*;

    #[allow(dead_code)]
    fn start_test<T: CheckArgs + Clone>(generation_type: T) -> GeneratorDetails {
        // run at every test start
        let arg_type = generation_type.clone().check_arguments();
        super::generate(arg_type, generation_type.get_length(), false)
    }
    #[test]
    fn test_ascii_string_generation() {
        let mut previous_string = String::new();
        for i in TESTCOUNT {
            let generation_type = StringArgs {
                encoding: String::from("ascii"),
                space: true,
                length: 32,
            };

            let password = start_test(generation_type.to_owned());
            let a = password.len();
            let b = password.get_password().len();

            let pwd_byte_size = password.len() as usize;
            let mut expected: usize = generation_type.length as usize * size_of::<u8>();

            if i > 1 {
                expected = previous_string.len();
            }
            assert_eq!(
                pwd_byte_size, expected,
                "bad string {pwd_byte_size}, {expected}"
            );
            assert_eq!(
                a, b as u8,
                "failed to determine the real size of the string {a} != {b}"
            );
            assert_ne!(previous_string, *password.get_password());
            assert_eq!(password.get_final_bytesize(), 32);
            previous_string = password.get_password().clone()
        }
    }
    #[test]
    fn test_ext_ascii_string_generation() {
        let mut previous_string = String::new();
        for _ in TESTCOUNT {
            let generation_type = StringArgs {
                encoding: String::from("extasc"),
                space: false,
                length: 32,
            };
            let password = start_test(generation_type.to_owned());
            let a = password.len();
            let b = generation_type.length;
            assert_eq!(
                a, b,
                "failed to determine the real size of the string {a} != {b}"
            );
            assert_ne!(previous_string, *password.get_password());
            let weirdchar = String::from(0xa0 as u8 as char);
            if password.get_password().contains(&weirdchar) {
                panic!("contains that weird char!")
            }
            previous_string = password.get_password().clone()
        }
    }
    #[test]
    fn test_check_bad_characters() {
        //! Checks the list for any possible bad characters inside of the password.
        let generation_type = StringArgs {
            encoding: String::from("extasc"),
            space: true,
            length: 32,
        };
        let password = start_test(generation_type);
        let charlist = password.get_possible_characters();
        for i in charlist {
            match *i as u8 {
                32..127 => (),
                0xa1..=0xac => (),
                0xae..=0xb3 => (),
                0xb5..=0xb7 => (),
                0xb9..=0xff => (),
                _ => panic!("Bad character! {}", *i as u8),
            }
        }
        let generation_type = StringArgs {
            encoding: String::from("ascii"),
            space: true,
            length: 32,
        };
        let password = start_test(generation_type);
        let charlist = password.get_possible_characters();
        for i in charlist {
            match *i as u8 {
                32..127 => (),
                _ => panic!("Bad character! {}", *i as u8),
            }
        }
    }
    #[test]
    fn test_numeric() {
        for _ in TESTCOUNT {
            let generation_type = IntegerArgs { length: 20 };
            let password = start_test(generation_type.to_owned());
            for i in password.get_password().chars() {
                let _: u8 = i.to_string().parse::<u8>().expect("was not a number");
            }
            assert_eq!(password.len(), 20);
            assert_eq!(password.get_final_bytesize(), 20);
        }
    }
    #[test]
    fn test_alpha() {
        for _ in TESTCOUNT {
            let generation_type = AlphaArgs {
                length: 20,
                alphabet: true,
                smallcase: true,
                upper: false,
            };
            let password = start_test(generation_type.to_owned())
                .get_password()
                .to_owned();
            for i in password.chars() {
                match i as u8 {
                    65..=90 => (),
                    97..=122 => panic!("String should generate all uppercase before return."),
                    _ => panic!("bad character!"),
                }
            }
        }
    }
}
