use std::collections::HashMap;

use bevy_derive::{Deref, DerefMut};
use ne_math::{Transform};
/// This is the first iteration renderer it needs to do: 
/// 1) Render separate meshes
/// 2) Render many instances of one mesh
/// 3) Render many seperate meshes
/// 4) Allow for easy user interface integration 
/// 5) Easy switching from camera
/// 6) Mesh loading from .gltf and .obj and maybe fbx files
/// 7) Level saving/loading from .nscene
/// 8) Effective editing of .nscene file...
pub use ne_math::{Vec2, Vec3, Quat, Mat4};
use ne_window::events::{OnWindowResized, OnWindowScaleFactorChanged, ExitApp, OnRedrawRequested,
                        OnWindowCloseRequested, OnFileDragAndDrop, OnCursorEntered, OnCursorLeft, OnReceivedCharacter, OnWindowFocused, OnKeyboardInput, ExitSequence, OnMouseMotion, OnMouseButton, OnMouseWheel};
                        use std::sync::Arc;

use ne_app::{App, Plugin, Events, ManualEventReader, Resource};
use render_structs::{RenderQueue, RenderDevice};
use tracing::{warn, debug, info};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
use wgpu::{util::DeviceExt, CommandBuffer, CommandEncoder, SurfaceConfiguration};
use winit::{
    event::{*, self},
    event_loop::{ControlFlow, EventLoop, EventLoopWindowTarget},
    window::{Fullscreen, Window}, dpi::PhysicalSize,
};
//export windowbuilder
pub use winit::{dpi::PhysicalPosition,window::{WindowBuilder}};

use cameras::free_fly_camera::{self, Projection};
use crate::{cameras::free_fly_camera::CameraUniform, user_interface::EditorUIState, mesh::{GpuMesh, Shapes, StaticMesh}};

#[cfg(feature = "editor_ui")]
pub mod cameras;
mod user_interface;
mod resources;
mod depth_texture;
mod render_modules;
pub mod mesh;
mod model;
mod shapes;
pub mod material;
pub mod render_structs;
pub mod math;
// pub mod scene;
// use Scene as CurrentScene; //will be used as a resource...

#[derive(Clone, Resource)]
pub struct DeltaTime {
    pub time: f32,
}
//move these
//===================================================
#[derive(Resource, Deref, DerefMut)]
pub struct NWindow(Window);
#[derive(Resource, Deref, DerefMut)]
pub struct NUiState(EditorUIState);
#[derive(Resource, Deref, DerefMut)]
pub struct NSurfaceConfig(SurfaceConfiguration);
#[derive(Resource, Deref, DerefMut)]
pub struct NCameraBuffer(wgpu::Buffer);

mod Render {
    use wgpu::Queue;
    use crate::cameras::free_fly_camera::CameraUniform;

    //updates camera, can be cleaner/faster/moved into camera.rs... maybe
    pub fn update_camera_buffer(queue:&Queue, camera_buffer:&wgpu::Buffer, camera_uniform:&CameraUniform) {
        queue.write_buffer(
            &camera_buffer,
            0,
            //TODO to_owned is right?
            bytemuck::cast_slice(&[camera_uniform.to_owned()]),
        );
    }
}
//===================================================
//try moving all of this to app as resources.
#[cfg(feature = "editor_ui")]
static mut FRAME_COUNT: u32 = 0;
#[derive(Resource)]
struct RenderState {
    surface: wgpu::Surface,
    device: RenderDevice,
    queue: RenderQueue,
    size: winit::dpi::PhysicalSize<u32>,
    depth_texture: depth_texture::DepthTexture,
}

