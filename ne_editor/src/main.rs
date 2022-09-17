
use bevy_ecs::{prelude::EventReader};
use ne::L;

use ne_app::App;
use ne_render::{ModelDescriptor, OnWindowCloseRequested,
                OnWindowResized, RenderPlugin,
                SceneLoader, Vec3, WindowSettings};
use ne_math::{Transform, Quat};
//TODO
mod interface;

//Yet to implement ne_app correctly, the functions are called in a certain order.
//If a add_model function is called before the renderer is initialized than, that would be bad.
//That's why we need commands, to give instructions that will be called after the initalization of all plugin.
fn main() {
    // std::env::set_var("RUST_BACKTRACE", "1");
    // vulkan, metal, dx12, dx11, or gl
    std::env::set_var("WGPU_BACKEND", "vulkan");
    std::env::set_var("neprint", "true");

    L::init_log!(tracing::Level::WARN);
    const WIDTH: f32 = 1600.0;
    const HEIGHT: f32 = 900.0;
    let mut sl = SceneLoader::default();

    //TODO why does it only accept cube and trapeprism2?
    let md =  ModelDescriptor {
        path: "shapes/cube.obj".to_string(),
        transform: Transform { position: Vec3::ZERO, rotation: Quat::default()},
        };
    sl.model_data.push(md);
    let md2 =  ModelDescriptor {
        path: "trapeprism2.obj".to_string(),
        transform: Transform { position: Vec3::new(1.0,1.0,1.0), rotation: Quat::default()}, };
    sl.model_data.push(md2);

    //TODO replace WindowSettings with WindowBuilder
    App::new()
        .insert_resource(
            WindowSettings {
            title: "Nightmare_Editor".to_string(),
            width: WIDTH,
            height: HEIGHT,
            // present_mode: PresentMode::AutoVsync,
            window_mode: ne_render::WindowMode::Windowed,
            ..WindowSettings::default()
        })
        .insert_resource(sl)

        //TODO currently working on a windowplugin
        // .add_plugin(WindowPlugin)
        .add_plugin(RenderPlugin)
        // .add_plugin(interface::EditorPlugin)
        .add_system(resize_sys)
        .add_system(exit_window)
        .run();
}

//on WindowResized
fn resize_sys(mut window_resized_events: EventReader<OnWindowResized>) {
    for event in window_resized_events.iter().rev() {
        ne::log!("window is resized w: {}, h:{}", event.width, event.height);
    }
}
fn exit_window(mut window_close_requested: EventReader<OnWindowCloseRequested>) {
    for event in window_close_requested.iter().rev() {
        //TODO GUI would you like to save? Yes, No, Cancel.
        ne::log!("Would you like to save?");
        ne::log!("exiting program");
        //Doesn't call any destructors, maybe a bad idea?
        std::process::exit(0);
    }
}