use core::f32::consts::PI;

use crate::effects::core::traits::{Envelope, EnvelopeValue};

pub struct Constant;

pub struct Fade {
    pub start_time: u32,
    pub duration: u32,
    pub inverted: bool,
}

pub struct Triangle {
    pub start_time: u32,
    pub period: u32,
}

pub struct Sine {
    pub start_time: u32,
    pub period: u32,
}

const SINE_LUT_SIZE: usize = 256;
static SINE_LUT: [u8; SINE_LUT_SIZE] = [
    128, 131, 134, 137, 140, 143, 146, 149, 152, 155, 158, 162, 165, 167, 170, 173, 176, 179, 182,
    185, 188, 190, 193, 196, 198, 201, 203, 206, 208, 211, 213, 215, 218, 220, 222, 224, 226, 228,
    230, 232, 234, 235, 237, 238, 240, 241, 243, 244, 245, 246, 248, 249, 250, 250, 251, 252, 253,
    253, 254, 254, 254, 255, 255, 255, 255, 255, 255, 255, 254, 254, 254, 253, 253, 252, 251, 250,
    250, 249, 248, 246, 245, 244, 243, 241, 240, 238, 237, 235, 234, 232, 230, 228, 226, 224, 222,
    220, 218, 215, 213, 211, 208, 206, 203, 201, 198, 196, 193, 190, 188, 185, 182, 179, 176, 173,
    170, 167, 165, 162, 158, 155, 152, 149, 146, 143, 140, 137, 134, 131, 128, 124, 121, 118, 115,
    112, 109, 106, 103, 100, 97, 93, 90, 88, 85, 82, 79, 76, 73, 70, 67, 65, 62, 59, 57, 54, 52,
    49, 47, 44, 42, 40, 37, 35, 33, 31, 29, 27, 25, 23, 21, 20, 18, 17, 15, 14, 12, 11, 10, 9, 7,
    6, 5, 5, 4, 3, 2, 2, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 2, 2, 3, 4, 5, 5, 6, 7, 9, 10, 11,
    12, 14, 15, 17, 18, 20, 21, 23, 25, 27, 29, 31, 33, 35, 37, 40, 42, 44, 47, 49, 52, 54, 57, 59,
    62, 65, 67, 70, 73, 76, 79, 82, 85, 88, 90, 93, 97, 100, 103, 106, 109, 112, 115, 118, 121,
    124,
];

pub struct Square {
    pub start_time: u32,
    pub period: u32,    // full period in ms
    pub duty_cycle: u8, // 0-255 -> 0-100%
}

pub struct Sawtooth {
    pub start_time: u32,
    pub period: u32,
}

pub struct ADSR {
    pub start_time: u32,
    pub attack: u32,           // Attack time in ms
    pub decay: u32,            // Decay time in ms
    pub sustain_level: u8,     // Sustain level (0 - 255)
    pub sustain_duration: u32, // How long to hold sustain
    pub release: u32,          // Release time in ms
}

impl<T> Envelope<T> for Constant
where
    T: EnvelopeValue,
{
    #[inline(always)]
    fn sample(&self, _now: u32) -> T {
        T::ONE
    }

    #[inline(always)]
    fn is_alive(&self, _now: u32) -> bool {
        true
    }
}

impl<T> Envelope<T> for Fade
where
    T: EnvelopeValue,
{
    #[inline(always)]
    fn sample(&self, now: u32) -> T {
        let elapsed = now.saturating_sub(self.start_time);

        if elapsed >= self.duration {
            return if self.inverted { T::ZERO } else { T::ONE };
        }

        let progress = T::from_progress(elapsed, self.duration);

        if self.inverted {
            T::MAX.saturating_sub(progress)
        } else {
            progress
        }
    }

    #[inline(always)]
    fn is_alive(&self, now: u32) -> bool {
        now < self.start_time + self.duration
    }
}

