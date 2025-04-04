use hound::{WavReader, WavSpec, WavWriter};
use std::{
    f32::consts::PI,
    fs::File,
    io::{Cursor, Read, Seek, Write},
};

use crate::shuffle_indices;

fn default_spec() -> WavSpec {
    WavSpec {
        channels: 1,
        sample_rate: 44100,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    }
}

/// A helper struct to easily manipulate WAV files or buffers.
#[derive(Clone)]
pub struct WavBuffer<P: Read + Seek + Clone> {
    /// The data buffer
    pub buffer: P,
    /// Audio specs (from hound)
    pub spec: Option<WavSpec>,
}

/// The most common type of WavBuffer
impl WavBuffer<Cursor<Vec<u8>>> {
    /// Generates a wav consisting of a sin wave. Useful for testing purposes.
    pub fn sin(seconds: i32) -> Self {
        // Define WAV file properties
        let spec = default_spec();

        // Create a buffer in memory
        let mut buffer = Vec::new();
        {
            let mut writer = WavWriter::new(Cursor::new(&mut buffer), spec).unwrap();

            for t in (0..44100 * seconds).map(|x| x as f32 / 44100.0) {
                let sample = (t * 440.0 * 2.0 * PI).sin();
                let amplitude = i16::MAX as f32;
                writer.write_sample((sample * amplitude) as i16).unwrap();
            }
            writer.finalize().map_err(|e| e.to_string()).unwrap();
        }
        Self {
            buffer: Cursor::new(buffer),
            spec: Some(spec),
        }
    }

    /// Creates a `WavBuffer` from a file.
    pub fn from_file(path: &str) -> std::io::Result<WavBuffer<Cursor<Vec<u8>>>> {
        let mut file = File::open(path)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        Ok(WavBuffer::new(Cursor::new(buffer)))
    }

    pub fn write_to_file(self, path: &str) -> std::io::Result<()> {
        let mut file = File::create(path)?;
        let buffer = self.buffer.into_inner();
        file.write_all(&buffer)?;
        Ok(())
    }

    /// Embeds a given vec of bytes into the buffer.
    pub fn embed_bytes(&mut self, bytes: &Vec<u8>, key: &str) -> Result<(), String> {
        let mut samples = self.read_samples()?;
        // get a shuffled array of every possible sample we can use.
        let indices = shuffle_indices(key, samples.len());

        // Count the number of bytes to embed
        let byte_count = bytes.len();
        let bit_count = byte_count * 8;

        // Store message length first (16-bit number, 2 bytes)
        let length_bytes = (byte_count as u16).to_be_bytes();

        // Combine the bytes and flatten into bits!
        let bits = length_bytes
            .into_iter()
            .chain(bytes.clone()) // Convert bytes to iterator
            .flat_map(|byte| (0..8).rev().map(move |bit_pos| (byte >> bit_pos) & 1)); // Convert bytes to bits

        for (bit, &index) in bits.zip(indices.iter().take(bit_count + 16)) {
            samples[index] = (samples[index] & !1) | bit as i16; // Store LSB
        }

        let mut buffer = Vec::new();
        let mut writer = WavWriter::new(
            Cursor::new(&mut buffer),
            self.spec.unwrap_or(default_spec()),
        )
        .map_err(|e| e.to_string())?;
        for sample in samples {
            writer.write_sample(sample).unwrap();
        }

        writer.finalize().map_err(|e| e.to_string())?;
        self.buffer = Cursor::new(buffer);

        Ok(())
    }

    /// Extracts bytes from the buffer.
    pub fn extract_bytes(&self, key: &str) -> Result<Vec<u8>, String> {
        let samples = self.read_samples()?;
        let indices = shuffle_indices(key, samples.len());

        // Read first 16 bits (message length)
        let length_bits: Vec<u8> = indices
            .iter()
            .take(16)
            .map(|&index| (samples[index] & 1) as u8)
            .collect();
        let message_length = length_bits
            .iter()
            .fold(0u16, |acc, &bit| (acc << 1) | bit as u16);

        // Read only `message_length` bytes
        let bits: Vec<u8> = indices
            .iter()
            .skip(16)
            .take(message_length as usize * 8)
            .map(|&index| (samples[index] & 1) as u8)
            .collect();

        let bytes: Vec<u8> = bits
            .chunks(8)
            .map(|chunk| chunk.iter().fold(0, |acc, &b| (acc << 1) | b))
            .collect();

        Ok(bytes)
    }
}

impl<P: Read + Seek + Clone> WavBuffer<P> {
    /// Creates a new `WavBuffer` from any readable + seekable source.
    pub fn new(buffer: P) -> Self {
        Self {
            buffer,
            spec: Some(default_spec()),
        }
    }

    /// Reads WAV file metadata and returns its specification.
    pub fn get_spec(&self) -> Result<WavSpec, hound::Error> {
        let my_clone = self.buffer.clone();
        let reader = WavReader::new(my_clone)?;
        Ok(reader.spec())
    }

    /// Reads all samples from the WAV file.
    pub fn read_samples(&self) -> Result<Vec<i16>, String> {
        let my_clone = self.buffer.clone();

        match WavReader::new(my_clone) {
            Ok(mut reader) => {
                let samples: Vec<i16> = reader.samples::<i16>().map(|s| s.unwrap()).collect();
                Ok(samples)
            }
            Err(e) => {
                return Err(e.to_string());
            }
        }
    }

    /// Writes samples to a new in-memory WAV buffer and returns it.
    pub fn write_samples(
        samples: &[i16],
        spec: WavSpec,
    ) -> Result<WavBuffer<Cursor<Vec<u8>>>, hound::Error> {
        let mut buffer = Vec::new();
        {
            let mut writer = WavWriter::new(Cursor::new(&mut buffer), spec)?;
            for &sample in samples {
                writer.write_sample(sample)?;
            }
        }
        Ok(WavBuffer::new(Cursor::new(buffer)))
    }
}

/// Allow `WavBuffer` to be converted into `WavReader`
impl<P: Read + Seek + Clone> Into<WavReader<P>> for WavBuffer<P> {
    fn into(self) -> WavReader<P> {
        WavReader::new(self.buffer).expect("Failed to create WavReader from WavBuffer")
    }
}

/// Allow `WavBuffer` to be converted into `WavWriter`
impl<P: Read + Seek + Write + Clone> Into<WavWriter<P>> for WavBuffer<P> {
    fn into(self) -> WavWriter<P> {
        WavWriter::new(
            self.buffer,
            WavSpec {
                channels: 1,
                sample_rate: 44100,
                bits_per_sample: 16,
                sample_format: hound::SampleFormat::Int,
            },
        )
        .expect("Failed to create WavWriter from WavBuffer")
    }
}