impl RenderState {
    async fn new(app: &mut App, window: &Window, window_settings: WindowSettings) -> Self {
        //================================================================================================================
        //Window and wgpu initialization
        //================================================================================================================
        let size = window.inner_size();
        // The instance is a handle to our GPU
        let backend = wgpu::util::backend_bits_from_env().unwrap_or_else(wgpu::Backends::all);
        let instance = wgpu::Instance::new(backend);
        let surface = unsafe { instance.create_surface(window) };
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                //will use the highest performance gpu.
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();
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
        let surface_format = surface.get_supported_formats(&adapter)[0];
        let surface_config = NSurfaceConfig(wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: window_settings.present_mode,
        });
        surface.configure(&device, &surface_config);
        //================================================================================================================
        //camera buffer
        //================================================================================================================
        //this will be the default camera transform.
        //this camera will be removed afterwards so that the user can implement their own.
        let camera = free_fly_camera::Camera::new(Vec3::new(1.5, 3.5, 15.0), -89.53, 0.007696513);
        let projection =
            free_fly_camera::Projection::new(surface_config.width, surface_config.height, 45.0, 0.1, 1000.0);
        let mut camera_uniform = CameraUniform::default();
        camera_uniform.update_view_proj(&camera, &projection);
        let camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Camera Buffer"),
            contents: bytemuck::cast_slice(&[camera_uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });
        //depth texture
        let depth_texture =
            depth_texture::DepthTexture::create_depth_texture(&device, &surface_config, "depth_texture");
        
            //================================================================================================================
        //TODO Scene loading could potentialy be here.
        //================================================================================================================
        // debug!("Load scene");

        #[cfg(feature = "editor_ui")]
            let ui_state = NUiState{0: EditorUIState::new(window, &device, &surface_format)};
        //this resource will be removed in runner
        app.insert_resource(camera_uniform);
        app.insert_resource(projection);
        #[cfg(feature = "editor_ui")]
        app.insert_resource(ui_state);
        // arc might cause slowdowns
        let queue  = RenderQueue(Arc::new(queue));
        let device  = RenderDevice(Arc::new(device));
        app.insert_resource(queue.clone());
        app.insert_resource(device.clone());

