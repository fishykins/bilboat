#[cfg(feature = "encryption")]
use crate::aes_siv::*;
use crate::{key_to_seed, Encrypt};
use rand::rngs::StdRng;
use rand::SeedableRng;
use rand::{seq::SliceRandom, Rng};

/// Embeds a binary message into randomly selected LSBs of PCM samples. If an encryption method is provided, it will be implimented. 
/// Otherwise, the default encryption will be applied (unless the crate feature "encryption" is disabled, in which case plain text is used).
pub fn embed_message(
    original_wav: &str,
    output_wav: &str,
    message: &str,
    key: &str,
    encryption: Option<Encrypt>,
) {
    let mut reader = hound::WavReader::open(original_wav).expect("Failed to open WAV file");
    let spec = reader.spec();
    let mut samples: Vec<i16> = reader.samples::<i16>().map(|s| s.unwrap()).collect();

    let seed = key_to_seed(key);
    let mut rng = StdRng::seed_from_u64(seed);
    let mut indices: Vec<usize> = (0..samples.len()).collect();
    indices.shuffle(&mut rng);

    let encrypted_message: String = if let Some(encryption) = encryption {
        encryption(message, key)
    } else {
        #[cfg(feature = "encryption")]
        {
            encrypt_aes_siv(message, key)
        }
        #[cfg(not(feature = "encryption"))]
        {
            message.to_string()
        }
    };

    let mut message_bits = encrypted_message
        .as_bytes()
        .iter()
        .flat_map(|&byte| (0..8).rev().map(move |i| (byte >> i) & 1))
        .chain((0..8).map(|_| 0)) // Append null terminator
        .take(indices.len());

    // Embed message or random noise into each sample
    for &index in indices.iter() {
        if let Some(bit) = message_bits.next() {
            samples[index] = (samples[index] & !1) | bit as i16; // Set LSB to message bit
        } else {
            // Generate random printable ASCII character (between 32 and 126)
            let random_char = rng.random_range(32..127) as u8;
            samples[index] = (samples[index] & !1) | random_char as i16;
        }
    }

    let mut writer = hound::WavWriter::create(output_wav, spec).expect("Failed to create WAV file");
    for sample in samples {
        writer.write_sample(sample).unwrap();
    }
}
