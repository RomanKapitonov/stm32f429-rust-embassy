use crate::effects::core::{
    pixel::Pixel,
    traits::{Generator, HueParameter, Parameter},
};

pub struct Chase<Pos, Width, Intensity, Hue, Sat>
where
    Pos: Parameter<u16>,
    Width: Parameter<u8>,
    Intensity: Parameter<u8>,
    Hue: HueParameter,
    Sat: Parameter<u8>,
{
    pub start_time: u32,
    pub duration: u32,
    pub position: Pos,
    pub width: Width,
    pub intensity: Intensity,
    pub hue: Hue,
    pub saturation: Sat,
}

impl<Pos, Width, Intensity, Hue, Sat> Generator for Chase<Pos, Width, Intensity, Hue, Sat>
where
    Pos: Parameter<u16>,
    Width: Parameter<u8>,
    Intensity: Parameter<u8>,
    Hue: HueParameter,
    Sat: Parameter<u8>,
{
    #[inline(always)]
    fn generate(&mut self, buffer: &mut [Pixel], now: u32) {
        let pos = self.position.sample(now);
        let width = self.width.sample(now);
        let intensity = self.intensity.sample(now);
        let hue = self.hue.sample(now);
        let saturation = self.saturation.sample(now);

        let center = pos as isize;
        let width_pixels = width as isize;

        for i in -width_pixels..=width_pixels {
            let idx = center + i;
            if idx >= 0 && (idx as usize) < buffer.len() {
                let dist = i.abs() as u8;

                // Calculate falloff: 255 at center, 0 at edges
                let falloff_u8 = if dist >= width {
                    0
                } else {
                    (((width - dist) as u16 * 255) / width as u16) as u8
                };

                // Scale intensity by falloff
                let scaled_intensity = ((intensity as u16 * falloff_u8 as u16) / 255) as u8;

                let color = Pixel::from_hsv(hue, saturation, scaled_intensity);
                buffer[idx as usize] = buffer[idx as usize].add(color);
            }
        }
    }

    #[inline(always)]
    fn is_alive(&self, now: u32) -> bool {
        now < self.start_time + self.duration
    }
}

pub struct Pulse<Width, Intensity, Hue, Sat>
where
    Width: Parameter<u8>,
    Intensity: Parameter<u8>,
    Hue: HueParameter,
    Sat: Parameter<u8>,
{
    pub start_time: u32,
    pub duration: u32,
    pub position: usize,
    pub spread_speed: f32, // Pixels per millisecond (keep f32 for smooth speed)
    pub width: Width,
    pub intensity: Intensity,
    pub hue: Hue,
    pub saturation: Sat,
}

impl<Width, Intensity, Hue, Sat> Generator for Pulse<Width, Intensity, Hue, Sat>
where
    Width: Parameter<u8>,
    Intensity: Parameter<u8>,
    Hue: HueParameter,
    Sat: Parameter<u8>,
{
    #[inline(always)]
    fn generate(&mut self, buffer: &mut [Pixel], now: u32) {
        let elapsed = now.saturating_sub(self.start_time);
        let width = self.width.sample(now);
        let intensity = self.intensity.sample(now);
        let hue = self.hue.sample(now);
        let saturation = self.saturation.sample(now);
        let distance = (elapsed as f32 * self.spread_speed) as isize;

        // Create two expanding pulses from the center position
        for edge_offset in [-distance, distance] {
            let width_pixels = width as isize;
            for i in -width_pixels..=width_pixels {
                let idx = self.position as isize + edge_offset + i;
                if idx >= 0 && (idx as usize) < buffer.len() {
                    let dist_from_edge = i.abs() as u8;

                    // Calculate falloff: 255 at center, 0 at edges
                    let falloff_u8 = if dist_from_edge >= width {
                        0
                    } else {
                        (((width - dist_from_edge) as u16 * 255) / width as u16) as u8
                    };

                    // Scale intensity by falloff
                    let scaled_intensity = ((intensity as u16 * falloff_u8 as u16) / 255) as u8;

                    let color = Pixel::from_hsv(hue, saturation, scaled_intensity);
                    buffer[idx as usize] = buffer[idx as usize].add(color);
                }
            }
        }
    }

    #[inline(always)]
    fn is_alive(&self, now: u32) -> bool {
        now < self.start_time + self.duration
    }
}
