use super::with_modifier::WithModifier;
use crate::effects::core::traits::{Generator, Modifier};

pub struct EffectBuilder<G> {
    generator: G,
}

impl<G> EffectBuilder<G> {
    #[inline(always)]
    pub fn new(generator: G) -> Self {
        Self { generator }
    }

    #[inline(always)]
    pub fn with_modifier<M: Modifier>(self, modifier: M) -> EffectBuilder<WithModifier<G, M>>
    where
        G: Generator,
    {
        EffectBuilder {
            generator: WithModifier {
                generator: self.generator,
                modifier,
            },
        }
    }

    #[inline(always)]
    pub fn build(self) -> G {
        self.generator
    }
}
