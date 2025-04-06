mod encryption;
mod wav_buffer;

pub use encryption::*;
use rand::{rngs::StdRng, seq::SliceRandom, SeedableRng};
use sha2::{Digest, Sha256};
pub use wav_buffer::*;

/// Converts a string key into a numeric seed using SHA-256 hashing.
pub(crate) fn key_to_seed(key: &str) -> u64 {
    let hash = Sha256::digest(key.as_bytes());
    u64::from_le_bytes(hash[..8].try_into().unwrap())
}

/// Creates a vec of shuffled indices.
pub(crate) fn shuffle_indices(key: &str, length: usize) -> Vec<usize> {
    let seed = key_to_seed(key);
    let mut rng = StdRng::seed_from_u64(seed);
    let mut indices: Vec<usize> = (0..length).collect();
    indices.shuffle(&mut rng);
    indices
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_wav_buffer_read_write() {
        let mut buffer = WavBuffer::sin(1);
        let bytes: Vec<u8> = vec![1, 2, 3, 4, 5, 6];

        buffer.embed_bytes(&bytes, "key").unwrap();

        buffer.read_samples().unwrap();

        let extracted = buffer.extract_bytes("key").unwrap();
        assert_eq!(bytes, extracted);
    }

    #[test]
    fn test_wav_buffer_wrong_key() {
        let mut buffer = WavBuffer::sin(1);
        let bytes: Vec<u8> = vec![1, 2, 3, 4, 5, 6];

        buffer.embed_bytes(&bytes, "key").unwrap();

        let extracted = buffer.extract_bytes("wrong_key").unwrap();
        assert_ne!(bytes, extracted);
    }
}
