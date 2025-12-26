use led_effects_macros::{generate_gamma_lut, generate_hsv_lut};

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Pixel {
    #[inline(always)]
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    pub const BLACK: Self = Self::new(0, 0, 0);

    /// Scale by factor (0-255)
    #[inline(always)]
    pub fn scale(&self, factor: u8) -> Self {
        Self {
            r: ((self.r as u16 * factor as u16) / 255) as u8,
            g: ((self.g as u16 * factor as u16) / 255) as u8,
            b: ((self.b as u16 * factor as u16) / 255) as u8,
        }
    }

    #[inline(always)]
    pub fn add(&self, other: Self) -> Self {
        Self {
            r: self.r.saturating_add(other.r),
            g: self.g.saturating_add(other.g),
            b: self.b.saturating_add(other.b),
        }
    }

    /// Create pixel from HSV (hue: 0-255, saturation: 0-255, value: 0-255)
    #[inline(always)]
    pub fn from_hsv(hue: u8, saturation: u8, value: u8) -> Self {
        HSV_LUT.get(hue, saturation, value)
    }

    /// Linear interpolation (t: 0-255)
    #[inline(always)]
    pub fn lerp(&self, other: &Self, t: u8) -> Self {
        Self {
            r: self.r + (((other.r as i16 - self.r as i16) * t as i16) / 255) as u8,
            g: self.g + (((other.g as i16 - self.g as i16) * t as i16) / 255) as u8,
            b: self.b + (((other.b as i16 - self.b as i16) * t as i16) / 255) as u8,
        }
    }

    /// Gamma 2.2 correction (standard for LEDs)
    #[inline(always)]
    pub fn gamma_correct(&self) -> Self {
        Self {
            r: GAMMA_22_LUT[self.r as usize],
            g: GAMMA_22_LUT[self.g as usize],
            b: GAMMA_22_LUT[self.b as usize],
        }
    }

    /// Adjust saturation (factor: 0-255, where 255 = full saturation)
    #[inline(always)]
    pub fn adjust_saturation(&self, factor: u8) -> Self {
        let (h, s, v) = self.to_hsv_u8();
        let new_s = ((s as u16 * factor as u16) / 255) as u8;
        Self::from_hsv(h, new_s, v)
    }

    /// Shift hue by amount (0-255, wraps around)
    #[inline(always)]
    pub fn shift_hue(&self, shift: u8) -> Self {
        let (h, s, v) = self.to_hsv_u8();
        let new_h = h.wrapping_add(shift);
        Self::from_hsv(new_h, s, v)
    }

    /// Convert RGB to HSV (returns hue: 0-255, saturation: 0-255, value: 0-255)
    #[inline(always)]
    pub fn to_hsv_u8(&self) -> (u8, u8, u8) {
        let max = self.r.max(self.g).max(self.b);
        let min = self.r.min(self.g).min(self.b);
        let delta = max - min;

        if delta == 0 {
            return (0, 0, max);
        }

        let saturation = ((delta as u16 * 255) / max as u16) as u8;

        // Calculate hue: map 0-360° to 0-255
        // Each of 6 sectors = 255/6 ≈ 42.5, we use 256/6 for better precision
        let hue = if max == self.r {
            let diff = if self.g >= self.b {
                ((self.g - self.b) as u32 * 256) / (delta as u32 * 6)
            } else {
                256 - (((self.b - self.g) as u32 * 256) / (delta as u32 * 6))
            };
            diff as u8
        } else if max == self.g {
            let diff = ((self.b as i16 - self.r as i16) as i32 * 256) / (delta as i32 * 6);
            (85 + diff) as u8
        } else {
            let diff = ((self.r as i16 - self.g as i16) as i32 * 256) / (delta as i32 * 6);
            (171 + diff) as u8
        };

        (hue, saturation, max)
    }
}

// ============================================================================
// LOOKUP TABLES
// ============================================================================

struct HsvLut {
    data: [Pixel; 256 * 8 * 8],
}

impl HsvLut {
    #[inline(always)]
    fn get(&self, hue: u8, saturation: u8, value: u8) -> Pixel {
        let sat_idx = (saturation >> 5) as usize;
        let val_idx = (value >> 5) as usize;
        let hue_idx = hue as usize;

        let index = hue_idx * 64 + sat_idx * 8 + val_idx;
        self.data[index]
    }
}

static HSV_LUT: HsvLut = HsvLut {
    data: generate_hsv_lut!(),
};

static GAMMA_22_LUT: [u8; 256] = generate_gamma_lut!();

const _: () = assert!(core::mem::size_of::<Pixel>() == 3);
