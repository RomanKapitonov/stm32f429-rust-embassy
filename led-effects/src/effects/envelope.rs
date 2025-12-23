// Trait for time-based envelope functions
pub trait Envelope {
    fn sample(&self, now: u64) -> f32;
    fn is_alive(&self, now: u64) -> bool;
}
