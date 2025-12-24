pub mod combinators;
pub mod easing;
pub mod timing;
pub mod utilities;

pub use combinators::{Clamp, Invert, Max, Min, Product, Scale, Sum};
pub use easing::{
    BackIn, BackInOut, BackOut, BounceIn, BounceInOut, BounceOut, EaseInCubic, EaseInExpo,
    EaseInOutCubic, EaseInOutExpo, EaseInOutQuad, EaseInOutQuart, EaseInQuad, EaseInQuart,
    EaseOutCubic, EaseOutExpo, EaseOutQuad, EaseOutQuart, ElasticIn, ElasticInOut, ElasticOut,
    Linear,
};
pub use timing::{ADSR, Constant, Fade, Sawtooth, Sine, Square, Triangle};
pub use utilities::{LoopCount, Pulse, TimeLimited, VelocityIntegral};
