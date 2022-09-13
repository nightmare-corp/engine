use std::borrow::Cow;
use instant::Instant;
use ::egui::FontDefinitions;
use egui_demo_lib::DemoWindows;
// use egui_winit_platform::{Platform, PlatformDescriptor};
use self::{ui_render_pass::RenderPassRecipe};
use self::{egui_window::{Platform, PlatformDescriptor}};
mod egui_window;
pub mod ui_render_pass;

pub struct EguiState {
    pub platform:Platform,
    pub render_pass:RenderPassRecipe,
    demo_window:DemoWindows,
    start_time:Instant,
}

//TODO see if it's faster to use these reference instead.
impl EguiState {
    //this should return egui_state?
    /// A simple egui + wgpu + winit based example.
    pub fn new(
        window:&winit::window::Window,
        surface:&wgpu::Surface, 
        device:&wgpu::Device,
        queue:&wgpu::Queue,
        surface_config:&wgpu::SurfaceConfiguration,
        adapter:&wgpu::Adapter,
        //adapter needed?
        surface_format:&wgpu::TextureFormat,
        ) -> Self { 
        //TODO
        let size = window.inner_size();
        // We use the egui_winit_platform crate as the platform.
        let platform = Platform::new(PlatformDescriptor {
            physical_width: size.width as u32,
            physical_height: size.height as u32,
            scale_factor: window.scale_factor(),
            font_definitions: FontDefinitions::default(),
            style: Default::default(),
        });

        //TODO this needs to be setup and stored and used in winit loop... A stored RenderPass
        // We use the egui_wgpu_backend crate as the render backend.
        //TODO move to state, the main state struct, because the state should own all RenderPassRecipes in a Vec..? 
        let render_pass = RenderPassRecipe::new(
            &device, *surface_format,
             1, 
             wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("egui.wgsl")))
        );
        // Display the demo application that ships with egui.
        let demo_window = egui_demo_lib::DemoWindows::default();
        let start_time = Instant::now();

        Self {
            platform, render_pass,
            demo_window, start_time,
        }

    }
    pub fn update_time(&mut self)
    {
        self.platform.update_time(self.start_time.elapsed().as_secs_f64());
    }
    pub fn begin_frame(&mut self)
    {
        self.platform.begin_frame();
    }
    pub fn end_frame(&mut self, window:&winit::window::Window) -> egui::FullOutput 
    {
        self.platform.end_frame(Some(window))
    }
    pub fn draw_ui(&mut self)
    {
        self.demo_window.ui(&self.platform.context());
    }
    pub fn handle_event<T>(&mut self, winit_event: &winit::event::Event<T>)
    {
        self.platform.handle_event(winit_event);
    }
}