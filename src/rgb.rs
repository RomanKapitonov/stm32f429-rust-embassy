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
