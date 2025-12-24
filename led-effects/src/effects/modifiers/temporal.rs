// fade

use crate::effects::core::{
    pixel::Pixel,
    traits::{Modifier, Parameter},
};

pub struct Decay<Rate>
where
    Rate: Parameter<f32>,
{
    pub rate: Rate,
}

impl<Rate> Modifier for Decay<Rate>
where
    Rate: Parameter<f32>,
{
    #[inline(always)]
    fn modify(&mut self, buffer: &mut [Pixel], now: u64) {
        let rate = self.rate.sample(now);
        for pixel in buffer.iter_mut() {
            *pixel = pixel.scale(rate);
        }
    }
}
