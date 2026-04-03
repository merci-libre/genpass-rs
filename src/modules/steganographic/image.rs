use core::str;
use std::{error::Error, io::Write, path::Path, process::exit};

use stegano;

use rpassword::read_password;
use steganography::{
    self,
    util::{file_as_dynamic_image, file_as_image_buffer, save_image_buffer},
};
// use stegano's utilities to encrypt a payload with AES-128 Encryption.

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

trait ClassicEncryption {
    fn encrypt(self, key: &str) -> Result<Vec<u8>, Box<dyn Error>>;
    fn decrypt(self, key: String) -> Option<String>;
}

impl ClassicEncryption for Vec<u8> {
    fn encrypt(mut self, key: &str) -> Result<Vec<u8>, Box<dyn Error>> {
        let length: usize = self.len();
        let excess: usize = length % 16;
        /*find the excess size of inputted string, if >0, pad the rest of the string with zeroes for encryption.*/
        if excess > 0 {
            for _i in 0..(16 - excess) {
                self.push(0);
            }
        }
        let password = &String::from_utf8_lossy(&self);
        // this breaks if modified-- do not touch.
        let encrypted = stegano::utils::encrypt_payload(key, password);
        Ok(encrypted) // finish
    }

    fn decrypt(self, key: String) -> Option<String> {
        // this function is giving issues with utf-8
        let decrypted = stegano::utils::decrypt_data(key.as_str(), &self);
        let password = match String::from_utf8(decrypted) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("Failed to convert: reason {e}");
                return None;
            }
        };
        Some(password)
    }
}

fn create_password() -> Result<String, Box<dyn Error>> {
    let mut key: String;
    loop {
        eprint!("Enter your password (Must be 6-16 characters): ");
        std::io::stdout().flush()?;
        key = read_password()?;
        std::io::stdout().flush()?;

        eprint!("Please re-enter your password: ");
        std::io::stdout().flush()?;
        let key2 = read_password()?;
        std::io::stdout().flush()?;

        if key == key2 && key.len() > 5 {
            break;
        } else {
            // handle passkey length issues...
            match key.len() {
                0..6 => {
                    eprintln!("Password is less than 6 characters! Please re-enter your password")
                }
                6..16 => (),
                _ => eprintln!(
                    "Password is greater than 16 characters! Please re-enter your password!"
                ),
            }
            // handle passkey mismatch
            if key != key2 {
                eprintln!("Passwords did not match! Please re-enter your password.");
            }
            key.clear();
        }
    }
    Ok(key)
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

    // password attempts
    let mut attempts = 4;
    while attempts > 0 {
        let out_buffer = dec.decode_alpha();
        let clean_buffer: Vec<u8> = out_buffer.into_iter().filter(|b| *b != 0xff_u8).collect();
        eprint!("Enter your password: ");

        std::io::stdout()
            .flush()
            .expect("Failed to flush the screen");

        let key = match read_password() {
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
