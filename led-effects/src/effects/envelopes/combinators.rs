use crate::effects::core::traits::Envelope;

pub struct Product<E1: Envelope, E2: Envelope> {
    pub env1: E1,
    pub env2: E2,
}

pub struct Sum<E1: Envelope, E2: Envelope> {
    pub env1: E1,
    pub env2: E2,
}

pub struct Min<E1: Envelope, E2: Envelope> {
    pub env1: E1,
    pub env2: E2,
}

pub struct Max<E1: Envelope, E2: Envelope> {
    pub env1: E1,
    pub env2: E2,
}

pub struct Invert<E: Envelope> {
    pub inner: E,
}

pub struct Clamp<E: Envelope> {
    pub inner: E,
    pub min: f32,
    pub max: f32,
}

pub struct Scale<E: Envelope> {
    pub inner: E,
    pub from_min: f32,
    pub from_max: f32,
    pub to_min: f32,
    pub to_max: f32,
}

impl<E1: Envelope, E2: Envelope> Envelope for Product<E1, E2> {
    #[inline(always)]
    fn sample(&self, now: u64) -> f32 {
        self.env1.sample(now) * self.env2.sample(now)
    }

    #[inline(always)]
    fn is_alive(&self, now: u64) -> bool {
        self.env1.is_alive(now) && self.env2.is_alive(now)
    }
}

impl<E1: Envelope, E2: Envelope> Envelope for Sum<E1, E2> {
    #[inline(always)]
    fn sample(&self, now: u64) -> f32 {
        (self.env1.sample(now) + self.env2.sample(now)).min(1.0)
    }

    #[inline(always)]
    fn is_alive(&self, now: u64) -> bool {
        self.env1.is_alive(now) || self.env2.is_alive(now)
    }
}

impl<E1: Envelope, E2: Envelope> Envelope for Min<E1, E2> {
    #[inline(always)]
    fn sample(&self, now: u64) -> f32 {
        self.env1.sample(now).min(self.env2.sample(now))
    }

    #[inline(always)]
    fn is_alive(&self, now: u64) -> bool {
        self.env1.is_alive(now) || self.env2.is_alive(now)
    }
}

impl<E1: Envelope, E2: Envelope> Envelope for Max<E1, E2> {
    #[inline(always)]
    fn sample(&self, now: u64) -> f32 {
        self.env1.sample(now).max(self.env2.sample(now))
    }

    #[inline(always)]
    fn is_alive(&self, now: u64) -> bool {
        self.env1.is_alive(now) || self.env2.is_alive(now)
    }
}

impl<E: Envelope> Envelope for Invert<E> {
    #[inline(always)]
    fn sample(&self, now: u64) -> f32 {
        1.0 - self.inner.sample(now)
    }

    #[inline(always)]
    fn is_alive(&self, now: u64) -> bool {
        self.inner.is_alive(now)
    }
}

impl<E: Envelope> Envelope for Clamp<E> {
    #[inline(always)]
    fn sample(&self, now: u64) -> f32 {
        self.inner.sample(now).clamp(self.min, self.max)
    }

    #[inline(always)]
    fn is_alive(&self, now: u64) -> bool {
        self.inner.is_alive(now)
    }
}

impl<E: Envelope> Envelope for Scale<E> {
    #[inline(always)]
    fn sample(&self, now: u64) -> f32 {
        let value = self.inner.sample(now);

        // Normalize to 0-1 based on input range
        let normalized = (value - self.from_min) / (self.from_max - self.from_min);

        // Map to output range
        self.to_min + normalized * (self.to_max - self.to_min)
    }

    #[inline(always)]
    fn is_alive(&self, now: u64) -> bool {
        self.inner.is_alive(now)
    }
}
