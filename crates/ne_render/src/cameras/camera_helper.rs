use std::f32::consts::FRAC_PI_2;

use ne_math::{Mat4, vec4};


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
