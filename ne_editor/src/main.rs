#![allow(warnings, unused)]

use std::collections::HashMap;
use std::thread::Thread;

use bevy_ecs::prelude::{EventReader, EventWriter, Component};
use bevy_ecs::system::{Res, ResMut, NonSend, Commands, Query};
use bevy_derive::Deref;
use ne::L::LogPlugin;
use ne_app::types::Name;
use ne_app::{App, Plugin, Resource};
use ne_bench::thread::println_current_thread_id;
use ne_math::{Vec3, Transform, Quat};
use ne_render::cameras::free_fly_camera::{self, Camera, CameraUniform, Projection, CameraController};
use ne_render::material::{Material, NamedMaterial};
use ne_render::mesh::{StaticMesh, GpuMesh, MeshPrimitives, NamedGpuMesh};
use ne_render::render_structs::{RenderDevice, RenderQueue};
use ne_render::{RenderPlugin, WindowSettings, DeltaTime, PhysicalPosition, NWindow, NCameraBuffer, NSurfaceConfig, material};
use ne_window::events::{
    ElementState, ExitApp, ExitSequence, OnKeyboardInput, OnRedrawRequested,
    OnWindowCloseRequested, OnWindowResized, VirtualKeyCode, OnMouseMotion, OnMouseButton, MouseButton, OnMouseWheel,
};
use tracing::info;
mod interface;

