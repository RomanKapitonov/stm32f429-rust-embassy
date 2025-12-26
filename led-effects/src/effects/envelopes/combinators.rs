use crate::effects::core::traits::{Envelope, EnvelopeValue};

pub struct Product<E1, E2> {
    pub env1: E1,
    pub env2: E2,
}

pub struct Sum<E1, E2> {
    pub env1: E1,
    pub env2: E2,
}

pub struct Min<E1, E2> {
    pub env1: E1,
    pub env2: E2,
}

pub struct Max<E1, E2> {
    pub env1: E1,
    pub env2: E2,
}

pub struct Invert<E> {
    pub inner: E,
}

pub struct Clamp<E, T> {
    pub inner: E,
    pub min: T,
    pub max: T,
}

impl<E1, E2, T> Envelope<T> for Product<E1, E2>
where
    E1: Envelope<T>,
    E2: Envelope<T>,
    T: EnvelopeValue,
{
    #[inline(always)]
    fn sample(&self, now: u32) -> T {
        let v1 = self.env1.sample(now);
        let v2 = self.env2.sample(now);

        v1.saturating_mul_div(v2)
    }

    #[inline(always)]
    fn is_alive(&self, now: u32) -> bool {
        self.env1.is_alive(now) && self.env2.is_alive(now)
    }
}

impl<E1, E2, T> Envelope<T> for Sum<E1, E2>
where
    E1: Envelope<T>,
    E2: Envelope<T>,
    T: EnvelopeValue,
{
    #[inline(always)]
    fn sample(&self, now: u32) -> T {
        let v1 = self.env1.sample(now);
        let v2 = self.env2.sample(now);

        v1.saturating_add(v2)
    }

    #[inline(always)]
    fn is_alive(&self, now: u32) -> bool {
        self.env1.is_alive(now) || self.env2.is_alive(now)
    }
}

impl<E1, E2, T> Envelope<T> for Min<E1, E2>
where
    E1: Envelope<T>,
    E2: Envelope<T>,
    T: EnvelopeValue,
{
    #[inline(always)]
    fn sample(&self, now: u32) -> T {
        let v1 = self.env1.sample(now);
        let v2 = self.env2.sample(now);

        v1.min(v2)
    }

    #[inline(always)]
    fn is_alive(&self, now: u32) -> bool {
        self.env1.is_alive(now) || self.env2.is_alive(now)
    }
}

impl<E1, E2, T> Envelope<T> for Max<E1, E2>
where
    E1: Envelope<T>,
    E2: Envelope<T>,
    T: EnvelopeValue,
{
    #[inline(always)]
    fn sample(&self, now: u32) -> T {
        let v1 = self.env1.sample(now);
        let v2 = self.env2.sample(now);

        v1.max(v2)
    }

    #[inline(always)]
    fn is_alive(&self, now: u32) -> bool {
        self.env1.is_alive(now) || self.env2.is_alive(now)
    }
}

impl<E, T> Envelope<T> for Invert<E>
where
    E: Envelope<T>,
    T: EnvelopeValue,
{
    #[inline(always)]
    fn sample(&self, now: u32) -> T {
        T::MAX.saturating_sub(self.inner.sample(now))
    }

    #[inline(always)]
    fn is_alive(&self, now: u32) -> bool {
        self.inner.is_alive(now)
    }
}

impl<E, T> Envelope<T> for Clamp<E, T>
where
    E: Envelope<T>,
    T: EnvelopeValue,
{
    #[inline(always)]
    fn sample(&self, now: u32) -> T {
        let value = self.inner.sample(now);

        value.clamp_value(self.min, self.max)
    }

    #[inline(always)]
    fn is_alive(&self, now: u32) -> bool {
        self.inner.is_alive(now)
    }
}
