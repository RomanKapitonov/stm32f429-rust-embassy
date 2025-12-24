use crate::effects::core::traits::Envelope;
// use core::cell::Cell;

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

pub struct Pulse {
    pub start_time: u64,
    pub attack: u64,
    pub decay: u64,
    pub decay_curve: f32,
}

pub struct VelocityIntegral<V: Envelope> {
    pub start_time: u64,
    pub velocity_envelope: V,
    pub initial_position: f32,
}

// pub struct VelocityIntegral<V> {
//     pub velocity_envelope: V,
//     pub start_time: u64,
//     pub initial_position: f32,
//     // These 'Cell' wrappers allow modification via &self
//     last_time: Cell<u64>,
//     accumulator: Cell<f32>,
// }

impl<E: Envelope> Envelope for TimeLimited<E> {
    #[inline(always)]
    fn sample(&self, now: u64) -> f32 {
        self.inner.sample(now)
    }

    #[inline(always)]
    fn is_alive(&self, now: u64) -> bool {
        now < self.start_time + self.duration && self.inner.is_alive(now)
    }
}

impl<E: Envelope> Envelope for LoopCount<E> {
    #[inline(always)]
    fn sample(&self, now: u64) -> f32 {
        self.inner.sample(now)
    }

    #[inline(always)]
    fn is_alive(&self, now: u64) -> bool {
        let elapsed = now.saturating_sub(self.start_time);
        let loops_completed = (elapsed / self.period) as u32;

        loops_completed < self.max_loops && self.inner.is_alive(now)
    }
}

impl Envelope for Pulse {
    #[inline(always)]
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

    #[inline(always)]
    fn is_alive(&self, now: u64) -> bool {
        now < self.start_time + self.attack + self.decay
    }
}

// impl<V: Envelope> VelocityIntegral<V> {
//     #[inline(always)]
//     pub fn new(envelope: V, start_time: u64, initial_position: f32) -> Self {
//         Self {
//             velocity_envelope: envelope,
//             start_time,
//             initial_position,
//             last_time: Cell::new(start_time),
//             accumulator: Cell::new(0.0),
//         }
//     }
// }

impl<V: Envelope> Envelope for VelocityIntegral<V> {
    #[inline(always)]
    // fn sample(&self, now: u64) -> f32 {
    //     let last = self.last_time.get();

    //     // Only calculate the delta if time has actually moved forward
    //     if now > last {
    //         let dt = (now - last) as f32 / 1000.0;
    //         let velocity = self.velocity_envelope.sample(now);

    //         // Increment the accumulator instead of re-running the whole loop
    //         let new_pos = (self.accumulator.get() + velocity * dt) % 1.0;

    //         self.accumulator.set(new_pos);
    //         self.last_time.set(now);
    //     }

    //     (self.initial_position + self.accumulator.get()) % 1.0
    // }
    fn sample(&self, now: u64) -> f32 {
        let elapsed = now.saturating_sub(self.start_time) as f32;

        // Approximate integral: sample velocity at intervals
        let steps = (elapsed / 16.0) as usize; // ~60 FPS
        let mut position = 0.0;

        for i in 0..steps {
            let t = self.start_time + (i as u64 * 16);
            let velocity = self.velocity_envelope.sample(t);
            position += velocity * 0.016; // dt = 16ms
        }

        (self.initial_position + position) % 1.0
    }
    #[inline(always)]
    fn is_alive(&self, now: u64) -> bool {
        self.velocity_envelope.is_alive(now)
    }
}
