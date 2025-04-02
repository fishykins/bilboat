#[cfg(feature = "encryption")]
use crate::aes_siv::*;
use crate::encryption::Unencrypt;
use crate::key_to_seed;
use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use rand::SeedableRng;

/// Extracts a binary message by using the key to determine embedding positions.
/// If an decryption method is provided, it will be implimented. 
/// Otherwise, the default decryption will be applied (unless the crate feature "encryption" is disabled, in which case the plain text will be returned).
pub fn extract_message(wav: &str, key: &str, unencryption: Option<Unencrypt>,) -> String {
    let mut wav_reader = hound::WavReader::open(wav).expect("Failed to open WAV");
    let samples: Vec<i16> = wav_reader.samples::<i16>().map(|s| s.unwrap()).collect();
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

    let encryption = String::from_utf8_lossy(&bytes).to_string();
    if let Some(unencryption) = unencryption {
        return unencryption(&encryption, key);
    }

    #[cfg(feature = "encryption")]
        {
            decrypt_aes_siv(&encryption, key)
        }
        #[cfg(not(feature = "encryption"))]
        {
            encryption
        }
    

}
