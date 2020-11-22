use super::{Osc, OscSettings};

#[derive(Default)]
pub struct SawOsc {
    sample: u64,
    settings: OscSettings,
}

impl SawOsc {
    pub fn new(settings: OscSettings) -> Self {
        Self {
            sample: 0,
            settings,
        }
    }
}

impl Osc for SawOsc {
    fn next_sample(&mut self) -> f32 {
        let time = self.sample as f32 / self.settings.sample_rate;
        let sample = 2.0 * self.settings.amplitude * time.mul_add(self.settings.frequency, -time.mul_add(self.settings.frequency, 0.5).floor());
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
