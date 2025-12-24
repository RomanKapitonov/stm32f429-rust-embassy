use crate::effects::core::Pixel;

pub trait Parameter<T> {
    fn sample(&self, now: u64) -> T;
}

pub trait Envelope {
    fn sample(&self, now: u64) -> f32;
    fn is_alive(&self, now: u64) -> bool;
}

pub trait Generator {
    fn generate(&mut self, buffer: &mut [Pixel], now: u64);
    fn is_alive(&self, now: u64) -> bool;
}

pub trait Modifier {
    fn modify(&mut self, buffer: &mut [Pixel], now: u64);
}

pub trait HueParameter {
    fn sample(&self, now: u64) -> f32;
}

pub trait Easing {
    fn ease(&self, t: f32) -> f32;
}
