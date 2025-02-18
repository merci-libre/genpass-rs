use core::str;
use std::{io, io::Write};

use std::process::exit;
use stegano;

use steganography::{
    self,
    util::{file_as_dynamic_image, file_as_image_buffer, save_image_buffer},
};

use rpassword::read_password;
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
    let conversion = match str::from_utf8(&enc_payload) {
        Ok(v) => v,
        Err(_e) => panic!("something went wrong encrypting the payload!"), // if process to convert vector back into a string fails, exit out of the program.
    };
    let encrypted = stegano::utils::encrypt_payload(key, &conversion);
    return encrypted; // finish
}

fn decrypt(input_vec: Vec<u8>, key: &str) {
    let decrypted = stegano::utils::decrypt_data(key, &input_vec);
    let conversion = match str::from_utf8(&decrypted) {
        Ok(v) => v,
        Err(_e) => panic!("something went wrong decrypting the payload!"), // if process to convert vector back into a string fails, exit out of the program.
    };
    println!("\n{}", conversion);
}

pub fn store(out_file: String, payload: String) {
    print!("Enter your password: ");
    std::io::stdout().flush().unwrap();
    let key = read_password().unwrap();
    let encryptedvector = encrypt(payload.as_bytes().to_vec(), key.as_str());

    // steganography stuff
    let img = file_as_dynamic_image(out_file.clone());
    let enc = steganography::encoder::Encoder::new(&encryptedvector, img);
    let result = enc.encode_alpha();
    save_image_buffer(result, out_file.to_string());
}
pub fn extract(in_file: String) {
    // decrypt from image.
    let encoded_img = file_as_image_buffer(in_file);
    let dec = steganography::decoder::Decoder::new(encoded_img);
    let out_buffer = dec.decode_alpha();
    let clean_buffer: Vec<u8> = out_buffer.into_iter().filter(|b| *b != 0xff_u8).collect();

    //password
    print!("Enter your password: ");
    std::io::stdout().flush().unwrap();
    let key = read_password().unwrap();

    decrypt(clean_buffer, key.as_str());
}
