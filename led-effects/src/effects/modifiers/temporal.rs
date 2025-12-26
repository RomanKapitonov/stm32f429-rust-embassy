use crate::effects::core::{
    pixel::Pixel,
    traits::{Modifier, Parameter},
};

pub struct Decay<Rate>
where
    Rate: Parameter<u8>,
{
    pub rate: Rate, // 0-255 (where 255 = no decay, 0 = instant fade)
}

impl<Rate> Modifier for Decay<Rate>
where
    Rate: Parameter<u8>,
{
    #[inline(always)]
    fn modify(&mut self, buffer: &mut [Pixel], now: u32) {
        let rate = self.rate.sample(now);
        for pixel in buffer.iter_mut() {
            *pixel = pixel.scale(rate);
        }
    }
}
