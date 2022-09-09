//thank you https://github.com/sotrh/learn-wgpu

//TODO EDITOR UI    #[cfg(feature = "editor_ui")]

use std::{iter, path::PathBuf};

use cameras::free_fly_camera;
use ne_math::{Vec2, Vec3, Quat, Mat4};
use ne::{warn, info, trace};
use ne_app::{App, Plugin, Events, ManualEventReader};
#[cfg(feature = "first_frame_time")]
use ne_app::FIRST_FRAME_TIME;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
use wgpu::util::DeviceExt;
use winit::{
    event::{*, self},
    event_loop::{ControlFlow, EventLoop, EventLoopWindowTarget},
    window::{Window, Fullscreen, WindowId}, dpi::PhysicalSize,
};
use model::{DrawModel, Vertex};
use crate::cameras::free_fly_camera::CameraUniform;

mod cameras;
mod model;
mod resources;
mod texture;

//move maybe
#[cfg(feature = "editor_ui")]
mod interface;

const NUM_INSTANCES_PER_ROW: u32 = 50;

struct Instance {
    position: Vec3,
    rotation: Quat,
}

impl Instance {
    fn to_raw(&self) -> InstanceRaw {
        InstanceRaw {
            model: (Mat4::from_translation(self.position)
                * Mat4::from_quat(self.rotation))
            .to_cols_array_2d(),
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct InstanceRaw {
    #[allow(dead_code)]
    model: [[f32; 4]; 4],
}

impl InstanceRaw {
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<InstanceRaw>() as wgpu::BufferAddress,
            // We need to switch from using a step mode of Vertex to Instance
            // This means that our shaders will only change to use the next
            // instance when the shader starts processing a new instance
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    // While our vertex shader only uses locations 0, and 1 now, in later tutorials we'll
                    // be using 2, 3, and 4, for Vertex. We'll start at slot 5 not conflict with them later
                    shader_location: 5,
                    format: wgpu::VertexFormat::Float32x4,
                },
                // A mat4 takes up 4 vertex slots as it is technically 4 vec4s. We need to define a slot
                // for each vec4. We don't have to do this in code though.
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 4]>() as wgpu::BufferAddress,
                    shader_location: 6,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 8]>() as wgpu::BufferAddress,
                    shader_location: 7,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 12]>() as wgpu::BufferAddress,
                    shader_location: 8,
                    format: wgpu::VertexFormat::Float32x4,
                },
            ],
        }
    }
}

struct State {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    render_pipeline: wgpu::RenderPipeline,
    obj_model: model::Model,
    
    camera: free_fly_camera::Camera,
    projection: free_fly_camera::Projection,
    camera_controller: free_fly_camera::CameraController,
    camera_uniform: free_fly_camera::CameraUniform,

    //Can we change this into a buffer vector? or is that bad?
    camera_buffer: wgpu::Buffer,
    camera_bind_group: wgpu::BindGroup,

    instances: Vec<Instance>,
    // #[allow(dead_code)]
    instance_buffer: wgpu::Buffer,
    depth_texture: texture::Texture,

    is_right_mouse_pressed:bool,
}


