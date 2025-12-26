use crate::effects::core::traits::Parameter;

pub struct StaticParam<T>(pub T);

impl<T: Copy> Parameter<T> for StaticParam<T> {
    #[inline(always)]
    fn sample(&self, _now: u32) -> T {
        self.0
    }
}
