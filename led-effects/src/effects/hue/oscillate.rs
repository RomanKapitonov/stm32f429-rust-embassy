// hue/oscillate.rs
use crate::effects::core::traits::HueParameter;

pub struct HueOscillate {
    pub start_time: u32,
    pub period: u32,
    pub hue1: u8, // 0-255
    pub hue2: u8, // 0-255
}

impl HueParameter for HueOscillate {
    #[inline(always)]
    fn sample(&self, now: u32) -> u8 {
        let elapsed = now.saturating_sub(self.start_time);
        let elapsed_in_cycle = (elapsed % self.period) as u64;

        // Triangle wave: 0 → period → 0
        let phase_doubled = elapsed_in_cycle * 2;
        let period_u64 = self.period as u64;

        if phase_doubled < period_u64 {
            // First half: interpolate hue1 → hue2
            let progress = ((phase_doubled * 255) / period_u64) as u8;
            let range = self.hue2.saturating_sub(self.hue1);
            self.hue1
                .saturating_add(((progress as u16 * range as u16) / 255) as u8)
        } else {
            // Second half: interpolate hue2 → hue1
            let descending = phase_doubled - period_u64;
            let progress = ((descending * 255) / period_u64) as u8;
            let range = self.hue1.saturating_sub(self.hue2);
            self.hue2
                .saturating_add(((progress as u16 * range as u16) / 255) as u8)
        }
    }
}
