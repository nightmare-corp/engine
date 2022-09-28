//=========================================
use bytemuck::{Pod, Zeroable};
use ne_math::{Vec3, Transform, Mat4};
use std::{borrow::Cow, f32::consts::{self, PI}, future::Future, mem, pin::Pin, task};
use wgpu::{util::DeviceExt, CommandBuffer};

use crate::{math::ToMat4, texture};

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
pub struct MeshPrimitives(Vec<Vertex>, Vec<u16>);
pub struct Shapes;
impl Shapes {
    //try: create_box(1.0, 1.0, 1.0);
    //try: create_box(10.0, 0.01, 10.0);
    pub fn create_box(scale_x:f32,scale_y:f32,scale_z:f32,) -> MeshPrimitives
    {
        let max_x= scale_x / 2.0;
        let min_x= -scale_x / 2.0;
        let max_y= scale_y / 2.0;
        let min_y= -scale_y / 2.0;
        let max_z= scale_z / 2.0;
        let min_z= -scale_z / 2.0;

        let vertex_data = [
            // bottom 
            Vertex::new([max_x, min_y, max_z], [0.0, 0.0]),
            Vertex::new([min_x, min_y, max_z], [1.0, 0.0]),
            Vertex::new([min_x, min_y, min_z], [1.0, 1.0]),
            Vertex::new([max_x, min_y, min_z], [0.0, 1.0]),
            // top (0.0, max_x, 0.0)
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
            // front (0.0, 0.0, max_x)
            Vertex::new([min_x, min_y, max_z], [0.0, 0.0]),
            Vertex::new([max_x, min_y, max_z], [1.0, 0.0]),
            Vertex::new([max_x, max_y, max_z], [1.0, 1.0]),
            Vertex::new([min_x, max_y, max_z], [0.0, 1.0]),
            // back (0.0, 0.0, min_x)
            Vertex::new([min_x, max_y, min_z], [1.0, 0.0]),
            Vertex::new([max_x, max_y, min_z], [0.0, 0.0]),
            Vertex::new([max_x, min_y, min_z], [0.0, 1.0]),
            Vertex::new([min_x, min_y, min_z], [1.0, 1.0]),

        ];
        let index_data: &[u16] = &[
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
    pub fn create_pyramid(scale_x:f32,scale_y:f32,scale_z:f32) -> MeshPrimitives
    {
        let max_x= scale_x / 2.0;
        let min_x= -scale_x / 2.0;
        let max_y= scale_y / 2.0;
        let min_y= -scale_y / 2.0;
        let max_z= scale_z / 2.0;
        let min_z= -scale_z / 2.0;

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
        let index_data: &[u16] = &[
            0, 1, 2, 2, 3, 0, // bottom
            4,5,6,
            7,8,9,
            10,11,12,
            13,14,15,
        ];
        MeshPrimitives(vertex_data.to_vec(), index_data.to_vec())
    }
        
    /// The radius  
    /// Latitudinal stacks
    /// Longitudinal sectors
    /// Try: Shapes::create_uv_sphere(1.0, 36, 18);
    pub fn create_uv_sphere(
        radius: f32,
        sectors: usize,
        stacks: usize,) -> MeshPrimitives
        {
            //ty bevy and http://www.songho.ca/opengl/gl_html
            let sectors2 = sectors as f32;
            let stacks2 = stacks as f32;
            let length_inv = 1. / radius;
            let sector_step = 2. * PI / sectors2;
            let stack_step = PI / stacks2;
    
            let mut vertices: Vec<Vertex> = Vec::with_capacity(stacks * sectors);
            //todo normals
            // let mut normals: Vec<[f32; 3]> = Vec::with_capacity(stacks * sectors);
            let mut indices: Vec<u16> = Vec::with_capacity(stacks * sectors * 2 * 3);
            for i in 0..stacks + 1 {
                let stack_angle = PI / 2. - (i as f32) * stack_step;
                let xy = radius * stack_angle.cos();
                let z = radius * stack_angle.sin();
    
                for j in 0..sectors + 1 {
                    let sector_angle = (j as f32) * sector_step;
                    let x = xy * sector_angle.cos();
                    let y = xy * sector_angle.sin();
                    vertices.push(Vertex::new([x, y, z], [(j as f32) / sectors2, (i as f32) / stacks2]));
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
                        indices.push(k1 as u16);
                        indices.push(k2 as u16);
                        indices.push((k1 + 1) as u16);
                    }
                    if i != stacks - 1 {
                        indices.push((k1 + 1) as u16);
                        indices.push(k2 as u16);
                        indices.push((k2 + 1) as u16);
                    }
                    k1 += 1;
                    k2 += 1;
                }
            }
            MeshPrimitives(vertices, indices)
        }
}

//TODO understand
fn create_texels(size: usize) -> Vec<u8> {
    (0..size * size)
        .map(|id| {
            let cx = 3.0 * (id % size) as f32 / (size - 1) as f32 - 2.0;
            let cy = 2.0 * (id / size) as f32 / (size - 1) as f32 - 1.0;
            let (mut x, mut y, mut count) = (cx, cy, 0);
            while count < 0xFF && x * x + y * y < 4.0 {
                let old_x = x;
                x = x * x - y * y + cx;
                y = 2.0 * old_x * y + cy;
                count += 1;
            }
            count
        })
        .collect()
}

/// A wrapper for `pop_error_scope` futures that panics if an error occurs.
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
}

