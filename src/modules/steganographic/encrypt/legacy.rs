use super::stegano_legacy;
pub trait ClassicEncryption {
    fn encrypt(self, key: String) -> Vec<u8>;
    fn decrypt(self, key: String) -> Option<String>;
}

impl ClassicEncryption for Vec<u8> {
    fn encrypt(self, key: String) -> Vec<u8> {
        //! Encrypts the inputted string from either the password generator or embed
        //! into AES-128.
        stegano_legacy::encrypt_payload(
            key.as_str(),
            String::from_utf8_lossy(&self).to_string().as_str(),
        )
    }

    fn decrypt(self, key: String) -> Option<String> {
        // this function is giving issues with utf-8
        let decrypted = stegano_legacy::decrypt_data(key.as_str(), self.as_slice());
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
