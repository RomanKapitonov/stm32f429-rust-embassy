use crate::effects::core::traits::HueParameter;

pub struct HueOscillate {
    pub start_time: u64,
    pub period: u64,
    pub hue1: f32,
    pub hue2: f32,
}
impl HueParameter for HueOscillate {
    #[inline(always)]
    fn sample(&self, now: u64) -> f32 {
        let elapsed = now.saturating_sub(self.start_time);
        let phase = (elapsed % self.period) as f32 / self.period as f32;

        // Triangle wave
        let t = if phase < 0.5 {
            phase * 2.0
        } else {
            2.0 - phase * 2.0
        };

        // Linear interpolation between hue1 and hue2
        self.hue1 + (self.hue2 - self.hue1) * t
    }
}
