use crate::effects::core::traits::HueParameter;

pub struct RotatingHue {
    pub start_time: u64,
    pub degrees_per_ms: f32,
}

impl HueParameter for RotatingHue {
    #[inline(always)]
    fn sample(&self, now: u64) -> f32 {
        let elapsed = now.saturating_sub(self.start_time) as f32;
        (elapsed * self.degrees_per_ms) % 360.0
    }
}
