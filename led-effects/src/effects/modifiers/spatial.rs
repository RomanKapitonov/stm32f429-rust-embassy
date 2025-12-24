use crate::effects::core::{
    pixel::Pixel,
    traits::{Modifier, Parameter},
};

pub struct Blur<Strength>
where
    Strength: Parameter<f32>,
{
    pub strength: Strength, // 0.0 = no blur, 1.0 = maximum blur
}

pub struct Shift<Offset>
where
    Offset: Parameter<isize>,
{
    pub offset: Offset, // Positive = shift right, negative = shift left
}

pub struct Mirror {
    pub center: usize, // Position to mirror from
}

pub struct Reverse;

impl<Strength> Modifier for Blur<Strength>
where
    Strength: Parameter<f32>,
{
    #[inline(always)]
    fn modify(&mut self, buffer: &mut [Pixel], now: u64) {
        if buffer.len() < 2 {
            return;
        }

        let strength = self.strength.sample(now);
        let mut prev = buffer[0];

        for i in 1..buffer.len() {
            let current = buffer[i];
            buffer[i] = current.lerp(&prev, strength * 0.5);
            prev = current;
        }
    }
}

impl<Offset> Modifier for Shift<Offset>
where
    Offset: Parameter<isize>,
{
    #[inline(always)]
    fn modify(&mut self, buffer: &mut [Pixel], now: u64) {
        let offset = self.offset.sample(now);
        let len = buffer.len();

        if offset == 0 || len == 0 {
            return;
        }

        let offset_normalized = offset.rem_euclid(len as isize) as usize;

        if offset > 0 {
            buffer.rotate_right(offset_normalized);
        } else {
            buffer.rotate_left(offset_normalized);
        }
    }
}

impl Modifier for Mirror {
    #[inline(always)]
    fn modify(&mut self, buffer: &mut [Pixel], _now: u64) {
        let len = buffer.len();

        for i in 0..self.center.min(len) {
            let mirror_idx = len.saturating_sub(i + 1);
            if mirror_idx < len {
                buffer[mirror_idx] = buffer[i];
            }
        }
    }
}

impl Modifier for Reverse {
    #[inline(always)]
    fn modify(&mut self, buffer: &mut [Pixel], _now: u64) {
        buffer.reverse();
    }
}
