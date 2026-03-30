use rand::{rngs::StdRng, Rng, SeedableRng};
use std::{io, thread, time};

use crate::args::{Password, PasswordType};

/* As of v.1.0.5, you don't need to uncomment any code for debugging purposes.*/

fn testing(password_info: &GeneratorDetails) {
    //! prints out debugging information if the user requests.
    let mut t = term::stderr().unwrap();
    let what_to_print = format!(
        "\nCurrently Generated String:\n{}\nfinal_bytesize={}\ntarget_bytesize={}\ntrue length={}\nmax_bytes={}",
        password_info.get_password(),
        password_info.get_final_bytesize(),
        password_info.get_target_bytesize(),
        password_info.len(),
        password_info.get_max_byte_size()
    );
    writeln!(t, "{what_to_print}").unwrap();
    thread::sleep(time::Duration::from_secs(2));

    // to make terminal output look nice:
    for _i in 0..6 {
        t.carriage_return().unwrap();
        t.delete_line().unwrap();
        t.reset().unwrap();
        t.cursor_up().unwrap();
    }
    io::Write::flush(&mut io::stdout()).expect("failed to flush output");
}

#[derive(Clone)]
pub struct GeneratorDetails {
    password: String,
    target_bytesize: u16,
    max_bytes: u16,
    final_bytesize: u16,
    true_length: u8,
}

impl GeneratorDetails {
    fn new(
        target: u16,
        true_length: u8,
        final_generation_size: u16,
        max_size: u16,
        string: String,
    ) -> GeneratorDetails {
        //! Creates a new Details struct for the Generated passwords.
        GeneratorDetails {
            password: string,
            target_bytesize: target,
            max_bytes: max_size,
            final_bytesize: final_generation_size,
            true_length: true_length,
        }
    }
    pub fn get_password(&self) -> &String {
        //! Gets the actual password and returns it
        //! as a reference.
        return &self.password;
    }
    fn get_target_bytesize(&self) -> u16 {
        //! Gets the target bytesize that the password generation engine automatically
        //! set when ran.
        return self.target_bytesize.to_owned();
    }
    fn get_max_byte_size(&self) -> u16 {
        //! gets the maximum size bytes the generator set at it's start.
        return self.max_bytes.to_owned();
    }
    fn get_final_bytesize(&self) -> u16 {
        //! gets the final bytesize after creating the string.
        return self.final_bytesize.to_owned();
    }
    fn len(&self) -> u8 {
        //! gets the actual length of the string-- excluding utf-8 padding.
        return self.true_length.to_owned();
    }
}

fn generate_password(length: u8, character_list: Vec<char>) -> GeneratorDetails {
    //! The actual password engine that genpass-rs uses, returns the generator
    //! details which is used in the `test()` modules and from versions 2.0.0+ on will be the
    //! defacto method of storing passwords.
    let mut bytesize: u16 = 0;
    let mut target_bytesize: u16 = length.into();
    let max_size: u16 = target_bytesize * 2;
    let mut truecount: u8 = 0;

    let mut final_password: String = String::new();
    let mut random = StdRng::from_os_rng();
    while bytesize != target_bytesize {
        let x: usize = random.random_range(0..character_list.len());
        let c: char = character_list.get(x).unwrap().to_owned();

        final_password.push(c);

        // count the current bytes and keeps track of the target bytesize.
        if x > 128 && target_bytesize < max_size {
            target_bytesize += 1;
            bytesize += 2;
            truecount += 1;
        } else {
            bytesize += 1;
            truecount += 1;
        }
    }
    GeneratorDetails::new(
        target_bytesize,
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

    match password_options.password_type() {
        PasswordType::Regular => {
            for i in char_min..=127 {
                valid_charlist.push(i as char);
            }

            // generates extasc if set as a parameter, and with a 'pwetty pwease',
            let char_ranges: [std::ops::RangeInclusive<i16>; 4] =
            // I used hex values for ease of reading. It's not true extended ascii--
            // but utf-8 up to value 255. These ranges essentially remove all non-printable
            // characters from being pushed to our character list.
                [0xa1..=0xac, 0xae..=0xb3, 0xb5..=0xb7, 0xb9..=0xff];
            if char_max == 255 {
                for x in char_ranges {
                    for i in x {
                        // looks scary-- but will always works as intended.
                        valid_charlist.push(i as u8 as char);
                    }
                }
            }
        }
        PasswordType::Alphanumeric => {
            // i forgot about the special chars between numbers and non-numbers. whoops.
            if char_min == 48 {
                let numbers: std::ops::RangeInclusive<u8> = char_min..=57;

                for i in numbers {
                    valid_charlist.push(i as char);
                }

                char_min = 65
            }

            for i in char_min..=char_max {
                valid_charlist.push(i as char);
            }
        }
        PasswordType::Numeric => {
            for i in char_min..=char_max {
                valid_charlist.push(i as char);
            }
        }
    }
    let password_details = generate_password(length, valid_charlist);

    if debug {
        dbg!(&password_options);
        testing(&password_details);
    }
    return password_details;
}

// tests
mod tests {
    #[allow(dead_code)]
    const TESTCOUNT: std::ops::Range<usize> = 0..50000;

    use super::GeneratorDetails;
    use crate::args::*;

    #[allow(dead_code)]
    fn start_test<T: CheckArgs + Clone>(generation_type: T) -> GeneratorDetails {
        // run at every test start
        let arg_type = generation_type.clone().check_arguments();
        return super::generate(arg_type, generation_type.get_length(), false);
    }
    #[test]
    fn test_ascii_string_generation() {
        let mut previous_string = String::new();
        for _ in TESTCOUNT {
            let generation_type = StringArgs {
                encoding: String::from("ascii"),
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
    fn test_numeric() {
        let mut previous_string = String::new();
        for _ in TESTCOUNT {
            let generation_type = IntegerArgs { length: 20 };
            let password = start_test(generation_type.to_owned());
            for i in password.get_password().chars() {
                let _: u8 = i.to_string().parse::<u8>().expect("was not a number");
            }
            assert_ne!(previous_string, *password.get_password());
            previous_string = password.get_password().clone()
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
                    97..=122 => (),
                    _ => panic!("bad character!"),
                }
            }
        }
    }
}
