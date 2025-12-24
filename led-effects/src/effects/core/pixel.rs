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

    #[inline(always)]
    pub fn scale(&self, f: f32) -> Self {
        Self {
            r: (self.r as f32 * f).min(255.0) as u8,
            g: (self.g as f32 * f).min(255.0) as u8,
            b: (self.b as f32 * f).min(255.0) as u8,
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

    #[inline(always)]
    pub fn from_hsv(h: f32, s: f32, v: f32) -> Self {
        let c = v * s;
        let h_prime = h / 60.0;
        let x = c * (1.0 - ((h_prime % 2.0) - 1.0).abs());
        let m = v - c;

        let (r, g, b) = if h_prime < 1.0 {
            (c, x, 0.0)
        } else if h_prime < 2.0 {
            (x, c, 0.0)
        } else if h_prime < 3.0 {
            (0.0, c, x)
        } else if h_prime < 4.0 {
            (0.0, x, c)
        } else if h_prime < 5.0 {
            (x, 0.0, c)
        } else {
            (c, 0.0, x)
        };

        Self {
            r: ((r + m) * 255.0) as u8,
            g: ((g + m) * 255.0) as u8,
            b: ((b + m) * 255.0) as u8,
        }
    }

    #[inline(always)]
    pub fn adjust_saturation(&self, factor: f32) -> Self {
        let (h, s, v) = self.to_hsv();
        Self::from_hsv(h, (s * factor).min(1.0), v)
    }

    #[inline(always)]
    pub fn shift_hue(&self, degrees: f32) -> Self {
        let (h, s, v) = self.to_hsv();
        Self::from_hsv((h + degrees) % 360.0, s, v)
    }

    #[inline(always)]
    pub fn to_hsv(&self) -> (f32, f32, f32) {
        let r = self.r as f32 / 255.0;
        let g = self.g as f32 / 255.0;
        let b = self.b as f32 / 255.0;

        let max = r.max(g).max(b);
        let min = r.min(g).min(b);
        let delta = max - min;

        let h = if delta == 0.0 {
            0.0
        } else if max == r {
            60.0 * (((g - b) / delta) % 6.0)
        } else if max == g {
            60.0 * (((b - r) / delta) + 2.0)
        } else {
            60.0 * (((r - g) / delta) + 4.0)
        };

        let s = if max == 0.0 { 0.0 } else { delta / max };
        let v = max;

        ((h + 360.0) % 360.0, s, v)
    }

    #[inline(always)]
    pub fn gamma_correct(&self, gamma: f32) -> Self {
        let correct = |channel: u8| {
            let normalized = channel as f32 / 255.0;
            let corrected = libm::powf(normalized, gamma);
            (corrected * 255.0) as u8
        };

        Self {
            r: correct(self.r),
            g: correct(self.g),
            b: correct(self.b),
        }
    }

    #[inline(always)]
    pub fn lerp(&self, other: &Self, t: f32) -> Self {
        Self {
            r: (self.r as f32 + (other.r as f32 - self.r as f32) * t) as u8,
            g: (self.g as f32 + (other.g as f32 - self.g as f32) * t) as u8,
            b: (self.b as f32 + (other.b as f32 - self.b as f32) * t) as u8,
        }
    }
}

const _: () = assert!(core::mem::size_of::<Pixel>() == 3);
