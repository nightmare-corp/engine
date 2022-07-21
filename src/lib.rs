mod consts;
use ne::*;

/// tracing::Level::INFO, tracing::Level::ERROR, tracing::Level::WARN
pub fn run_engine(log_level: tracing::Level, title:&str)
{
    init_log!(log_level);
    if consts::CONF_UI {
        info!("UI enabled");
    }
    else {
        info!("UI disabled!");
    }
    //initialize renderer, NOTE: hasn't been tested for wasm32
    pollster::block_on(ne_render::init_renderer(title));
}