use crate::ffi::LedChannelInfo;
use crate::rgb::{Rgb, RgbBuffer};

pub struct LedChannel<const N: usize> {
    buffer: RgbBuffer<N>,
    channel_number: u8,
}

impl<const N: usize> LedChannel<N> {
    pub const fn new(channel_number: u8) -> Self {
        Self {
            buffer: RgbBuffer::new(),
            channel_number,
        }
    }

    pub fn channel_info(&self) -> LedChannelInfo {
        LedChannelInfo::from_buffer(&self.buffer, self.channel_number)
    }

    pub fn fill(&mut self, r: u8, g: u8, b: u8) {
        self.buffer.fill(Rgb::new(r, g, b));
    }

    pub fn buffer_mut(&mut self) -> &mut RgbBuffer<N> {
        &mut self.buffer
    }

    pub fn as_mut_slice(&mut self) -> &mut [Rgb] {
        self.buffer.as_mut_slice()
    }
}
