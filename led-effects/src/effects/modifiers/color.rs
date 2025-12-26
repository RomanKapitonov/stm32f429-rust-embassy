use crate::effects::core::{
    pixel::Pixel,
    traits::{Modifier, Parameter},
};

pub struct Brightness<Factor>
where
    Factor: Parameter<u8>,
{
    pub factor: Factor, // 0-255 (where 255 = 100% brightness)
}

pub struct Saturation<Factor>
where
    Factor: Parameter<u8>,
{
    pub factor: Factor, // 0-255 (where 0 = grayscale, 255 = full saturation)
}

pub struct HueShift<Amount>
where
    Amount: Parameter<u8>,
{
    pub amount: Amount, // 0-255 (wraps around hue wheel)
}

pub struct GammaCorrection; // Fixed gamma 2.2 correction

impl<Factor> Modifier for Brightness<Factor>
where
    Factor: Parameter<u8>,
{
    #[inline(always)]
    fn modify(&mut self, buffer: &mut [Pixel], now: u32) {
        let factor = self.factor.sample(now);
        for pixel in buffer.iter_mut() {
            *pixel = pixel.scale(factor);
        }
    }
}

impl<Factor> Modifier for Saturation<Factor>
where
    Factor: Parameter<u8>,
{
    #[inline(always)]
    fn modify(&mut self, buffer: &mut [Pixel], now: u32) {
        let factor = self.factor.sample(now);
        for pixel in buffer.iter_mut() {
            *pixel = pixel.adjust_saturation(factor);
        }
    }
}

impl<Amount> Modifier for HueShift<Amount>
where
    Amount: Parameter<u8>,
{
    #[inline(always)]
    fn modify(&mut self, buffer: &mut [Pixel], now: u32) {
        let shift = self.amount.sample(now);
        for pixel in buffer.iter_mut() {
            *pixel = pixel.shift_hue(shift);
        }
    }
}

impl Modifier for GammaCorrection {
    #[inline(always)]
    fn modify(&mut self, buffer: &mut [Pixel], _now: u32) {
        for pixel in buffer.iter_mut() {
            *pixel = pixel.gamma_correct();
        }
    }
}
