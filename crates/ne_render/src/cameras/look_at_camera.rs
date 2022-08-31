use std::f32::consts::FRAC_PI_2;

use ne_math::{Mat4, vec4};
use winit::{event::{ElementState, KeyboardInput, VirtualKeyCode, WindowEvent, MouseScrollDelta}, dpi::PhysicalPosition};

#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: Mat4 = Mat4::from_cols(
    vec4(1.0, 0.0, 0.0, 0.0,),
    vec4(0.0, 1.0, 0.0, 0.0,),
    vec4(0.0, 0.0, 0.5, 0.0,),
    vec4(0.0, 0.0, 0.5, 1.0,),
);
const SAFE_FRAC_PI_2: f32 = FRAC_PI_2 - 0.0001;

pub struct CameraFields {
    pub pos: ne_math::Vec3,
    pub target: ne_math::Vec3,
    pub up: ne_math::Vec3,
    pub aspect: f32,
    pub fovy: f32,
    pub znear: f32,
    pub zfar: f32,

    pub yaw: f32,
    pub pitch: f32,
}
//TODO1 Why this NOT? :?
impl Default for CameraFields {
    fn default() -> Self {
        CameraFields {
            pos: (0.0, 30.0, 10.0).into(),
            target: (0.0, 0.0, 0.0).into(),
            up: (0.0, 1.0, 0.0).into(),
            aspect: 1.777,
            fovy: 45.0,
            znear: 0.1,
            zfar: 100.0,
            yaw:0.0,
            pitch:0.0,
        }
    }
}
pub struct Camera {
    //location of camera
    pos: ne_math::Vec3,
    target: ne_math::Vec3,
    up: ne_math::Vec3,
    aspect: f32,
    fovy: f32,
    znear: f32,
    zfar: f32,

    yaw: f32,
    pitch: f32,
}
impl Camera {
    pub fn new(camera_fields: CameraFields) -> Self {
        Self { pos: camera_fields.pos, target: camera_fields.target, up: camera_fields.up,
             aspect: camera_fields.aspect, fovy: camera_fields.fovy, znear: camera_fields.znear, zfar: camera_fields.zfar,
            yaw: camera_fields.yaw, pitch: camera_fields.pitch }
    }
    fn build_view_projection_matrix(&self) -> Mat4 {
        //TODO rh->lh
        let view = ne_math::Mat4::look_at_rh(self.pos, self.target, self.up);
        let proj = ne_math::Mat4::perspective_rh(self.fovy, self.aspect, self.znear, self.zfar);
        proj * view
    }
    pub fn set_aspect(&mut self, aspect: f32) {
        self.aspect = aspect;
    }
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    view_proj: [[f32; 4]; 4],
}

impl CameraUniform {
    pub fn new() -> Self {
        Self {
            view_proj: Mat4::IDENTITY.to_cols_array_2d(),
        }
    }
    pub fn update_view_proj(&mut self, camera: &Camera) {
        self.view_proj = (OPENGL_TO_WGPU_MATRIX * camera.build_view_projection_matrix()).to_cols_array_2d();
    }
}

pub struct CameraController {
    speed: f32,
    is_up_pressed: bool,
    is_down_pressed: bool,
    is_forward_pressed: bool,
    is_backward_pressed: bool,
    is_left_pressed: bool,
    is_right_pressed: bool,

    rotate_horizontal: f32,
    rotate_vertical: f32,

    mouse_sensitivity: f32,

}

impl CameraController {
    pub fn new(speed: f32) -> Self {
        Self {
            speed,
            is_up_pressed: false,
            is_down_pressed: false,
            is_forward_pressed: false,
            is_backward_pressed: false,
            is_left_pressed: false,
            is_right_pressed: false,

            rotate_horizontal: 0.0,
            rotate_vertical: 0.0,

            mouse_sensitivity: 7.0,
        }
    }

