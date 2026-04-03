use super::stegano_legacy;
use std::error::Error;
pub trait ClassicEncryption {
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
        let encrypted = stegano_legacy::encrypt_payload(key, password);
        Ok(encrypted) // finish
    }

    fn decrypt(self, key: String) -> Option<String> {
        // this function is giving issues with utf-8
        let decrypted = stegano_legacy::decrypt_data(key.as_str(), &self);
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
