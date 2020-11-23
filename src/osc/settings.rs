use super::Error;

#[derive(Debug)]
pub struct OscSettings {
    pub sample_rate: f32,
    pub frequency: f32,
    pub amplitude: f32,
}

impl OscSettings {
    pub fn new(sample_rate: f32, frequency: f32, amplitude: f32) -> Result<Self, Error> {
        if sample_rate <= 0.0 {
            return Err(Error::InvalidSampleRate(sample_rate));
        }
        if frequency <= 0.0 {
            return Err(Error::InvalidFrequency(frequency));
        }
        if amplitude < 0.0 || amplitude > 1.0 {
            return Err(Error::InvalidAmplitude(amplitude));
        }
        Ok(Self {
            sample_rate,
            frequency,
            amplitude,
        })
    }
}

impl Default for OscSettings {
    fn default() -> Self {
        Self {
            sample_rate: 44_100.0,
            frequency: 440.0,
            amplitude: 0.5,
        }
    }
}
