use ne_math::{Mat4, vec4};
use winit::event::{ElementState, KeyboardInput, VirtualKeyCode, WindowEvent};

#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: Mat4 = Mat4::from_cols(
    vec4(1.0, 0.0, 0.0, 0.0,),
    vec4(0.0, 1.0, 0.0, 0.0,),
    vec4(0.0, 0.0, 0.5, 0.0,),
    vec4(0.0, 0.0, 0.5, 1.0,),
);

pub struct CameraFields {
    pub pos: ne_math::Vec3,
    pub target: ne_math::Vec3,
    pub up: ne_math::Vec3,
    pub aspect: f32,
    pub fovy: f32,
    pub znear: f32,
    pub zfar: f32,
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
}
impl Camera {
    pub fn new(camera_fields: CameraFields) -> Self {
        Self { pos: camera_fields.pos, target: camera_fields.target, up: camera_fields.up,
             aspect: camera_fields.aspect, fovy: camera_fields.fovy, znear: camera_fields.znear, zfar: camera_fields.zfar }
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
//TODO separate projection
/* pub struct Projection {
    aspect: f32,
    fovy: f32,
    znear: f32,
    zfar: f32,
}

//split projection from camera
impl Projection {
    pub fn new(
        width: u32,
        height: u32,
        //TODO fovy ->F
        fovy: f32,
        znear: f32,
        zfar: f32,
    ) -> Self {
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
    pub fn set_aspect(&mut self, aspect: f32) {
        self.aspect = aspect;
    }
} */
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
                    VirtualKeyCode::Space => {
                        self.is_up_pressed = is_pressed;
                        true
                    }
                    VirtualKeyCode::LShift => {
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

    pub fn update_camera(&self, camera: &mut Camera, dt:instant::Duration) {
        //TODO CAMERA WASD TO MOVE
        //TODO MOUSE TO ROTATE

        let dt = dt.as_secs_f32();
        // Move forward/backward and left/right

        // let (yaw_sin, yaw_cos) = camera.yaw.0.sin_cos();
        // let forward = Vector3::new(yaw_cos, 0.0, yaw_sin).normalize();
        // let right = Vector3::new(-yaw_sin, 0.0, yaw_cos).normalize();
        // camera.position += forward * (self.amount_forward - self.amount_backward) * self.speed * dt;
        // camera.position += right * (self.amount_right - self.amount_left) * self.speed * dt;

        // Move in/out (aka. "zoom")
        // Note: this isn't an actual zoom. The camera's position
        // changes when zooming. I've added this to make it easier
        // to get closer to an object you want to focus on.

        // let (pitch_sin, pitch_cos) = camera.pitch.0.sin_cos();
        // let scrollward = Vector3::new(pitch_cos * yaw_cos, pitch_sin, pitch_cos * yaw_sin).normalize();
        // camera.position += scrollward * self.scroll * self.speed * self.sensitivity * dt;
        // self.scroll = 0.0;

        /* 
        // Move up/down. Since we don't use roll, we can just
        // modify the y coordinate directly.
        camera.position.y += (self.amount_up - self.amount_down) * self.speed * dt;

        // Rotate
        camera.yaw += Rad(self.rotate_horizontal) * self.sensitivity * dt;
        camera.pitch += Rad(-self.rotate_vertical) * self.sensitivity * dt;

        // If process_mouse isn't called every frame, these values
        // will not get set to zero, and the camera will rotate
        // when moving in a non cardinal direction.
        self.rotate_horizontal = 0.0;
        self.rotate_vertical = 0.0;

        // Keep the camera's angle from going too high/low.
        if camera.pitch < -Rad(SAFE_FRAC_PI_2) {
            camera.pitch = -Rad(SAFE_FRAC_PI_2);
        } else if camera.pitch > Rad(SAFE_FRAC_PI_2) {
            camera.pitch = Rad(SAFE_FRAC_PI_2);
        }
        */
        ///

        let forward = camera.target - camera.pos;
        let forward_norm = forward.normalize();
        let forward_mag = forward.length();

        // Prevents glitching when camera gets too close to the
        // center of the scene.
        if self.is_forward_pressed && forward_mag > self.speed {
            camera.pos += forward_norm * self.speed;
        }
        if self.is_backward_pressed {
            camera.pos -= forward_norm * self.speed;
        }
        let right = forward_norm.cross(camera.up);
//TODO I know:
//1. There is a value: model-view-projection(matrix) that gets fed into the shader.
//2. There is a function that changes the camera paramters on wasd key press.
// All I need to do is process the model-view-projection variable the right way...

        if self.is_right_pressed {
            // Rescale the distance between the target and eye so
            // that it doesn't change. The eye therefore still
            // lies on the circle made by the target and eye.
            camera.pos += right * self.speed;
        }
        if self.is_left_pressed {
            camera.pos -= right * self.speed;
        }


    // if (glfwGetKey(window, GLFW_KEY_A) == GLFW_PRESS)
    //     cameraPos -= glm::normalize(glm::cross(cameraFront, cameraUp)) * cameraSpeed;
    // if (glfwGetKey(window, GLFW_KEY_D) == GLFW_PRESS)
    //     cameraPos += glm::normalize(glm::cross(cameraFront, cameraUp)) * cameraSpeed;

    }
}
