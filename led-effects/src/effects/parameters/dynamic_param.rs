use crate::effects::core::traits::{Envelope, EnvelopeValue, Parameter};
use core::marker::PhantomData;

pub struct DynamicParam<E, T> {
    pub envelope: E,
    pub min: f32,
    pub max: f32,
    _phantom: PhantomData<T>,
}

impl<E, T> DynamicParam<E, T> {
    pub fn new(envelope: E, min: f32, max: f32) -> Self {
        Self {
            envelope,
            min,
            max,
            _phantom: PhantomData,
        }
    }
}

impl<E, T> Parameter<u8> for DynamicParam<E, T>
where
    E: Envelope<T>,
    T: EnvelopeValue,
{
    #[inline(always)]
    fn sample(&self, now: u32) -> u8 {
        let t = self.envelope.sample(now);
        let t_normalized = t.to_u32() as f32 / T::MAX.to_u32() as f32;
        let range = self.max - self.min;
        (self.min + t_normalized * range) as u8
    }
}

impl<E, T> Parameter<u16> for DynamicParam<E, T>
where
    E: Envelope<T>,
    T: EnvelopeValue,
{
    #[inline(always)]
    fn sample(&self, now: u32) -> u16 {
        let t = self.envelope.sample(now);
        let t_normalized = t.to_u32() as f32 / T::MAX.to_u32() as f32;
        let range = self.max - self.min;
        (self.min + t_normalized * range) as u16
    }
}

impl<E, T> Parameter<f32> for DynamicParam<E, T>
where
    E: Envelope<T>,
    T: EnvelopeValue,
{
    #[inline(always)]
    fn sample(&self, now: u32) -> f32 {
        let t = self.envelope.sample(now);
        let t_normalized = t.to_u32() as f32 / T::MAX.to_u32() as f32;
        self.min + (self.max - self.min) * t_normalized
    }
}
