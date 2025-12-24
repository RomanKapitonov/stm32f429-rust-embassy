use crate::effects::core::traits::{Envelope, Parameter};

pub struct DynamicParam<E: Envelope> {
    pub envelope: E,
    pub min: f32,
    pub max: f32,
}

impl<E: Envelope> Parameter<u8> for DynamicParam<E> {
    #[inline(always)]
    fn sample(&self, now: u64) -> u8 {
        let t = self.envelope.sample(now);
        (self.min + (self.max - self.min) * t) as u8
    }
}

impl<E: Envelope> Parameter<f32> for DynamicParam<E> {
    #[inline(always)]
    fn sample(&self, now: u64) -> f32 {
        let t = self.envelope.sample(now);
        self.min + (self.max - self.min) * t
    }
}
