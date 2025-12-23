//! # Envelope Combinators
//!
//! This module provides combinators for composing complex envelopes from simpler ones.
//! Combinators allow you to create sophisticated control curves by combining basic
//! envelope shapes using mathematical operations.
//!
//! ## Overview
//!
//! Envelope combinators fall into two categories:
//!
//! ### Binary Combinators
//!
//! These combine two envelopes using mathematical operations:
//! - [`Product`] - Multiplies two envelopes (useful for amplitude modulation)
//! - [`Sum`] - Adds two envelopes together (clamped to 1.0)
//! - [`Min`] - Takes the minimum of two envelopes
//! - [`Max`] - Takes the maximum of two envelopes
//!
//! ### Unary Combinators
//!
//! These transform a single envelope:
//! - [`Invert`] - Inverts an envelope (1.0 - value)
//! - [`Clamp`] - Constrains envelope values to a range
//! - [`Scale`] - Maps envelope values from one range to another
//!
//! ## Examples
//!
//! ### Creating a Pulsing Effect
//!
//! Multiply a fade-in envelope with a sine wave to create a pulsing fade-in:
//!
//! ```ignore
//! use crate::effects::envelopes::{Fade, Sine};
//! use crate::effects::envelopes::combinator::Product;
//!
//! let pulse_fade = Product {
//!     env1: Fade { start_time: 0, period: 2000, inverted: false },
//!     env2: Sine { start_time: 0, period: 100 },
//! };
//! ```
//!
//! ### Clamping Values
//!
//! Prevent an envelope from going below 0.2 (minimum brightness):
//!
//! ```ignore
//! use crate::effects::envelopes::Triangle;
//! use crate::effects::envelopes::combinator::Clamp;
//!
//! let clamped_triangle = Clamp {
//!     inner: Triangle { start_time: 0, period: 1000 },
//!     min: 0.2,
//!     max: 1.0,
//! };
//! ```
//!
//! ### Complex Compositions
//!
//! Chain multiple combinators together:
//!
//! ```ignore
//! use crate::effects::envelopes::{Fade, Sine};
//! use crate::effects::envelopes::combinator::{Product, Scale};
//!
//! let complex = Scale {
//!     inner: Product {
//!         env1: Fade { start_time: 0, period: 2000, inverted: false },
//!         env2: Sine { start_time: 0, period: 200 },
//!     },
//!     from_min: 0.0,
//!     from_max: 1.0,
//!     to_min: 0.5,    // Never go below 50%
//!     to_max: 1.0,
//! };
//! ```

use crate::effects::Envelope;

/// Multiplies two envelopes together (amplitude modulation).
///
/// The output value is the product of both input envelopes. This is useful for
/// creating amplitude modulation effects, where one envelope controls the overall
/// shape and another modulates it.
///
/// # Lifetime
///
/// The product is alive only when both envelopes are alive (AND operation).
///
/// # Examples
///
/// ```ignore
/// use crate::effects::envelopes::{Fade, Triangle};
/// use crate::effects::envelopes::combinator::Product;
///
/// // Create a triangle wave that fades in
/// let modulated = Product {
///     env1: Fade { start_time: 0, period: 1000, inverted: false },
///     env2: Triangle { start_time: 0, period: 200 },
/// };
/// ```
///
/// # Mathematical Operation
///
/// ```text
/// output(t) = env1(t) × env2(t)
/// ```
pub struct Product<E1: Envelope, E2: Envelope> {
    pub env1: E1,
    pub env2: E2,
}

/// Adds two envelopes together (clamped to 1.0).
///
/// The output is the sum of both input envelopes, clamped to a maximum of 1.0.
/// This is useful for layering effects or combining multiple control sources.
///
/// # Lifetime
///
/// The sum is alive when either envelope is alive (OR operation).
///
/// # Examples
///
/// ```ignore
/// use crate::effects::envelopes::{Fade, Triangle};
/// use crate::effects::envelopes::combinator::Sum;
///
/// // Combine a slow fade with a fast oscillation
/// let combined = Sum {
///     env1: Fade { start_time: 0, period: 2000, inverted: false },
///     env2: Triangle { start_time: 0, period: 100 },
/// };
/// ```
///
/// # Mathematical Operation
///
/// ```text
/// output(t) = min(env1(t) + env2(t), 1.0)
/// ```
pub struct Sum<E1: Envelope, E2: Envelope> {
    pub env1: E1,
    pub env2: E2,
}

