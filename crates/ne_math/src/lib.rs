//! Provides math types and functionality for the nightmare_engine. (yoinked from bevy.) thank you glam
//!
//! The commonly used types are vectors like [`Vec2`] and [`Vec3`],
//! matrices like [`Mat2`], [`Mat3`] and [`Mat4`] and orientation representations
//! like [`Quat`].

/// The `bevy_math` prelude.
pub mod prelude {
    #[doc(hidden)]
    pub use crate::{
        BVec2, BVec3, BVec4, EulerRot, IVec2, IVec3, IVec4, Mat2, Mat3, Mat4, Quat, UVec2, UVec3,
        UVec4, Vec2, Vec3, Vec4,
    };
}
use bevy_ecs::prelude::Component;
pub use glam::*;
pub use rand;


///to radians from degrees
pub fn to_radians(deg: f32) -> f32 {
    deg * (std::f32::consts::PI / 180.0)
}
///to degrees from radians
pub fn to_degrees(rad: f32) -> f32 {
    rad * (180.0 / std::f32::consts::PI)
}

/// sometimes it's useful to quickly add/decrease a single unit of each value.
/// using the functions: ``add_one()`` and ``decrease_one()``
/// also possible to assign arbitrary values using ``randomize()``
pub trait QuickMath {
    fn add_one(&mut self);
    fn decrease_one(&mut self);
    fn randomize() -> Self;
}
//TODO scale.
#[derive(Debug, Clone, Component)]
pub struct Transform {
    pub pos: Vec3,
    pub rot: Quat,
}
impl Default for Transform {
    fn default() -> Self {
        Self {
            pos: Vec3::ZERO,
            rot: Quat::default(),
        }
    }
}
impl QuickMath for Transform {
    fn add_one(&mut self) {
        self.pos.add_one();
        self.rot.add_one();
    }
    fn decrease_one(&mut self) {
        self.pos.decrease_one();
        self.rot.decrease_one();
    }

    fn randomize() -> Self {
        todo!()
    }
}
impl QuickMath for Vec3 {
    fn add_one(&mut self) {
        self.x+=1.0; self.y+=1.0; self.z+=1.0;
    }
    fn decrease_one(&mut self) {
        self.x-=1.0; self.y-=1.0; self.z-=1.0;
    }

    fn randomize() -> Self {
        todo!()
    }
}

impl QuickMath for Quat {
    //TODO 
    fn add_one(&mut self) {
        todo!();
        // self.x+=1.0; self.y+=1.0; self.z+=1.0;
    }
    fn decrease_one(&mut self) {
        todo!();
        // self.x-=1.0; self.y-=1.0; self.z-=1.0;
    }
    fn randomize() -> Self {
        todo!()
    }
}
impl QuickMath for f32 {
    fn add_one(&mut self) {
    }
    fn decrease_one(&mut self) {
    }
    fn randomize() -> Self {
        rand::random::<f32>()
    }
}