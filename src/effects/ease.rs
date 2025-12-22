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
    pub amplitude: f32, // 1.0 is standard, higher = more overshoot
    pub period: f32,    // 0.3 is standard, higher = more oscillations
}
pub struct ElasticIn {
    pub amplitude: f32,
    pub period: f32,
}
pub struct ElasticInOut {
    pub amplitude: f32,
    pub period: f32,
}

impl Easing for Linear {
    #[inline(always)]
    fn ease(&self, t: f32) -> f32 {
        t
    }
}

impl Easing for EaseInQuad {
    #[inline(always)]
    fn ease(&self, t: f32) -> f32 {
        t * t
    }
}

impl Easing for EaseInCubic {
    #[inline(always)]
    fn ease(&self, t: f32) -> f32 {
        t * t * t
    }
}

impl Easing for EaseInQuart {
    #[inline(always)]
    fn ease(&self, t: f32) -> f32 {
        t * t * t * t
    }
}

impl Easing for EaseInExpo {
    #[inline(always)]
    fn ease(&self, t: f32) -> f32 {
        if t == 0.0 {
            0.0
        } else {
            (2.0_f32).powf(10.0 * (t - 1.0))
        }
    }
}

impl Easing for EaseOutQuad {
    #[inline(always)]
    fn ease(&self, t: f32) -> f32 {
        t * (2.0 - t)
    }
}

impl Easing for EaseOutCubic {
    #[inline(always)]
    fn ease(&self, t: f32) -> f32 {
        let t = t - 1.0;
        t * t * t + 1.0
    }
}

impl Easing for EaseOutQuart {
    #[inline(always)]
    fn ease(&self, t: f32) -> f32 {
        let t = t - 1.0;
        1.0 - t * t * t * t
    }
}

impl Easing for EaseOutExpo {
    #[inline(always)]
    fn ease(&self, t: f32) -> f32 {
        if t == 1.0 {
            1.0
        } else {
            1.0 - (2.0_f32).powf(-10.0 * t)
        }
    }
}

impl Easing for EaseInOutQuad {
    #[inline(always)]
    fn ease(&self, t: f32) -> f32 {
        if t < 0.5 {
            2.0 * t * t
        } else {
            -1.0 + (4.0 - 2.0 * t) * t
        }
    }
}

impl Easing for EaseInOutCubic {
    #[inline(always)]
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
    #[inline(always)]
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
    #[inline(always)]
    fn ease(&self, t: f32) -> f32 {
        if t == 0.0 {
            0.0
        } else if t == 1.0 {
            1.0
        } else if t < 0.5 {
            (2.0_f32).powf(20.0 * t - 10.0) / 2.0
        } else {
            (2.0 - (2.0_f32).powf(-20.0 * t + 10.0)) / 2.0
        }
    }
}

impl Easing for BounceOut {
    #[inline(always)]
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
    #[inline(always)]
    fn ease(&self, t: f32) -> f32 {
        1.0 - BounceOut.ease(1.0 - t)
    }
}

impl Easing for BounceInOut {
    #[inline(always)]
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
    #[inline(always)]
    fn ease(&self, t: f32) -> f32 {
        if t == 0.0 {
            0.0
        } else if t == 1.0 {
            1.0
        } else {
            use core::f32::consts::PI;
            let s = self.period / 4.0;
            self.amplitude * (2.0_f32).powf(-10.0 * t) * ((t - s) * (2.0 * PI) / self.period).sin()
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
    #[inline(always)]
    fn ease(&self, t: f32) -> f32 {
        if t == 0.0 {
            0.0
        } else if t == 1.0 {
            1.0
        } else {
            use core::f32::consts::PI;
            let s = self.period / 4.0;
            -(self.amplitude
                * (2.0_f32).powf(10.0 * (t - 1.0))
                * ((t - 1.0 - s) * (2.0 * PI) / self.period).sin())
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
    #[inline(always)]
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
                    * (2.0_f32).powf(10.0 * t)
                    * ((t - s) * (2.0 * PI) / self.period).sin())
            } else {
                0.5 * self.amplitude
                    * (2.0_f32).powf(-10.0 * t)
                    * ((t - s) * (2.0 * PI) / self.period).sin()
                    + 1.0
            }
        }
    }
}
