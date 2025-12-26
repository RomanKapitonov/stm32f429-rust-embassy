use crate::effects::core::{
    pixel::Pixel,
    traits::{Generator, HueParameter, Parameter},
};

pub struct SolidColor<Intensity, Hue, Sat>
where
    Intensity: Parameter<u8>,
    Hue: HueParameter,
    Sat: Parameter<u8>,
{
    pub start_time: u32,
    pub duration: u32,
    pub intensity: Intensity,
    pub hue: Hue,
    pub saturation: Sat,
}

impl<Intensity, Hue, Sat> Generator for SolidColor<Intensity, Hue, Sat>
where
    Intensity: Parameter<u8>,
    Hue: HueParameter,
    Sat: Parameter<u8>,
{
    #[inline(always)]
    fn generate(&mut self, buffer: &mut [Pixel], now: u32) {
        let intensity = self.intensity.sample(now);
        let hue = self.hue.sample(now);
        let saturation = self.saturation.sample(now);

        let color = Pixel::from_hsv(hue, saturation, intensity);
        buffer.fill(color);
    }

    #[inline(always)]
    fn is_alive(&self, now: u32) -> bool {
        now < self.start_time + self.duration
    }
}

pub struct Gradient<Intensity, Hue1, Hue2, Sat>
where
    Intensity: Parameter<u8>,
    Hue1: HueParameter,
    Hue2: HueParameter,
    Sat: Parameter<u8>,
{
    pub start_time: u32,
    pub duration: u32,
    pub intensity: Intensity,
    pub hue_start: Hue1,
    pub hue_end: Hue2,
    pub saturation: Sat,
}

impl<Intensity, Hue1, Hue2, Sat> Generator for Gradient<Intensity, Hue1, Hue2, Sat>
where
    Intensity: Parameter<u8>,
    Hue1: HueParameter,
    Hue2: HueParameter,
    Sat: Parameter<u8>,
{
    #[inline(always)]
    fn generate(&mut self, buffer: &mut [Pixel], now: u32) {
        let intensity = self.intensity.sample(now);
        let hue_start = self.hue_start.sample(now);
        let hue_end = self.hue_end.sample(now);
        let saturation = self.saturation.sample(now);

        let len = buffer.len();
        if len == 0 {
            return;
        }

        for (i, pixel) in buffer.iter_mut().enumerate() {
            // Integer interpolation: t ranges from 0 to 255
            let t = ((i as u32 * 255) / (len - 1).max(1) as u32) as u8;

            // Interpolate hue (handles wrapping)
            let hue = if hue_end >= hue_start {
                // Normal interpolation
                hue_start + (((hue_end - hue_start) as u16 * t as u16) / 255) as u8
            } else {
                // Wrapping interpolation (e.g., 250 -> 10 goes through 0)
                hue_start.wrapping_add(
                    (((256 + hue_end as u16 - hue_start as u16) * t as u16) / 255) as u8,
                )
            };

            let color = Pixel::from_hsv(hue, saturation, intensity);
            *pixel = color;
        }
    }

    #[inline(always)]
    fn is_alive(&self, now: u32) -> bool {
        now < self.start_time + self.duration
    }
}

pub struct Stripes<Intensity, Hue1, Hue2, Sat, Width>
where
    Intensity: Parameter<u8>,
    Hue1: HueParameter,
    Hue2: HueParameter,
    Sat: Parameter<u8>,
    Width: Parameter<usize>,
{
    pub start_time: u32,
    pub duration: u32,
    pub intensity: Intensity,
    pub hue1: Hue1,
    pub hue2: Hue2,
    pub saturation: Sat,
    pub stripe_width: Width,
}

impl<Intensity, Hue1, Hue2, Sat, Width> Generator for Stripes<Intensity, Hue1, Hue2, Sat, Width>
where
    Intensity: Parameter<u8>,
    Hue1: HueParameter,
    Hue2: HueParameter,
    Sat: Parameter<u8>,
    Width: Parameter<usize>,
{
    #[inline(always)]
    fn generate(&mut self, buffer: &mut [Pixel], now: u32) {
        let intensity = self.intensity.sample(now);
        let hue1 = self.hue1.sample(now);
        let hue2 = self.hue2.sample(now);
        let saturation = self.saturation.sample(now);
        let width = self.stripe_width.sample(now).max(1);

        for (i, pixel) in buffer.iter_mut().enumerate() {
            let stripe_index = (i / width) % 2;
            let hue = if stripe_index == 0 { hue1 } else { hue2 };
            let color = Pixel::from_hsv(hue, saturation, intensity);
            *pixel = color;
        }
    }

    #[inline(always)]
    fn is_alive(&self, now: u32) -> bool {
        now < self.start_time + self.duration
    }
}
