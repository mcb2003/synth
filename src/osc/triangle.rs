use std::f32::consts::PI;

use super::Osc;

pub struct TriangleOsc {
    sample: u64,
    sample_rate: f32,
    frequency: f32,
    amplitude: f32,
}

impl TriangleOsc {
    pub fn new(sample_rate: f32, frequency: f32, amplitude: f32) -> Self {
        Self {
            sample: 0,
            sample_rate, frequency, amplitude
        }
    }
}

impl Osc for TriangleOsc {
    fn next_sample(&mut self) -> f32 {
        let sample = ((2.0 * self.amplitude) / PI) * (2.0 * PI * self.frequency * (self.sample as f32 / self.sample_rate)).sin().asin();
        self.sample += 1;
        sample
    }

    fn sample(&self) -> u64 {
        self.sample
    }

    fn cycle(&self) -> Option<u64> {
        Some((self.sample as f32 / self.frequency) as u64)
    }
}
