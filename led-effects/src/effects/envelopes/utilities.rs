use crate::effects::core::traits::{Envelope, EnvelopeValue};

pub struct TimeLimited<E> {
    pub inner: E,
    pub start_time: u32,
    pub duration: u32,
}

pub struct LoopCount<E> {
    pub inner: E,
    pub start_time: u32,
    pub period: u32,
    pub max_loops: u32,
}

pub struct Pulse {
    pub start_time: u32,
    pub attack: u32,
    pub decay: u32,
}

// Add exponential decay LUT (256 entries, exp(-5*t) for t in 0-1)
const DECAY_LUT_SIZE: usize = 256;
static DECAY_LUT: [u8; DECAY_LUT_SIZE] = [
    255, 250, 245, 240, 235, 230, 226, 221, 217, 212, 208, 204, 200, 196, 192, 188, 184, 181, 177,
    174, 170, 167, 163, 160, 157, 154, 151, 148, 145, 142, 139, 136, 134, 131, 128, 126, 123, 121,
    118, 116, 114, 111, 109, 107, 105, 103, 101, 99, 97, 95, 93, 91, 89, 87, 86, 84, 82, 81, 79,
    77, 76, 74, 73, 71, 70, 68, 67, 65, 64, 63, 61, 60, 59, 58, 56, 55, 54, 53, 52, 51, 50, 49, 48,
    47, 46, 45, 44, 43, 42, 41, 40, 39, 38, 37, 37, 36, 35, 34, 33, 33, 32, 31, 30, 30, 29, 28, 28,
    27, 26, 26, 25, 24, 24, 23, 23, 22, 22, 21, 21, 20, 20, 19, 19, 18, 18, 17, 17, 16, 16, 15, 15,
    15, 14, 14, 13, 13, 13, 12, 12, 12, 11, 11, 11, 10, 10, 10, 9, 9, 9, 9, 8, 8, 8, 8, 7, 7, 7, 7,
    7, 6, 6, 6, 6, 6, 5, 5, 5, 5, 5, 5, 4, 4, 4, 4, 4, 4, 4, 3, 3, 3, 3, 3, 3, 3, 3, 3, 2, 2, 2, 2,
    2, 2, 2, 2, 2, 2, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0,
];

pub struct VelocityIntegral<V> {
    pub start_time: u32,
    pub velocity_envelope: V,
    pub initial_position: u32,
    pub dt_ms: u32,
    pub velocity_scale: u32,
}

impl<E, T> Envelope<T> for TimeLimited<E>
where
    E: Envelope<T>,
{
    #[inline(always)]
    fn sample(&self, now: u32) -> T {
        self.inner.sample(now)
    }

    #[inline(always)]
    fn is_alive(&self, now: u32) -> bool {
        now < self.start_time + self.duration && self.inner.is_alive(now)
    }
}

impl<E, T> Envelope<T> for LoopCount<E>
where
    E: Envelope<T>,
{
    #[inline(always)]
    fn sample(&self, now: u32) -> T {
        self.inner.sample(now)
    }

    #[inline(always)]
    fn is_alive(&self, now: u32) -> bool {
        let elapsed = now.saturating_sub(self.start_time);
        let loops_completed = elapsed / self.period;

        loops_completed < self.max_loops && self.inner.is_alive(now)
    }
}

impl<T> Envelope<T> for Pulse
where
    T: EnvelopeValue,
{
    #[inline(always)]
    fn sample(&self, now: u32) -> T {
        let elapsed = now.saturating_sub(self.start_time);

        if elapsed < self.attack {
            // Attack phase: ramp 0 → MAX
            T::from_progress(elapsed, self.attack)
        } else if elapsed < self.attack + self.decay {
            // Decay phase: exponential decay using LUT
            let decay_elapsed = elapsed - self.attack;
            let decay_progress =
                ((decay_elapsed as u64 * DECAY_LUT_SIZE as u64) / self.decay as u64) as usize;
            let decay_value = DECAY_LUT[decay_progress.min(DECAY_LUT_SIZE - 1)];

            // Map u8 decay value to T
            T::from_progress(decay_value as u32, 255)
        } else {
            T::ZERO
        }
    }

    #[inline(always)]
    fn is_alive(&self, now: u32) -> bool {
        now < self.start_time + self.attack + self.decay
    }
}

impl<V, T> Envelope<T> for VelocityIntegral<V>
where
    V: Envelope<T>,
    T: EnvelopeValue,
{
    #[inline(always)]
    fn sample(&self, now: u32) -> T {
        let elapsed = now.saturating_sub(self.start_time);
        let steps = (elapsed / self.dt_ms).max(1);

        // Accumulate in u32 (wide enough for any position)
        let mut position: u32 = self.initial_position;

        for i in 0..steps {
            let t = self.start_time + (i * self.dt_ms);
            let velocity = self.velocity_envelope.sample(t);

            // Convert T velocity to u32, scale it, then add to position
            let velocity_u32 = velocity.to_u32();
            let position_delta = (velocity_u32 * self.velocity_scale) / T::MAX.to_u32();

            position = position.saturating_add(position_delta);
        }

        // Map position to T range with wrapping
        let wrapped = position % T::MAX.to_u32();
        T::from_progress(wrapped, T::MAX.to_u32())
    }

    #[inline(always)]
    fn is_alive(&self, now: u32) -> bool {
        self.velocity_envelope.is_alive(now)
    }
}
