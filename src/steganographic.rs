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
*
*
* */

fn encrypt(input_string_vector: Vec<u8>, key: &str) -> Vec<u8> {
    let mut enc_payload = input_string_vector;
    let length: usize = enc_payload.len().into();
    let excess: usize = length % 16;
    /*find the excess size of inputted string, if >0, pad the rest of the string with zeroes for encryption.*/
    if excess > 0 {
        for _i in 0..(16 - excess) {
            enc_payload.push(0);
        }
    }
    // catch keyless encryption:: if you want no key, use option -u
    if key == "" {
        eprintln!("There is no key to encrypt the payload, exiting the program safely.");
        exit(1);
    }
    let conversion = match str::from_utf8(&enc_payload) {
        Ok(v) => v,
        Err(_e) => {
            eprintln!("Error: Something went wrong encrypting the payload!");
            exit(1)
        } // if process to convert vector back into a string fails, exit out of the program.
    };
    let encrypted = stegano::utils::encrypt_payload(key, &conversion);
    return encrypted; // finish
}

fn decrypt(input_vec: &Vec<u8>, key: &str, count: u8) -> u8 {
    let decrypted = stegano::utils::decrypt_data(key, &input_vec);
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

fn format_output(mut input: String, output_fname: String) -> String {
    let mut substring = "_output";
    if output_fname.eq(&String::from("")) {
        input.push_str(substring);
    } else {
        input = output_fname
    }
    if input.contains(".png") {
        input = input.replace(".png", "");
    }
    if input.contains(".jpeg") {
        input = input.replace(".jpeg", "");
    }

    if input.contains(".jpg") {
        input = input.replace(".jpg", "");
    }

    input.push_str(".png");
    // handle duplicate outputs.
    if Path::new(&input).exists() {
        let mut i: u32 = 0;
        while Path::new(&input).exists() {
            loop {
                if input.contains(".png") {
                    input = input.replace(".png", "");
                }
                let mut new_outputsub = format!("{}{}", substring, i);
                let prev_substring: String;
                match i {
                    0 => prev_substring = substring.to_string(),
                    _ => prev_substring = format!("{}{}", substring, i - 1),
                }
                if input.contains(new_outputsub.as_str()) {
                    new_outputsub = format!("{}{}", substring, i);
                }
                input = input.replace(prev_substring.as_str(), "");

                input = input.replace(substring, "");
                input.push_str(new_outputsub.as_str());
                break;
            }
            input.push_str(".png");
            i += 1;
        }
    }
    return input;
}

pub fn store(out_file: String, output_fname: String, payload: String, unencrypted: bool) -> bool {
    let mut vector = Vec::new();
    if payload.len() > 240 {
        // exits the program with an error from the main file because the payload is greater than 240 characters.
        return false;
    } else {
        let mut mout = out_file.clone();
        let mut key = String::new();
        if !unencrypted {
            // create a password

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
                        0..6 => eprintln!(
                            "Password is less than 6 characters! Please re-enter your password"
                        ),
                        6..16=>(),
                        _=>eprintln!("Password is greater than 16 characters! Please re-enter your password!"),
                    }
                    // handle passkey mismatch
                    if key != key2 {
                        eprintln!("Passwords did not match! Please re-enter your password.");
                    }
                    key.clear();
                }
            }
        }
        // set the vector to the payload in bytes, and match whether the payload is encrypted or
        // not.
        match unencrypted {
            true => vector = payload.as_bytes().to_vec(),
            false => vector = encrypt(payload.as_bytes().to_vec(), key.as_str()),
        }

        // steganography stuff
        let img = file_as_dynamic_image(out_file.clone());
        let enc = steganography::encoder::Encoder::new(&vector, img);
        let result = enc.encode_alpha();

        // format the output to a readable format.
        mout = format_output(mout, output_fname);
        println!("Storing data into {}", mout);

        save_image_buffer(result, mout.to_string());
        println!("Saved buffer to {}", mout);
        return true;
    }
}

pub fn extract(in_file: String) {
    // decrypt from image.
    let encoded_img = file_as_image_buffer(in_file);
    let dec = steganography::decoder::Decoder::new(encoded_img);
    let out_buffer = dec.decode_alpha();
    let clean_buffer: Vec<u8> = out_buffer.into_iter().filter(|b| *b != 0xff_u8).collect();

    //password

    let mut attempts = 4;
    while attempts != 0 {
        print!("Enter your password: ");
        std::io::stdout().flush().unwrap();
        let key = read_password().unwrap();

        attempts = decrypt(&clean_buffer, key.as_str(), attempts);
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
