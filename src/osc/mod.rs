mod error;
pub use error::Error;
mod settings;
pub use settings::OscSettings;

mod sine;
pub use sine::SineOsc;
mod triangle;
pub use triangle::TriangleOsc;
mod saw;
pub use saw::SawOsc;
mod pulse;
pub use pulse::PulseOsc;

pub trait Osc {
    fn next_sample(&mut self) -> f32;
    fn sample(&self) -> u64;
    fn cycle(&self) -> Option<u64> {
        None
    }
}

impl Iterator for dyn Osc {
    type Item = f32;
    fn next(&mut self) -> Option<Self::Item> {
        Some(self.next_sample())
    }
}
