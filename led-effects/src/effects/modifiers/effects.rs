use crate::effects::core::{
    pixel::Pixel,
    traits::{HueParameter, Modifier, Parameter},
};

pub struct Trail<DecayRate>
where
    DecayRate: Parameter<u8>,
{
    pub decay_rate: DecayRate, // 0-255 (where 255 = no decay, 128 = 50% decay)
}

pub struct Sparkle<Chance, Hue, Sat, Intensity>
where
    Chance: Parameter<u8>,
    Hue: HueParameter,
    Sat: Parameter<u8>,
    Intensity: Parameter<u8>,
{
    pub chance: Chance,
    pub hue: Hue,
    pub saturation: Sat,
    pub intensity: Intensity,
    pub seed: u32,
}

impl<DecayRate> Modifier for Trail<DecayRate>
where
    DecayRate: Parameter<u8>,
{
    #[inline(always)]
    fn modify(&mut self, buffer: &mut [Pixel], now: u32) {
        let decay = self.decay_rate.sample(now);
        for pixel in buffer.iter_mut() {
            *pixel = pixel.scale(decay);
        }
    }
}

impl<Chance, Hue, Sat, Intensity> Modifier for Sparkle<Chance, Hue, Sat, Intensity>
where
    Chance: Parameter<u8>,
    Hue: HueParameter,
    Sat: Parameter<u8>,
    Intensity: Parameter<u8>,
{
    #[inline(always)]
    fn modify(&mut self, buffer: &mut [Pixel], now: u32) {
        let chance = self.chance.sample(now);
        let hue = self.hue.sample(now);
        let saturation = self.saturation.sample(now);
        let intensity = self.intensity.sample(now);

        for pixel in buffer.iter_mut() {
            self.seed = self.seed.wrapping_mul(1103515245).wrapping_add(12345);
            if ((self.seed >> 24) as u8) < chance {
                *pixel = Pixel::from_hsv(hue, saturation, intensity);
            }
        }
    }
}
