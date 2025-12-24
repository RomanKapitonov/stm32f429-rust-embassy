// Movement-based generators that create dynamic spatial patterns

use crate::effects::core::{
    pixel::Pixel,
    traits::{Generator, HueParameter, Parameter},
};

pub struct Chase<Pos, Width, Intensity, Hue, Sat>
where
    Pos: Parameter<f32>,
    Width: Parameter<f32>,
    Intensity: Parameter<f32>,
    Hue: HueParameter,
    Sat: Parameter<f32>,
{
    pub start_time: u64,
    pub duration: u64,
    pub position: Pos,
    pub width: Width,
    pub intensity: Intensity,
    pub hue: Hue,
    pub saturation: Sat,
}

impl<Pos, Width, Intensity, Hue, Sat> Generator for Chase<Pos, Width, Intensity, Hue, Sat>
where
    Pos: Parameter<f32>,
    Width: Parameter<f32>,
    Intensity: Parameter<f32>,
    Hue: HueParameter,
    Sat: Parameter<f32>,
{
    #[inline(always)]
    fn generate(&mut self, buffer: &mut [Pixel], now: u64) {
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
                let dist = i.abs() as f32;
                let falloff = (1.0 - (dist / width)).max(0.0);
                let color = Pixel::from_hsv(hue, saturation, intensity * falloff);
                buffer[idx as usize] = buffer[idx as usize].add(color);
            }
        }
    }

    #[inline(always)]
    fn is_alive(&self, now: u64) -> bool {
        now < self.start_time + self.duration
    }
}

/// Pulse generator that creates expanding waves from a center point
///
/// Creates symmetric pulses that expand outward from a fixed position,
/// useful for creating ripple or pulse effects.
pub struct Pulse<Width, Intensity, Hue, Sat>
where
    Width: Parameter<f32>,
    Intensity: Parameter<f32>,
    Hue: HueParameter,
    Sat: Parameter<f32>,
{
    pub start_time: u64,
    pub duration: u64,
    pub position: usize,
    pub spread_speed: f32,
    pub width: Width,
    pub intensity: Intensity,
    pub hue: Hue,
    pub saturation: Sat,
}

impl<Width, Intensity, Hue, Sat> Generator for Pulse<Width, Intensity, Hue, Sat>
where
    Width: Parameter<f32>,
    Intensity: Parameter<f32>,
    Hue: HueParameter,
    Sat: Parameter<f32>,
{
    #[inline(always)]
    fn generate(&mut self, buffer: &mut [Pixel], now: u64) {
        let elapsed = now.saturating_sub(self.start_time) as f32;
        let width = self.width.sample(now);
        let intensity = self.intensity.sample(now);
        let hue = self.hue.sample(now);
        let saturation = self.saturation.sample(now);
        let distance = (elapsed * self.spread_speed) as isize;

        // Create two expanding pulses from the center position
        for edge_offset in [-distance, distance] {
            let width_pixels = width as isize;
            for i in -width_pixels..=width_pixels {
                let idx = self.position as isize + edge_offset + i;
                if idx >= 0 && (idx as usize) < buffer.len() {
                    let dist_from_edge = i.abs() as f32;
                    let falloff = (1.0 - (dist_from_edge / width)).max(0.0);

                    let color = Pixel::from_hsv(hue, saturation, intensity * falloff);
                    buffer[idx as usize] = buffer[idx as usize].add(color);
                }
            }
        }
    }

    #[inline(always)]
    fn is_alive(&self, now: u64) -> bool {
        now < self.start_time + self.duration
    }
}
