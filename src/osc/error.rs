#[derive(Debug)]
pub enum Error {
    InvalidSampleRate(f32),
        InvalidFrequency(f32),
        InvalidAmplitude(f32),
}
