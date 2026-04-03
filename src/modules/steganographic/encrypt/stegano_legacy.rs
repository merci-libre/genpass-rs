/*
*
* This code is a snippet from another repository-> stegano
*
* This was included here as a backup since they kind of deleted their account,
* and I no longer wanted to include the entire project as a dependency.
*
* credit to wiseaidev for this snippet.
* sourced from stegano: https://github.com/wiseaidev/stegano/blob/main/src/utils.rs
*
* I do plan on switching to aes256 at some point as well for default encryption, and use
* this as a legacy decryption process. Otherwise, it's just here to fulfill it's role as
* the encryption and decryption process.
*/

use aes::cipher::{generic_array::GenericArray, BlockDecrypt, BlockEncrypt, KeyInit};
use aes::Aes128;

pub fn pad_with_zeros(slice: &[u8]) -> [u8; 16] {
    let mut padded_array: [u8; 16] = [0; 16];
    let len = std::cmp::min(slice.len(), padded_array.len());
    padded_array[..len].copy_from_slice(&slice[..len]);
    padded_array
}

pub fn encrypt_payload(key: &str, payload: &str) -> Vec<u8> {
    let in_key: &[u8; 16] = &pad_with_zeros(key.as_bytes());
    let key = GenericArray::clone_from_slice(in_key);

    if payload.len() <= 16 {
        let in_payload: &[u8; 16] = &pad_with_zeros(payload.as_bytes());
        let mut block = GenericArray::clone_from_slice(in_payload);

        let cipher = Aes128::new(&key);
        cipher.encrypt_block(&mut block);
        block.to_vec()
    } else {
        let mut encrypted_data: Vec<u8> = Vec::new();

        for (i, chunk) in payload.as_bytes().chunks_exact(16).enumerate() {
            let in_payload: &[u8; 16] = &pad_with_zeros(chunk);
            let mut block = GenericArray::clone_from_slice(in_payload);

            let cipher = Aes128::new(&key);
            cipher.encrypt_block(&mut block);

            if i > 0 {
                encrypted_data.extend_from_slice(&block);
            } else {
                encrypted_data = block.to_vec();
            }
        }

        encrypted_data
    }
}
pub fn decrypt_data(key: &str, data: &[u8]) -> Vec<u8> {
    let in_key: &[u8; 16] = &pad_with_zeros(key.as_bytes());
    let key = GenericArray::clone_from_slice(in_key);

    let mut decrypted_data: Vec<u8> = Vec::new();

    for (i, chunk) in data.chunks_exact(16).enumerate() {
        let in_payload: &[u8; 16] = &pad_with_zeros(chunk);
        let mut block = GenericArray::clone_from_slice(in_payload);

        let cipher = Aes128::new(&key);
        cipher.decrypt_block(&mut block);

        if i > 0 {
            decrypted_data.extend_from_slice(&block);
        } else {
            decrypted_data = block.to_vec();
        }
    }

    decrypted_data
}