        // let mesh_creator = MeshCreator{ camera_buffer, config: surface_config, device };
        // app.insert_resource(mesh_creator);
        app.insert_resource(surface_config);
        app.insert_resource(NCameraBuffer(camera_buffer));
        Self {
            surface,
            device,
            queue,
            size,
            depth_texture,
        }
    }
    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>, surface_conf: &mut NSurfaceConfig, projection:&mut Projection) {
        if new_size.width > 0 && new_size.height > 0 {
            //TODO camera projection needs to be resized..?
            projection.resize(new_size.width, new_size.height);
            self.size = new_size;
            surface_conf.width = new_size.width;
            surface_conf.height = new_size.height;
            self.surface.configure(&self.device, surface_conf);
            self.depth_texture =
                depth_texture::DepthTexture::create_depth_texture(&self.device, surface_conf, "depth_texture");
        }
    }
    //TODO double&triple buffer
    fn create_encoder(&self) -> CommandEncoder {
        self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            })
    }
    fn render(&mut self, app: &mut App, delta_time: f32) -> Result<(), wgpu::SurfaceError> {
        let output_frame = self.surface.get_current_texture()?;
        let output_view = output_frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut cmd_buffers = Vec::<CommandBuffer>::new();
        //new encoder
        let mut encoder = self.create_encoder();
        //clear frame and set background color.
        {
            //Try to reuse this renderpass
            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
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
                    view: &output_view,
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
            //ecs approach:
            // let mut rng = ne_math::rand::thread_rng();
            //TODO query transform as well...
            //TODO make a bundle with mesh and transform
            //TODO how to move this functionality to main?
            let mut query = app.world.query::<(&GpuMesh, &mut Transform)>();
            let iter = query.iter_mut(&mut app.world);
            for (mesh, mut transform) in iter {
                rpass.push_debug_group("Prepare data for draw.");
                rpass.set_pipeline(&mesh.pipeline);
                rpass.set_bind_group(0, &mesh.bind_group, &[]);
                #[cfg(feature = "mesh_16bit")]
                rpass.set_index_buffer(mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
                #[cfg(not(feature = "mesh_16bit"))]
                rpass.set_index_buffer(mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
                rpass.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
                rpass.draw_indexed(0..mesh.index_count as u32, 0, 0..1);

                //modifies transform.
                // let a:f32 = ne_math::rand::Rng::gen_range(&mut rng, -0.06..0.06);
                // transform.pos.y+=a;
                // //optimize/study
                // let mvp_matrix = transform.to_raw();
                // let ref_matrix =mvp_matrix.as_ref();
                // //update location
                // self.queue.write_buffer(
                //     &mesh.model_buffer,
                //     0,
                //     bytemuck::cast_slice(&[ref_matrix.to_owned()]),
                // );
            }
        }
        cmd_buffers.push(
            encoder.finish()
        );
        // cmd_buffers.push(encoder.finish());
        //new encoder
        let mut encoder = self.create_encoder();
        // UI RENDERING! WIll be rendered on top of the previous output
        #[cfg(feature = "editor_ui")]
        {
            let world = app.world.cell();
            let mut ui_state = world.resource_mut::<NUiState>();
            let ctx: &egui::Context = &ui_state.platform.context();
            // Begin to draw the UI frame.
            ui_state.platform.begin_frame();

            let screen_size = ctx.input().screen_rect.size();
            let default_width = (screen_size.x - 20.0).min(400.0);

            egui::Window::new("Control Panel")
                // .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                .default_width(default_width)
                .default_height(ctx.available_rect().height())
                .vscroll(true)
                .open(&mut true)
                .resizable(false)
                .collapsible(true)
                .show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        //TODO suggestion hashmap instead of separate (string bools)...
                        let widget = &mut ui_state.widget_diagnostic;
                        ui.checkbox(&mut widget.1, 
                            &widget.0);
                        let widget = &mut ui_state.widget_file_explorer;
                        ui.checkbox(&mut widget.1, 
                            &widget.0);
                    });
                });
            if ui_state.widget_diagnostic.1 {
                //calculate fps...
                let fps = 1.0 / delta_time;
                let average_fps: f32;
                unsafe {
                    FRAME_COUNT += 1;
                }
                let f = world.resource::<ne_app::FirstFrameTime>().get_time();
                unsafe
                    {
                        let time_passed = (instant::Instant::now() - f).as_secs_f32();
                        average_fps = FRAME_COUNT as f32 / time_passed;
                    }
                egui::Window::new(&ui_state.widget_diagnostic.0)
                    .default_width(default_width)
                    .default_height(ctx.available_rect().height() - 46.0)
                    .vscroll(true)
                    .open(&mut true)
                    .resizable(false)
                    .collapsible(true)
                    .show(ctx, |ui| {
                        ui.heading("fps:");
                        ui.label(format!("current: {}", fps));
                        ui.label(format!("average: {}", average_fps));
                        //wgpu will add this feature soon, hopefully.
                        ui.label(format!("1% low: {}", "todo..."));
                        //do not know how to implement.
                        ui.label(format!("memory usage: {}", "no"));

                        egui::CollapsingHeader::new("cpu").show(ui, |ui| {
                            // for each core:
                            ui.label(format!("core 1: {}", "todo... %"));
                        });
                        egui::CollapsingHeader::new("gpu").show(ui, |ui| {
                            ui.label(format!("usage: {}", "todo... mhz"));
                            ui.label(format!("mem usage: {}", "todo... gb"));
                        });
                    });
            }
            if ui_state.widget_file_explorer.1 {
                egui::Window::new(&ui_state.widget_file_explorer.0)
                    .default_width(default_width)
                    .default_height(ctx.available_rect().height() + 46.0)
                    .vscroll(true)
                    .open(&mut true)
                    .resizable(false)
                    .collapsible(true)
                    .show(ctx, |ui| {
                        ui.horizontal(|ui| {});
                    });
            }
            let ft = world.resource::<ne_app::FirstFrameTime>().get_time();
            let now = instant::Instant::now();
            let time_passed = (now - ft).as_secs_f64();

            ui_state.update_time(time_passed);

            // End the UI frame. We could now handle the output and draw the UI with the backend.
            let full_output = ui_state.platform.end_frame(Some(&world.resource::<NWindow>()));
            let paint_jobs = ui_state.platform.context().tessellate(full_output.shapes);

            // Upload all resources for the GPU.
            let screen_descriptor = user_interface::ui_render_pass::ScreenDescriptor {
                physical_width: world.resource::<NSurfaceConfig>().width,
                physical_height: world.resource::<NSurfaceConfig>().height,
                scale_factor: world.resource::<NWindow>().scale_factor() as f32,
            };
            let tdelta: egui::TexturesDelta = full_output.textures_delta;
            ui_state.render_pass
                .add_textures(&self.device, &self.queue, &tdelta)
                .expect("add texture ok");
            ui_state.render_pass.update_buffers(&self.device, &self.queue, &paint_jobs, &screen_descriptor);

            // Record all render passes.
            ui_state.render_pass
                .execute(
                    &mut encoder,
                    &output_view,
                    &paint_jobs,
                    &screen_descriptor,
                    None,
                )
                .unwrap();
            cmd_buffers.push(encoder.finish());

            // removes ui data for some reason..?
            ui_state.render_pass
                .remove_textures(tdelta)
                .expect("remove texture ok");
        }
        // the number of submit() calls should be limited to a few per frame (e.g. 1-5).
        self.queue.submit(cmd_buffers);
        output_frame.present();

        Ok(())
    }
}
// Stores the handles of meshes loaded on the gpu. 
// TODO implement a weak/strong handle system
// TODO maybe just use entitites for this ("name", GpuMesh)
// #[derive(Resource, Deref)]
// pub struct LoadedMeshes{ 
//     pub map:HashMap<String,GpuMesh>,
// }
// impl LoadedMeshes {
//     fn new() -> Self { LoadedMeshes { map: HashMap::new() } }
// }
///TODO
/// sets runner using .set_runner()
pub struct RenderPlugin;
impl Plugin for RenderPlugin {
    fn setup(&self, app: &mut App) {
        app.add_plugin(ne_window::WindowEventPlugin);
        //prepare resources.
        let event_loop = EventLoop::new();
        let win_settings = app.world.get_resource::<WindowSettings>().unwrap_or(&WindowSettings::default())
        .clone();
        let window = NWindow {0: create_window(&win_settings, &event_loop)};
        let state =
            pollster::block_on(RenderState::new(app, &window, win_settings));
        //initial delta time. only supposed to be read outside this file...
        let delta_time = DeltaTime { time: 0.0 };
        app.insert_resource(delta_time.clone())
        //these resources will be removed in loop.
        .insert_non_send_resource(event_loop)
        .insert_non_send_resource(window)
        .insert_resource(state)

        .set_runner(main_loop);
    }
}
#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
fn main_loop(mut app: App) {
    let event_loop =
        app.world.remove_non_send_resource::<EventLoop<()>>().unwrap();
    let mut state = app.world.remove_resource::<RenderState>().unwrap();

    //benchmark values.
    let mut once_benchmark = true;
    let mut last_render_time = instant::Instant::now();
    //exit window event reader
    let mut app_exit_event_reader = ManualEventReader::<ExitApp>::default();
    //TODO this really needs to be the first thing to ever be called in this game engine? the engine needs a kind of startup logic that will always be performed first to prevent horrible unwrap errors.
    let event_handler =
        move |event: Event<()>,
              _: &EventLoopWindowTarget<()>, //not sure what to do with event_loop: &EventLoopWindowTarget
              control_flow: &mut ControlFlow| {
            //maybe move this after if !state.input(event) {
            match event {
                // I wonder if this is the better place for app.update()
                event::Event::NewEvents(_) => {
                    // //TODO commands to instruct
                    // app.world.resource::<NWindow>().set_cursor_visible(false);
                    // _ = app.world.resource::<NWindow>().set_cursor_position(
                    //     PhysicalPosition::new(app.world.resource::<NWindow>().inner_size().width / 2, app.world.resource::<NWindow>().inner_size().height / 2));
                    app.update();
                }
                event::Event::LoopDestroyed => {
                    {
                        let world = app.world.cell();
                        let mut window_close_requested_events =
                            world.resource_mut::<Events<ExitSequence>>();
                        window_close_requested_events.send(
                            ExitSequence {});
                    }
                    //last update
                    app.update();
                }
                event::Event::MainEventsCleared => {
                    app.world.resource::<NWindow>().request_redraw();
                }
                event::Event::WindowEvent {
                    event,
                    window_id,
                } => {
                        // let world = app.world.cell();
                        if window_id == app.world.resource::<NWindow>().id() {
                            #[cfg(feature = "editor_ui")]
                            app.world.resource_mut::<NUiState>().handle_event(&event);
                            match event {
                                WindowEvent::CloseRequested => {
                                    let mut window_close_requested_events =
                                    app.world.resource_mut::<Events<OnWindowCloseRequested>>();
                                    window_close_requested_events.send(
                                        OnWindowCloseRequested { id: window_id });
                                    *control_flow = ControlFlow::Exit;
                                }
                                WindowEvent::KeyboardInput {
                                    input:
                                    KeyboardInput {
                                        virtual_keycode: Some(key),
                                        state,
                                        ..
                                    },
                                    ..
                                } => {
                                    let mut keyboard_input_events =
                                    app.world.resource_mut::<Events<OnKeyboardInput>>();
                                    keyboard_input_events.send(OnKeyboardInput::new(key, state));
                                }
                                WindowEvent::Resized(physical_size) => {
                                    let world = app.world.cell();
                                    let mut surface = world.resource_mut::<NSurfaceConfig>();
                                    let mut projection = world.resource_mut::<Projection>();
                                    //TODO MOVE TASK: decouple window and renderer
                                    state.resize(physical_size, 
                                    &mut surface,
                                        &mut projection);
                                    let mut resize_events
                                        = world.resource_mut::<Events<OnWindowResized>>();
                                    resize_events.send(OnWindowResized {
                                        id: window_id,
                                        width: world.resource::<NWindow>().inner_size().width as f32,
                                        height: world.resource::<NWindow>().inner_size().height as f32,
                                    });
                                }
                                WindowEvent::ScaleFactorChanged { scale_factor, new_inner_size } => {
                                    let world = app.world.cell();
                                    let mut surface = world.resource_mut::<NSurfaceConfig>();
                                    let mut projection = world.resource_mut::<Projection>();
                                    state.resize(*new_inner_size, 
                                     &mut surface,
                                     &mut projection);
                                    let mut scale_event = world.resource_mut
                                        ::<Events<OnWindowScaleFactorChanged>>();
                                    scale_event.send(OnWindowScaleFactorChanged
                                    { id: window_id, scale_factor })
                                }
                                WindowEvent::Focused(focused)
                                => {
                                    let mut focused_events =
                                    app.world.resource_mut::<Events<OnWindowFocused>>();
                                    focused_events.send(OnWindowFocused {
                                        id: window_id,
                                        //TODO why does this have to be dereferenced?
                                        focused,
                                    });
                                }
                                WindowEvent::HoveredFile(path_buf) => {
                                    let mut events = app.world.resource_mut::<Events<OnFileDragAndDrop>>();
                                    events.send(OnFileDragAndDrop::HoveredFile {
                                        id: window_id,
                                        path_buf: path_buf.to_path_buf(),
                                    });
                                }
                                WindowEvent::DroppedFile(path_buf) => {
                                    let mut events = app.world.resource_mut::<Events<OnFileDragAndDrop>>();
                                    events.send(OnFileDragAndDrop::DroppedFile {
                                        id: window_id,
                                        path_buf: path_buf.to_path_buf(),
                                    });
                                }
                                WindowEvent::CursorEntered { .. /* device id needed? */ }
                                => {
                                    let mut cursor_entered_events =
                                    app.world.resource_mut::<Events<OnCursorEntered>>();
                                    cursor_entered_events.send(OnCursorEntered { id: window_id });
                                }
                                WindowEvent::CursorLeft { .. } => {
                                    let mut cursor_left_event
                                        = app.world.resource_mut::<Events<OnCursorLeft>>();
                                    cursor_left_event.send(OnCursorLeft { id: window_id });
                                }
                                //TODO TEST
                                WindowEvent::ReceivedCharacter(c) => {
                                    let mut char_input_events =
                                    app.world.resource_mut::<Events<OnReceivedCharacter>>();

                                    char_input_events.send(OnReceivedCharacter {
                                        id: window_id,
                                        //TODO this dereference hits performance?
                                        char: c,
                                    });
                                }
                                WindowEvent::MouseInput { state, button, .. } => {
                                    let mut mouse_button_input_events =
                                    app.world.resource_mut::<Events<OnMouseButton>>();
                                mouse_button_input_events.send(OnMouseButton {
                                    state,
                                    button,
                                });
                                }
                                WindowEvent::MouseWheel { delta, .. } => {
                                    let mut mouse_wheel_event =
                                    app.world.resource_mut::<Events<OnMouseWheel>>();
                                    mouse_wheel_event.send(OnMouseWheel{delta});

                                }
                                _ => {}
                            }
                        }
                    }
                //TODO move
                event::Event::DeviceEvent {
                    event: DeviceEvent::MouseMotion { delta, }, .. }
                    => {
                    //TODO we need a mouse event
                    let mut mouse_motion_events = app.world.resource_mut::<Events<OnMouseMotion>>();
                    mouse_motion_events.send(OnMouseMotion {
                        delta: Vec2::new(delta.0 as f32, delta.1 as f32),
                    });
                }
                event::Event::RedrawRequested(window_id) if window_id == app.world.resource::<NWindow>().id() => {
                    //hope this gets optimized
                    if once_benchmark //do once
                    {
                        app.insert_resource(ne_app::FirstFrameTime::default());
                        once_benchmark = false;
                    }
                    //is it expensive to use delta_time as a resource?
                    let now = instant::Instant::now();
                    let delta_time = DeltaTime { time: (now - last_render_time).as_secs_f32() };
                    app.insert_resource(delta_time.clone());
                    let delta_time = delta_time.time;
                    last_render_time = now;
                    
                    match state.render(&mut app, delta_time) {
                        Ok(_) => {}
                        // Reconfigure the surface if it's lost or outdated
                        Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                            let world = app.world.cell();
                            let mut surface = world.resource_mut::<NSurfaceConfig>();
                            let mut projection = world.resource_mut::<Projection>();
                            state.resize(state.size, 
                                &mut surface,
                            &mut projection);
                        }
                        // The system is out of memory, we should probably quit
                        Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                        // We're ignoring timeouts
                        Err(wgpu::SurfaceError::Timeout) => warn!("Surface timeout"),
                    }
                    //better?
                    // let world = app.world.cell();
                    let mut frame_events = app.world.resource_mut::<Events<OnRedrawRequested>>();
                    frame_events.send(OnRedrawRequested {});
                    Render::update_camera_buffer(&state.queue, app.world.resource::<NCameraBuffer>(), &app.world.resource::<CameraUniform>());
                }
                event::Event::RedrawEventsCleared => {
                    let app_exit_events =
                    app.world.resource::<Events<ExitApp>>();
                    if app_exit_event_reader.iter(app_exit_events).last().is_some() {
                        *control_flow = ControlFlow::Exit;
                    }
                }
                _ =>
                    {}
            }
        };
    event_loop.run(event_handler);
}
/// ================================================================================================
/// Window functionality
/// ================================================================================================
pub use wgpu::PresentMode;
/// Describes the information needed for creating a window.
///
/// This should be set up before adding the [`WindowPlugin`](crate::WindowPlugin).
/// Most of these settings can also later be configured through the [`Window`](crate::Window) resource.
///
/// See [`examples/window/window_settings.rs`] for usage.
///
/// [`examples/window/window_settings.rs`]: https://github.com/bevyengine/bevy/blob/latest/examples/window/window_settings.rs
#[derive(Debug, Resource, Clone)]
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
    //does this work..?
    modes.sort_by(|a, b| {
        use std::cmp::Ordering::*;
        match abs_diff(a.size().width, width).cmp(&abs_diff(b.size().width, width)) {
            Equal => {
                match abs_diff(a.size().height, height).cmp(&abs_diff(b.size().height, height)) {
                    Equal => b.refresh_rate_millihertz().cmp(&a.refresh_rate_millihertz()),
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
        WindowMode::Windowed => {}
        WindowMode::BorderlessFullscreen => {
            // let mut monitor_index = 0; //todo
            let monitor = event_loop
                .available_monitors()
                .next()
                .expect("no monitor found!");
            let fullscreen = Some(Fullscreen::Borderless(Some(monitor.clone())));
            info!("Setting mode: {:?}", fullscreen);
            wind = wind.with_fullscreen(fullscreen);
        }
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
            )), );
            info!("Setting mode: {:?}", fullscreen);
            wind = wind.with_fullscreen(fullscreen);
        }
    }
    //TODO
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
/// maximized it may have a size outside of these limits...
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
