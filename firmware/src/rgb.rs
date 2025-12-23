// Re-export Rgb from led-effects
pub use led_effects::Rgb;

/// Buffer for holding RGB LED values
///
/// This is a thin wrapper around an array of RGB values that provides
/// FFI-compatible memory layout for the WS2812 driver.
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