impl State {
    async fn new(window: &Window, window_settings: WindowSettings) -> Self {
        let size = window.inner_size();

        // The instance is a handle to our GPU
        // BackendBit::PRIMARY => Vulkan + Metal + DX12 + Browser WebGPU
        warn!("WGPU setup");
        let backend = wgpu::util::backend_bits_from_env().unwrap_or_else(wgpu::Backends::all);
        ne::log!("backend: ", backend);
        let instance = wgpu::Instance::new(backend);
        
        let surface = unsafe { instance.create_surface(window) };

        //TODO this crashes when we use opengl or dx11? does dx11 need to be installed on pc? are drivers outdated?
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                //will use the highest performance gpu.
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap(); //TODO maybe replace this unwrap with a match?
        warn!("device and queue");
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    features: wgpu::Features::empty(),
                    // WebGL doesn't support all of wgpu's features, so if
                    // we're building for the web we'll have to disable some.
                    limits: if cfg!(target_arch = "wasm32") {
                        wgpu::Limits::downlevel_webgl2_defaults()
                    } else {
                        wgpu::Limits::default()
                    },
                },
                // Some(&std::path::Path::new("trace")), // Trace path
                None, // Trace path
            )
            .await
            .unwrap();

        warn!("Surface");
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_supported_formats(&adapter)[0],
            width: size.width,
            height: size.height,
            present_mode: window_settings.present_mode,
        };

        surface.configure(&device, &config);

        let texture_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            view_dimension: wgpu::TextureViewDimension::D2,
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
                label: Some("texture_bind_group_layout"),
            });

            //TODO accessibility 
            let camera = free_fly_camera::Camera::new(Vec3::new(0.0, 5.0, 10.0), -90.0, -20.0);
            let projection =
            free_fly_camera::Projection::new(config.width, config.height, 45.0, 0.1, 100.0);
            //TODO accessibility 
            let camera_controller = free_fly_camera::CameraController::new(4.0, 0.8);
    
            let mut camera_uniform = CameraUniform::new();
            camera_uniform.update_view_proj(&camera, &projection);
    

        //Buffer that will put camera-vpm-matrix into shader
        let camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Camera Buffer"),
            contents: bytemuck::cast_slice(&[camera_uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        const SPACE_BETWEEN: f32 = 3.0;
        let instances = (0..NUM_INSTANCES_PER_ROW)
            .flat_map(|z| {
                (0..NUM_INSTANCES_PER_ROW).map(move |x| {
                    let x = SPACE_BETWEEN * (x as f32 - NUM_INSTANCES_PER_ROW as f32 / 2.0);
                    let z = SPACE_BETWEEN * (z as f32 - NUM_INSTANCES_PER_ROW as f32 / 2.0);

                    let position = Vec3 { x, y: 0.0, z };

                    //How to know if position is zero???
                    let rotation = if let Some(pos) = position.try_normalize() {
                        Quat::from_axis_angle(pos, ne_math::to_radians(45.0))
                    } else {
                        Quat::from_axis_angle(Vec3::Z, 0.0)
                    };
                    Instance { position, rotation }
                })
            })
            .collect::<Vec<_>>();

        let instance_data = instances.iter().map(Instance::to_raw).collect::<Vec<_>>();
        let instance_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Instance Buffer"),
            contents: bytemuck::cast_slice(&instance_data),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let camera_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
                label: Some("camera_bind_group_layout"),
            });

        let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &camera_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: camera_buffer.as_entire_binding(),
            }],
            label: Some("camera_bind_group"),
        });

        warn!("Load model");
        let obj_model = resources::load_model(
            //TODO Other models.
            "trapeprism2.obj",
            &device,
            &queue,
            &texture_bind_group_layout,
        )
        .await
        .unwrap();

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("shader.wgsl"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../../../assets/shaders/shader.wgsl").into()),
        });

        let depth_texture =
            texture::Texture::create_depth_texture(&device, &config, "depth_texture");

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[&texture_bind_group_layout, &camera_bind_group_layout],
                push_constant_ranges: &[],
            });

        let render_pipeline = 
        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[model::ModelVertex::desc(), InstanceRaw::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState {
                        color: wgpu::BlendComponent::REPLACE,
                        alpha: wgpu::BlendComponent::REPLACE,
                    }),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                // Setting this to anything other than Fill requires Features::POLYGON_MODE_LINE
                // or Features::POLYGON_MODE_POINT
                polygon_mode: wgpu::PolygonMode::Fill,
                // Requires Features::DEPTH_CLIP_CONTROL
                unclipped_depth: false,
                // Requires Features::CONSERVATIVE_RASTERIZATION
                conservative: false,
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: texture::Texture::DEPTH_FORMAT,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            // If the pipeline will be used with a multiview render pass, this
            // indicates how many array layers the attachments will have.
            multiview: None,
        });

        Self {
            surface,
            device,
            queue,
            config,
            size,
            render_pipeline,
            obj_model,
            camera,
            projection,
            camera_controller,
            camera_buffer,
            camera_bind_group,
            camera_uniform,
            instances,
            instance_buffer,
            depth_texture,

            is_right_mouse_pressed: false,
        }
    }

    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.projection.resize(new_size.width, new_size.height);
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
            self.depth_texture =
                texture::Texture::create_depth_texture(&self.device, &self.config, "depth_texture");
        }
    }
    fn input(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        virtual_keycode: Some(key),
                        state,
                        ..
                    },
                ..
            } => self.camera_controller.process_keyboard(*key, *state),
            WindowEvent::MouseWheel { delta, .. } => {
                self.camera_controller.process_scroll(delta);
                true
            }
            WindowEvent::MouseInput {
                button: MouseButton::Right,
                state,
                ..
            } => {
                self.is_right_mouse_pressed = *state == ElementState::Pressed;
                true
            }
            _ => false,
        }

    }

    //updates camera, can be cleaner/faster/moved into camera.rs... maybe
    fn update(&mut self, dt:f32) {
        self.camera_controller.update_camera(&mut self.camera,dt);
        self.camera_uniform.update_view_proj(&self.camera, &self.projection);
        self.queue.write_buffer(
            &self.camera_buffer,
            0,
            bytemuck::cast_slice(&[self.camera_uniform]),
        );
    }

    //TODO double&triple buffer
    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: true,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.depth_texture.view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: true,
                    }),
                    stencil_ops: None,
                }),
            });

            render_pass.set_vertex_buffer(1, self.instance_buffer.slice(..));
            render_pass.set_pipeline(&self.render_pipeline);

            //TODO BIG
            //UNDERSTAD how is this rendererd
            // understand how transforms work
            // understand how locations work?

            //Is instanced draw just better..?
            render_pass.draw_model_instanced(
                &self.obj_model,
                0..self.instances.len() as u32,
                &self.camera_bind_group,
            );           

            // render_pass.draw_model(&self.obj_model, &self.camera_bind_group);
        }

        self.queue.submit(iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}

