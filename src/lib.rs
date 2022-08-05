pub mod prelude;
use prelude::*;
// TODO replace with something better:
// TODO move to ne_editor...
// I want a file of cfgs but I don't know how it works.
pub const CONF_UI: bool = false;

/// tracing::Level::INFO, tracing::Level::ERROR, tracing::Level::WARN
pub fn run_engine(log_level: tracing::Level, title:&str)
{
    init_log!(log_level);
    if CONF_UI {
        info!("UI enabled");
    }
    else {
        info!("UI disabled!");
    }
    //initialize renderer, NOTE: hasn't been tested for wasm32
    pollster::block_on(ne_render::init_renderer(title));
}

#[allow(non_camel_case_types)]
pub struct nightmare_engine
{
}
//TODO Is this needed?
// impl Default for nightmare_engine
// {
//     // fn default() -> Self {
//     // }
// }
impl nightmare_engine
{
    pub fn new() -> nightmare_engine
    {
        // nightmare_engine::default()
        nightmare_engine::empty()
    }
    pub fn empty() -> nightmare_engine {
        Self {
        }
    }
    pub fn add_plugin(self) -> nightmare_engine {

        self
    }
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

