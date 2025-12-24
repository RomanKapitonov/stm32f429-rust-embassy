use core::f32::consts::PI;

use crate::effects::core::traits::Envelope;

pub struct Constant;

pub struct Fade {
    pub start_time: u64,
    pub duration: u64,
    pub inverted: bool,
}

pub struct Triangle {
    pub start_time: u64,
    pub period: u64,
}

pub struct Sine {
    pub start_time: u64,
    pub period: u64,
}

pub struct Square {
    pub start_time: u64,
    pub period: u64,     // full period in ms
    pub duty_cycle: f32, // PWM duty cycle (0.0 to 1.0)
}

pub struct Sawtooth {
    pub start_time: u64,
    pub period: u64,
}

pub struct ADSR {
    pub start_time: u64,
    pub attack: u64,           // Attack time in ms
    pub decay: u64,            // Decay time in ms
    pub sustain_level: f32,    // Sustain level (0.0 - 1.0)
    pub sustain_duration: u64, // How long to hold sustain
    pub release: u64,          // Release time in ms
}

impl Envelope for Constant {
    #[inline(always)]
    fn sample(&self, _now: u64) -> f32 {
        1.0
    }

    #[inline(always)]
    fn is_alive(&self, _now: u64) -> bool {
        true
    }
}

impl Envelope for Fade {
    #[inline(always)]
    fn sample(&self, now: u64) -> f32 {
        let elapsed = now.saturating_sub(self.start_time);
        let progress = (elapsed as f32 / self.duration as f32).min(1.0);

        if self.inverted {
            1.0 - progress
        } else {
            progress
        }
    }

    #[inline(always)]
    fn is_alive(&self, now: u64) -> bool {
        now < self.start_time + self.duration
    }
}

impl Envelope for Triangle {
    #[inline(always)]
    fn sample(&self, now: u64) -> f32 {
        let elapsed = now.saturating_sub(self.start_time);
        let phase = (elapsed % self.period) as f32 / self.period as f32;
        if phase < 0.5 {
            phase * 2.0
        } else {
            2.0 - phase * 2.0
        }
    }

    #[inline(always)]
    fn is_alive(&self, _now: u64) -> bool {
        true
    }
}

impl Envelope for Sine {
    #[inline(always)]
    fn sample(&self, now: u64) -> f32 {
        let elapsed = now.saturating_sub(self.start_time);
        let phase = (elapsed as f32 / self.period as f32) * 2.0 * PI;
        (libm::sinf(phase) + 1.0) * 0.5 // [-1, 1] -> [0, 1]
    }

    #[inline(always)]
    fn is_alive(&self, _now: u64) -> bool {
        true
    }
}

impl Envelope for Square {
    #[inline(always)]
    fn sample(&self, now: u64) -> f32 {
        let elapsed = now.saturating_sub(self.start_time);
        let phase = (elapsed % self.period) as f32 / self.period as f32;
        if phase < self.duty_cycle { 1.0 } else { 0.0 }
    }

    #[inline(always)]
    fn is_alive(&self, _now: u64) -> bool {
        true
    }
}

impl Envelope for Sawtooth {
    #[inline(always)]
    fn sample(&self, now: u64) -> f32 {
        let elapsed = now.saturating_sub(self.start_time);
        let phase = (elapsed % self.period) as f32 / self.period as f32;
        phase // Linear ramp from 0 to 1
    }

    #[inline(always)]
    fn is_alive(&self, _now: u64) -> bool {
        true
    }
}

impl Envelope for ADSR {
    #[inline(always)]
    fn sample(&self, now: u64) -> f32 {
        let elapsed = now.saturating_sub(self.start_time);

        if elapsed < self.attack {
            // Attack phase: ramp up from 0 to 1
            elapsed as f32 / self.attack as f32
        } else if elapsed < self.attack + self.decay {
            // Decay phase: fall from 1 to sustain_level
            let t = (elapsed - self.attack) as f32 / self.decay as f32;
            1.0 - (1.0 - self.sustain_level) * t
        } else if elapsed < self.attack + self.decay + self.sustain_duration {
            // Sustain phase: hold at sustain_level
            self.sustain_level
        } else {
            // Release phase: fall from sustain_level to 0
            let release_start = self.attack + self.decay + self.sustain_duration;
            let t = (elapsed - release_start) as f32 / self.release as f32;
            self.sustain_level * (1.0 - t.min(1.0))
        }
    }

    #[inline(always)]
    fn is_alive(&self, now: u64) -> bool {
        let total = self.attack + self.decay + self.sustain_duration + self.release;
        now < self.start_time + total
    }
}
