pub mod prelude;
use prelude::*;
pub mod app;
// TODO replace with something better:
// TODO move to ne_editor...
// I want a file of cfgs but I don't know how it works.
pub const CONF_UI: bool = false;

/// tracing::Level::INFO, tracing::Level::ERROR, tracing::Level::WARN
pub fn run_engine(log_level: tracing::Level, title:&str)
{
    //TODO it doesn't print "UI disabled!"
    L::init_log!(log_level);
    
    warn!("UI disabled!");

    if CONF_UI {
        info!("UI enabled");
    }
    else {
        info!("UI disabled!");
    }
    //initialize renderer, NOTE: hasn't been tested for wasm32
    pollster::block_on(ne_render::init_renderer(title));
}

#[macro_export]
macro_rules! get_engine_assets_dir{
    () =>
    {
        // find_file!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets")
        // concat!(env!("CARGO_MANIFEST_DIR"), "/assets")
        todo!();
    }
}

