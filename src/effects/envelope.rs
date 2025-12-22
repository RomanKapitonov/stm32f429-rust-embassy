pub trait Envelope {
    fn sample(&self, now: u64) -> f32;
    fn is_alive(&self, now: u64) -> bool;
}

#[derive(Copy, Clone)]
pub struct Fade {
    pub start_time: u64,
    pub duration: u64,
    pub inverted: bool,
}

impl Envelope for Fade {
    fn sample(&self, now: u64) -> f32 {
        let elapsed = now.saturating_sub(self.start_time);
        let progress = (elapsed as f32 / self.duration as f32).min(1.0);
        if self.inverted {
            1.0 - progress
        } else {
            progress
        }
    }

    fn is_alive(&self, now: u64) -> bool {
        now < self.start_time + self.duration
    }
}

#[derive(Copy, Clone)]
pub struct Triangle {
    pub start_time: u64,
    pub period: u64,
}

impl Envelope for Triangle {
    fn sample(&self, now: u64) -> f32 {
        let elapsed = now.saturating_sub(self.start_time);
        let phase = (elapsed % self.period) as f32 / self.period as f32;
        if phase < 0.5 {
            phase * 2.0
        } else {
            2.0 - phase * 2.0
        }
    }

    fn is_alive(&self, _now: u64) -> bool {
        true
    }
}
