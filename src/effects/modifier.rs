use super::parameter::Parameter;
use crate::rgb::Rgb;

pub trait Modifier {
    fn modify(&mut self, buffer: &mut [Rgb], now: u64);
}

pub struct Sparkle<Chance>
where
    Chance: Parameter<u8>,
{
    pub chance: Chance,
    pub seed: u32,
}

impl<Chance> Modifier for Sparkle<Chance>
where
    Chance: Parameter<u8>,
{
    fn modify(&mut self, buffer: &mut [Rgb], now: u64) {
        let chance = self.chance.sample(now);
        for pixel in buffer.iter_mut() {
            self.seed = self.seed.wrapping_mul(1103515245).wrapping_add(12345);
            if ((self.seed >> 24) as u8) < chance {
                *pixel = Rgb::from_hsv(0.0, 0.0, 1.0);
            }
        }
    }
}
