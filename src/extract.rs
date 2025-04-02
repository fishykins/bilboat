use crate::encryption::Unencrypt;
use crate::key_to_seed;
use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use rand::SeedableRng;

/// Extracts a binary message by using the key to determine embedding positions.
pub fn extract_message(stego_wav: &str, key: &str, unencryption: Option<Unencrypt>,) -> String {
    let mut stego_reader = hound::WavReader::open(stego_wav).expect("Failed to open stego WAV");
    let stego_samples: Vec<i16> = stego_reader.samples::<i16>().map(|s| s.unwrap()).collect();
    let seed = key_to_seed(key);
    let mut rng = StdRng::seed_from_u64(seed);
    let mut indices: Vec<usize> = (0..stego_samples.len()).collect();
    indices.shuffle(&mut rng);

    let bits: Vec<u8> = indices
        .iter()
        .map(|&index| (stego_samples[index] & 1) as u8) // Read LSB directly
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
    encryption

}
