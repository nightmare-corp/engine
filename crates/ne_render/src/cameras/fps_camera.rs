use std::f32::consts::FRAC_PI_2;

use instant::Duration;
use ne_math::{Vec3,to_radians, vec4, Mat4};

//----------------------------------------------------------------------------
//calculates view projection from quats, RT (rotation translation)
//----------------------------------------------------------------------------

#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: Mat4 = Mat4::from_cols(
    vec4(1.0, 0.0, 0.0, 0.0,),
    vec4(0.0, 1.0, 0.0, 0.0,),
    vec4(0.0, 0.0, 0.5, 0.0,),
    vec4(0.0, 0.0, 0.5, 1.0,),
);
const SAFE_FRAC_PI_2: f32 = FRAC_PI_2 - 0.0001;

// Defines several possible options for camera movement. Used as abstraction to stay away from window-system specific input methods
pub enum CameraMovement {
    FORWARD,
    BACKWARD,
    LEFT,
    RIGHT,
    UP,
    DOWN
}
//changes with window
pub struct Projection {
    aspect: f32,
    fov: f32,
    znear: f32,
    zfar: f32,
}
impl Default for Projection
{
    fn default() -> Self
    {
        Self {
            aspect: todo!(),
            fov: 75.0,
            znear: todo!(),
            zfar: todo!(),
        }
    }
}    
impl Projection{
    pub fn new<F: Into<f32>>(
        width: u32,
        height: u32,
        fov: F,
        znear: f32,
        zfar: f32,
    ) -> Self {
        Self {
            aspect: width as f32 / height as f32,
            fov: fov.into(),
            znear,
            zfar,
        }
    }
    pub fn resize(&mut self, width: u32, height: u32) {
        self.aspect = width as f32 / height as f32;
    }
    // pub fn calc_matrix(&self) -> Mat4 {
    //     OPENGL_TO_WGPU_MATRIX * perspective(self.fov, self.aspect,
    //          self.znear, self.zfar)
    // }
}

pub struct MyCamera
{
    // euler Angles
    pub yaw:f32  ,
    pub pitch:f32,
    pub speed:f32,
    pub mouse_sense:f32,
    //
    pub pos: Vec3,
    pub front: Vec3,
    pub up: Vec3,
    pub right: Vec3,
    pub world_up: Vec3,
}
// impl Default for MyCamera
// {
//     fn default() -> Self
//     {
//         Self {
//             world_up:(0.0,0.0,0.0).into(),
//             yaw:-90.0, pitch:0.0, speed:5.0,
//             mouse_sense: 1.0,

//             pos:(0.0,0.0,0.0).into(),front:(0.0,0.0,0.0).into(),
//             up:(0.0,0.0,1.0).into(),right:(0.0,0.0,0.0).into(),
//         }
//     }
// }
impl MyCamera{
    pub fn new(yaw: f32, pitch: f32, speed: f32, mouse_sense: f32,
         fov: f32, pos: Vec3, front: Vec3, up: Vec3, right: Vec3, world_up: Vec3) -> 
    Self { Self { yaw, pitch, speed, mouse_sense, pos, front, up, right, world_up } }

    fn process_keyboard(&mut self, direction:CameraMovement, dt:Duration)
    {
        let velocity = self.speed * dt.as_secs_f32();
        match direction
        {   
            CameraMovement::FORWARD => {self.pos += self.front * velocity},
            CameraMovement::BACKWARD => {self.pos -= self.front * velocity},

            CameraMovement::LEFT => {self.pos -= self.right * velocity},
            CameraMovement::RIGHT => {self.pos += self.right * velocity},

            //uses world up not character-relative up
            CameraMovement::UP => {self.pos += self.world_up * velocity},
            CameraMovement::DOWN => {self.pos -= self.world_up * velocity},
        }
    }
    //TODO OPTIMIZE see if this can be made faster
    // fn calc_matrix(&mut self) -> ne_math::Mat4
    // {
    //     // ne_math::Mat4::look_at_rh(self.pos, (self.pos+self.front), self.up)
    //     look_to_rh(
    //         self.position,
    //         Vector3::new(
    //             self.yaw.0.cos(),
    //             self.pitch.0.sin(),
    //             self.yaw.0.sin(),
    //         ).normalize(),
    //         Vector3::unit_y(),
    //     )

    // }
    fn update_camera(&mut self)
    {
        // calculate the new Front vector..
        let mut front:Vec3 = Vec3::ZERO;
        front.x = to_radians(self.yaw).sin() * to_radians(self.pitch).cos();
        front.y = to_radians(self.yaw).cos() * to_radians(self.pitch).cos();
        front.z = to_radians(self.pitch).sin();
        // TODO Will panic if `self` is zero length when `glam_assert` is enabled. .normalize() is dangerous?
        self.front = front.normalize();

        // also re-calculate the Right and Up vector
        self.right = (self.front.cross(self.world_up)).normalize();  // normalize the vectors, because their length gets closer to 0 the more you look up or down which results in slower movement.
        self.up = (self.right.cross(self.front).normalize());

    }
}
