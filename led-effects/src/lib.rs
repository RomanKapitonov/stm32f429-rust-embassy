#![cfg_attr(not(feature = "std"), no_std)]

pub mod effects;

// Reexports
// Core traits
pub use effects::{Envelope, Generator, HueParameter, Modifier, Parameter};

// Core types
pub use effects::Pixel;

// Envelope types
pub use effects::{
    ADSR,
    // Combinators
    Clamp,
    // Basic waveforms
    Constant,
    // Easing
    EaseInOutQuad,
    EaseInQuad,
    EaseOutQuad,
    // Utilities
    EnvelopePulse,
    Fade,
    Invert,
    Linear,
    LoopCount,
    Max,
    Min,
    Product,
    Sawtooth,
    Sine,
    Square,
    Sum,
    TimeLimited,
    Triangle,
    VelocityIntegral,
};

// Parameters
pub use effects::{DynamicParam, StaticParam};

// Hue parameters
pub use effects::{HueOscillate, RotatingHue, StaticHue};

// Generators
pub use effects::{Chase, Gradient, Pulse, SolidColor, Stripes};

// Modifiers
pub use effects::{
    Blur, Brightness, EffectBuilder, GammaCorrection, HueShift, Mirror, Reverse, Saturation, Shift,
    Trail, WithModifier,
};
