//! # WS2812 LED Driver Module
//!
//! This module provides a safe, zero-copy interface to the WS2812 LED hardware driver.
//! It wraps the C FFI functions and provides Rust-safe abstractions.
//!
//! ## Architecture
//!
//! - **FFI Layer** ([`ffi`]) - Raw C bindings to hardware driver
//! - **Channel Layer** ([`channel`]) - Safe wrappers for LED channels
//! - **Global Driver** - Thread-safe singleton for managing all channels
//!
//! ## Usage
//!
//! ### Initialize the driver
//!
//! ```ignore
//! use crate::driver;
//!
//! // Initialize the global driver (call once at startup)
//! driver::init_global_driver();
//! ```
//!
//! ### Setup channels
//!
//! ```ignore
//! // Initialize a channel
//! driver::with_driver(|driver| {
//!     driver.init_channel(0);  // Initialize channel 0
//! });
//! ```
//!
//! ### Use channels with effects
//!
//! ```ignore
//! use led_effects::{Generator, Pulse, Static};
//!
//! let mut effect = Pulse { /* ... */ };
//!
//! driver::with_driver(|driver| {
//!     if let Some(channel) = driver.channel_mut(0) {
//!         // Write directly to hardware buffer - zero copy!
//!         effect.generate(channel.buffer_mut(), now);
//!     }
//! });
//!
//! // Send data to LEDs via DMA
//! driver::with_driver(|driver| {
//!     driver.refresh();
//! });
//! ```

pub mod channel;
pub mod ffi;

// Re-export commonly used types and functions
pub use channel::{LedChannel, LedDriver};
pub use ffi::{LedChannelInfo, WS2812_NUM_CHANNELS};

// Re-export global driver functions
pub use channel::{init_global_driver, with_driver};
