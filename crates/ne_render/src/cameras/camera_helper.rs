use std::f32::consts::FRAC_PI_2;

use ne_math::{Mat4, vec4, Vec3};


#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: Mat4 = Mat4::from_cols(
    vec4(1.0, 0.0, 0.0, 0.0,),
    vec4(0.0, 1.0, 0.0, 0.0,),
    vec4(0.0, 0.0, 0.5, 0.0,),
    vec4(0.0, 0.0, 0.5, 1.0,),
);
pub const SAFE_FRAC_PI_2: f32 = FRAC_PI_2 - 0.0001;
#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct CameraUniform {
    view_proj: [[f32; 4]; 4],
}

//Wonder if glam-rs has this... it should have it I think
/// Create a homogeneous transformation matrix that will cause a vector to point at
/// `dir`, using `up` for orientation.
pub fn look_to_rh(eye: Vec3, dir: Vec3, up: Vec3) -> Mat4 {
    let f = dir.normalize();
    let s = f.cross(up).normalize();
    let u = s.cross(f);
    // #[cfg_attr(rustfmt, rustfmt_skip)]

    //does this automatically have uhhh simd optimizations?
    Mat4::from_cols(
    vec4(s.x, u.x, -f.x, 0.0,),
    vec4(s.y, u.y, -f.y, 0.0,),
    vec4(s.z, u.z, -f.z, 0.0,),
    vec4(-eye.dot(s), -eye.dot(u), eye.dot(f), 1.0,))
}