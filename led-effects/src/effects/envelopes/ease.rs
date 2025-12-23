pub trait Easing {
    fn ease(&self, t: f32) -> f32;
}

pub struct Linear;
pub struct EaseInQuad;
pub struct EaseInCubic;
pub struct EaseInQuart;
pub struct EaseInExpo;
pub struct EaseOutQuad;
pub struct EaseOutCubic;
pub struct EaseOutQuart;
pub struct EaseOutExpo;
pub struct EaseInOutQuad;
pub struct EaseInOutCubic;
pub struct EaseInOutQuart;
pub struct EaseInOutExpo;
pub struct BounceOut;
pub struct BounceIn;
pub struct BounceInOut;
pub struct ElasticOut {
    pub amplitude: f32,
    pub period: f32,
}
pub struct ElasticIn {
    pub amplitude: f32,
    pub period: f32,
}
pub struct ElasticInOut {
    pub amplitude: f32,
    pub period: f32,
}
pub struct BackOut {
    pub overshoot: f32,
}
pub struct BackIn {
    pub overshoot: f32,
}
pub struct BackInOut {
    pub overshoot: f32,
}

impl Easing for Linear {
    fn ease(&self, t: f32) -> f32 {
        t
    }
}

impl Easing for EaseInQuad {
    fn ease(&self, t: f32) -> f32 {
        t * t
    }
}

impl Easing for EaseInCubic {
    fn ease(&self, t: f32) -> f32 {
        t * t * t
    }
}

impl Easing for EaseInQuart {
    fn ease(&self, t: f32) -> f32 {
        t * t * t * t
    }
}

impl Easing for EaseInExpo {
    fn ease(&self, t: f32) -> f32 {
        if t == 0.0 {
            0.0
        } else {
            libm::powf(2.0_f32, 10.0 * (t - 1.0))
        }
    }
}

impl Easing for EaseOutQuad {
    fn ease(&self, t: f32) -> f32 {
        t * (2.0 - t)
    }
}

impl Easing for EaseOutCubic {
    fn ease(&self, t: f32) -> f32 {
        let t = t - 1.0;
        t * t * t + 1.0
    }
}

impl Easing for EaseOutQuart {
    fn ease(&self, t: f32) -> f32 {
        let t = t - 1.0;
        1.0 - t * t * t * t
    }
}

impl Easing for EaseOutExpo {
    fn ease(&self, t: f32) -> f32 {
        if t == 1.0 {
            1.0
        } else {
            1.0 - libm::powf(2.0_f32, -10.0 * t)
        }
    }
}

impl Easing for EaseInOutQuad {
    fn ease(&self, t: f32) -> f32 {
        if t < 0.5 {
            2.0 * t * t
        } else {
            -1.0 + (4.0 - 2.0 * t) * t
        }
    }
}

impl Easing for EaseInOutCubic {
    fn ease(&self, t: f32) -> f32 {
        if t < 0.5 {
            4.0 * t * t * t
        } else {
            let t = 2.0 * t - 2.0;
            1.0 + t * t * t / 2.0
        }
    }
}

impl Easing for EaseInOutQuart {
    fn ease(&self, t: f32) -> f32 {
        if t < 0.5 {
            8.0 * t * t * t * t
        } else {
            let t = t - 1.0;
            1.0 - 8.0 * t * t * t * t
        }
    }
}

impl Easing for EaseInOutExpo {
    fn ease(&self, t: f32) -> f32 {
        if t == 0.0 {
            0.0
        } else if t == 1.0 {
            1.0
        } else if t < 0.5 {
            libm::powf(2.0_f32, 20.0 * t - 10.0) / 2.0
        } else {
            (2.0 - libm::powf(2.0_f32, -20.0 * t + 10.0)) / 2.0
        }
    }
}

impl Easing for BounceOut {
    fn ease(&self, t: f32) -> f32 {
        const N1: f32 = 7.5625;
        const D1: f32 = 2.75;

        if t < 1.0 / D1 {
            N1 * t * t
        } else if t < 2.0 / D1 {
            let t = t - 1.5 / D1;
            N1 * t * t + 0.75
        } else if t < 2.5 / D1 {
            let t = t - 2.25 / D1;
            N1 * t * t + 0.9375
        } else {
            let t = t - 2.625 / D1;
            N1 * t * t + 0.984375
        }
    }
}

