pub mod prelude;

pub use ne::*;
pub use ne::{L, warn, info};

// TODO REMOVE
// I want a file of cfgs
pub const CONF_UI: bool = false;

// pub fn run_engine(title:&str)
// {
//     warn!("UI disabled!");
//     if CONF_UI {
//         info!("UI enabled");
//     }
//     else {
//         info!("UI disabled!");
//     }
//     //initialize renderer, NOTE: hasn't been tested for wasm32
//     pollster::block_on(ne_render::init_renderer(title));
// }
pub fn ne_render_init()
{

}
pub fn ne_render_running()
{

}