struct NightmareEditor;
impl Plugin for NightmareEditor {
    fn setup(&self, app: &mut App) {
        app.add_system(on_exit)
        .add_plugin(NECamera);
    }
}
#[derive(Debug, Resource)]
struct MouseValues {
    pub is_right_mouse_pressed:bool,
}
fn env() {
    std::env::set_var("RUST_BACKTRACE", "1");
    // vulkan, metal, dx12, dx11, or gl
    std::env::set_var("WGPU_BACKEND", "vulkan");
    std::env::set_var("neprint", "true");
    //TODO does not print from other crates at all.
    std::env::set_var("RUST_LOG", "debug,wgpu=warn,naga=warn");
}
//Yet to implement ne_app correctly, the functions are called in a certain order.
//If a add_model function is called before the renderer is initialized than, that would be bad.
//That's why we need commands, to give instructions that will be called after the initalization of all plugin.
fn main() {
    env();
    const WIDTH: f32 = 800.0;
    const HEIGHT: f32 = 800.0;
    App::new()
        .add_plugin(LogPlugin::default())
        .insert_resource(WindowSettings {
            title: "Nightmare_Editor".to_string(),
            width: WIDTH,
            height: HEIGHT,
            present_mode: ne_render::PresentMode::Immediate,
            window_mode: ne_render::WindowMode::Windowed,
            ..WindowSettings::default()
        })
        .add_plugin(RenderPlugin)
        .add_plugin(NightmareEditor)
        .add_system(on_keyboard_pressed)
        .insert_resource(MouseValues{ is_right_mouse_pressed: false })
        .add_system(mouse_motion)
        .add_system(on_scroll)
        //ALPHA
        .add_startup_system(prepare_meshes)
        .add_system(on_mouse_button)
        .run();
}
//=======================================
// ALPHA
//=======================================
fn on_mouse_button (
    mut mouse_button: EventReader<OnMouseButton>,
    mut mouse_values: ResMut<MouseValues>,
    mut commands: Commands,
    //mesh
    device: Res<RenderDevice>,
    queue: Res<RenderQueue>,
    surface_conf: Res<NSurfaceConfig>,
    camera_buffer: Res<NCameraBuffer>,
    material_query: Query<(&Name, &Material)>,
) {
    for event in mouse_button.iter().rev() {
        if (event.button == MouseButton::Right && event.state == ElementState::Pressed) {
            mouse_values.is_right_mouse_pressed =true;
        }
        else {
            mouse_values.is_right_mouse_pressed =false;
        }
        if (event.button == MouseButton::Left && event.state == ElementState::Pressed) {
            //TODO improve: use {a} instead of {path_to_file}
            let count = 1000;
            let mut mesh_primss: Vec<Vec<MeshPrimitives>> = Vec::new();
            // let a = ne_files::find_file!("../../../", "/engine_assets/3D/double_cube.obj");
            let path_to_file = "./engine_assets/3D/double_cube.obj";
            // println!("A: {}, B: {}", a, path_to_file);
            let m = pollster::block_on(MeshPrimitives::from_obj(path_to_file)).unwrap();
            for _ in 0..count {
                mesh_primss.push(m.clone());
            }
            //Maybe it's cheaper to keep a hashmap of string and material-entities. Who knows how expensive material_query.iter() is.
            let mut mat3 = None;
            for (name, material) in material_query.iter() {
                if *name == Name::new("red_brick") {
                    println!("mat3 set!");
                    mat3 = Some(material);
                }
            }
            let size_of_meshes = mesh_primss.len();
            let y = 2.0;
            //only spawn if mat3 is initialized:
            match mat3 {
                Some(material) => {
                    let mut base_transform = Transform { pos: Vec3 { x: -2.0 * (size_of_meshes as f32) / 2.0, y: y, z: 4.0 }, rot: Quat::default() };
                for meshes in mesh_primss {
                    base_transform.pos.x += 2.0;
                for mesh in meshes {
                    {
                    let mesh = StaticMesh::new(
                        &camera_buffer,
                        &surface_conf, &device,
                        base_transform.clone(),
                        mesh,
                        mat3.unwrap()
                    );
                    commands.spawn(mesh);
            }}}},
                None => println!("mat3 is not init....... "),
            }
        }
    }
}
//TODO This while need to be replaced by a scene loaded used internally in ne_render. will allow for a traditional game engine setup.
fn prepare_meshes(
    mut commands: Commands,
    // mesh
    device: Res<RenderDevice>,
    queue: Res<RenderQueue>,
    surface_conf: Res<NSurfaceConfig>,
    camera_buffer: Res<NCameraBuffer>,
) {
    //materials
    let mat1 = Material::from_bytes(&device, &queue,
        include_bytes!("../../engine_assets/textures/grid.png"), Some("grid.png")).unwrap();
    let mat2 = material::Material::from_bytes(&device, &queue,
        include_bytes!("../../engine_assets/textures/orangebricks.png"), Some("orangebricks.png")).unwrap();
    let mat3 = material::Material::from_bytes(&device, &queue,
    include_bytes!("../../engine_assets/textures/redbricks.png"), Some("redbricks.png")).unwrap();

    //single platform mesh
    let transform_platform = Transform { pos: Vec3 { x: 0.0, y: 0.0, z: 0.0 }, rot: Quat::default() };
    let mesh = StaticMesh::new(
        &camera_buffer,
        &surface_conf, &device,
        transform_platform,
        ne_render::mesh::Shapes::create_box(20.0, 0.1, 20.0),
        &mat1
    );
    commands.spawn(mesh);
    let x:String = "mat1".into();
    commands.spawn(NamedMaterial{name: "default".into(), material: mat1 });
    commands.spawn(NamedMaterial{name: "brick".into(), material: mat2 });
    commands.spawn(NamedMaterial{name: "red_brick".into(), material: mat3 });

}
//=======================================
//              ^^ ALPHA ^^
//=======================================
struct NECamera;
impl Plugin for NECamera {
    fn setup(&self, app: &mut App) {
        let camera = free_fly_camera::Camera::new(Vec3::new(1.5, 3.5, 15.0), -89.53, 0.0);
        let camera_controller = free_fly_camera::CameraController::new(4.0, 0.0012);
        //todo projection is currently in ne_render but it belongs here.
        //that means resize also needs to be moved here...
        //it will allow me to change znear/zfar 
        // let projection =
        // free_fly_camera::Projection::new(surface_config.width, surface_config.height, 45.0, 0.1, 1000.0);
        app
        .insert_resource(camera)
        .insert_resource(camera_controller)
        .add_system(camera_on_redraw);
    }
}
///updates camera.
fn camera_on_redraw(
    mut redraw_event: EventReader<OnRedrawRequested>,
    mut camera_uniform: ResMut<CameraUniform>,
    mut camera_controller: ResMut<CameraController>,
    mut camera: ResMut<Camera>,
    projection: Res<Projection>,
    dt: Res<DeltaTime>,
) { 
    for event in redraw_event.iter().rev() 
    {
        camera_controller.update_camera(&mut camera, dt.time);
        camera_uniform.update_view_proj(&camera, &projection);
    }
}
fn resize_sys(mut window_resized_events: EventReader<OnWindowResized>) {
    for event in window_resized_events.iter().rev() {
        info!("window is resized w: {}, h:{}", event.width, event.height);
    }
}
fn on_exit(mut program_end_event: EventReader<ExitSequence>) {
    for _ in program_end_event.iter().rev() {
        //TODO this is suppoed to pop up a small window: with yes/no/cancel buttons. noy sure how cancel would work here tbh...
        ne::log!("Would you like to save?");
        ne::log!("exiting program");
    }
}
fn mouse_motion(
    mut mouse_motion: EventReader<OnMouseMotion>,
    mouse_values: Res<MouseValues>,
    mut camera_controller: ResMut<CameraController>,
    window: NonSend<NWindow>,
) {
    //TODO fix: when other mouse buttons is pressed functionality is interrupted
    for event in mouse_motion.iter().rev() {
        if mouse_values.is_right_mouse_pressed {
            camera_controller.process_mouse(event.delta.x, event.delta.y);
            window.set_cursor_visible(false);
            _ = window.set_cursor_position(
                PhysicalPosition::new(window.inner_size().width / 2, window.inner_size().height / 2));
        } else {
            window.set_cursor_visible(true);
        }
    }
}
//should this be fused with mouse_motion
fn on_scroll(
    mut mouse_wheel: EventReader<OnMouseWheel>,
    mut camera_controller: ResMut<CameraController>,
) {
    for event in mouse_wheel.iter().rev() {
        //TODO mouse wheel event
        camera_controller.process_scroll(&event.delta);
    }
}
fn on_keyboard_pressed(
    mut keyboard_input: EventReader<OnKeyboardInput>,
    mut exit_event: EventWriter<ExitApp>,
    mut camera_controller: ResMut<CameraController>,
    //somehow get mutable exit boolean here. No figure out a way to easily obtain data here.
) {
    for event in keyboard_input.iter().rev() {
        //camera controller
        let amount = if event.state == ElementState::Pressed {
            1.0
        } else {
            0.0
        };
        match event.key {
            VirtualKeyCode::W | VirtualKeyCode::Up => {
                camera_controller.amount_forward = amount;
            }
            VirtualKeyCode::S | VirtualKeyCode::Down => {
                camera_controller.amount_backward = amount;

            }
            VirtualKeyCode::A | VirtualKeyCode::Left => {
                camera_controller.amount_left = amount;
            }
            VirtualKeyCode::D | VirtualKeyCode::Right => {
                camera_controller.amount_right = amount;
            }
            VirtualKeyCode::E => {
                camera_controller.amount_up = amount;
            }
            VirtualKeyCode::Q => {
                camera_controller.amount_down = amount;
            }
            _ => {}
        }
        //further only pressed.
        if (event.state != ElementState::Pressed) {
            return;
        };
        match event.key {
            VirtualKeyCode::Z => {
                println!("Z is pressed");
            }
            VirtualKeyCode::Escape => {
                println!("ESCAPE");
                exit_event.send(ExitApp{});
            }
            VirtualKeyCode::Z => {
                println!("Z is pressed");
            }
            _ => {
            }
        }
    }
}