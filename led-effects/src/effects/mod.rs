pub mod builder;
pub mod envelope;
pub mod envelopes;
pub mod generator;
pub mod hue;
pub mod modifier;
pub mod modifiers;
pub mod parameter;

pub use builder::EffectBuilder;
pub use envelope::Envelope;
pub use envelopes::{Fade, Triangle};
pub use generator::{Generator, Pulse, WithModifier};
pub use modifier::Modifier;
pub use modifiers::{Brightness, GammaCorrection, HueShift, Saturation};
pub use parameter::{Dynamic, HueParameter, Parameter, RotatingHue, Static};
