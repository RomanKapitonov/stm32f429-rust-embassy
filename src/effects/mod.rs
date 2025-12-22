pub mod builder;
pub mod envelope;
pub mod generator;
pub mod modifier;
pub mod parameter;

pub use builder::EffectBuilder;
pub use envelope::{Envelope, Fade, Triangle};
pub use generator::{Generator, Pulse, WithModifier};
pub use modifier::{Modifier, Sparkle};
pub use parameter::{Dynamic, HueParameter, Parameter, RotatingHue, Static};
