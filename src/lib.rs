mod embed;
mod encryption;
mod extract;

pub use embed::embed_message;
pub use encryption::Encrypt;
pub use extract::extract_message;
use sha2::{Digest, Sha256};

/// Converts a string key into a numeric seed using SHA-256 hashing.
pub(crate) fn key_to_seed(key: &str) -> u64 {
    let hash = Sha256::digest(key.as_bytes());
    u64::from_le_bytes(hash[..8].try_into().unwrap())
}

#[cfg(test)]
mod tests {
    use crate::*;
    
    #[test]
    fn test_embed_and_extract_message_correct_key() {
        let original_wav = "input.wav";
        let stego_wav = "stego.wav";
        let secret_message = "Hello, Rust!";
        let key = "super_secret_passphrase";

        embed_message(original_wav, stego_wav, secret_message, key, None);
        let extracted_message = extract_message(stego_wav, key, None);

        assert_eq!(extracted_message, secret_message);
    }

    #[test]
    fn test_embed_and_extract_message_incorrect_key() {
        let original_wav = "input.wav";
        let stego_wav = "stego.wav";
        let secret_message = "Hello, Rust!";
        let correct_key = "super_secret_passphrase";
        let incorrect_key = "wrong_keyzz";

        embed_message(original_wav, stego_wav, secret_message, correct_key, None);
        let extracted_message = extract_message(stego_wav, incorrect_key, None);

        // The extracted message should not match the original message with the wrong key
        assert_ne!(extracted_message, secret_message);
        assert!(extracted_message.chars().all(|c| c.is_ascii() && c != '\0')); // Ensure random chars
    }
}
