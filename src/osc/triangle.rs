use std::f32::consts::PI;

use super::{Osc, OscSettings};

#[derive(Default)]
pub struct TriangleOsc {
    sample: u64,
    settings: OscSettings,
}

impl TriangleOsc {
    pub fn new(settings: OscSettings) -> Self {
        Self {
            sample: 0,
            settings,
        }
    }
}

impl Osc for TriangleOsc {
    fn next_sample(&mut self) -> f32 {
        let sample = ((2.0 * self.settings.amplitude) / PI)
            * (2.0
                * PI
                * self.settings.frequency
                * (self.sample as f32 / self.settings.sample_rate))
                .sin()
                .asin();
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