impl Easing for BounceIn {
    fn ease(&self, t: f32) -> f32 {
        1.0 - BounceOut.ease(1.0 - t)
    }
}

impl Easing for BounceInOut {
    fn ease(&self, t: f32) -> f32 {
        if t < 0.5 {
            (1.0 - BounceOut.ease(1.0 - 2.0 * t)) / 2.0
        } else {
            (1.0 + BounceOut.ease(2.0 * t - 1.0)) / 2.0
        }
    }
}

impl ElasticOut {
    pub fn standard() -> Self {
        Self {
            amplitude: 1.0,
            period: 0.3,
        }
    }
}

impl Easing for ElasticOut {
    fn ease(&self, t: f32) -> f32 {
        if t == 0.0 {
            0.0
        } else if t == 1.0 {
            1.0
        } else {
            use core::f32::consts::PI;
            let s = self.period / 4.0;
            self.amplitude
                * libm::powf(2.0_f32, -10.0 * t)
                * libm::sinf((t - s) * (2.0 * PI) / self.period)
                + 1.0
        }
    }
}

impl ElasticIn {
    pub fn standard() -> Self {
        Self {
            amplitude: 1.0,
            period: 0.3,
        }
    }
}

impl Easing for ElasticIn {
    fn ease(&self, t: f32) -> f32 {
        if t == 0.0 {
            0.0
        } else if t == 1.0 {
            1.0
        } else {
            use core::f32::consts::PI;
            let s = self.period / 4.0;
            -(self.amplitude
                * libm::powf(2.0_f32, 10.0 * (t - 1.0))
                * libm::sinf((t - 1.0 - s) * (2.0 * PI) / self.period))
        }
    }
}

impl ElasticInOut {
    pub fn standard() -> Self {
        Self {
            amplitude: 1.0,
            period: 0.3,
        }
    }
}

impl Easing for ElasticInOut {
    fn ease(&self, t: f32) -> f32 {
        if t == 0.0 {
            0.0
        } else if t == 1.0 {
            1.0
        } else {
            use core::f32::consts::PI;
            let s = self.period / 4.0;
            let t = t * 2.0 - 1.0;

            if t < 0.0 {
                -0.5 * (self.amplitude
                    * libm::powf(2.0_f32, 10.0 * t)
                    * libm::sinf((t - s) * (2.0 * PI) / self.period))
            } else {
                0.5 * self.amplitude
                    * libm::powf(2.0_f32, -10.0 * t)
                    * libm::sinf((t - s) * (2.0 * PI) / self.period)
                    + 1.0
            }
        }
    }
}

impl BackOut {
    pub fn standard() -> Self {
        Self { overshoot: 1.70158 }
    }
}

impl Easing for BackOut {
    fn ease(&self, t: f32) -> f32 {
        let c1 = self.overshoot;
        let c3 = c1 + 1.0;
        let t = t - 1.0;
        1.0 + c3 * t * t * t + c1 * t * t
    }
}

impl BackIn {
    pub fn standard() -> Self {
        Self { overshoot: 1.70158 }
    }
}

impl Easing for BackIn {
    fn ease(&self, t: f32) -> f32 {
        let c1 = self.overshoot;
        let c3 = c1 + 1.0;
        c3 * t * t * t - c1 * t * t
    }
}

impl BackInOut {
    pub fn standard() -> Self {
        Self { overshoot: 1.70158 }
    }
}

impl Easing for BackInOut {
    fn ease(&self, t: f32) -> f32 {
        let c1 = self.overshoot;
        let c2 = c1 * 1.525;

        if t < 0.5 {
            let t = 2.0 * t;
            (t * t * ((c2 + 1.0) * t - c2)) / 2.0
        } else {
            let t = 2.0 * t - 2.0;
            (t * t * ((c2 + 1.0) * t + c2) + 2.0) / 2.0
        }
    }
}
