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
