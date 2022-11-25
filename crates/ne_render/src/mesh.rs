//=========================================
use crate::{depth_texture, material, math::ToMat4, render_structs::RenderDevice};
use bevy_ecs::prelude::{Component, Bundle};
use bytemuck::{Pod, Zeroable};
use ne_app::types::Name;
use ne_math::Transform;
use std::{borrow::Cow, f32::consts::PI, mem};
use wgpu::{util::DeviceExt};
///y is up
#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct Vertex {
    pos: [f32; 4],
    tex_coord: [f32; 2],
}
impl Vertex {
    pub fn new(pos: [f32; 3], tc: [f32; 2]) -> Vertex {
        Vertex {
            pos: [pos[0], pos[1], pos[2], 1.0],
            tex_coord: [tc[0], tc[1]],
        }
    }
}
#[cfg(feature = "mesh_16bit")]
type MeshIndex = u16;
#[cfg(not(feature = "mesh_16bit"))]
type MeshIndex = u32;
/// a collection of meshes TODO: and materials.
/// TODO materials
/// TODO maybe implement the ecs way..?
pub struct Model {
    pub meshes: Vec<MeshPrimitives>,
    // materials: Vec<Material>,
}
impl Model {
    pub fn new(meshes: Vec<MeshPrimitives>) -> Self {
        Self { meshes }
    }
}
//TODODO LARGHE MESHESHES WITH HIGHT VERTEX COUNT
#[derive(Clone)]
pub struct MeshPrimitives(Vec<Vertex>, Vec<MeshIndex>);
impl MeshPrimitives {
    //TODO I don't like this... somehow gotta implement include_str() or something to verify each file.
    //TODO return Model instead of MeshPrimitives.
    pub async fn from_obj(file_name: &str) -> anyhow::Result<Vec<Self>> {
        //TODO replace by assert?
        println!(
            "loading: {} exists: {}",
            file_name,
            std::path::Path::new(file_name).exists()
        );
        //TODO opportunity
        let load_options = tobj::LoadOptions {
            triangulate: true,
            single_index: true,
            ..Default::default()
        };
        //TODO No need to use obj materials, just use default engine material.
        //TODO use tobj::load_obj_buf_async

        let (models, _) = tobj::load_obj(file_name, &load_options).unwrap();

        //TODO support multiple models...
        //model into mesh_primitve
        let meshes = models
            .into_iter()
            .map(|m| {
                let vertices = (0..m.mesh.positions.len() / 3)
                    .map(|i| {
                        Vertex::new(
                            [
                                m.mesh.positions[i * 3],
                                m.mesh.positions[i * 3 + 1],
                                m.mesh.positions[i * 3 + 2],
                            ],
                            [m.mesh.texcoords[i * 2], m.mesh.texcoords[i * 2 + 1]],
                        )
                    })
                    .collect::<Vec<_>>();
                //TODO check if mesh has too many indices then end program
                //TODO abstract
                let indices = m.mesh.indices.iter().map(|&e| e as MeshIndex).collect();
                MeshPrimitives {
                    0: vertices,
                    1: indices,
                }
            })
            .collect::<Vec<_>>();
        //TODO only returns first mesh primitives...
        Ok(meshes)
    }
}
pub struct Shapes;
impl Shapes {
    //try: create_box(1.0, 1.0, 1.0);
    //try: create_box(10.0, 0.01, 10.0);
    pub fn create_box(scale_x: f32, scale_y: f32, scale_z: f32) -> MeshPrimitives {
        let max_x = scale_x / 2.0;
        let min_x = -scale_x / 2.0;
        let max_y = scale_y / 2.0;
        let min_y = -scale_y / 2.0;
        let max_z = scale_z / 2.0;
        let min_z = -scale_z / 2.0;

        let vertex_data = [
            // bottom (0.0, min_y, 0.0)
            Vertex::new([max_x, min_y, max_z], [0.0, 0.0]),
            Vertex::new([min_x, min_y, max_z], [1.0, 0.0]),
            Vertex::new([min_x, min_y, min_z], [1.0, 1.0]),
            Vertex::new([max_x, min_y, min_z], [0.0, 1.0]),
            // top (0.0, max_y, 0.0)
            Vertex::new([max_x, max_y, min_z], [1.0, 0.0]),
            Vertex::new([min_x, max_y, min_z], [0.0, 0.0]),
            Vertex::new([min_x, max_y, max_z], [0.0, 1.0]),
            Vertex::new([max_x, max_y, max_z], [1.0, 1.0]),
            // right (max_x, 0.0, 0.0)
            Vertex::new([max_x, min_y, min_z], [0.0, 0.0]),
            Vertex::new([max_x, max_y, min_z], [1.0, 0.0]),
            Vertex::new([max_x, max_y, max_z], [1.0, 1.0]),
            Vertex::new([max_x, min_y, max_z], [0.0, 1.0]),
            // left (min_x, 0.0, 0.0)
            Vertex::new([min_x, min_y, max_z], [1.0, 0.0]),
            Vertex::new([min_x, max_y, max_z], [0.0, 0.0]),
            Vertex::new([min_x, max_y, min_z], [0.0, 1.0]),
            Vertex::new([min_x, min_y, min_z], [1.0, 1.0]),
            // front (0.0, 0.0, max_z)
            Vertex::new([min_x, min_y, max_z], [0.0, 0.0]),
            Vertex::new([max_x, min_y, max_z], [1.0, 0.0]),
            Vertex::new([max_x, max_y, max_z], [1.0, 1.0]),
            Vertex::new([min_x, max_y, max_z], [0.0, 1.0]),
            // back (0.0, 0.0, min_z)
            Vertex::new([min_x, max_y, min_z], [1.0, 0.0]),
            Vertex::new([max_x, max_y, min_z], [0.0, 0.0]),
            Vertex::new([max_x, min_y, min_z], [0.0, 1.0]),
            Vertex::new([min_x, min_y, min_z], [1.0, 1.0]),
        ];
        let index_data: &[MeshIndex] = &[
            0, 1, 2, 2, 3, 0, // bottom
            4, 5, 6, 6, 7, 4, // top
            8, 9, 10, 10, 11, 8, // right
            12, 13, 14, 14, 15, 12, // left
            16, 17, 18, 18, 19, 16, // front
            20, 21, 22, 22, 23, 20, // back
        ];
        MeshPrimitives(vertex_data.to_vec(), index_data.to_vec())
    }
    //try: create_pyramid(1.0, 1.0, 1.0);
    pub fn create_pyramid(scale_x: f32, scale_y: f32, scale_z: f32) -> MeshPrimitives {
        let max_x = scale_x / 2.0;
        let min_x = -scale_x / 2.0;
        let max_y = scale_y / 2.0;
        let min_y = -scale_y / 2.0;
        let max_z = scale_z / 2.0;
        let min_z = -scale_z / 2.0;

        let top = Vertex::new([0.0, max_y, 0.0], [1.0, 0.0]);
        let vertex_data = [
            // bottom
            Vertex::new([max_x, min_y, max_z], [0.0, 0.0]),
            Vertex::new([min_x, min_y, max_z], [1.0, 0.0]),
            Vertex::new([min_x, min_y, min_z], [1.0, 1.0]),
            Vertex::new([max_x, min_y, min_z], [0.0, 1.0]),
            // right
            Vertex::new([max_x, min_y, min_z], [0.0, 0.0]),
            top,
            Vertex::new([max_x, min_y, max_z], [0.0, 1.0]),
            // left
            Vertex::new([min_x, min_y, max_z], [0.0, 0.0]),
            top,
            Vertex::new([min_x, min_y, min_z], [0.0, 1.0]),
            // front
            Vertex::new([max_x, min_y, max_z], [0.0, 0.0]),
            top,
            Vertex::new([min_x, min_y, max_z], [0.0, 1.0]),
            // back
            Vertex::new([min_x, min_y, min_z], [0.0, 0.0]),
            top,
            Vertex::new([max_x, min_y, min_z], [0.0, 1.0]),
        ];
        let index_data: &[MeshIndex] = &[
            0, 1, 2, 2, 3, 0, // bottom
            4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15,
        ];
        MeshPrimitives(vertex_data.to_vec(), index_data.to_vec())
    }
    /// The radius  
    /// Latitudinal stacks
    /// Longitudinal sectors
    /// Try: Shapes::create_uv_sphere(1.0, 36, 18);
    pub fn create_uv_sphere(radius: f32, sectors: usize, stacks: usize) -> MeshPrimitives {
        //ty bevy and http://www.songho.ca/opengl/gl_html
        let sectors2 = sectors as f32;
        let stacks2 = stacks as f32;
        let sector_step = 2. * PI / sectors2;
        let stack_step = PI / stacks2;

        let mut vertices: Vec<Vertex> = Vec::with_capacity(stacks * sectors);
        //todo normals
        // let mut normals: Vec<[f32; 3]> = Vec::with_capacity(stacks * sectors);
        let mut indices: Vec<MeshIndex> = Vec::with_capacity(stacks * sectors * 2 * 3);
        for i in 0..stacks + 1 {
            let stack_angle = PI / 2. - (i as f32) * stack_step;
            let xy = radius * stack_angle.cos();
            let z = radius * stack_angle.sin();

            for j in 0..sectors + 1 {
                let sector_angle = (j as f32) * sector_step;
                let x = xy * sector_angle.cos();
                let y = xy * sector_angle.sin();
                vertices.push(Vertex::new(
                    [x, y, z],
                    [(j as f32) / sectors2, (i as f32) / stacks2],
                ));
                // normals.push([x * length_inv, y * length_inv, z * length_inv]);
            }
        }
        // indices
        //  k1--k1+1
        //  |  / |
        //  | /  |
        //  k2--k2+1
        for i in 0..stacks {
            let mut k1 = i * (sectors + 1);
            let mut k2 = k1 + sectors + 1;
            for _j in 0..sectors {
                if i != 0 {
                    indices.push(k1 as MeshIndex);
                    indices.push(k2 as MeshIndex);
                    indices.push((k1 + 1) as MeshIndex);
                }
                if i != stacks - 1 {
                    indices.push((k1 + 1) as MeshIndex);
                    indices.push(k2 as MeshIndex);
                    indices.push((k2 + 1) as MeshIndex);
                }
                k1 += 1;
                k2 += 1;
            }
        }
        MeshPrimitives(vertices, indices)
    }
}
//TODO error handling for wasm..?
/* /// A wrapper for `pop_error_scope` futures that panics if an error occurs.
///
/// Given a future `inner` of an `Option<E>` for some error type `E`,
/// wait for the future to be ready, and panic if its value is `Some`.
///
/// This can be done simpler with `FutureExt`, but we don't want to add
/// a dependency just for this small case.
struct ErrorFuture<F> {
    inner: F,
}
impl<F: Future<Output = Option<wgpu::Error>>> Future for ErrorFuture<F> {
    type Output = ();
    fn poll(self: Pin<&mut Self>, cx: &mut task::Context<'_>) -> task::Poll<()> {
        let inner = unsafe { self.map_unchecked_mut(|me| &mut me.inner) };
        inner.poll(cx).map(|error| {
            if let Some(e) = error {
                panic!("Rendering {}", e);
            }
        })
    }
} */
// pub struct MeshDescriptor {
//     mesh_data: MeshPrimitives,
//     transform: Transform,
// }
//gotta try ecs somehow in a github fork
/// Stores buffers and such
#[derive(Component)]
pub struct GpuMesh {
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub index_count: usize,
    pub bind_group: wgpu::BindGroup,
    pub pipeline: wgpu::RenderPipeline,
    pub model_buffer: wgpu::Buffer,
}
/// Mesh that holds transform data.
/// spawn this one to easily reuse this gpu mesh...
/// TODO weak clone?
#[derive(Bundle)]
pub struct NamedGpuMesh {
    pub name:Name,
    pub mesh_data:GpuMesh,
}
/// Mesh that holds transform data.
#[derive(Bundle)]
pub struct StaticMesh {
    mesh_data:GpuMesh,
    transform:Transform,
}
//this feels roundabout and weird.
//it would make more sense if it also had a render(...) function.
//but I want to easily test out ecs.
/* pub  */trait MeshDataTrait {
    fn get_mesh_data(&self) -> &GpuMesh;
}

