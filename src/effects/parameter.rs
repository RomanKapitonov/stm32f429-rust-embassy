use super::envelope::Envelope;

pub trait Parameter<T> {
    fn sample(&self, now: u64) -> T;
}

pub trait HueParameter {
    fn sample(&self, now: u64) -> f32;
}

#[derive(Copy, Clone)]
pub struct Static<T>(pub T);

impl<T: Copy> Parameter<T> for Static<T> {
    fn sample(&self, _now: u64) -> T {
        self.0
    }
}

impl HueParameter for Static<f32> {
    fn sample(&self, _now: u64) -> f32 {
        self.0 % 360.0
    }
}

#[derive(Copy, Clone)]
pub struct Dynamic<E: Envelope> {
    pub envelope: E,
    pub min: f32,
    pub max: f32,
}

impl<E: Envelope> Parameter<f32> for Dynamic<E> {
    fn sample(&self, now: u64) -> f32 {
        let t = self.envelope.sample(now);
        self.min + (self.max - self.min) * t
    }
}

impl<E: Envelope> Parameter<u8> for Dynamic<E> {
    fn sample(&self, now: u64) -> u8 {
        let t = self.envelope.sample(now);
        (self.min + (self.max - self.min) * t) as u8
    }
}

#[derive(Copy, Clone)]
pub struct RotatingHue {
    pub start_time: u64,
    pub degrees_per_ms: f32,
}

impl HueParameter for RotatingHue {
    fn sample(&self, now: u64) -> f32 {
        let elapsed = now.saturating_sub(self.start_time) as f32;
        (elapsed * self.degrees_per_ms) % 360.0
    }
}
