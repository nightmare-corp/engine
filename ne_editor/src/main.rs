use bevy_ecs::prelude::EventReader;
use nightmare_engine::*;

use ne_app::{App, Plugin};
use ne_render::{RenderPlugin, WindowSettings, WindowResized};

// User interface allows you to build interface of any kind.
// use crate::interface::EditorPlugin; //TODO move
mod interface;

fn gui_event_system()
{

}
//on WindowResized
fn random_system(mut window_resized_events: EventReader<WindowResized>)
{
    for event in window_resized_events.iter().rev() {
        println!("window is resized w: {}, h:{}", event.width, event.height);
    }
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

        .add_system(random_system)
        // .add_plugin(EditorPlugin)

        /* TODO modular plugins
        CHECK IF REQUIRED PLUGINS ARE ALREADY LOADED
        .add_plugin(InputPlugin)
        .add_plugin(WindowPlugin)
        .add_plugin(WinitPlugin)
        .add_plugin(TimePlugin)
        .add_plugin(AssetPlugin)
        .add_plugin(RenderPlugin)
        .add_plugin(Logger)
          */
        // .add_system(test_running)
        .run();
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