//TODO HOW TO SHORTEN THIS?
fn main_loop(app: App) {
    //TODO why is main_loop before exiting??????
    println!("main_loop");

    //is this async implementation any good?
    pollster::block_on(init_renderer(app));

    println!("main_loop done");

}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
async fn init_renderer(mut app: App) {
    println!("init_renderer");
    // app.update(); //moved}

    let event_loop = EventLoop::new();
    //TODO can this code be shrinked?
    let win_settings =  app.world.get_resource::<WindowSettings>()
        .cloned().unwrap_or_default();
    let window = create_window(&win_settings, &event_loop);
    
    let mut state = State::new(&window, win_settings).await;

    trace!("pre event_loop.run");

    //benchmark values.
    #[cfg(feature = "first_frame_time")]
    let mut once_benchmark = true;
    let mut last_render_time = instant::Instant::now();
    let mut frame_count:u64 = 1;
    //exit window event reader
    let mut app_exit_event_reader = ManualEventReader::<AppExit>::default();

    #[cfg(feature = "print_fps")]
    let mut fpsd = ne_bench::FPSData::default();

    let event_handler = 
    move |event: Event<()>,
        _: &EventLoopWindowTarget<()>, //not sure what to do with event_loop: &EventLoopWindowTarget
        control_flow: &mut ControlFlow| {
        match event {
            event::Event::MainEventsCleared => {
                window.request_redraw();
                app.update(); //is this supposed to be here?
            },
            event::Event::WindowEvent {
                    ref event,
                    window_id,
                } if window_id == window.id() => {
                    let world = app.world.cell();
    
                    if !state.input(event) {
                        match event {
                            WindowEvent::CloseRequested => {
                                let mut window_close_requested_events =
                                    world.resource_mut::
                                        <Events<OnWindowCloseRequested>>();
                                window_close_requested_events.send(
                                    OnWindowCloseRequested { id: window_id });
                            }
                            WindowEvent::KeyboardInput {
                                input:
                                    KeyboardInput {
                                        state: ElementState::Pressed,
                                        virtual_keycode: Some(VirtualKeyCode::Escape),
                                        ..
                                    },
                                ..
                            } => *control_flow = ControlFlow::Exit,
                            WindowEvent::Resized(physical_size) => {
    
                                //TODO MOVE TASK: decouple window and renderer
                                state.resize(*physical_size);
                                
                                let mut resize_events
                                = world.resource_mut::<Events<OnWindowResized>>();
                                resize_events.send(OnWindowResized {
                                    id: window_id,
                                    width: window.inner_size().width as f32,
                                    height: window.inner_size().height as f32,
                                });
                            }
                            WindowEvent::ScaleFactorChanged { scale_factor, new_inner_size } => {
                                state.resize(**new_inner_size);

                                let mut scale_event = world.resource_mut
                                ::<Events<OnWindowScaleFactorChanged>>();
                                scale_event.send(OnWindowScaleFactorChanged 
                                    { id: window_id, scale_factor: *scale_factor })

                            }
                            WindowEvent::Focused(focused)
                            => {
                                let mut focused_events = 
                                    world.resource_mut::<Events<OnWindowFocused>>();
                                focused_events.send(OnWindowFocused {
                                    id: window_id,
                                    //TODO why does this have to be dereferenced?
                                    focused: *focused });
                            }
                            WindowEvent::HoveredFile(path_buf) => {
                                let mut events = world.resource_mut::<Events<OnFileDragAndDrop>>();
                                events.send(OnFileDragAndDrop::HoveredFile {
                                    id: window_id,
                                    //TODO TEST
                                    path_buf: path_buf.to_path_buf(),
                                });
                            }
                            WindowEvent::DroppedFile(path_buf) => {
                                let mut events = world.resource_mut::<Events<OnFileDragAndDrop>>();
                                events.send(OnFileDragAndDrop::DroppedFile {
                                    id: window_id,
                                    path_buf: path_buf.to_path_buf(),
                                });
                            }
                                                    //TODO implement multiple windows before implementing this one.
                        WindowEvent::CursorMoved { position, .. }
                        => {
                            // TODO
                            // let mut cursor_moved_event 
                            //     = world.resource_mut::<Events<OnCursorMoved>>();

                            //     //idk what's happening here
                            //     let winit_window = winit_windows.get_window(window_id).unwrap();
                            //     let inner_size = winit_window.inner_size();
        
                            //     // move origin to bottom left
                            //     let y_position = inner_size.height as f64 - position.y;
        
                            //     let physical_position = DVec2::new(position.x, y_position);
                            // cursor_moved_event.send(OnCursorMoved {
                            //         id: window_id, 
                            //         position: (physical_position / window.scale_factor()).as_vec2(), 
                            //     })

                        }
                        WindowEvent::CursorEntered{.. /* device id needed? */ } 
                        => {
                            let mut cursor_entered_events =
                                world.resource_mut::<Events<OnCursorEntered>>();
                            cursor_entered_events.send(OnCursorEntered { id: window_id });
                        }
                        WindowEvent::CursorLeft { .. } => {
                            let mut cursor_left_event 
                            = world.resource_mut::<Events<OnCursorLeft>>();
                            cursor_left_event.send(OnCursorLeft { id: window_id });
                        }
                        //TODO TEST
                        WindowEvent::ReceivedCharacter(c) => {
                            let mut char_input_events =
                                world.resource_mut::<Events<OnReceivedCharacter>>();
    
                            char_input_events.send(OnReceivedCharacter {
                                id: window_id,
                                //TODO this dereference hits performance? Measure this
                                char: *c,
                            });
                        }
                         _ => {}
                        }
                    }
                }
            event::Event::DeviceEvent {
                    event: DeviceEvent::MouseMotion{ delta, },
                    .. // We're not using device_id currently
                } => if state.is_right_mouse_pressed {
                    state.camera_controller.process_mouse(delta.0, delta.1);
                    window.set_cursor_visible(false);
                } else {
                    window.set_cursor_visible(true);
                }
            event::Event::RedrawRequested(window_id) if window_id == window.id() => {
                    //hope this gets optimized somehow
                    #[cfg(feature = "first_frame_time")]
                    if once_benchmark //do once
                    {
                        unsafe{
                            FIRST_FRAME_TIME = Some(instant::Instant::now());
                        }
                        once_benchmark=false;
                    }
                    let now = instant::Instant::now();
                    let delta_time = (now - last_render_time).as_secs_f32();
                    last_render_time = now;
    
                    #[cfg(feature = "print_fps")]
                    {
                        //TODO move maybe
                        let fps = 1.0/delta_time;
                        // a little messy
                        frame_count+=1;
                        unsafe
                        {
                            let time_passed = (now - FIRST_FRAME_TIME.unwrap()).as_secs_f32();
                            let average_fps = frame_count as f32/time_passed;
                            
                            println!("fps:{:<14}fps | avg:{:<14}fps | 1%LOW:{:<10}fps",fps,average_fps, fpsd.get_lowest(fps));
                        }
                    }
                    state.update(delta_time);
    
                    match state.render() {
                        Ok(_) => {}
                        // Reconfigure the surface if it's lost or outdated
                        Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                            state.resize(state.size)
                        }
                        // The system is out of memory, we should probably quit
                        Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                        // We're ignoring timeouts
                        Err(wgpu::SurfaceError::Timeout) => warn!("Surface timeout"),
                    }

                    // //add event right here... maybe?
                    // let redraw_event = app.world.get_resource::<Events<Redraw>>() {

                    // }
                    //sent an event from here, frame is done!
                    //TODO remove..?
                    let world = app.world.cell();
                    let mut frame_events = world.resource_mut::<Events<FrameEvent>>();
                    frame_events.send(FrameEvent {});

                }
                event::Event::RedrawEventsCleared => {
                    //measure
                    if let Some(app_exit_events) = 
                        app.world.get_resource::<Events<AppExit>>() {
                        if app_exit_event_reader.iter(app_exit_events).last().is_some() {
                            *control_flow = ControlFlow::Exit;
                        }
                    }
                }
                _ => 
                {
    
                }
            }
    };
    println!("event loop run");
    run(event_loop, event_handler);
}