impl<T> Envelope<T> for Triangle
where
    T: EnvelopeValue,
{
    #[inline(always)]
    fn sample(&self, now: u32) -> T {
        let elapsed = now.saturating_sub(self.start_time);
        let elapsed_in_cycle = elapsed % self.period;

        // Double the phase to avoid division (0 to 2*period range)
        let phase_doubled = elapsed_in_cycle * 2;

        if phase_doubled < self.period {
            // First half: ascending 0 → MAX
            T::from_progress(phase_doubled, self.period)
        } else {
            // Second half: descending MAX → 0
            let descending = phase_doubled - self.period;
            T::MAX.saturating_sub(T::from_progress(descending, self.period))
        }
    }

    #[inline(always)]
    fn is_alive(&self, _now: u32) -> bool {
        true
    }
}

impl<T> Envelope<T> for Sine
where
    T: EnvelopeValue,
{
    #[inline(always)]
    fn sample(&self, now: u32) -> T {
        let elapsed = now.saturating_sub(self.start_time);
        let elapsed_in_cycle = (elapsed % self.period) as u64;

        let phase_index = ((elapsed_in_cycle * SINE_LUT_SIZE as u64) / self.period as u64) as usize;
        let sine_u8 = SINE_LUT[phase_index & (SINE_LUT_SIZE - 1)];

        T::from_progress(sine_u8 as u32, 255)
    }

    #[inline(always)]
    fn is_alive(&self, _now: u32) -> bool {
        true
    }
}

impl<T> Envelope<T> for Square
where
    T: EnvelopeValue,
{
    #[inline(always)]
    fn sample(&self, now: u32) -> T {
        let elapsed = now.saturating_sub(self.start_time);
        let elapsed_in_cycle = (elapsed % self.period) as u64;

        let threshold = (self.period as u64 * self.duty_cycle as u64) / 255;

        if elapsed_in_cycle < threshold {
            T::MAX
        } else {
            T::ZERO
        }
    }

    #[inline(always)]
    fn is_alive(&self, _now: u32) -> bool {
        true
    }
}

impl<T> Envelope<T> for Sawtooth
where
    T: EnvelopeValue,
{
    #[inline(always)]
    fn sample(&self, now: u32) -> T {
        let elapsed = now.saturating_sub(self.start_time);
        let elapsed_in_cycle = elapsed % self.period;

        T::from_progress(elapsed_in_cycle, self.period)
    }

    #[inline(always)]
    fn is_alive(&self, _now: u32) -> bool {
        true
    }
}

impl<T> Envelope<T> for ADSR
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
            // Decay phase: fall MAX → sustain_level
            let decay_elapsed = elapsed - self.attack;
            let decay_progress = T::from_progress(decay_elapsed, self.decay);

            // Interpolate: MAX - (MAX - sustain_level) * progress
            let sustain_t = T::from_progress(self.sustain_level as u32, 255);
            let range = T::MAX.saturating_sub(sustain_t);
            let drop = range.saturating_mul_div(decay_progress);
            T::MAX.saturating_sub(drop)
        } else if elapsed < self.attack + self.decay + self.sustain_duration {
            // Sustain phase: hold at sustain_level
            T::from_progress(self.sustain_level as u32, 255)
        } else {
            // Release phase: fall sustain_level → 0
            let release_start = self.attack + self.decay + self.sustain_duration;
            let release_elapsed = elapsed.saturating_sub(release_start);

            if release_elapsed >= self.release {
                return T::ZERO;
            }

            let release_progress = T::from_progress(release_elapsed, self.release);
            let sustain_t = T::from_progress(self.sustain_level as u32, 255);

            // sustain_level * (1 - progress)
            let remaining = T::MAX.saturating_sub(release_progress);
            sustain_t.saturating_mul_div(remaining)
        }
    }

    #[inline(always)]
    fn is_alive(&self, now: u32) -> bool {
        let total = self.attack + self.decay + self.sustain_duration + self.release;
        now < self.start_time + total
    }
}
