#[cfg(feature = "encryption")]
use crate::aes_siv::*;
use crate::{key_to_seed, Encryption, WavBuffer};
use hound::WavWriter;
use rand::rngs::StdRng;
use rand::SeedableRng;
use rand::{seq::SliceRandom, Rng};
use std::io::{Cursor, Read, Seek};

/// Embeds the given bytes into the wav buffer. This will apply a shuffle on the sample order as a final layer of encryption.
pub fn embed_bytes<R, I>(
    wav: &WavBuffer<R>,
    bytes: &mut I,
    key: &str,
) -> Result<WavBuffer<Cursor<Vec<u8>>>, String>
where
    R: Read + Seek + Clone,
    I: Iterator<Item = u8>,
{
    let spec = wav.clone().get_spec().expect("Failed to read WAV specs");
    let mut samples: Vec<i16> = wav.read_samples().expect("Failed to read WAV samples");

    let seed = key_to_seed(key);
    let mut rng = StdRng::seed_from_u64(seed);
    let mut indices: Vec<usize> = (0..samples.len()).collect();
    indices.shuffle(&mut rng);

    // Embed message or random noise into each sample
    for &index in indices.iter() {
        if let Some(bit) = bytes.next() {
            samples[index] = (samples[index] & !1) | bit as i16; // Set LSB to message bit
        } else {
            // Generate random printable ASCII character
            let random_char = rng.random_range(32..127) as u8;
            samples[index] = (samples[index] & !1) | random_char as i16;
        }
    }

    // Write to an in-memory buffer instead of a file
    let mut buffer = Vec::new();
    {
        let mut writer =
            WavWriter::new(Cursor::new(&mut buffer), spec).map_err(|e| e.to_string())?;
        for sample in samples {
            writer.write_sample(sample).unwrap();
        }
    }

    Ok(WavBuffer::new(Cursor::new(buffer)))
}

/// Embeds a binary message into randomly selected LSBs (least significant bit) of PCM samples. If an encryption method is provided, it will be implimented.
/// Otherwise, the default encryption will be applied (unless the crate feature "encryption" is disabled, in which case plain text is used).
pub fn embed_message<R: Read + Seek + Clone>(
    wav: &WavBuffer<R>,
    message: &str,
    key: &str,
    encryption: Encryption,
) -> Result<WavBuffer<Cursor<Vec<u8>>>, String> {
    let encrypted_message: String = match encryption {
        Encryption::None => message.to_string(),
        #[cfg(feature = "encryption")]
        Encryption::AesSiv => encrypt_aes_siv(message, key),
        Encryption::Custom(encryption) => encryption(message, key),
    };

    let samples: Vec<i16> = wav.read_samples().expect("Failed to read WAV samples");

    let mut message_bytes = encrypted_message
        .as_bytes()
        .iter()
        .flat_map(|&byte| (0..8).rev().map(move |i| (byte >> i) & 1))
        .chain((0..8).map(|_| 0)) // Append null terminator
        .take(samples.len());

    embed_bytes(wav, &mut message_bytes, key)
}
