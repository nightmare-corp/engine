
use bevy_ecs::{prelude::EventReader};
use ne::{L, info};
use ne_app::App;
use ne_render::{RenderPlugin,
                Vec3, WindowSettings};
                use ne_render::PresentMode::Fifo;
use ne_math::{Transform, Quat};
use ne_window::events::{OnWindowResized, OnWindowCloseRequested};
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
    // let mut sl = SceneLoader::default();
    // //TODO why does it only accept cube and trapeprism2?
    // let md =  ModelDescriptor {
    //     path: "trapeprism2.obj".to_string(),
    //     transform: Transform { position: Vec3::ZERO, rotation: Quat::default()},
    //     };
    // sl.model_data.push(md);
    // let md2 =  ModelDescriptor {
    //     path: "shapes/cube.obj".to_string(),
    //     transform: Transform { position: Vec3::new(1.0,1.0,1.0), rotation: Quat::default()}, };
    // sl.model_data.push(md2);

    //TODO replace WindowSettings with WindowBuilder.. maybe
    App::new()
        .insert_resource(
            WindowSettings {
            title: "Nightmare_Editor".to_string(),
            width: WIDTH,
            height: HEIGHT,
            present_mode: Fifo,
            window_mode: ne_render::WindowMode::Windowed,
            ..WindowSettings::default()
        })
        // .insert_resource(sl)
        .add_plugin(RenderPlugin)
        .add_system(resize_sys)
        .add_system(exit_window)
        .run();
}
//on WindowResized
fn resize_sys(mut window_resized_events: EventReader<OnWindowResized>) {
    for event in window_resized_events.iter().rev() {
        ne::info!("window is resized w: {}, h:{}", event.width, event.height);
    }
}
fn exit_window(mut window_close_requested: EventReader<OnWindowCloseRequested>) {
    for _ in window_close_requested.iter().rev() {
        //TODO GUI would you like to save? Yes, No, Cancel.
        ne::log!("Would you like to save?");
        ne::log!("exiting program");
        //Doesn't call any destructors, maybe a bad idea?
        std::process::exit(0);
    }
}