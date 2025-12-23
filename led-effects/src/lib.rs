#![cfg_attr(not(feature = "std"), no_std)]

//! # LED Effects Library
//!
//! A no_std library for creating sophisticated LED lighting effects using envelopes,
//! generators, and modifiers. This library provides a composable system for building
//! complex LED animations for embedded systems.
//!
//! ## Features
//!
//! - **Envelope System**: Time-based control curves (ADSR, fades, waveforms, etc.)
//! - **Envelope Combinators**: Compose complex envelopes from simple ones
//! - **Effect Generators**: High-level effects that operate on LED buffers
//! - **Parameter System**: Dynamic parameters driven by envelopes
//! - **HSV Color Support**: Easy color manipulation
//! - **no_std Compatible**: Works in embedded environments
//!
//! ## Quick Start
//!
//! ```ignore
//! use led_effects::{Rgb, Generator, Pulse, Static};
//!
//! let mut effect = Pulse {
//!     start_time: 0,
//!     duration: 2000,
//!     position: 30,
//!     spread_speed: 0.02,
//!     width: Static(3.0),
//!     intensity: Static(1.0),
//!     hue: Static(240.0),
//!     saturation: Static(1.0),
//! };
//!
//! let mut buffer = vec![Rgb::BLACK; 100];
//! effect.generate(&mut buffer, current_time_ms);
//! ```

pub mod effects;
pub mod rgb;

// Re-export commonly used types at the crate root
pub use effects::{
    // Modifiers
    // Sparkle,

    // Parameters
    Dynamic,
    // Builder
    EffectBuilder,
    // Core traits
    Envelope,
    // Envelope implementations
    Fade,
    Generator,
    HueParameter,
    Modifier,
    Parameter,

    // Generators
    Pulse,
    RotatingHue,
    Static,

    Triangle,

    WithModifier,
};

pub use rgb::Rgb;