/// Takes the minimum value of two envelopes at each point in time.
///
/// The output is always the smaller of the two envelope values. This can be used
/// to create "gating" effects where one envelope limits another.
///
/// # Lifetime
///
/// The min is alive when either envelope is alive (OR operation).
///
/// # Examples
///
/// ```ignore
/// use crate::effects::envelopes::{Fade, Constant};
/// use crate::effects::envelopes::combinator::Min;
///
/// // Fade in but never exceed 70% brightness
/// let limited_fade = Min {
///     env1: Fade { start_time: 0, period: 1000, inverted: false },
///     env2: Constant,  // Returns 0.7 when scaled appropriately
/// };
/// ```
///
/// # Mathematical Operation
///
/// ```text
/// output(t) = min(env1(t), env2(t))
/// ```
pub struct Min<E1: Envelope, E2: Envelope> {
    pub env1: E1,
    pub env2: E2,
}

/// Takes the maximum value of two envelopes at each point in time.
///
/// The output is always the larger of the two envelope values. This ensures
/// a minimum brightness/intensity even when one envelope is low.
///
/// # Lifetime
///
/// The max is alive when either envelope is alive (OR operation).
///
/// # Examples
///
/// ```ignore
/// use crate::effects::envelopes::{Fade, Sine};
/// use crate::effects::envelopes::combinator::Max;
///
/// // Oscillate but maintain minimum brightness from fade
/// let bright_oscillation = Max {
///     env1: Fade { start_time: 0, period: 1000, inverted: false },
///     env2: Sine { start_time: 0, period: 200 },
/// };
/// ```
///
/// # Mathematical Operation
///
/// ```text
/// output(t) = max(env1(t), env2(t))
/// ```
pub struct Max<E1: Envelope, E2: Envelope> {
    pub env1: E1,
    pub env2: E2,
}

/// Inverts an envelope (flips it upside down).
///
/// The output is `1.0 - input`, effectively reversing the envelope's values.
/// This is useful for creating inverse effects or "negative" envelopes.
///
/// # Lifetime
///
/// The inverted envelope has the same lifetime as the inner envelope.
///
/// # Examples
///
/// ```ignore
/// use crate::effects::envelopes::Fade;
/// use crate::effects::envelopes::combinator::Invert;
///
/// // Create a fade-out by inverting a fade-in
/// let fade_out = Invert {
///     inner: Fade { start_time: 0, period: 1000, inverted: false },
/// };
/// // This is equivalent to: Fade { inverted: true }
/// ```
///
/// # Mathematical Operation
///
/// ```text
/// output(t) = 1.0 - input(t)
/// ```
pub struct Invert<E: Envelope> {
    pub inner: E,
}

/// Constrains envelope values to a specified range.
///
/// Values below `min` are raised to `min`, and values above `max` are lowered
/// to `max`. This is useful for preventing extreme values or maintaining minimum
/// brightness/intensity levels.
///
/// # Lifetime
///
/// The clamped envelope has the same lifetime as the inner envelope.
///
/// # Examples
///
/// ```ignore
/// use crate::effects::envelopes::Sine;
/// use crate::effects::envelopes::combinator::Clamp;
///
/// // Oscillate between 30% and 80% brightness
/// let limited_sine = Clamp {
///     inner: Sine { start_time: 0, period: 500 },
///     min: 0.3,
///     max: 0.8,
/// };
/// ```
///
/// # Mathematical Operation
///
/// ```text
/// output(t) = clamp(input(t), min, max)
///           = max(min, min(input(t), max))
/// ```
pub struct Clamp<E: Envelope> {
    /// The envelope to clamp
    pub inner: E,
    /// Minimum output value
    pub min: f32,
    /// Maximum output value
    pub max: f32,
}

