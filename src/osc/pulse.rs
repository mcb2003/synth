use super::{Osc, OscSettings};

pub struct PulseOsc {
    sample: u64,
    duty_cycle: f32,
    settings: OscSettings,
}

impl PulseOsc {
    pub fn new(settings: OscSettings, duty_cycle: f32) -> Self {
        Self {
            sample: 0,
            settings, duty_cycle,
        }
    }

    pub fn with_duty_cycle(duty_cycle: f32) -> Self {
        Self {
            duty_cycle,
            .. Self::default()
        }
    }
}

impl Default for PulseOsc {
    fn default() -> Self {
        Self {
            sample: 0,
            duty_cycle: 0.5,
            settings: OscSettings::default(),
        }
    }
}

impl Osc for PulseOsc {
    fn next_sample(&mut self) -> f32 {
        let sample = if ((self.sample as f32) * 4.0 % self.settings.frequency) <= (self.duty_cycle * self.settings.frequency) {
            self.settings.amplitude
        } else {
            -self.settings.amplitude
        };
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
