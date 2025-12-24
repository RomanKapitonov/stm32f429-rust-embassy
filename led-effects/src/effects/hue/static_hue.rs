use crate::effects::core::traits::HueParameter;

pub struct StaticHue {
    pub hue: f32, // 0-360 degrees
}

impl HueParameter for StaticHue {
    #[inline(always)]
    fn sample(&self, _now: u64) -> f32 {
        self.hue % 360.0
    }
}
