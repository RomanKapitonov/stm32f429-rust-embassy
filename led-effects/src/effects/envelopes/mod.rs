// Envelope combinators for composing complex envelopes
pub mod combinator;
pub mod ease;
pub mod misc;
pub mod timing;

// Re-export commonly used types
pub use combinator::{Clamp, Invert, Max, Min, Product, Scale, Sum};
pub use ease::{
    BackIn, BackInOut, BackOut, BounceIn, BounceInOut, BounceOut, EaseInCubic, EaseInExpo,
    EaseInOutCubic, EaseInOutExpo, EaseInOutQuad, EaseInOutQuart, EaseInQuad, EaseInQuart,
    EaseOutCubic, EaseOutExpo, EaseOutQuad, EaseOutQuart, Easing, ElasticIn, ElasticInOut,
    ElasticOut, Linear,
};
pub use misc::{LoopCount, Pulse, TimeLimited};
pub use timing::{Constant, Fade, Sawtooth, Sine, Square, Triangle, ADSR};
