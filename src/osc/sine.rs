use std::f32::consts::PI;

use super::{Osc, OscSettings};

#[derive(Default)]
pub struct SineOsc {
    sample: u64,
    settings: OscSettings,
}

impl SineOsc {
    pub fn new(settings: OscSettings) -> Self {
        Self {
            sample: 0,
            settings,
        }
    }
}

impl Osc for SineOsc {
    fn next_sample(&mut self) -> f32 {
        let sample = self.settings.amplitude * (2.0 * PI * self.settings.frequency * (self.sample as f32 / self.settings.sample_rate)).sin();
        self.sample += 1;
        sample
    }

    fn sample(&self) -> u64 {
        self.sample
    }

    fn cycle(&self) -> Option<u64> {
        Some((self.sample as f32 / self.settings.frequency) as u64)
    }
}
