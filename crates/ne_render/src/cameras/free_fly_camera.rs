use ne_app::Resource;
// use cgmath::*;
use ne_math::{vec4, Mat4, Vec3};
use winit::dpi::PhysicalPosition;
use winit::event::*;

use super::camera_helper::{OPENGL_TO_WGPU_MATRIX, SAFE_FRAC_PI_2};
#[derive(Debug, Resource)]
pub struct Camera {
    pub position: Vec3,
    yaw: f32,
    pitch: f32,
}
impl Camera {
    pub fn new(position: Vec3, yaw: f32, pitch: f32) -> Self {
        Self {
            position: position,
            yaw: yaw,
            pitch: pitch,
        }
    }
    pub fn calc_matrix(&self) -> Mat4 {
        let (sin_pitch, cos_pitch) = self.pitch.sin_cos();
        let (sin_yaw, cos_yaw) = self.yaw.sin_cos();

        //this be weird
        look_to_rh(
            self.position,
            Vec3::new(cos_pitch * cos_yaw, sin_pitch, cos_pitch * sin_yaw).normalize(),
            Vec3::Y,
        )
    }
}
pub fn look_to_rh(eye: Vec3, dir: Vec3, up: Vec3) -> Mat4 {
    let f = dir.normalize();
    let s = f.cross(up).normalize();
    let u = s.cross(f);
    // #[cfg_attr(rustfmt, rustfmt_skip)]

    //does this automatically have uhhh simd optimizations?
    Mat4::from_cols(
        vec4(s.x, u.x, -f.x, 0.0),
        vec4(s.y, u.y, -f.y, 0.0),
        vec4(s.z, u.z, -f.z, 0.0),
        vec4(-eye.dot(s), -eye.dot(u), eye.dot(f), 1.0),
    )
}
#[derive(Resource)]
pub struct Projection {
    aspect: f32,
    fovy: f32,
    znear: f32,
    zfar: f32,
}
impl Projection {
    pub fn new(width: u32, height: u32, fovy: f32, znear: f32, zfar: f32) -> Self {
        Self {
            aspect: width as f32 / height as f32,
            fovy: fovy.into(),
            znear,
            zfar,
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.aspect = width as f32 / height as f32;
    }

    pub fn calc_matrix(&self) -> Mat4 {
        OPENGL_TO_WGPU_MATRIX
            * Mat4::perspective_rh_gl(self.fovy, self.aspect, self.znear, self.zfar)
    }
}
#[derive(Debug, Resource)]
pub struct CameraController {
    pub amount_left: f32,
    pub amount_right: f32,
    pub amount_forward: f32,
    pub amount_backward: f32,
    pub amount_up: f32,
    pub amount_down: f32,
    rotate_horizontal: f32,
    rotate_vertical: f32,
    scroll: f32,
    speed: f32,
    sensitivity: f32,
}
impl CameraController {
    pub fn new(speed: f32, sensitivity: f32) -> Self {
        Self {
            amount_left: 0.0,
            amount_right: 0.0,
            amount_forward: 0.0,
            amount_backward: 0.0,
            amount_up: 0.0,
            amount_down: 0.0,
            rotate_horizontal: 0.0,
            rotate_vertical: 0.0,
            scroll: 0.0,
            speed,
            sensitivity,
        }
    }
/*     
    pub fn process_keyboard(&mut self, key: VirtualKeyCode, state: ElementState) -> bool {
       let amount = if state == ElementState::Pressed {
            1.0
        } else {
            0.0
        };
        match key {
            VirtualKeyCode::W | VirtualKeyCode::Up => {
                self.amount_forward = amount;
                true
            }
            VirtualKeyCode::S | VirtualKeyCode::Down => {
                self.amount_backward = amount;
                true
            }
            VirtualKeyCode::A | VirtualKeyCode::Left => {
                self.amount_left = amount;
                true
            }
            VirtualKeyCode::D | VirtualKeyCode::Right => {
                self.amount_right = amount;
                true
            }
            //up
            VirtualKeyCode::E => {
                self.amount_up = amount;
                true
            }
            //down
            VirtualKeyCode::Q => {
                self.amount_down = amount;
                true
            }
            _ => false,
        }
    }
     */ 
    pub fn process_mouse(&mut self, mouse_dx: f32, mouse_dy: f32) {
        self.rotate_horizontal += mouse_dx;
        self.rotate_vertical += mouse_dy;
    }
    pub fn process_scroll(&mut self, delta: &MouseScrollDelta) {
        let a = match delta {
            MouseScrollDelta::LineDelta(_, scroll) => -scroll * -2.0,
            MouseScrollDelta::PixelDelta(PhysicalPosition { y: scroll, .. }) => -*scroll as f32,
        };
        //cleanable
        if self.speed + a > 1.0 && self.speed + a < 25.0 {
            self.speed += a;
        }
    }
    pub fn update_camera(&mut self, camera: &mut Camera, dt: f32) {
        // Move forward/backward and left/right
        let (yaw_sin, yaw_cos) = camera.yaw.sin_cos();
        let forward = Vec3::new(yaw_cos, 0.0, yaw_sin).normalize();
        let right = Vec3::new(-yaw_sin, 0.0, yaw_cos).normalize();
        camera.position += forward * (self.amount_forward - self.amount_backward) * self.speed * dt;
        camera.position += right * (self.amount_right - self.amount_left) * self.speed * dt;

        // Move in/out (aka. "zoom")
        // Note: this isn't an actual zoom. The camera's position
        // changes when zooming. I've added this to make it easier
        // to get closer to an object you want to focus on.
        let (pitch_sin, pitch_cos) = camera.pitch.sin_cos();
        let scrollward = Vec3::new(pitch_cos * yaw_cos, pitch_sin, pitch_cos * yaw_sin).normalize();
        camera.position += scrollward * self.scroll * self.speed * self.sensitivity * dt;
        self.scroll = 0.0;
        // Move up/down. Since we don't use roll, we can just
        // modify the y coordinate directly.
        camera.position.y += (self.amount_up - self.amount_down) * self.speed * dt;

        // Rotate
        // 0.0013188
        // 0.0167976
        // Need to enable camera smoothing for low fps... somehow...
        camera.yaw += self.rotate_horizontal * self.sensitivity/*  * dt */;
        camera.pitch += -self.rotate_vertical * self.sensitivity/*  * dt */;
        //also this implementation does not make sense, with delta time.
        //because we need the total rotation to always be the same after the same amount of time.
        //but here we would only rotate just for a fraction (total_rotation/dt) ... 
        //resetting the rotate_horizontal and rotate_vertical is wrong, 
        //it needs to slowly eaten, bite size related to delta time.
        self.rotate_horizontal = 0.0;
        self.rotate_vertical = 0.0;

        //new implementation attempt:
        // let bite_size_multiplier = self.sensitivity * dt;
        // camera.yaw += self.rotate_horizontal * self.sensitivity/*  * dt */;
        // camera.pitch += -self.rotate_vertical * self.sensitivity/*  * dt */;
        // self.rotate_horizontal -= bite_size_multiplier;
        // self.rotate_vertical -= bite_size_multiplier;

        // keep the camera angle from going too high/low
        if camera.pitch < -SAFE_FRAC_PI_2 {
            camera.pitch = -SAFE_FRAC_PI_2;
        } else if camera.pitch > SAFE_FRAC_PI_2 {
            camera.pitch = SAFE_FRAC_PI_2;
        }
    }
}

#[repr(C)]
#[derive(Debug, Resource, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    //TODO
    view_position: [f32; 4], //TODO implement view position -> maybe normals and lights
    view_proj: [[f32; 4]; 4],
}
impl Default for CameraUniform {
    fn default() -> Self {
        Self {
            view_position: [0.0; 4],
            // view_proj: cgmath::Matrix4::identity().into(),
            view_proj: Mat4::IDENTITY.to_cols_array_2d(),
        }
    }
}
impl CameraUniform {
    // pub(crate) fn new(view_position: [f32; 4], view_proj: [[f32; 4]; 4]) -> Self { Self { view_position, view_proj } }
    // UPDATED!
    pub fn update_view_proj(&mut self, camera: &Camera, projection: &Projection) {
        self.view_position = camera.position.extend(1.0).into();
        self.view_proj = (projection.calc_matrix() * camera.calc_matrix()).to_cols_array_2d();
    }
}
