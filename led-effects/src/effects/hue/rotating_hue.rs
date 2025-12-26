// hue/rotating_hue.rs
use crate::effects::core::traits::HueParameter;

pub struct RotatingHue {
    pub start_time: u32,
    pub degrees_per_ms: f32, // Keep as f32 for smooth rotation
}

impl HueParameter for RotatingHue {
    #[inline(always)]
    fn sample(&self, now: u32) -> u8 {
        let elapsed = now.saturating_sub(self.start_time) as f32;
        let degrees = (elapsed * self.degrees_per_ms) % 360.0;
        ((degrees * 255.0) / 360.0) as u8
    }
}
