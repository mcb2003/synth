#[derive(Debug)]
pub struct OscSettings {
    pub sample_rate: f32,
    pub frequency: f32,
    pub amplitude: f32,
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
