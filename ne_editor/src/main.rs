use bevy_asset::Assets;
use bevy_ecs::{prelude::EventReader, system::{Commands, ResMut}};
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
    // std::env::set_var("WGPU_BACKEND", "dx11");

    L::init_log!(tracing::Level::INFO);
    const WIDTH: f32 = 1600.0;
    const HEIGHT: f32 = 900.0;

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
        // .add_startup_system(setup_scene)

        //TODO currently working on a windowplugin
        // .add_plugin(WindowPlugin)
        .add_plugin(RenderPlugin)
        // .add_plugin(interface::EditorPlugin)
        .add_system(resize_sys)
        .add_system(exit_window)
        .run();
}

//Commands are used to modify World...? but how
// fn setup_scene(    
//     mut commands: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<StandardMaterial>>,
// )
// {
//     ne::log!("HELLLOOO");
// }



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