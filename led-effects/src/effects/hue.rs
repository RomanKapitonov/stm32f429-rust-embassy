pub trait HueParameter {
    fn sample(&self, now: u64) -> f32;
}

pub struct StaticHue {
    pub hue: f32, // 0-360 degrees
}

pub struct RotatingHue {
    pub start_time: u64,
    pub degrees_per_ms: f32,
}

pub struct HueOscillate {
    pub start_time: u64,
    pub period: u64,
    pub hue1: f32,
    pub hue2: f32,
}

impl HueParameter for StaticHue {
    fn sample(&self, _now: u64) -> f32 {
        self.hue % 360.0
    }
}

impl HueParameter for RotatingHue {
    fn sample(&self, now: u64) -> f32 {
        let elapsed = now.saturating_sub(self.start_time) as f32;
        (elapsed * self.degrees_per_ms) % 360.0
    }
}

impl HueParameter for HueOscillate {
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
