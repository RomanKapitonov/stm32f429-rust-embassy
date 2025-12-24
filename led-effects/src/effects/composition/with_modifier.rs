use crate::effects::core::{
    pixel::Pixel,
    traits::{Generator, Modifier},
};

pub struct WithModifier<G: Generator, M: Modifier> {
    pub generator: G,
    pub modifier: M,
}

impl<G: Generator, M: Modifier> Generator for WithModifier<G, M> {
    #[inline(always)]
    fn generate(&mut self, buffer: &mut [Pixel], now: u64) {
        self.generator.generate(buffer, now);
        self.modifier.modify(buffer, now);
    }

    #[inline(always)]
    fn is_alive(&self, now: u64) -> bool {
        self.generator.is_alive(now)
    }
}
