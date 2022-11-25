use ::egui::FontDefinitions;
use winit::event::WindowEvent;
use std::borrow::Cow;
// use egui_winit_platform::{Platform, PlatformDescriptor};
use self::egui_window::{Platform, PlatformDescriptor};
use self::ui_render_pass::RenderPassRecipe;
mod egui_window;
pub mod ui_render_pass;
//TODO extendable ui with egui.
//1) zero overhead
//2) immediate ui function callable from render()
//3) bool to hide and show ui.

// pub mod editor_widget;
mod basic_window;
pub struct EditorUIState {
    pub platform: Platform,
    pub render_pass: RenderPassRecipe,
    pub widget_diagnostic: (String, bool),
    pub widget_file_explorer: (String, bool),
    // pub optional_widgets: Vec<(String, bool)>,
}
//TODO see if it's faster to use these reference instead.
impl EditorUIState {
    //this should return egui_state?
    /// A simple egui + wgpu + winit based example.
    pub fn new(
        window: &winit::window::Window,
        device: &wgpu::Device,
        surface_format: &wgpu::TextureFormat,
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
            &device,
            *surface_format,
            1,
            wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("egui.wgsl"))),
        );
        // Display the demo application that ships with egui.
        Self {
            platform,
            render_pass,
            widget_diagnostic: ("Diagnostics Window".to_owned(), true),
            widget_file_explorer: ("File Explorer".to_owned(), false),
        }
    }
    pub fn update_time(&mut self, elapsed_time_as_seconds: f64) {
        self.platform.update_time(elapsed_time_as_seconds);
    }
    pub fn handle_event(&mut self, event: &WindowEvent) {
        self.platform.handle_event(event);
    }
}