fn run<F>(event_loop: EventLoop<()>, event_handler: F) -> !
where
    F: 'static + FnMut(Event<'_, ()>, &EventLoopWindowTarget<()>, &mut ControlFlow),
{
    event_loop.run(event_handler)
}
// #[derive(Default)]
///sets runner using .set_runner()
pub struct RenderPlugin;
impl Plugin for RenderPlugin {
    fn setup(&self, app: &mut App) {

        //TODO what's going onnnnn
        println!("setup");
        app
        .add_event::<OnWindowResized>()
        .add_event::<AppExit>()
        .add_event::<FrameEvent>()


        //TODO
        .add_event::<OnWindowCloseRequested>()
        .add_event::<OnWindowFocused>()

//      .add_event::<OnWindowMoved>()
        .add_event::<OnWindowScaleFactorChanged>()
        .add_event::<OnFileDragAndDrop>()

        .add_event::<OnCursorMoved>()
        .add_event::<OnCursorEntered>()
        .add_event::<OnCursorLeft>()

        .add_event::<OnReceivedCharacter>()
        //todo
/*     
        .add_event::<CreateWindow>()
        .add_event::<WindowCreated>()
        .add_event::<WindowClosed>()
        .add_event::<WindowBackendScaleFactorChanged>()
*/



        // .init_resource::<Windows>()
        .set_runner(main_loop);
    }
}
/// ================================================================================================
/// Window functionality
/// ================================================================================================

