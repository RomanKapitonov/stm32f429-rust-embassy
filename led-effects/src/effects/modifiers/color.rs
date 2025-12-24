use crate::effects::core::{
    pixel::Pixel,
    traits::{Modifier, Parameter},
};

pub struct Brightness<Factor>
where
    Factor: Parameter<f32>,
{
    pub factor: Factor,
}

pub struct Saturation<Factor>
where
    Factor: Parameter<f32>,
{
    pub factor: Factor, // 0.0 = grayscale, 1.0 = original, >1.0 = oversaturated
}

pub struct HueShift<Amount>
where
    Amount: Parameter<f32>,
{
    pub amount: Amount, // Degrees to shift (0-360)
}

pub struct GammaCorrection<Gamma>
where
    Gamma: Parameter<f32>,
{
    pub gamma: Gamma, // Typical values: 2.2 for displays, 1.0 = linear
}

impl<Factor> Modifier for Brightness<Factor>
where
    Factor: Parameter<f32>,
{
    #[inline(always)]
    fn modify(&mut self, buffer: &mut [Pixel], now: u64) {
        let factor = self.factor.sample(now);
        for pixel in buffer.iter_mut() {
            *pixel = pixel.scale(factor);
        }
    }
}

impl<Factor> Modifier for Saturation<Factor>
where
    Factor: Parameter<f32>,
{
    #[inline(always)]
    fn modify(&mut self, buffer: &mut [Pixel], now: u64) {
        let factor = self.factor.sample(now);
        for pixel in buffer.iter_mut() {
            *pixel = pixel.adjust_saturation(factor);
        }
    }
}

impl<Amount> Modifier for HueShift<Amount>
where
    Amount: Parameter<f32>,
{
    #[inline(always)]
    fn modify(&mut self, buffer: &mut [Pixel], now: u64) {
        let shift = self.amount.sample(now);
        for pixel in buffer.iter_mut() {
            *pixel = pixel.shift_hue(shift);
        }
    }
}

impl<Gamma> Modifier for GammaCorrection<Gamma>
where
    Gamma: Parameter<f32>,
{
    #[inline(always)]
    fn modify(&mut self, buffer: &mut [Pixel], now: u64) {
        let gamma = self.gamma.sample(now);
        for pixel in buffer.iter_mut() {
            *pixel = pixel.gamma_correct(gamma);
        }
    }
}
