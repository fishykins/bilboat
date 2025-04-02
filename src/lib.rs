mod embed;
mod encryption;
mod extract;
mod wav_buffer;

pub use embed::embed_message;
pub use encryption::*;
pub use extract::extract_message;
pub use wav_buffer::*;
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
        let secret_message = "Hello, Rust!";
        let key = "super_secret_passphrase";

        let wav = WavBuffer::sin(1);

        let buffer = embed_message(
            &wav,
            secret_message,
            key,
            Encryption::None,
        ).unwrap();

        let extracted = extract_message(&buffer, key, Encryption::None);
        assert_eq!(extracted, secret_message);

        let extracted_wrong = extract_message(&buffer, "wrong_key", Encryption::None);
        assert_ne!(extracted_wrong, secret_message);
    }
}
