use bevy_ecs::prelude::EventReader;
use nightmare_engine::*;

use ne_app::App;
use ne_render::{OnWindowCloseRequested, OnWindowResized, RenderPlugin, WindowSettings};
use ne_gui::*;

//TODO
mod interface;

fn gui_event_system() {}

// use ne_window::WindowPlugin;
fn main() {
    // std::env::set_var("RUST_BACKTRACE", "1");
    // vulkan, metal, dx12, dx11, or gl
    //WGPU_BACKEND isnt read by wgpu at all, just the examples, you can change the backend flags in the instance parameters
    // std::env::set_var("WGPU_BACKEND", "dx11");

    L::init_log!(tracing::Level::INFO);

    let width = 1600.;
    let height = 900.;

    //TODO replace WindowSettings with WindowBuilder
    App::new()
        .insert_resource(
            WindowSettings {
            title: "Nightmare_Editor".to_string(),
            width: width,
            height: height,
            // present_mode: PresentMode::AutoVsync,
            window_mode: ne_render::WindowMode::Windowed,
            ..WindowSettings::default()
        })
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
        println!("window is resized w: {}, h:{}", event.width, event.height);
    }
}
fn exit_window(mut window_close_requested: EventReader<OnWindowCloseRequested>) {
    for event in window_close_requested.iter().rev() {
        //TODO GUI would you like to save? Yes, No, Cancel.
        println!("Would you like to save?");
        println!("exiting program");
        //Doesn't call any destructors, maybe a bad idea?
        std::process::exit(0);
    }
}