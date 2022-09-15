use bevy_asset::Assets;
use bevy_ecs::{prelude::EventReader, system::{Commands, ResMut}};
use ne::L;
use nightmare_engine::*;

use ne_app::App;
use ne_render::{ModelDescriptor, OnWindowCloseRequested,
                OnWindowResized, RenderPlugin, Scene,
                SceneLoader, Vec3, WindowSettings};
use ne_gui::*;

//TODO
mod interface;

fn gui_event_system() {}

// use ne_window::WindowPlugin;
fn main() {
    // std::env::set_var("RUST_BACKTRACE", "1");
    // vulkan, metal, dx12, dx11, or gl
    // std::env::set_var("WGPU_BACKEND", "dx11");

    L::init_log!(tracing::Level::ERROR);
    const WIDTH: f32 = 1600.0;
    const HEIGHT: f32 = 900.0;
    let mut sl = SceneLoader::default();

    //This whole idea is quite stupid.
    //tweak it a little
    let md =  ModelDescriptor {
        path: "trapeprism2.obj".to_string(),
        location: Vec3::ZERO };
    let md2 =  ModelDescriptor {
        path: "trapeprism2.obj".to_string(),
        location: Vec3::new(1.0,1.0,1.0) };
    sl.model_data.push(md);
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