/// Maps envelope values from one range to another (linear scaling).
///
/// This combinator performs linear interpolation to remap values from an input
/// range `[from_min, from_max]` to an output range `[to_min, to_max]`. It's
/// particularly useful for:
/// - Converting normalized (0-1) envelopes to specific value ranges
/// - Shifting envelope baselines
/// - Inverting while scaling (by swapping to_min and to_max)
///
/// # Lifetime
///
/// The scaled envelope has the same lifetime as the inner envelope.
///
/// # Examples
///
/// ## Basic Scaling
///
/// ```ignore
/// use crate::effects::envelopes::Triangle;
/// use crate::effects::envelopes::combinator::Scale;
///
/// // Map a 0-1 triangle wave to 50-100 range
/// let scaled_triangle = Scale {
///     inner: Triangle { start_time: 0, period: 1000 },
///     from_min: 0.0,
///     from_max: 1.0,
///     to_min: 50.0,
///     to_max: 100.0,
/// };
/// ```
///
/// ## Inverting While Scaling
///
/// ```ignore
/// use crate::effects::envelopes::Fade;
/// use crate::effects::envelopes::combinator::Scale;
///
/// // Create an inverted fade (fade-out) scaled to 0.2-0.8
/// let inverted_fade = Scale {
///     inner: Fade { start_time: 0, period: 1000, inverted: false },
///     from_min: 0.0,
///     from_max: 1.0,
///     to_min: 0.8,  // Swapped: high value maps to input's low
///     to_max: 0.2,  // Swapped: low value maps to input's high
/// };
/// ```
///
/// # Mathematical Operation
///
/// ```text
/// normalized(t) = (input(t) - from_min) / (from_max - from_min)
/// output(t) = to_min + normalized(t) × (to_max - to_min)
/// ```
pub struct Scale<E: Envelope> {
    /// The envelope to scale
    pub inner: E,
    /// Minimum value of the input range (typically 0.0)
    pub from_min: f32,
    /// Maximum value of the input range (typically 1.0)
    pub from_max: f32,
    /// Minimum value of the output range
    pub to_min: f32,
    /// Maximum value of the output range
    pub to_max: f32,
}

impl<E1: Envelope, E2: Envelope> Envelope for Product<E1, E2> {
    fn sample(&self, now: u64) -> f32 {
        self.env1.sample(now) * self.env2.sample(now)
    }

    fn is_alive(&self, now: u64) -> bool {
        self.env1.is_alive(now) && self.env2.is_alive(now)
    }
}

impl<E1: Envelope, E2: Envelope> Envelope for Sum<E1, E2> {
    fn sample(&self, now: u64) -> f32 {
        (self.env1.sample(now) + self.env2.sample(now)).min(1.0)
    }

    fn is_alive(&self, now: u64) -> bool {
        self.env1.is_alive(now) || self.env2.is_alive(now)
    }
}

impl<E1: Envelope, E2: Envelope> Envelope for Min<E1, E2> {
    fn sample(&self, now: u64) -> f32 {
        self.env1.sample(now).min(self.env2.sample(now))
    }

    fn is_alive(&self, now: u64) -> bool {
        self.env1.is_alive(now) || self.env2.is_alive(now)
    }
}

impl<E1: Envelope, E2: Envelope> Envelope for Max<E1, E2> {
    fn sample(&self, now: u64) -> f32 {
        self.env1.sample(now).max(self.env2.sample(now))
    }

    fn is_alive(&self, now: u64) -> bool {
        self.env1.is_alive(now) || self.env2.is_alive(now)
    }
}

impl<E: Envelope> Envelope for Invert<E> {
    fn sample(&self, now: u64) -> f32 {
        1.0 - self.inner.sample(now)
    }

    fn is_alive(&self, now: u64) -> bool {
        self.inner.is_alive(now)
    }
}

impl<E: Envelope> Envelope for Clamp<E> {
    fn sample(&self, now: u64) -> f32 {
        self.inner.sample(now).clamp(self.min, self.max)
    }

    fn is_alive(&self, now: u64) -> bool {
        self.inner.is_alive(now)
    }
}

impl<E: Envelope> Envelope for Scale<E> {
    fn sample(&self, now: u64) -> f32 {
        let value = self.inner.sample(now);

        // Normalize to 0-1 based on input range
        let normalized = (value - self.from_min) / (self.from_max - self.from_min);

        // Map to output range
        self.to_min + normalized * (self.to_max - self.to_min)
    }

    fn is_alive(&self, now: u64) -> bool {
        self.inner.is_alive(now)
    }
}
