use chrono::Utc;
use nightmare_engine::*;

use ne_app1::{App, Plugin};

fn main() {
    // env::set_var("RUST_BACKTRACE", "1");
    L::init_log!(tracing::Level::INFO);

    //We are always gonna use as many threads as we can?
    //Nah that sounds dangerous... minimum of 3 threads.
    //after that it's using total threads -2.
    
    //how do I simply run a function over and over again on the main thread? ``non-send``
    App::new()
    //TODO
    // .add_plugin(Logger)
    //TODO
    .add_plugin(Renderer)


    //TASK add a modular system like bevy has. 
    //CHECK IF REQUIRED PLUGINS ARE ALREADY LOADED
    // .add_plugin(InputPlugin)
    // .add_plugin(WindowPlugin)
    // .add_plugin(WinitPlugin)

    // .add_plugin(TimePlugin)
    // .add_plugin(AssetPlugin)
    // .add_plugin(RenderPlugin)

    .add_running(test_running)
    .run(); 
}

fn test_running()
{
    let t = Utc::now().time();
    println!("{:?}", t);
}

fn main_loop(app:App)
{
    nightmare_engine::run_engine( "ne_editor");
}
struct Renderer;
impl Plugin for Renderer
{
    fn setup(&self, app: &mut App) {
        app.set_runner(main_loop);
    }
}

struct Logger;
impl Plugin for Logger
{
    fn setup(&self, app: &mut App)
    {
        //this is annoying... because we neeed certain variablesss to outlive this function inside main..?

        //So we have to simply add resources! This is very much possible and easy even
    }
}
//----------------------------------------------------------------------------------