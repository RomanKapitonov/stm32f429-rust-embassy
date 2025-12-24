// Core module - traits and pixel type
pub mod core;

// Submodules
pub mod blend;
pub mod composition;
pub mod envelopes;
pub mod generators;
pub mod hue;
pub mod modifiers;
pub mod parameters;

// Re-export core traits and types
pub use core::{Envelope, Generator, HueParameter, Modifier, Parameter, Pixel};

// Re-export composition utilities
pub use composition::{EffectBuilder, WithModifier};

// Re-export common envelope types
pub use envelopes::{
    ADSR,
    // Combinators
    Clamp,
    Constant,
    EaseInOutQuad,
    EaseInQuad,
    EaseOutQuad,
    // Basic envelopes
    Fade,
    Invert,
    // Easing functions
    Linear,
    LoopCount,
    Max,
    Min,
    Product,
    // Utilities
    Pulse as EnvelopePulse,
    Sawtooth,
    Scale,
    Sine,
    Square,
    Sum,
    TimeLimited,
    Triangle,
    VelocityIntegral,
};

// Re-export parameter types
pub use parameters::{DynamicParam, StaticParam};

// Re-export hue parameters
pub use hue::{HueOscillate, RotatingHue, StaticHue};

// Re-export modifiers
pub use modifiers::{
    Blur, Brightness, Decay, GammaCorrection, HueShift, Mirror, Reverse, Saturation, Shift,
    Sparkle, Trail,
};

// Re-export generators
pub use generators::{Chase, Gradient, Pulse, SolidColor, Stripes};
