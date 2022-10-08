use std::borrow::Cow;
use ::egui::FontDefinitions;
use self::basic_window::{ControlWindow, UserInterface};
// use egui_winit_platform::{Platform, PlatformDescriptor};
use self::{ui_render_pass::RenderPassRecipe};
use self::{egui_window::{Platform, PlatformDescriptor}};
mod egui_window;
pub mod ui_render_pass;
// pub mod editor_widget;
mod basic_window;
pub struct EguiState {
    pub platform:Platform,
    pub render_pass:RenderPassRecipe,
    demo_window:ControlWindow,
}

//TODO see if it's faster to use these reference instead.
impl EguiState {
    //this should return egui_state?
    /// A simple egui + wgpu + winit based example.
    pub fn new(
        window:&winit::window::Window,
        device:&wgpu::Device,
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
        let demo_window = ControlWindow::default();

        Self {
            platform, render_pass,
            demo_window, 
        }

    }
    pub fn update_time(&mut self, elapsed_time_as_seconds:f64)
    {
        self.platform.update_time(elapsed_time_as_seconds);
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
        self.demo_window.update(&self.platform.context());
    }
    pub fn handle_event<T>(&mut self, winit_event: &winit::event::Event<T>)
    {
        self.platform.handle_event(winit_event);
    }
}