/// Describes the information needed for creating a window.
///
/// This should be set up before adding the [`WindowPlugin`](crate::WindowPlugin).
/// Most of these settings can also later be configured through the [`Window`](crate::Window) resource.
///
/// See [`examples/window/window_settings.rs`] for usage.
///
/// [`examples/window/window_settings.rs`]: https://github.com/bevyengine/bevy/blob/latest/examples/window/window_settings.rs
#[derive(Debug, Clone)]
pub struct WindowSettings {
    /// Sets the title that displays on the window top bar, on the system task bar and other OS specific places.
    ///
    /// ## Platform-specific
    /// - Web: Unsupported.
    pub title: String,
    /// The requested logical width of the window's client area.
    /// May vary from the physical width due to different pixel density on different monitors.
    pub width: f32,
    /// The requested logical height of the window's client area.
    /// May vary from the physical height due to different pixel density on different monitors.
    pub height: f32,
    /// ## Platform-specific
    /// - iOS / Android / Web: Unsupported.
    /// - macOS X: Not working as expected.
    /// - Windows 11: Not working as expected
    /// macOS X transparent works with winit out of the box, so this issue might be related to: <https://github.com/gfx-rs/wgpu/issues/687>
    /// Windows 11 is related to <https://github.com/rust-windowing/winit/issues/2082>
    /// Sets whether the background of the window should be transparent.
    pub transparent: bool,
    /// - iOS / Android / Web: Unsupported.
    pub resizable: bool,
    /// Sets whether the window should have borders and bars.
    pub decorations: bool,
    /// Sets whether the cursor is visible when the window has focus.
    //TODO
    pub cursor_visible: bool,
    /// Sets whether the window locks the cursor inside its borders when the window has focus.
    //TODO
    pub cursor_locked: bool,
    /// Whether or not to fit the canvas element's size to its parent element's size.
    ///
    /// **Warning**: this will not behave as expected for parents that set their size according to the size of their
    /// children. This creates a "feedback loop" that will result in the canvas growing on each resize. When using this
    /// feature, ensure the parent's size is not affected by its children.
    ///
    /// This value has no effect on non-web platforms.
    //TODO
    pub fit_canvas_to_parent: bool,
    /// Controls when a frame is presented to the screen.
    #[doc(alias = "vsync")]
    /// The window's [`PresentMode`].
    /// Used to select whether or not VSync is used
    pub present_mode: wgpu::PresentMode,
    /// The position on the screen that the window will be placed at.
    pub position: WindowPosition,
    /// Sets minimum and maximum resize limits.
    pub resize_constraints: WindowResizeConstraints,
    /// Overrides the window's ratio of physical pixels to logical pixels.
    ///
    /// If there are some scaling problems on X11 try to set this option to `Some(1.0)`.
    pub scale_factor_override: Option<f64>,
    /// Sets whether the window is resizable.
    /// ## Platform-specific
    /// Sets the [`WindowMode`](crate::WindowMode).
    pub window_mode: WindowMode,
    /// The "html canvas" element selector.
    /// If set, this selector will be used to find a matching html canvas element,
    /// rather than creating a new one.
    /// Uses the [CSS selector format](https://developer.mozilla.org/en-US/docs/Web/API/Document/querySelector).
    /// This value has no effect on non-web platforms.
    pub canvas: Option<String>,
}
impl Default for WindowSettings {
    fn default() -> Self {
        WindowSettings {
            title: "app".to_string(),
            width: 1280.,
            height: 720.,

            resizable: true,
            decorations: true,
            cursor_locked: false,
            cursor_visible: true,
            transparent: false,
            fit_canvas_to_parent: false,
            present_mode: wgpu::PresentMode::Immediate,
                        
            //TODO THESE NEED TO BE ENABLED ON STARTUP
            position: WindowPosition::Automatic,
            resize_constraints: WindowResizeConstraints::default(),
            scale_factor_override: None,

            window_mode: WindowMode::Windowed,
            canvas: None,
        }
    }
}

