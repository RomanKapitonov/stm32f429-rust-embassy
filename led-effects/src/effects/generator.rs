use super::modifier::Modifier;
use super::parameter::{HueParameter, Parameter};
use crate::rgb::Rgb;

pub trait Generator {
    fn generate(&mut self, buffer: &mut [Rgb], now: u64);
    fn is_alive(&self, now: u64) -> bool;
}

pub struct WithModifier<G: Generator, M: Modifier> {
    pub generator: G,
    pub modifier: M,
}

impl<G: Generator, M: Modifier> Generator for WithModifier<G, M> {
    fn generate(&mut self, buffer: &mut [Rgb], now: u64) {
        self.generator.generate(buffer, now);
        self.modifier.modify(buffer, now);
    }

    fn is_alive(&self, now: u64) -> bool {
        self.generator.is_alive(now)
    }
}

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
    fn generate(&mut self, buffer: &mut [Rgb], now: u64) {
        let elapsed = now.saturating_sub(self.start_time) as f32;
        let width = self.width.sample(now);
        let intensity = self.intensity.sample(now);
        let hue = self.hue.sample(now);
        let saturation = self.saturation.sample(now);
        let distance = (elapsed * self.spread_speed) as isize;

        for edge_offset in [-distance, distance] {
            let width_pixels = width as isize;
            for i in -width_pixels..=width_pixels {
                let idx = self.position as isize + edge_offset + i;
                if idx >= 0 && (idx as usize) < buffer.len() {
                    let dist_from_edge = i.abs() as f32;
                    let falloff = (1.0 - (dist_from_edge / width)).max(0.0);

                    let color = Rgb::from_hsv(hue, saturation, intensity * falloff);
                    buffer[idx as usize] = buffer[idx as usize].add(color);
                }
            }
        }
    }

    fn is_alive(&self, now: u64) -> bool {
        now < self.start_time + self.duration
    }
}