    pub fn process_events(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        state,
                        virtual_keycode: Some(keycode),
                        ..
                    },
                ..
            } => {
                let is_pressed = *state == ElementState::Pressed;
                match keycode {
                    VirtualKeyCode::Q => {
                        self.is_up_pressed = is_pressed;
                        true
                    }
                    VirtualKeyCode::E => {
                        self.is_down_pressed = is_pressed;
                        true
                    }
                    VirtualKeyCode::W | VirtualKeyCode::Up => {
                        self.is_forward_pressed = is_pressed;
                        true
                    }
                    VirtualKeyCode::A | VirtualKeyCode::Left => {
                        self.is_left_pressed = is_pressed;
                        true
                    }
                    VirtualKeyCode::S | VirtualKeyCode::Down => {
                        self.is_backward_pressed = is_pressed;
                        true
                    }
                    VirtualKeyCode::D | VirtualKeyCode::Right => {
                        self.is_right_pressed = is_pressed;
                        true
                    }
                    _ => false,
                }
            }
            _ => false,
        }
    }
    pub fn update_camera(&mut self, camera: &mut Camera, dt:instant::Duration) {
        //TODO CAMERA WASD TO MOVE
        //TODO MOUSE TO ROTATE

        let dt = dt.as_secs_f32();
        //MOUSE
        // calculate yaw and pitch
        camera.yaw += self.rotate_horizontal * self.mouse_sensitivity * dt;
        camera.pitch += -self.rotate_vertical * self.mouse_sensitivity * dt;

        //|TODOrecalculate look at with rotation
        // camera.target 

                // If process_mouse isn't called every frame, these values
        // will not get set to zero, and the camera will rotate
        // when moving in a non cardinal direction.
        self.rotate_horizontal = 0.0;
        self.rotate_vertical = 0.0;
        // Keep the camera's angle from going too high/low.
        if camera.pitch < SAFE_FRAC_PI_2 {
            camera.pitch = SAFE_FRAC_PI_2;
        } else if camera.pitch >SAFE_FRAC_PI_2 {
            camera.pitch = SAFE_FRAC_PI_2;
        }
        let mut forward = camera.target - camera.pos;
        //TODO somehow calculate forward from yaw and pitch
        forward.x = camera.yaw.sin() * camera.pitch.cos();
        forward.y = camera.yaw.cos() * camera.pitch.cos();
        forward.z = camera.pitch.sin(); 
        let forward_norm = forward.normalize();
        let forward_mag = forward.length();

        //KEYS
        // prevents glitching when camera gets too close to the
        // center of the scene.
        if self.is_forward_pressed /* && forward_mag > self.speed */ {
            camera.pos += forward_norm * self.speed* dt;
        }
        if self.is_backward_pressed {
            camera.pos -= forward_norm * self.speed* dt;
        }
        // Move forward/backward and left/right
        if self.is_right_pressed {
            let right = forward_norm.cross(camera.up);
            camera.pos += right * self.speed* dt;
            camera.target = camera.pos + forward;
        }
        if self.is_left_pressed {
            let right = forward_norm.cross(camera.up);

            camera.pos -= right * self.speed* dt;
        camera.target = camera.pos + forward;
        }
        //move up and down
        if self.is_up_pressed {
            camera.pos +=  camera.up * self.speed* dt;
        camera.target = camera.pos + forward;

        }
        if self.is_down_pressed {
            camera.pos -=  camera.up * self.speed* dt;
            camera.target = camera.pos + forward;

        }
    }
    pub fn process_scroll(&mut self, delta: &MouseScrollDelta) {
        // self.scroll = match delta {
        //     // I'm assuming a line is about 100 pixels
        //     MouseScrollDelta::LineDelta(_, scroll) => -scroll * 0.5,
        //     MouseScrollDelta::PixelDelta(PhysicalPosition { y: scroll, .. }) => -*scroll as f32,
        // };

        let a = match delta {
            MouseScrollDelta::LineDelta(_, scroll) => -scroll*-2.0,
            MouseScrollDelta::PixelDelta(PhysicalPosition {y: scroll, ..}) => -*scroll as f32,
        };
        //cleanable
        if self.speed +a> 1.0 && self.speed +a < 25.0
{            self.speed +=a;
}

    }

    //TODO need to use mouse_dx and mouse_dy, multiply by yaw and pitch
    pub fn process_mouse(&mut self, mouse_dx: f64, mouse_dy: f64) {
        self.rotate_horizontal = mouse_dx as f32;
        self.rotate_vertical = mouse_dy as f32;


    }


}
