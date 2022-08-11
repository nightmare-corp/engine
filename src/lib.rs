//Thanks bevy!

pub mod prelude;
use bevy_ecs::{schedule::{IntoSystemDescriptor}};

pub use ne::*;
// TODO replace with something better:
// TODO move to ne_editor...
// I want a file of cfgs but I don't know how it works.
pub const CONF_UI: bool = false;

/// tracing::Level::INFO, tracing::Level::ERROR, tracing::Level::WARN
pub fn run_engine(log_level: tracing::Level, title:&str)
{
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

pub struct App
{

}
//TODO Is this needed?
// impl Default for app
// {
//     // fn default() -> Self {
//     // }
// }
impl App
{
    pub fn new() -> App
    {
        
        // App::default()
        App::empty()
        
    }
    pub fn empty() -> App {

        //todo
        Self {
        }
    }
    pub fn add_plugin<T>(&mut self, plugin: T) -> &mut Self
    where
        T: Plugin,
    {
        debug!("Initializing: {}", plugin.name());
        plugin.setup(self);
        self
    }
    // pub fn run_constructor()
    // {

    // }
        /// Adds a system to the [startup stage](Self::add_default_stages) of the app's [`Schedule`].
    ///
    /// * For adding a system that runs every frame, see [`add_system`](Self::add_system).
    /// * For adding a system to a specific stage, see [`add_system_to_stage`](Self::add_system_to_stage).
    pub fn add_startup_func<Params>(
        &mut self,
        system: impl IntoSystemDescriptor<Params>,
    ) -> &mut Self {
        self.add_startup_system_to_stage(StartupStage::Startup, system)
    }
    
    pub fn add_running_func(&mut self,) -> &mut Self 
    {
        
    }

    //===========================================================
    // pub fn add_startup_system<Params>(
    //     &mut self,
    //     system: impl IntoSystemDescriptor<Params>,
    // ) -> &mut Self {
    //     self.add_startup_system_to_stage(StartupStage::Startup, system)
    // }
}
pub trait Plugin: /* Any + Send + Sync */ {
    /// Configures the [`App`] to which this plugin is added.
    fn setup(&self, app: &mut App);
    /// Configures a name for the [`Plugin`] which is primarily used for debugging.
    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
}