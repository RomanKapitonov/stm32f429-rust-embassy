use crate::effects::core::traits::{HueParameter, Parameter};

pub struct StaticParam<T>(pub T);

impl<T: Copy> Parameter<T> for StaticParam<T> {
    #[inline(always)]
    fn sample(&self, _now: u64) -> T {
        self.0
    }
}

impl HueParameter for StaticParam<f32> {
    #[inline(always)]
    fn sample(&self, _now: u64) -> f32 {
        self.0 % 360.0
    }
}
