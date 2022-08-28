//thank you https://github.com/sotrh/learn-wgpu
use std::iter;

use ne_math::{Vec2, Vec3, Quat, Mat4};
use instant::Duration;
use ne::warn;
use ne_app1::{App, Plugin};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
use wgpu::util::DeviceExt;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::Window, dpi::PhysicalSize,
};
use model::{DrawModel, Vertex};
use crate::camera::CameraFields;

mod camera;
mod my_camera;
mod model;
mod resources;
mod texture;

const NUM_INSTANCES_PER_ROW: u32 = 100;

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
    
    camera: camera::Camera,
    camera_controller: camera::CameraController,
    camera_uniform: camera::CameraUniform,
    camera_buffer: wgpu::Buffer,
    camera_bind_group: wgpu::BindGroup,

    instances: Vec<Instance>,
    // #[allow(dead_code)]
    instance_buffer: wgpu::Buffer,
    depth_texture: texture::Texture,
}

impl State {
    async fn new(window: &Window, window_settings: WindowSettings) -> Self {
        let size = window.inner_size();

        // The instance is a handle to our GPU
        // BackendBit::PRIMARY => Vulkan + Metal + DX12 + Browser WebGPU
        warn!("WGPU setup");
        let instance = wgpu::Instance::new(wgpu::Backends::all());
        let surface = unsafe { instance.create_surface(window) };
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();
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

        let camera = camera::Camera::new(
            CameraFields{
            aspect: (config.width as f32 / config.height as f32),
            fovy: 45.0, znear: 0.1, zfar: 100.0,
            ..camera::CameraFields::default()});
        let camera_controller = camera::CameraController::new(0.2);

        let mut camera_uniform = camera::CameraUniform::new();
        camera_uniform.update_view_proj(&camera);

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
            source: wgpu::ShaderSource::Wgsl(include_str!("../../../assets/shader.wgsl").into()),
        });

        let depth_texture =
            texture::Texture::create_depth_texture(&device, &config, "depth_texture");

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[&texture_bind_group_layout, &camera_bind_group_layout],
                push_constant_ranges: &[],
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
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
            camera_controller,
            camera_buffer,
            camera_bind_group,
            camera_uniform,
            instances,
            instance_buffer,
            depth_texture,
        }
    }

    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.camera.set_aspect(self.config.width as f32 / self.config.height as f32);
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
            self.depth_texture =
                texture::Texture::create_depth_texture(&self.device, &self.config, "depth_texture");
        }
    }
    fn input(&mut self, event: &WindowEvent) -> bool {
        self.camera_controller.process_events(event)
    }

    //updates camera, can be cleaner/faster/moved into camera.rs
    fn update(&mut self, dt:Duration) {
        self.camera_controller.update_camera(&mut self.camera,dt);
        self.camera_uniform.update_view_proj(&self.camera);
        self.queue.write_buffer(
            &self.camera_buffer,
            0,
            bytemuck::cast_slice(&[self.camera_uniform]),
        );
    }

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
    pollster::block_on(init_renderer(app));
}
#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
async fn init_renderer(mut app: App) {
/*     cfg_if::cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            std::panic::set_hook(Box::new(console_error_panic_hook::hook));
            console_log::init_with_level(log::Level::Warn).expect("Could't initialize logger");
        } else {
            // env_logger::init(); //already inited
        }
    } */
    let mut last_render_time = instant::Instant::now();

    let event_loop = EventLoop::new();
    //TODO can this code be shrinked?
    let win_settings =  app.world.get_resource::<WindowSettings>()
        .cloned().unwrap_or_default();
    let window = create_window(&win_settings, &event_loop);
    /*     #[cfg(target_arch = "wasm32")]

       {
           // Winit prevents sizing with CSS, so we have to set
           // the size manually when on web.
           use winit::dpi::PhysicalSize;
           window.set_inner_size(PhysicalSize::new(450, 400));

           use winit::platform::web::WindowExtWebSys;
           web_sys::window()
               .and_then(|win| win.document())
               .and_then(|doc| {
                   let dst = doc.get_element_by_id("wasm-example")?;
                   let canvas = web_sys::Element::from(window.canvas());
                   dst.append_child(&canvas).ok()?;
                   Some(())
               })
               .expect("Couldn't append canvas to document body.");
       }
    */
    // State::new uses async code, so we're going to wait for it to finish
    
    let mut state = State::new(&window, win_settings).await;
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;
        //update app
        app.update();
        match event {
            Event::MainEventsCleared => window.request_redraw(),
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => {
                if !state.input(event) {
                    match event {
                        WindowEvent::CloseRequested
                        | WindowEvent::KeyboardInput {
                            input:
                                KeyboardInput {
                                    state: ElementState::Pressed,
                                    virtual_keycode: Some(VirtualKeyCode::Escape),
                                    ..
                                },
                            ..
                        } => *control_flow = ControlFlow::Exit,
                        WindowEvent::Resized(physical_size) => {
                            state.resize(*physical_size);
                        }
                        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                            state.resize(**new_inner_size);
                        }
                        _ => {}
                    }
                }
            }
            Event::RedrawRequested(window_id) if window_id == window.id() => {
                //NPP
                let now = instant::Instant::now();
                //TODO move to global/ne_time (new crate) if it's ever needed.
                let delta_time:Duration = now - last_render_time;
                last_render_time = now;
                

                //TODO This is interesting... can we replace it by a placeholder to inject stuff in? Then again hard coded isn't bad..? 
                //But bevy_ecs does have something convenient here
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
                    Err(wgpu::SurfaceError::Timeout) => log::warn!("Surface timeout"),
                }
            }
            _ => {}
        }
    });
}

// #[derive(Default)]
pub struct RenderPlugin;
impl Plugin for RenderPlugin {
    fn setup(&self, app: &mut App) {
        app
/*         .add_event::<WindowResized>()
        .add_event::<CreateWindow>()
        .add_event::<WindowCreated>()
        .add_event::<WindowClosed>()
        .add_event::<WindowCloseRequested>()
        .add_event::<RequestRedraw>()
        .add_event::<CursorMoved>()
        .add_event::<CursorEntered>()
        .add_event::<CursorLeft>()
        .add_event::<ReceivedCharacter>()
        .add_event::<WindowFocused>()
        .add_event::<WindowScaleFactorChanged>()
        .add_event::<WindowBackendScaleFactorChanged>()
        .add_event::<FileDragAndDrop>()
        .add_event::<WindowMoved>() */
        // .init_resource::<Windows>()
        
        .set_runner(main_loop);
    }
}
/// ================================================================================================
/// Events
/// ================================================================================================



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
    pub mode: WindowMode,
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
            present_mode: wgpu::PresentMode::Fifo,
                        
            //TODO THESE NEED TO BE ENABLED ON STARTUP
            position: WindowPosition::Automatic,
            resize_constraints: WindowResizeConstraints::default(),
            scale_factor_override: None,

            mode: WindowMode::Windowed,
            canvas: None,
        }
    }
}
fn create_window(win_settings: &WindowSettings, event_loop: &EventLoop<()>) -> Window
{
    let wind = winit::window::WindowBuilder::new()
    .with_title(win_settings.title.clone())
    .with_inner_size(PhysicalSize::new(win_settings.width, win_settings.height))
    .with_transparent(win_settings.transparent)
    .with_resizable(win_settings.resizable)
    .with_decorations(win_settings.decorations);
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

