use hound::{WavReader, WavSpec, WavWriter};
use std::{
    f32::consts::PI,
    fs::File,
    io::{Cursor, Read, Seek, Write},
};

/// A helper struct to easily manipulate WAV files or buffers.
#[derive(Clone)]
pub struct WavBuffer<P: Read + Seek + Clone> {
    pub buffer: P,
}

impl WavBuffer<Cursor<Vec<u8>>> {
    /// Generates a wav consisting of a sin wave. Useful for testing purposes.
    pub fn sin(seconds: i32) -> Self {
        // Define WAV file properties
        let spec = WavSpec {
            channels: 1,
            sample_rate: 44100,
            bits_per_sample: 16,
            sample_format: hound::SampleFormat::Int,
        };

        // Create a buffer in memory
        let mut buffer = Vec::new();
        {
            let mut writer = WavWriter::new(Cursor::new(&mut buffer), spec).unwrap();

            for t in (0..44100 * seconds).map(|x| x as f32 / 44100.0) {
                let sample = (t * 440.0 * 2.0 * PI).sin();
                let amplitude = i16::MAX as f32;
                writer.write_sample((sample * amplitude) as i16).unwrap();
            }
        }
        Self {
            buffer: Cursor::new(buffer),
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
}

impl<P: Read + Seek + Clone> WavBuffer<P> {
    /// Creates a new `WavBuffer` from any readable + seekable source.
    pub fn new(buffer: P) -> Self {
        Self { buffer }
    }


    /// Reads WAV file metadata and returns its specification.
    pub fn get_spec(&self) -> Result<WavSpec, hound::Error> {
        let my_clone = self.buffer.clone();
        let reader = WavReader::new(my_clone)?;
        Ok(reader.spec())
    }

    /// Reads all samples from the WAV file.
    pub fn read_samples(&self) -> Result<Vec<i16>, hound::Error> {
        let my_clone = self.buffer.clone();
        let mut reader = WavReader::new(my_clone)?;
        let samples: Vec<i16> = reader.samples::<i16>().map(|s| s.unwrap()).collect();
        Ok(samples)
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
