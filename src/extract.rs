#[cfg(feature = "encryption")]
use crate::aes_siv::*;
use crate::key_to_seed;
use crate::{encryption::Encryption, WavBuffer};
use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use rand::SeedableRng;
use std::io::{Read, Seek};

pub fn extract_bytes<R: Read + Seek + Clone>(wav: &WavBuffer<R>, key: &str) -> Vec<u8> {
    let samples: Vec<i16> = wav.read_samples().expect("Failed to read WAV samples");

    let seed = key_to_seed(key);
    let mut rng = StdRng::seed_from_u64(seed);
    let mut indices: Vec<usize> = (0..samples.len()).collect();
    indices.shuffle(&mut rng);

    let bits: Vec<u8> = indices
        .iter()
        .map(|&index| (samples[index] & 1) as u8) // Read LSB directly
        .collect();

    // Ensure we extract full bytes before stopping
    let mut bytes: Vec<u8> = bits
        .chunks(8)
        .map(|chunk| chunk.iter().fold(0, |acc, &b| (acc << 1) | b))
        .filter(|&b| b.is_ascii())
        .collect();

    // Remove trailing null terminators
    if let Some(pos) = bytes.iter().position(|&b| b == 0) {
        bytes.truncate(pos);
    }

    bytes
}

/// Extracts a binary message by using the key to determine embedding positions.
/// If an decryption method is provided, it will be implimented.
/// Otherwise, the default decryption will be applied (unless the crate feature "encryption" is disabled, in which case the plain text will be returned).
pub fn extract_message<R: Read + Seek + Clone>(
    wav: &WavBuffer<R>,
    key: &str,
    decryption: Encryption,
) -> String {
    let bytes = extract_bytes(wav, key);
    let encryption = String::from_utf8_lossy(&bytes).to_string();

    match decryption {
        Encryption::None => encryption.to_string(),
        #[cfg(feature = "encryption")]
        Encryption::AesSiv => decrypt_aes_siv(&encryption, key),
        Encryption::Custom(method) => method(&encryption, key),
    }
}
