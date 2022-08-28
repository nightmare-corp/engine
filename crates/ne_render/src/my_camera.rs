// use ne_math::Vec3;

// use crate::camera::Camera;

// // Defines several possible options for camera movement. Used as abstraction to stay away from window-system specific input methods
// pub enum Camera_Movement {
//     FORWARD,
//     BACKWARD,
//     LEFT,
//     RIGHT,
//     UP,
//     DOWN
// }

// struct camera
// {
//     // euler Angles
//     pub yaw:f32  ,
//     pub pitch:f32,
//     pub speed:f32,
//     pub mouse_sense:f32,
//     pub fov:f32  ,
//     //
//     pub pos: Vec3,
//     pub front: Vec3,
//     pub up: Vec3,
//     pub right: Vec3,
//     pub world_up: Vec3,
// }

// impl Default for camera
// {
//     fn default() -> Self
//     {
//         Self {
//             world_up:(0.0,0.0,0.0).into(),
//             yaw:-90.0, pitch:0.0, speed:5.0,fov:75.0,
//             mouse_sense: 1.0,

//             pos:(0.0,0.0,0.0).into(),front:(0.0,0.0,0.0).into(),
//             up:(0.0,0.0,1.0).into(),right:(0.0,0.0,0.0).into(),
//         }
//     }
// }
// impl Camera{
//     fn get_view_matrix() -> ne_math::Mat4
//     {

//     }
//     fn update_camera()
//     {
//         Self.front = glm::radians(Yaw).sin * cos(glm::radians(Pitch));
//     }
// }
