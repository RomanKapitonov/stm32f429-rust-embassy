pub mod complex;
pub mod movement;
pub mod static_patterns;

// Re-export generators
pub use movement::{Chase, Pulse};
pub use static_patterns::{Gradient, SolidColor, Stripes};
