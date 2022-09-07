use bevy_ecs::prelude::EventReader;
use nightmare_engine::*;

use ne_app::{App, Plugin};
use ne_render::{RenderPlugin, WindowSettings, OnWindowResized, OnWindowCloseRequested};

// User interface allows you to build interface of any kind.
// use crate::interface::EditorPlugin; //TODO move
mod interface;

fn gui_event_system()
{

}

// use ne_window::WindowPlugin;
fn main() {
    // env::set_var("RUST_BACKTRACE", "1");
    L::init_log!(tracing::Level::INFO);

  let width = 1600.;
  let height = 900.;

    App::new()
        .insert_resource(WindowSettings {
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
        .add_plugin(interface::RandomPlugin)

        .add_system(resize_sys)
        .add_system(exit_window)

        // .add_plugin(EditorPlugin)
        .run();
}

//on WindowResized
fn resize_sys(mut window_resized_events: EventReader<OnWindowResized>)
{
    for event in window_resized_events.iter().rev() {
        println!("window is resized w: {}, h:{}", event.width, event.height);
    }
}
fn exit_window(mut window_close_requested: EventReader<OnWindowCloseRequested>)
{
    for event in window_close_requested.iter().rev() {
        //TODO GUI would you like to save? Yes, No, Cancel. 
        println!("Would you like to save?");
        println!("exiting program");
        //Doesn't call any destructors, maybe a bad idea?
        std::process::exit(0);
    }
}

//TODO important!
//struct editor_camera;
// fn setup(&self, app: &mut App)
// {
//   app.add_events(... WASD, MOUSE, SCROLLWHEEL, );
//
// }

//TODO
// struct Logger;
// impl Plugin for Logger {
//     fn setup(&self, app: &mut App) {
//         //this is annoying... because we neeed certain variablesss to outlive this function inside main..?
//         //So we have to simply add resources! This is very much possible and easy even
//     }
// }
//----------------------------------------------------------------------------------
