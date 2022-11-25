use std::sync::Arc;

use bevy_derive::{Deref, DerefMut};
use ne_app::Resource;
use wgpu::Queue;

/// This GPU device is responsible for the creation of most rendering and compute resources.
#[derive(Resource, Clone, Deref, DerefMut)]
pub struct RenderDevice (pub Arc<wgpu::Device>);
// impl From<Arc<wgpu::Device>> for RenderDevice {
//     fn from(device: Arc<wgpu::Device>) -> Self {
//         Self { device }
//     }
// }
/// This queue is used to enqueue tasks for the GPU to execute asynchronously.
#[derive(Resource, Clone, Deref, DerefMut)]
pub struct RenderQueue(pub Arc<Queue>);