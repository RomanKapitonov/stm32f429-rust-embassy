use crate::effects::core::Pixel;

pub trait Parameter<T> {
    fn sample(&self, now: u32) -> T;
}

pub trait Envelope<T> {
    fn sample(&self, now: u32) -> T;
    fn is_alive(&self, now: u32) -> bool;
}

pub trait Generator {
    fn generate(&mut self, buffer: &mut [Pixel], now: u32);
    fn is_alive(&self, now: u32) -> bool;
}

pub trait Modifier {
    fn modify(&mut self, buffer: &mut [Pixel], now: u32);
}

pub trait HueParameter {
    fn sample(&self, now: u32) -> u8;
}

pub trait Easing {
    fn ease(&self, t: f32) -> f32;
}

// Helper traits:

pub trait EnvelopeValue: Copy + Sized + Ord {
    const ZERO: Self;
    const ONE: Self;
    const MAX: Self;

    fn from_progress(elapsed: u32, duration: u32) -> Self;
    fn saturating_add(self, other: Self) -> Self;
    fn saturating_mul_div(self, other: Self) -> Self;
    fn saturating_sub(self, other: Self) -> Self;
    fn clamp_value(self, min: Self, max: Self) -> Self;
    fn to_u32(self) -> u32;
}

impl EnvelopeValue for u8 {
    const ZERO: Self = 0;
    const ONE: Self = 1;
    const MAX: Self = 255;

    #[inline(always)]
    fn from_progress(elapsed: u32, duration: u32) -> Self {
        ((elapsed * Self::MAX as u32) / duration) as Self
    }

    #[inline(always)]
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }

    #[inline(always)]
    fn saturating_mul_div(self, other: Self) -> Self {
        ((self as u16 * other as u16) / Self::MAX as u16) as Self
    }

    #[inline(always)]
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }

    #[inline(always)]
    fn clamp_value(self, min: Self, max: Self) -> Self {
        self.clamp(min, max)
    }

    #[inline(always)]
    fn to_u32(self) -> u32 {
        self as u32
    }
}

impl EnvelopeValue for u16 {
    const ZERO: Self = 0;
    const ONE: Self = 1;
    const MAX: Self = 65535;

    #[inline(always)]
    fn from_progress(elapsed: u32, duration: u32) -> Self {
        ((elapsed * Self::MAX as u32) / duration) as Self
    }

    #[inline(always)]
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }

    #[inline(always)]
    fn saturating_mul_div(self, other: Self) -> Self {
        ((self as u32 * other as u32) / Self::MAX as u32) as Self
    }

    #[inline(always)]
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }

    #[inline(always)]
    fn clamp_value(self, min: Self, max: Self) -> Self {
        self.clamp(min, max)
    }

    #[inline(always)]
    fn to_u32(self) -> u32 {
        self as u32
    }
}

impl EnvelopeValue for u32 {
    const ZERO: Self = 0;
    const ONE: Self = 1;
    const MAX: Self = 4_294_967_295;

    #[inline(always)]
    fn from_progress(elapsed: u32, duration: u32) -> Self {
        ((elapsed * Self::MAX as u32) / duration) as Self
    }

    #[inline(always)]
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }

    #[inline(always)]
    fn saturating_mul_div(self, other: Self) -> Self {
        ((self as u64 * other as u64) / Self::MAX as u64) as Self
    }

    #[inline(always)]
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }

    #[inline(always)]
    fn clamp_value(self, min: Self, max: Self) -> Self {
        self.clamp(min, max)
    }

    #[inline(always)]
    fn to_u32(self) -> u32 {
        self as u32
    }
}
