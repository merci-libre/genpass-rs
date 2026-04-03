use super::encrypt::legacy::ClassicEncryption;

use console;
use std::{error::Error, io::Write, path::Path, process::exit};

use steganography::{
    self,
    util::{file_as_dynamic_image, file_as_image_buffer, save_image_buffer},
};

/*
* all password handling and things with encryption and embedding into jpeg or png photo occurs in
* this file. The purpose of this file is to handle all standard input regarding the aes-128 key
* */
trait OutputFormatting {
    fn format_output(&mut self, output_fname: String) -> String;
}
impl OutputFormatting for String {
    fn format_output(&mut self, output_fname: String) -> String {
        //! Formats the output string to prevent overwriting the
        //! file at a given path. Outputs should be unique.
        let substring = "_output";
        if output_fname.eq(&String::from("")) {
            self.push_str(substring);
        } else {
            *self = output_fname
        }

        let image_extensions = [".png", ".jpeg", ".jpg"];
        for extension in image_extensions {
            if self.contains(extension) {
                *self = self.replace(extension, "");
            }
        }

        self.push_str(".png");

        // handle duplicate outputs.
        let mut i: i64 = 0;

        // this might be able to be re-written better but it works
        while Path::new(&self).exists() {
            let previous_iteration = i - 1;

            //remove .png
            if self.contains(".png") {
                *self = self.replace(".png", "");
            }

            let prev_substring: String = match i {
                0 => substring.to_string(),
                _ => format!("{}{}", substring, previous_iteration), // i.e. _output1 -> output0
            };

            *self = self.replace(prev_substring.as_str(), "");

            // add our substring
            let new_outputsub = format!("{}{}", substring, i);
            self.push_str(new_outputsub.as_str());
            self.push_str(".png");
            i += 1;
        }
        return self.clone();
    }
}

fn create_password() -> Result<String, Box<dyn Error>> {
    let mut password = String::new();
    eprint!("Enter your password (Must be 6-16 characters): ");
    let term = console::Term::stderr();
    let mut low_strength_reask = true;
    while password.len() < 6 || low_strength_reask {
        std::io::stderr().flush()?;
        password = term.read_secure_line()?;
        std::io::stderr().flush()?;

        // handle passkey length issues...
        match password.len() {
            0..6 => {
                eprint!("Password is less than 6 characters! Please enter a valid password:");
                password.clear();
                continue;
            }
            6..16 => (),
            _ => {
                eprint!("Password is greater than 16 characters! Please enter a valid password:");
                password.clear();
                continue;
            }
        }

        eprint!("Please re-enter your password: ");
        std::io::stderr().flush()?;
        let re_enter = term.read_secure_line()?;
        std::io::stderr().flush()?;

        // handle passkey mismatch
        if password != re_enter {
            eprint!("Passwords did not match! Please re-enter your password:");
            password.clear();
            continue;
        }

        low_strength_reask =
            crate::modules::passwords::zxcvbn::check_password_strength(password.as_str());

        if low_strength_reask {
            eprint!("\nEnter your password (Must be 6-16 characters): ");
        }
    }

    Ok(password)
}

pub fn store(
    mut in_file_path: String,
    output_fname: String,
    payload: &String,
    unencrypted: bool,
) -> Result<(), Box<dyn Error>> {
    //! stores a payload into a given image. Takes the file and outp

    let mut string_to_store: Vec<u8> = Vec::from(payload.as_bytes().to_vec());
    if !unencrypted {
        // create a password
        let key = create_password()?;
        //end key
        string_to_store = string_to_store.encrypt(key.as_str())?;
    }
    // steganography stuff
    let img = file_as_dynamic_image(in_file_path.to_owned());
    let enc = steganography::encoder::Encoder::new(&string_to_store, img);
    let steg_image = enc.encode_alpha();

    // format the output to a readable format.
    in_file_path.format_output(output_fname);
    eprintln!("Storing data into {}", in_file_path);

    save_image_buffer(steg_image, in_file_path.to_string());
    Ok(eprintln!("Saved buffer to {}", in_file_path))
}

pub fn extract(in_file: &String) {
    //! Extracts the text from an image given a reference to the path.

    // decrypt from image.
    let file_buffer = String::from(in_file);
    let encoded_img = file_as_image_buffer(file_buffer);
    let dec = steganography::decoder::Decoder::new(encoded_img);
    let term = console::Term::stderr();

    // password attempts
    let mut attempts = 4;
    while attempts > 0 {
        let out_buffer = dec.decode_alpha();
        let clean_buffer: Vec<u8> = out_buffer.into_iter().filter(|b| *b != 0xff_u8).collect();
        eprint!("Enter your password: ");

        std::io::stdout()
            .flush()
            .expect("Failed to flush the screen");

        let key = match term.read_secure_line() {
            Ok(e) => e,
            Err(e) => {
                eprintln!("Error parsing key: {e}");
                exit(1)
            }
        };

        match clean_buffer.decrypt(key) {
            Some(v) => {
                println!("{}", v);
                attempts = 0;
            }
            None => {
                attempts -= 1;
                eprintln!("Password failed to decrypt. You have {attempts} attempt(s) remaining.");
            }
        }
    }
}

pub fn extract_raw_unencrypted(path: &String) -> Result<String, Box<dyn Error>> {
    let encoded_img = file_as_image_buffer(String::from(path));
    let dec = steganography::decoder::Decoder::new(encoded_img);
    let out_buffer = dec.decode_alpha();
    let clean_buffer: Vec<u8> = out_buffer.into_iter().filter(|b| *b != 0xff_u8).collect();

    let string = String::from_utf8(clean_buffer)?;
    Ok(string)
}
