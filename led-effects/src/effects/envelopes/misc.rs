use crate::effects::Envelope;

pub struct Pulse {
    pub start_time: u64,
    pub attack: u64,
    pub decay: u64,
    pub decay_curve: f32, // Exp decay factor
}

pub struct TimeLimited<E: Envelope> {
    pub inner: E,
    pub start_time: u64,
    pub duration: u64,
}

pub struct LoopCount<E: Envelope> {
    pub inner: E,
    pub start_time: u64,
    pub period: u64,
    pub max_loops: u32,
}

impl Envelope for Pulse {
    fn sample(&self, now: u64) -> f32 {
        let elapsed = now.saturating_sub(self.start_time);

        if elapsed < self.attack {
            elapsed as f32 / self.attack as f32
        } else if elapsed < self.attack + self.decay {
            let t = (elapsed - self.attack) as f32 / self.decay as f32;
            libm::expf(-self.decay_curve * t)
        } else {
            0.0
        }
    }

    fn is_alive(&self, now: u64) -> bool {
        now < self.start_time + self.attack + self.decay
    }
}

impl<E: Envelope> Envelope for TimeLimited<E> {
    fn sample(&self, now: u64) -> f32 {
        self.inner.sample(now)
    }

    fn is_alive(&self, now: u64) -> bool {
        now < self.start_time + self.duration && self.inner.is_alive(now)
    }
}

impl<E: Envelope> Envelope for LoopCount<E> {
    fn sample(&self, now: u64) -> f32 {
        self.inner.sample(now)
    }

    fn is_alive(&self, now: u64) -> bool {
        let elapsed = now.saturating_sub(self.start_time);
        let loops_completed = (elapsed / self.period) as u32;

        loops_completed < self.max_loops && self.inner.is_alive(now)
    }
}
