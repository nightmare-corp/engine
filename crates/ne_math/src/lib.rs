//! Provides math types and functionality for the nightmare_engine. (yoinked from bevy.) thank you glam
//!
//! The commonly used types are vectors like [`Vec2`] and [`Vec3`],
//! matrices like [`Mat2`], [`Mat3`] and [`Mat4`] and orientation representations
//! like [`Quat`].

#![warn(missing_docs)]

/// The `bevy_math` prelude.
pub mod prelude {
    #[doc(hidden)]
    pub use crate::{
        BVec2, BVec3, BVec4, EulerRot, IVec2, IVec3, IVec4, Mat2, Mat3, Mat4, Quat, UVec2, UVec3,
        UVec4, Vec2, Vec3, Vec4,
    };
}
pub use glam::*;
///to radians from degrees
pub fn to_radians(deg: f32) -> f32
{
    deg * (std::f32::consts::PI/180.0)
}
///to degrees from radians
pub fn to_degrees(rad: f32) -> f32
{
    rad * (180.0/std::f32::consts::PI)
}

#[derive(Debug, Clone)]
pub struct Transform {
    pub pos: Vec3,
    pub rot: Quat,
}

impl Default for Transform {
    fn default() -> Self {
        Self { pos: Vec3::ZERO, rot: Quat::default() }
    }
}