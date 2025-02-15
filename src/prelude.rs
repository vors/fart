//! The most common functionality re-exported.

pub use crate::{
    canvas::{Canvas, CanvasSpace, Inches, Millis},
    path::{LineCommand, Path, ToPaths, ToPathsExt},
    process::Process,
    user_const, Config,
};
pub use euclid::{point2, vec2};
pub use fart_aabb::{Aabb, ToAabb};
pub use fart_utils::{clamp, map_range};
pub use lazy_static::lazy_static;
pub use noise::NoiseFn;
pub use rand::{
    distributions::{Distribution, Normal, Uniform},
    Rng,
};
