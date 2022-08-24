use chrono::Utc;
use nightmare_engine::*;

use ne_app1::{App, Plugin};
use ne_render::{Renderer, WindowSettings};
fn main() {
    // env::set_var("RUST_BACKTRACE", "1");
    L::init_log!(tracing::Level::INFO);
    //how do I simply run a function over and over again on the main thread? ``non-send``
    App::new()
        .insert_resource(WindowSettings {
            title: "Nightmare_Editor".to_string(),
            width: 500.,
            height: 500.,
            // present_mode: PresentMode::AutoVsync,
            // ..default()
        })
        //TODO
        // .add_plugin(Logger)
        .add_plugin(Renderer)
        /* TODO modular plugins
        CHECK IF REQUIRED PLUGINS ARE ALREADY LOADED
         .add_plugin(InputPlugin)
         .add_plugin(WindowPlugin)
         .add_plugin(WinitPlugin)
         .add_plugin(TimePlugin)
         .add_plugin(AssetPlugin)
         .add_plugin(RenderPlugin) */
        // .add_running(test_running)
        .run();
}

//WOW it works
fn test_running() {
    let t = Utc::now().time();
    println!("{:?}", t);
}

struct Logger;
impl Plugin for Logger {
    fn setup(&self, app: &mut App) {
        //this is annoying... because we neeed certain variablesss to outlive this function inside main..?
        //So we have to simply add resources! This is very much possible and easy even
    }
}
//----------------------------------------------------------------------------------
