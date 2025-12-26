use crate::effects::core::traits::HueParameter;

pub struct StaticHue {
    hue_normalized: u8, // 0-255
}

impl StaticHue {
    #[inline(always)]
    pub const fn from_degrees(degrees: u16) -> Self {
        Self {
            hue_normalized: ((degrees as u32 * 255) / 360) as u8,
        }
    }

    #[inline(always)]
    pub const fn from_normalized(value: u8) -> Self {
        Self {
            hue_normalized: value,
        }
    }

    // Named color constants
    pub const RED: Self = Self { hue_normalized: 0 };
    pub const ORANGE: Self = Self::from_degrees(30);
    pub const YELLOW: Self = Self::from_degrees(60);
    pub const GREEN: Self = Self::from_degrees(120);
    pub const CYAN: Self = Self::from_degrees(180);
    pub const BLUE: Self = Self::from_degrees(240);
    pub const MAGENTA: Self = Self::from_degrees(300);
}

impl HueParameter for StaticHue {
    #[inline(always)]
    fn sample(&self, _now: u32) -> u8 {
        self.hue_normalized
    }
}