//window
pub fn get_fitting_videomode(
    monitor: &winit::monitor::MonitorHandle,
    width: u32,
    height: u32,
) -> winit::monitor::VideoMode {
    let mut modes = monitor.video_modes().collect::<Vec<_>>();

    fn abs_diff(a: u32, b: u32) -> u32 {
        if a > b {
            return a - b;
        }
        b - a
    }

    modes.sort_by(|a, b| {
        use std::cmp::Ordering::*;
        match abs_diff(a.size().width, width).cmp(&abs_diff(b.size().width, width)) {
            Equal => {
                match abs_diff(a.size().height, height).cmp(&abs_diff(b.size().height, height)) {
                    Equal => b.refresh_rate().cmp(&a.refresh_rate()),
                    default => default,
                }
            }
            default => default,
        }
    });

    modes.first().unwrap().clone()
}

fn create_window(win_settings: &WindowSettings, event_loop: &EventLoop<()>) -> Window
{
    let mut wind = winit::window::WindowBuilder::new()
    .with_title(win_settings.title.clone())
    .with_inner_size(PhysicalSize::new(win_settings.width, win_settings.height))
    .with_transparent(win_settings.transparent)
    .with_resizable(win_settings.resizable)
    .with_decorations(win_settings.decorations);

    match win_settings.window_mode
    {
        //incomplete 
        WindowMode::Windowed => {},
        WindowMode::BorderlessFullscreen => {
            // let mut monitor_index = 0; //todo
            let monitor = event_loop
            .available_monitors()
            .next()
            .expect("no monitor found!");
            let fullscreen = Some(Fullscreen::Borderless(Some(monitor.clone())));
            info!("Setting mode: {:?}", fullscreen);
            wind = wind.with_fullscreen(fullscreen);
        },
        WindowMode::SizedFullscreen => todo!(),
        WindowMode::Fullscreen => {
            // let mut monitor_index = 0; //todo
            let monitor = event_loop
            .available_monitors()
            .next()
            .expect("no monitor found!");
            let fullscreen = Some(Fullscreen::Exclusive(get_fitting_videomode(
                &monitor,
                win_settings.width as u32,
                win_settings.height as u32,
            )),);
            info!("Setting mode: {:?}", fullscreen);
            wind = wind.with_fullscreen(fullscreen);
        },
    }

    //TODO ...
    // match (win_settings.mode)
    // {
    //     WindowMode::Windowed => (),
    //     WindowMode::BorderlessFullscreen => wind.with_fullscreen(),
    //     WindowMode::SizedFullscreen => todo!(),
    //     WindowMode::Fullscreen => wind.with_fullscreen(),
    // }

    wind
    .build(&event_loop)
    .unwrap()
}
/// Defines the way a window is displayed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowMode {
    /// Creates a window that uses the given size.
    Windowed,
    /// Creates a borderless window that uses the full size of the screen.
    BorderlessFullscreen,
    /// Creates a fullscreen window that will render at desktop resolution.
    ///
    /// The app will use the closest supported size from the given size and scale it to fit the screen.
    SizedFullscreen,
    /// Creates a fullscreen window that uses the maximum supported size.
    Fullscreen,
}