//Might have some use in the future. ideas are still brewing
// pub struct MeshCreator {
//     camera_buffer: wgpu::Buffer,
//     config: wgpu::SurfaceConfiguration,
//     device: RenderDevice,
// }
// impl MeshCreator {
//     pub fn new(camera_buffer:  wgpu::Buffer, config:  wgpu::SurfaceConfiguration, device: RenderDevice) -> Self 
//     { Self { camera_buffer, config, device } }
// }
impl StaticMesh {
    //TODO extract parameters into parts
    pub fn new(
        camera_buffer: &wgpu::Buffer,
        config: &wgpu::SurfaceConfiguration,
        device: &wgpu::Device,
        transform: Transform,
        mesh_prim: MeshPrimitives,
        mat: &material::Material,
    ) -> Self {
        let mesh_data = GpuMesh::new(camera_buffer, config,  device, transform.clone(), mesh_prim, mat);
        Self {mesh_data,transform } }
    pub fn update_transform() {
    }
}
impl GpuMesh {
    pub fn new(
        camera_buffer: &wgpu::Buffer,
        config: &wgpu::SurfaceConfiguration,
        device: &wgpu::Device,
        transform: Transform,
        mesh_data: MeshPrimitives,
        mat: &material::Material,
    ) -> Self {
        // Create the vertex and index buffers
        let vertex_size = mem::size_of::<Vertex>();

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(&mesh_data.0),
            usage: wgpu::BufferUsages::VERTEX,
        });
        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(&mesh_data.1),
            usage: wgpu::BufferUsages::INDEX,
        });
        // Create pipeline layout
        //TODO completely understand this
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("everything_bind_group_layout"),
            entries: &[
                //model matrix
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: wgpu::BufferSize::new(64),
                    },
                    count: None,
                },
                //texture
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        multisampled: false,
                        view_dimension: wgpu::TextureViewDimension::D2,
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    },
                    count: None,
                },
                //texture sampler
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    // This should match the filterable field of the
                    // corresponding Texture entry above.
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
                //camera
                wgpu::BindGroupLayoutEntry {
                    binding: 3,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
        });
        //TODO completely understand this
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });
        // Create other resources
        let mvp_matrix = transform.to_raw();
        let mx_ref: &[f32; 16] = mvp_matrix.as_ref();
        let model_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Uniform Buffer"),
            contents: bytemuck::cast_slice(mx_ref),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let camera_bge = wgpu::BindGroupEntry {
            binding: 3,
            resource: camera_buffer.as_entire_binding(),
        };
        // Create bind group
        //TODO split maybe
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: model_buffer.as_entire_binding(),
                },
                //texture
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(&mat.view),
                },
                //sampler
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::Sampler(&mat.sampler),
                },
                camera_bge,
            ],
            label: None,
        });
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("basic_cube.wgsl"))),
        });
        //I know that this handles how the vertex and uv data is read..?
        let vertex_buffers = [wgpu::VertexBufferLayout {
            array_stride: vertex_size as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x4,
                    offset: 0,
                    shader_location: 0,
                },
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x2,
                    offset: 4 * 4, //std::mem::size_of::<[f32; 4]>() as wgpu::BufferAddress,
                    shader_location: 1,
                },
            ],
        }];
        //TODO branch into two different fragment shaders: fs_texture & fs_color ... For now..? Or maybe first implement multiple materials and fbx, gltf, obj... Yea.
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &vertex_buffers,
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(config.format.into())],
            }),
            primitive: wgpu::PrimitiveState {
                cull_mode: Some(wgpu::Face::Back),
                ..Default::default()
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: depth_texture::DepthTexture::DEPTH_FORMAT,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
        });
        // Done
        GpuMesh {
            vertex_buffer,
            index_buffer,
            index_count: mesh_data.1.len(),
            bind_group,
            pipeline,
            model_buffer,
        }
    }
}