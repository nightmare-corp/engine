use wgpu::{Queue, Device};

pub struct SceneLoader {
}
impl Default for SceneLoader {
    fn default() -> Self {
        SceneLoader::new()
    }
}
impl SceneLoader {
    pub fn new() -> Self { 
        Self { }
    }
    ///Add model data to the scene loader
    fn push_model_data(&mut self)
    {
    }
}
/// Holds all scene components, the dvd for the dvd player
/// ### Arguments
/// * `models` - StaticMeshManagerManager,
pub struct Scene {
}
impl Scene {
    /// Will load everything from SceneLoader
    pub async fn new(scene:SceneLoader,device:&Device, queue:&Queue,
                     texture_bind_group_layout: &wgpu::BindGroupLayout,) -> Self {
        Self { 
        }
    }
}