/// Defines where window should be placed at on creation.
#[derive(Debug, Clone, Copy)]
pub enum WindowPosition {
    /// Position will be set by the window manager
    Automatic,
    /// Window will be centered on the selected monitor
    ///
    /// Note that this does not account for window decorations.
    Centered(MonitorSelection),
    /// The window's top-left corner will be placed at the specified position (in pixels)
    ///
    /// (0,0) represents top-left corner of screen space.
    At(Vec2),
}

/// Defines which monitor to use.
#[derive(Debug, Clone, Copy)]
pub enum MonitorSelection {
    /// Uses current monitor of the window.
    Current,
    /// Uses primary monitor of the system.
    Primary,
    /// Uses monitor with the specified index.
    Number(usize),
}

//Needed?
/// The size limits on a window.
///
/// These values are measured in logical pixels, so the user's
/// scale factor does affect the size limits on the window.
/// Please note that if the window is resizable, then when the window is
/// maximized it may have a size outside of these limits. The functionality
/// required to disable maximizing is not yet exposed by winit.
#[derive(Debug, Clone, Copy)]
pub struct WindowResizeConstraints {
    pub min_width: f32,
    pub min_height: f32,
    pub max_width: f32,
    pub max_height: f32,
}

impl Default for WindowResizeConstraints {
    fn default() -> Self {
        Self {
            min_width: 180.,
            min_height: 120.,
            max_width: f32::INFINITY,
            max_height: f32::INFINITY,
        }
    }
}

impl WindowResizeConstraints {
    #[must_use]
    pub fn check_constraints(&self) -> Self {
        let WindowResizeConstraints {
            mut min_width,
            mut min_height,
            mut max_width,
            mut max_height,
        } = self;
        min_width = min_width.max(1.);
        min_height = min_height.max(1.);
        if max_width < min_width {
            warn!(
                "The given maximum width {} is smaller than the minimum width {}",
                max_width, min_width
            );
            max_width = min_width;
        }
        if max_height < min_height {
            warn!(
                "The given maximum height {} is smaller than the minimum height {}",
                max_height, min_height
            );
            max_height = min_height;
        }
        WindowResizeConstraints {
            min_width,
            min_height,
            max_width,
            max_height,
        }
    }
}


