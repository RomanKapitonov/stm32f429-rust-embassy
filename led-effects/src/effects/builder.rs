use super::generator::{Generator, WithModifier};
use super::modifier::Modifier;

pub struct EffectBuilder<G> {
    generator: G,
}

impl<G> EffectBuilder<G> {
    pub fn new(generator: G) -> Self {
        Self { generator }
    }

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

    pub fn build(self) -> G {
        self.generator
    }
}
