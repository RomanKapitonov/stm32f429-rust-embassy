use crate::Rgb;
use crate::effects::Modifier;
use crate::effects::Parameter;

pub struct Brightness<Factor>
where
    Factor: Parameter<f32>,
{
    pub factor: Factor,
}

impl<Factor> Modifier for Brightness<Factor>
where
    Factor: Parameter<f32>,
{
    fn modify(&mut self, buffer: &mut [Rgb], now: u64) {
        let factor = self.factor.sample(now);
        for pixel in buffer.iter_mut() {
            *pixel = pixel.scale(factor);
        }
    }
}

pub struct Saturation<Factor>
where
    Factor: Parameter<f32>,
{
    pub factor: Factor, // 0.0 = grayscale, 1.0 = original, >1.0 = oversaturated
}

impl<Factor> Modifier for Saturation<Factor>
where
    Factor: Parameter<f32>,
{
    fn modify(&mut self, buffer: &mut [Rgb], now: u64) {
        let factor = self.factor.sample(now);
        for pixel in buffer.iter_mut() {
            *pixel = pixel.adjust_saturation(factor);
        }
    }
}

pub struct HueShift<Amount>
where
    Amount: Parameter<f32>,
{
    pub amount: Amount, // Degrees to shift (0-360)
}

impl<Amount> Modifier for HueShift<Amount>
where
    Amount: Parameter<f32>,
{
    fn modify(&mut self, buffer: &mut [Rgb], now: u64) {
        let shift = self.amount.sample(now);
        for pixel in buffer.iter_mut() {
            *pixel = pixel.shift_hue(shift);
        }
    }
}

pub struct GammaCorrection<Gamma>
where
    Gamma: Parameter<f32>,
{
    pub gamma: Gamma, // Typical values: 2.2 for displays, 1.0 = linear
}

impl<Gamma> Modifier for GammaCorrection<Gamma>
where
    Gamma: Parameter<f32>,
{
    fn modify(&mut self, buffer: &mut [Rgb], now: u64) {
        let gamma = self.gamma.sample(now);
        for pixel in buffer.iter_mut() {
            *pixel = pixel.gamma_correct(gamma);
        }
    }
}