/// ================================================================================================
/// Events
/// ================================================================================================
/// A window event that is sent whenever a window's logical size has changed.
#[derive(Debug, Clone)]
pub struct OnWindowResized {
    pub id: winit::window::WindowId,
    /// The new logical width of the window.
    pub width: f32,
    /// The new logical height of the window.
    pub height: f32,
}
/// An event that is sent whenever a new window is created.
///
/// To create a new window, send a [`CreateWindow`] event - this
/// event will be sent in the handler for that event.
#[derive(Debug, Clone)]
pub struct OnWindowCreated {
    pub id: WindowId,
}

/// An event that is sent whenever the operating systems requests that a window
/// be closed. This will be sent when the close button of the window is pressed.
///
/// If the default [`WindowPlugin`] is used, these events are handled
/// by [closing] the corresponding [`Window`].  
/// To disable this behaviour, set `close_when_requested` on the [`WindowPlugin`]
/// to `false`.
///
/// [`WindowPlugin`]: crate::WindowPlugin
/// [`Window`]: crate::Window
/// [closing]: crate::Window::close
#[derive(Debug, Clone)]
pub struct OnWindowCloseRequested {
    pub id: WindowId,
}

/// An event that is sent whenever a window is closed. This will be sent by the
/// handler for [`Window::close`].
///
/// [`Window::close`]: crate::Window::close
#[derive(Debug, Clone)]
pub struct OnWindowClosed {
    pub id: WindowId,
}
/// An event reporting that the mouse cursor has moved on a window.
///
/// The event is sent only if the cursor is over one of the application's windows.
/// It is the translated version of [`WindowEvent::CursorMoved`] from the `winit` crate.
///
/// Not to be confused with the [`MouseMotion`] event from `bevy_input`.
///
/// [`WindowEvent::CursorMoved`]: https://docs.rs/winit/latest/winit/event/enum.WindowEvent.html#variant.CursorMoved
/// [`MouseMotion`]: bevy_input::mouse::MouseMotion
#[derive(Debug, Clone)]
pub struct OnCursorMoved {
    /// The identifier of the window the cursor has moved on.
    pub id: WindowId,

    /// The position of the cursor, in window coordinates.
    pub position: Vec2,
}
/// An event that is sent whenever the user's cursor enters a window.
#[derive(Debug, Clone)]
pub struct OnCursorEntered {
    pub id: WindowId,
}
/// An event that is sent whenever the user's cursor leaves a window.
#[derive(Debug, Clone)]
pub struct OnCursorLeft {
    pub id: WindowId,
}


/// An event that indicates a window has received or lost focus.
#[derive(Debug, Clone)]
pub struct OnWindowFocused {
    pub id: WindowId,
    pub focused: bool,
}

/// Events related to files being dragged and dropped on a window.
#[derive(Debug, Clone)]
pub enum OnFileDragAndDrop {
    DroppedFile { id: WindowId, path_buf: PathBuf },

    HoveredFile { id: WindowId, path_buf: PathBuf },

    HoveredFileCancelled { id: WindowId },
}

/// An event that is sent when a window is repositioned in physical pixels.
#[derive(Debug, Clone)]
pub struct OnWindowMoved {
    pub id: WindowId,
    pub position: Vec2,
}
//TODO implement these maybe:
/* /// An event that indicates that a new window should be created.
#[derive(Debug, Clone)]
pub struct OnCreateWindow {
    pub id: WindowId,
    pub descriptor: WindowDescriptor,
} */
/// An event that is sent whenever a window receives a character from the OS or underlying system.
#[derive(Debug, Clone)]
pub struct OnReceivedCharacter {
    pub id: WindowId,
    pub char: char,
}
/// An event that indicates a window's scale factor has changed.
#[derive(Debug, Clone)]
pub struct OnWindowScaleFactorChanged {
    pub id: WindowId,
    pub scale_factor: f64,
}
/* /// An event that indicates a window's OS-reported scale factor has changed.
#[derive(Debug, Clone)]
pub struct OnWindowBackendScaleFactorChanged {
    pub id: WindowId,
    pub scale_factor: f64,
} */
// #[derive(Debug, Clone)]
/// Reader in loop that will end the event loop.
pub struct AppExit;
/// An event that is sent when a frame has been rendered 
/// Inside of RedrawRequested in the eventloop
pub struct FrameEvent;