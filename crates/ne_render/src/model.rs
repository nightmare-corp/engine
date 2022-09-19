use std::ops::Range;
use bevy_ecs::prelude::{Component, Bundle};
use ne_math::Transform;
use wgpu::{BindGroup, Device, util::DeviceExt};

use crate::{texture, math::ToTransformRaw};

pub trait Vertex {
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a>;
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ModelVertex {
    pub position: [f32; 3],
    pub tex_coords: [f32; 2],
    pub normal: [f32; 3],
}

impl Vertex for ModelVertex {
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<ModelVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 5]>() as wgpu::BufferAddress,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x3,
                },
            ],
        }
    }
}

pub struct Material {
    pub name: String,
    pub diffuse_texture: texture::Texture,
    pub bind_group: wgpu::BindGroup,
}

pub(crate) struct InstancedMeshManager{
    loaded_paths:Vec<String>,
    meshes:Vec<InstancedMesh>,
}
impl Default for InstancedMeshManager {
    fn default() -> Self {
        Self {
            loaded_paths: Vec::new(), 
            meshes: Vec::new(),
        }
    }

}
impl InstancedMeshManager {
    // pub fn add_instance(&mut self, path:String, transform:Transform) 
    // {
    //     if !self.loaded_paths.contains(&path) {
    //         self.insert_new_mesh(&path);
    //         self.loaded_paths.push(path);
    //     }
    // }
    // pub fn add_instances(&mut self, path:String, transforms:Vec<Transform>) 
    // {

    // }
    // pub fn insert_new_mesh(&mut self, path:&String)
    // {
        
    //     self.meshes.push()
    // }
}

///Will hold all of the same meshes and draw on screen for each model matrix in vec!
/// ### Arguments
/// * `world_transforms` - Vec< Transform>,
/// * `matrix_buffer` - wgpu::Buffer,
/// * `meshes` - Mesh
pub struct InstancedMesh {
    // pub ids
    pub mesh:Mesh,
    //don't understand how this works:
    // pub material:Vec<Materials>,
    pub model_transforms:Vec<Transform>,
    //TODO remove pub
    pub matrix_buffer:wgpu::Buffer,
    //Is this also useful to save?
    // pub model_matrices:Vec<Transform>,
}
impl InstancedMesh {
    //TODO would be pretty cool if it could either accept a Vec or a single element, maybe tuples?
    pub fn new(device:&Device, model_transforms:Vec<Transform>, mesh:Mesh) -> Self {
        //create from above data
        let matrix_buffer:wgpu::Buffer = Self::transforms_to_buffer(device, &model_transforms);
        Self {
            mesh,
            model_transforms,
            matrix_buffer,
        }
    }
    ///If transform is None then it will be Transform::default();
    fn add_instance(&mut self, transform:Option<Transform>, device:&Device)
    {
        todo!();
        //TODO optimize also if it expects many more resize the vec +=10/20/50/100  

        //turn transform into model_matrix -> clone into buffer -> save this buffer.

        //if transform is none set transform as default...? possible?

        //I want it to be called right after renderer??
        //this is why commands are interesting
        //lets implement commands just to make sure that we always 100% of the time get it called after renderer intialization
        //but on construct needs to be called before the loop.
        //is what I think
        //let me have a look.

        // let t = transform.unwrap_or_default();
        // {
        //     let raw = t.to_raw();
        //     let instance_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        //         label: Some("Instance Buffer"),
        //         contents: bytemuck::cast_slice(&instance_data),
        //         usage: wgpu::BufferUsages::VERTEX,
        //     });
        //     self.model_matrices.push();
            
        // }
        // self.world_transforms.push();
    }
    ///TODO optimize maybe
    fn add_many_instance_internal(&mut self, device:&Device, transforms:&mut Vec<Transform>, )
    {
        self.model_transforms.append(transforms);
        self.update_matrix_buffer(device);
    }
    /// world_transforms -> matrix_buffer 
    fn update_matrix_buffer(&mut self, device:&Device) {
        self.matrix_buffer = Self::transforms_to_buffer(device, &self.model_transforms);
    }
    //TODO move function
    fn transforms_to_buffer(device:&Device, transforms:&Vec<Transform>) -> wgpu::Buffer
    {
        let instance_data = transforms.iter().map(ToTransformRaw::to_raw).collect::<Vec<_>>();
        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Instance Buffer"),
            contents: bytemuck::cast_slice(&instance_data),
            usage: wgpu::BufferUsages::VERTEX,
        })
    }
} 

pub struct Mesh {
    pub name: String,
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    // pub
    pub num_elements: u32,
    pub material: usize,
}
//contains only one *type* of mesh.
pub struct RuntimeModel {
    //it will draw by mesh
    // pub model_projections: wgpu::Buffer,
    pub mesh: Mesh,
    pub materials: Vec<Material>,
}
impl RuntimeModel {
    pub fn new(mesh:Mesh, materials:Vec<Material>) -> Self
    {
        Self{mesh,materials}
    }
}
pub trait DrawModel<'a> {
    // fn draw_mesh(
    //     &mut self,
    //     mesh: &'a Mesh,
    //     material: &'a Material,
    //     camera_bind_group: &'a wgpu::BindGroup,
    // );
    // fn draw_mesh_instanced(
    //     &mut self,
    //     mesh: &'a Mesh,
    //     material: &'a Material,
    //     instances: Range<u32>,
    //     camera_bind_group: &'a wgpu::BindGroup,
    // );
    // fn draw_model(&mut self, model: &'a RuntimeModel, camera_bind_group: &'a wgpu::BindGroup);
    // fn draw_model_instanced(
    //     &mut self,
    //     model: &'a RuntimeModel,
    //     instances: Range<u32>,
    //     camera_bind_group: &'a wgpu::BindGroup,
    // );
}

impl<'a, 'b> DrawModel<'b> for wgpu::RenderPass<'a>
where
    'b: 'a,
{
    // fn draw_mesh(
    //     &mut self,
    //     mesh: &'b Mesh,
    //     material: &'b Material,
    //     camera_bind_group: &'b wgpu::BindGroup,
    // ) {
    //     self.draw_mesh_instanced(mesh, material, 0..1, camera_bind_group);
    // }

    // fn draw_mesh_instanced(
    //     &mut self,
    //     mesh: &'b Mesh,
    //     material: &'b Material,
    //     instances: Range<u32>,
    //     camera_bind_group: &'b wgpu::BindGroup,
    // ) {
    //     self.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
    //     self.set_index_buffer(mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
    //     self.set_bind_group(0, &material.bind_group, &[]);
    //     self.set_bind_group(1, camera_bind_group, &[]);
    //     self.draw_indexed(0..mesh.num_elements, 0, instances);
    // }

    // fn draw_model(&mut self, model: &'b RuntimeModel, camera_bind_group: &'b wgpu::BindGroup) {
    //     self.draw_model_instanced(model, 0..1, camera_bind_group);
    // }
    // ///draw many of this mesh
    // fn draw_model_instanced(
    //     &mut self,
    //     model: &'b RuntimeModel,
    //     instances: Range<u32>,
    //     camera_bind_group: &'b wgpu::BindGroup,
    // ) {
    //     for mesh in &model.meshes {
    //         let material = &model.materials[mesh.material];
    //         self.draw_mesh_instanced(mesh, material, instances.clone(), camera_bind_group);
    //     }
    // }
}
