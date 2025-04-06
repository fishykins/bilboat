#[derive(Clone, Copy, Debug)]
pub enum Encryption {
    /// Plain text
    None,
    /// Default AES-Siv encryption
    #[cfg(feature = "encryption")]
    Default,
    /// Custom encryption method
    Custom(fn(data: &Vec<u8>, key: &str) -> Vec<u8>),
}

#[cfg(feature = "encryption")]
impl Default for Encryption {
    fn default() -> Self {
        Encryption::Default
    }
}
#[cfg(not(feature = "encryption"))]
impl Default for Encryption {
    fn default() -> Self {
        Encryption::None
    }
}


#[cfg(feature = "encryption")]
pub mod aes_siv {
    use aes_siv::{siv::Aes256Siv, KeyInit};
    use generic_array::typenum;
    use rand::Rng;
    use sha2::{digest::generic_array::GenericArray, Digest, Sha256};

    /// Derives a fixed-length 32-byte key from a user-provided passphrase.
    fn derive_key_from_passphrase(passphrase: &str) -> GenericArray<u8, typenum::U64> {
        let hash = Sha256::digest(passphrase.as_bytes());
        let mut key_bytes = Vec::with_capacity(64);
        key_bytes.extend_from_slice(&hash);
        key_bytes.extend_from_slice(&hash); // Repeat the 32-byte hash to make it 64 bytes

        GenericArray::clone_from_slice(&key_bytes)
    }

    /// Encrypts a message using AES-SIV and a user-provided key.
    pub fn encrypt_aes_siv(message: &Vec<u8>, key: &str) -> Vec<u8> {
        let key_bytes = derive_key_from_passphrase(key);
        let mut cipher = Aes256Siv::new(&key_bytes); // Cipher must be mutable

        // Generate a random 16-byte nonce
        let nonce: [u8; 16] = rand::rng().random();

        // Encrypt the message
        let ciphertext = cipher
            .encrypt(&[&nonce], message.as_slice())
            .expect("Encryption failed");

        // Concatenate nonce + ciphertext and Base64 encode
        let mut combined = nonce.to_vec();
        combined.extend(ciphertext);
        combined
    }

    /// Decrypts an AES-SIV encrypted message using the user-provided key.
    pub fn decrypt_aes_siv(data: &Vec<u8>, key: &str) -> Vec<u8> {
        let key_bytes = derive_key_from_passphrase(key);
        let mut cipher = Aes256Siv::new(&key_bytes); // Cipher must be mutable

        // Decode from Base64
        //let data = safe_base64_decode(encrypted, key);

        if data.len() < 16 {
            // Bad decrypt- not enough data to form a solid nonce/cipher so just return. 
            return data.clone()
        }
        
        // Split into nonce and ciphertext
        let (nonce, ciphertext) = data.split_at(16);

        // Decrypt
        match cipher.decrypt(&[nonce], ciphertext) {
            Ok(result) => result,
            Err(_) => ciphertext.to_vec(),
        }
    }

    #[cfg(test)]
    mod tests {
        use crate::aes_siv::*;

        #[test]
        fn test_aes_siv_encryption() {
            let secret_message = "Hello, Rust!".as_bytes().to_vec();
            let key = "super_secret_passphrase";

            let encrpytion = encrypt_aes_siv(&secret_message, key);
            let decrypted_message = decrypt_aes_siv(&encrpytion, key);
            let wrong_decrypted_message = decrypt_aes_siv(&encrpytion, "wrong_key");
            assert_eq!(decrypted_message, secret_message);
            assert_ne!(wrong_decrypted_message, secret_message);
        }
    }
}