pub struct Example {
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    index_count: usize,
    bind_group: wgpu::BindGroup,
    // uniform_buffer: wgpu::Buffer,
    pipeline: wgpu::RenderPipeline,
    transform: Transform,
}

impl Example {
    #[must_use]
    pub fn init(
        camera_buffer: &wgpu::Buffer,
        config: &wgpu::SurfaceConfiguration,
        _adapter: &wgpu::Adapter,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        transform: Transform,
        // TODO tuple is not easily readable.
        mesh_data: MeshPrimitives,
    ) -> Self {
        // Create the vertex and index buffers
        let vertex_size = mem::size_of::<Vertex>();

        // let mesh_data = Shapes::create_uv_sphere(1.0, 36, 18);//Shapes::create_pyramid(2.0,2.0,2.0);
        
        let vertex_buffer= device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(&mesh_data.0),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer= device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(&mesh_data.1),
            usage: wgpu::BufferUsages::INDEX,
        });

        // Create pipeline layout
        //TODO completely understand this
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
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
                        sample_type: wgpu::TextureSampleType::Uint,
                        view_dimension: wgpu::TextureViewDimension::D2,
                    },
                    count: None,
                },
                //camera
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }
            ],
        });
        //TODO completely understand this
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        // Create the texture
        //TODO textures.
        //TODO would be cool if vertices can be random colors, maybe even efficient for the gpu..?

        let size = 256u32;
        let texels = create_texels(size as usize);
        let texture_extent = wgpu::Extent3d {
            width: size,
            height: size,
            depth_or_array_layers: 1,
        };
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: None,
            size: texture_extent,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::R8Uint,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        });
        let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        queue.write_texture(
            texture.as_image_copy(),
            &texels,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(std::num::NonZeroU32::new(size).unwrap()),
                rows_per_image: None,
            },
            texture_extent,
        );

        // Create other resources
        let mvp_matrix = transform.to_raw();
        let mx_ref: &[f32; 16] = mvp_matrix.as_ref();
        let uniform_buffer= device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Uniform Buffer"),
            contents: bytemuck::cast_slice(mx_ref),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        }); 
        // Create bind group
        //DPDP what is this?
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: uniform_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(&texture_view),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: camera_buffer.as_entire_binding(),
                },
            ],
            label: None,
        });

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("basic_cube.wgsl"))),
        });

        //DPDP I fail to completely understand this 
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
        //TODO what's happening here?
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
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
        });
        // Done
        Example {
            vertex_buffer,
            index_buffer,
            index_count: mesh_data.1.len(),
            bind_group,
            // uniform_buffer,
            pipeline,
            transform,
        }
    }
    pub fn update(&mut self, _event: winit::event::WindowEvent) {
        //empty
    }
    #[must_use]
    pub fn resize(
        &mut self,
        config: &wgpu::SurfaceConfiguration,
        _device: &wgpu::Device,
        queue: &wgpu::Queue,
    ) {
        // let model_matrix = Self::generate_matrix(
            // config.width as f32 / config.height as f32,
            //  self.transform.to_raw());
        // let mx_ref: &[f32; 16] = model_matrix.as_ref();
        // queue.write_buffer(&self.uniform_buffer, 0, bytemuck::cast_slice(mx_ref));
    }
    #[must_use]
    pub fn render(
        &mut self,
        view: &wgpu::TextureView,
        device: &wgpu::Device,
        depth_texture: &texture::Texture,
    ) -> CommandBuffer {
        device.push_error_scope(wgpu::ErrorFilter::Validation);
        let mut encoder =
            device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        {
            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });
            rpass.push_debug_group("Prepare data for draw.");
            rpass.set_pipeline(&self.pipeline);
            rpass.set_bind_group(0, &self.bind_group, &[]);
            rpass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            rpass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            rpass.pop_debug_group();
            rpass.insert_debug_marker("Draw!");
            rpass.draw_indexed(0..self.index_count as u32, 0, 0..1);
        }
        encoder.finish()
    }
}



//=========================================


/* pub trait Vertex {
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


//oop
pub struct Mesh {
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    index_count: usize,
    bind_group: wgpu::BindGroup,
    uniform_buffer: wgpu::Buffer,
    pipeline: wgpu::RenderPipeline,
}

impl Mesh {
    ///path relative from assets, transform,   
    pub fn load_mesh_from_path(path: &str, transform:Transform) -> Self {
        //build string

        //read .obj file -> extract vertices, indices, (material maybe)

        //load these onto the gpu.

        //return the buffers which is just the locations on the gpu where things are stored.
        Mesh { vertex_buffer: todo!(), index_buffer: todo!(), index_count: todo!(), bind_group: todo!(), uniform_buffer: todo!(), pipeline: todo!() }
    }
    pub fn draw(&self, encoder: /* &mut  */CommandEncoder) -> CommandBuffer
    {
        //what does the encoder do
        //render pass?
        //commandbuffer
        //descriptors
        //how to correctly move modelprojection/transform of object to gpu?
        //how to write shaders to help with this
        //is the camera already in here?


        encoder.finish()
    }
    // pub fn Draw(&self, rpass: RenderPass) -> CommandBuffer
    // {

    // }
} */