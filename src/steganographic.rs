use core::str;
use std::{io::Write, path::Path, process::exit};

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
trait Formatting {
    fn format_output(&mut self, output_fname: String) -> String;
}
impl Formatting for String {
    fn format_output(&mut self, output_fname: String) -> String {
        let substring = "_output";
        if output_fname.eq(&String::from("")) {
            self.push_str(substring);
        } else {
            *self = output_fname
        }
        if self.contains(".png") {
            *self = self.replace(".png", "");
        }
        if self.contains(".jpeg") {
            *self = self.replace(".jpeg", "");
        }

        if self.contains(".jpg") {
            *self = self.replace(".jpg", "");
        }

        self.push_str(".png");
        // handle duplicate outputs.
        if Path::new(&self).exists() {
            let mut i: u32 = 0;
            while Path::new(&self).exists() {
                loop {
                    if self.contains(".png") {
                        *self = self.replace(".png", "");
                    }
                    let mut new_outputsub = format!("{}{}", substring, i);
                    let prev_substring: String;
                    match i {
                        0 => prev_substring = substring.to_string(),
                        _ => prev_substring = format!("{}{}", substring, i - 1),
                    }
                    if self.contains(new_outputsub.as_str()) {
                        new_outputsub = format!("{}{}", substring, i);
                    }
                    *self = self.replace(prev_substring.as_str(), "");

                    *self = self.replace(substring, "");
                    self.push_str(new_outputsub.as_str());
                    break;
                }
                self.push_str(".png");
                i += 1;
            }
        }
        return self.clone();
    }
}

trait ClassicEncryption {
    fn encrypt(&mut self, key: &str) -> Vec<u8>;
    fn decrypt(self, key: &str, count: u8) -> u8;
}

impl ClassicEncryption for Vec<u8> {
    fn encrypt(&mut self, key: &str) -> Vec<u8> {
        let payload = self.clone();
        let length: usize = payload.len() as usize;
        let excess: usize = length % 16;
        /*find the excess size of inputted string, if >0, pad the rest of the string with zeroes for encryption.*/
        if excess > 0 {
            for _i in 0..(16 - excess) {
                self.push(0);
            }
        }
        let conversion = match String::from_utf8(payload) {
            Ok(v) => v,
            Err(_e) => {
                eprintln!("Error: Something went wrong encrypting the payload!");
                exit(1)
            } // if process to convert vector back into a string fails, exit out of the program.
        };
        let encrypted = stegano::utils::encrypt_payload(key, &conversion);
        return encrypted; // finish
    }
    fn decrypt(self, key: &str, count: u8) -> u8 {
        let decrypted = stegano::utils::decrypt_data(key, &self);
        let conversion = match str::from_utf8(&decrypted) {
            Ok(v) => v,
            Err(_e) => {
                if count > 1 {
                    eprintln!("Wrong password. You have ({}) more attempts.", count - 1);
                }
                return count - 1;
            } // if process to convert vector back into a string fails, exit out of the program.
        };
        println!("\n{}", conversion);
        return 0;
    }
}

fn create_password() -> String {
    let mut key = String::new();
    loop {
        print!("Enter your password (Must be 6-16 characters): ");
        std::io::stdout().flush().unwrap();
        key = read_password().unwrap();
        std::io::stdout().flush().unwrap();

        print!("Please re-enter your password: ");
        std::io::stdout().flush().unwrap();
        let key2 = read_password().unwrap();
        std::io::stdout().flush().unwrap();

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
    return key;
}

pub fn store(
    mut out_file: String,
    output_fname: String,
    payload: String,
    unencrypted: bool,
) -> bool {
    if payload.len() > 240 {
        // exits the program with an error from the main file because the payload is greater than 240 characters.
        return false;
    }

    let mut vector = Vec::from(payload.as_bytes().to_vec());
    if !unencrypted {
        // create a password
        let key = create_password();
        //end key
        vector = vector.encrypt(key.as_str());
    } else {
        ()
    }
    // set the vector to the payload in bytes, and match whether the payload is encrypted or
    // not.

    // steganography stuff
    let img = file_as_dynamic_image(out_file.clone());
    dbg!(&vector);
    let enc = steganography::encoder::Encoder::new(&vector, img);
    let result = enc.encode_alpha();

    // format the output to a readable format.
    out_file.format_output(output_fname);
    println!("Storing data into {}", out_file);

    save_image_buffer(result, out_file.to_string());
    println!("Saved buffer to {}", out_file);
    return true;
}

pub fn extract(in_file: String) {
    // decrypt from image.
    let encoded_img = file_as_image_buffer(in_file);
    let dec = steganography::decoder::Decoder::new(encoded_img);

    //password
    let mut attempts = 4;
    while attempts != 0 {
        let out_buffer = dec.decode_alpha();
        let mut clean_buffer: Vec<u8> = out_buffer.into_iter().filter(|b| *b != 0xff_u8).collect();
        print!("Enter your password: ");
        std::io::stdout().flush().unwrap();
        let key = read_password().unwrap();

        attempts = clean_buffer.decrypt(key.as_str(), attempts);
    }
}

pub fn extract_raw(in_file: String) {
    let encoded_img = file_as_image_buffer(in_file);
    let dec = steganography::decoder::Decoder::new(encoded_img);
    let out_buffer = dec.decode_alpha();
    let clean_buffer: Vec<u8> = out_buffer.into_iter().filter(|b| *b != 0xff_u8).collect();

    let string = String::from_utf8_lossy(&clean_buffer);
    println!("{}", string);
}
