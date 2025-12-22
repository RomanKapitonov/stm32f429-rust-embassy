#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Rgb {
    #[inline]
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    pub const BLACK: Self = Self::new(0, 0, 0);

    pub fn scale(&self, f: f32) -> Self {
        Self {
            r: (self.r as f32 * f).min(255.0) as u8,
            g: (self.g as f32 * f).min(255.0) as u8,
            b: (self.b as f32 * f).min(255.0) as u8,
        }
    }

    pub fn add(&self, other: Self) -> Self {
        Self {
            r: self.r.saturating_add(other.r),
            g: self.g.saturating_add(other.g),
            b: self.b.saturating_add(other.b),
        }
    }

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
}

const _: () = assert!(core::mem::size_of::<Rgb>() == 3);

#[repr(transparent)]
pub struct RgbBuffer<const N: usize> {
    leds: [Rgb; N],
}

impl<const N: usize> RgbBuffer<N> {
    #[inline]
    pub const fn new() -> Self {
        Self {
            leds: [Rgb::BLACK; N],
        }
    }

    #[inline]
    pub fn as_bytes_ptr(&self) -> *const u8 {
        self.leds.as_ptr() as *const u8
    }

    #[inline]
    pub fn fill(&mut self, color: Rgb) {
        self.leds.fill(color);
    }

    pub fn as_mut_slice(&mut self) -> &mut [Rgb] {
        &mut self.leds
    }
}

impl<const N: usize> core::ops::Index<usize> for RgbBuffer<N> {
    type Output = Rgb;
    fn index(&self, index: usize) -> &Self::Output {
        &self.leds[index]
    }
}

impl<const N: usize> core::ops::IndexMut<usize> for RgbBuffer<N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.leds[index]
    }
}
