use std::borrow::Cow;
use instant::Instant;

use ::egui::FontDefinitions;
use egui_demo_lib::DemoWindows;
use egui_winit_platform::{Platform, PlatformDescriptor};

use self::render_pass::RenderPassRecipe;

pub mod render_pass;

// /// A custom event type for the winit app.
// enum Event {
//     RequestRedraw,
// }

// /// This is the repaint signal type that egui needs for requesting a repaint from another thread.
// /// It sends the custom RequestRedraw event to the winit event loop.
// struct ExampleRepaintSignal(std::sync::Mutex<winit::event_loop::EventLoopProxy<Event>>);

// impl epi::backend::RepaintSignal for ExampleRepaintSignal {
//     fn request_repaint(&self) {
//         self.0.lock().unwrap().send_event(Event::RequestRedraw).ok();
//     }
// }

// //TODO HONESTLY IT'S better just putting in a mutable RendererState ...? 
// //We need to clean state first tho, it has to many members
pub struct ScreenDescriptor {
    /// Width of the window in physical pixel.
    pub physical_width: u32,
    /// Height of the window in physical pixel.
    pub physical_height: u32,
    /// HiDPI scale factor.
    pub scale_factor: f32,
}

impl ScreenDescriptor {
    fn logical_size(&self) -> (u32, u32) {
        let logical_width = self.physical_width as f32 / self.scale_factor;
        let logical_height = self.physical_height as f32 / self.scale_factor;
        (logical_width as u32, logical_height as u32)
    }
}

pub struct EguiState {
    pub platform:Platform,
    pub render_pass:RenderPassRecipe,
    demo_window:DemoWindows,
    start_time:Instant,
    //TODO is lifetime implemented correctly?
    // device:&'a wgpu::Device,
    // surface_config:&'a wgpu::SurfaceConfiguration,
}
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
            // device,
            // surface_config 
        }

        //THIS ALL NEEDS TO BE IN UPDATES OR WELL, INDEPENDENT UPDATES
        //RedrawRequested needs to use the output frame of the previous RenderPass

        // event_loop.run(move |event, _, control_flow| {
        //     // Pass the winit events to the platform integration.

        //TODO THIS IS IMPORTANT 
        //it handles mouse and keyboard for egui? Or even more?
        //So this is litterally everythignt aht needs to be in the winit loop???
            


    }
    //TODO just put     ``state.ui_ctx.platform.handle_event(&event)`` at top of eventloop
    // fn input<T>(&self, winit_event: &winit::Event<T>)
    // {
    //     self.platform.handle_event(&event);
    // }

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
// impl EguiState<'_> {
//     //this should return egui_state?
//     /// A simple egui + wgpu + winit based example.
//     fn setup(
//         &mut self,
//         window:&winit::window::Window,
//         surface:&wgpu::Surface, 
//         device:&wgpu::Device,
//         queue:&wgpu::Queue,
//         surface_config:&wgpu::SurfaceConfiguration,
//         adapter:&wgpu::Adapter,
//         //adapter needed?
//         surface_format:&wgpu::TextureFormat,
//         ) {
//         //TODO
//         let size = window.inner_size();
//         // We use the egui_winit_platform crate as the platform.
//         self.platform = Platform::new(PlatformDescriptor {
//             physical_width: size.width as u32,
//             physical_height: size.height as u32,
//             scale_factor: window.scale_factor(),
//             font_definitions: FontDefinitions::default(),
//             style: Default::default(),
//         });

//         //TODO this needs to be setup and stored and used in winit loop... A stored RenderPass
//         // We use the egui_wgpu_backend crate as the render backend.
//         self.render_pass = RenderPassRecipe::new(
//             &device, *surface_format,
//              1, 
//              wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("egui.wgsl")))
//             );
//         // Display the demo application that ships with egui.
//         self.demo_window = egui_demo_lib::DemoWindows::default();
//         self.start_time = Instant::now();



//         //THIS ALL NEEDS TO BE IN UPDATES OR WELL, INDEPENDENT UPDATES
//         //RedrawRequested needs to use the output frame of the previous RenderPass

//         // event_loop.run(move |event, _, control_flow| {
//         //     // Pass the winit events to the platform integration.

//         //TODO THIS IS IMPORTANT 
//         //it handles mouse and keyboard for egui? Or even more?
//         //So this is litterally everythignt aht needs to be in the winit loop???
            
//         self.platform.handle_event(&event);

//     }
//     /// Time of day as seconds since midnight. Used for clock in demo app.
//     pub fn seconds_since_midnight() -> f64 {
//         //returns 10.0 instead of time
//         10.0
//     }
//     //TODO this render_pass somehow needs the texture view? or well the output of the previous render_pass? 
//     ///put in the event redraw_requested.
//     /// 
//     /// I am unfamilar with how to implement this with lifetimes...
//     pub fn redraw_ui(&mut self, queue:&mut wgpu::Queue)
//     {
//         self.platform.update_time( self.start_time.elapsed().as_secs_f64());

//         let output_frame = match surface.get_current_texture() {
//             Ok(frame) => frame,
//             Err(wgpu::SurfaceError::Outdated) => {
//                 // This error occurs when the app is minimized on Windows.
//                 // Silently return here to prevent spamming the console with:
//                 // "The underlying surface has changed, and therefore the swap chain must be updated"
//                 return;
//             }
//             Err(e) => {
//                 ene::log!("Dropped frame with error: {}", e);
//                 return;
//             }
//         };
//         let output_view = output_frame
//             .texture
//             .create_view(&wgpu::TextureViewDescriptor::default());

//         // Begin to draw the UI frame.
//         self.platform.begin_frame();

//         // Draw the demo application.
//         self.demo_window.ui(&self.platform.context());

//         // End the UI frame. We could now handle the output and draw the UI with the backend.
//         let full_output = self.platform.end_frame(Some(&window));
//         let paint_jobs = self.platform.context().tessellate(full_output.shapes);

//         let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
//             label: Some("encoder"),
//         });

//         // Upload all resources for the GPU.
//         let screen_descriptor = ScreenDescriptor {
//             physical_width: self.surface_config.width,
//             physical_height: self.surface_config.height,
//             scale_factor: window.scale_factor() as f32,
//         };
//         let tdelta: egui::TexturesDelta = full_output.textures_delta;
//         self.render_pass
//             .add_textures(&self.device, queue, &tdelta)
//             .expect("add texture ok");
//         self.render_pass.update_buffers(&device, &queue, &paint_jobs, &screen_descriptor);

//         // Record all render passes.
//         self.render_pass
//             .execute(
//                 &mut encoder,
//                 &output_view,
//                 &paint_jobs,
//                 &screen_descriptor,
//                 Some(wgpu::Color::BLACK),
//             )
//             .unwrap();
//         // Submit the commands.
//         queue.submit(iter::once(encoder.finish()));

//         // Redraw egui
//         output_frame.present();

//         self.render_pass
//             .remove_textures(tdelta)
//             .expect("remove texture ok");

//         // Suppport reactive on windows only, but not on linux.
//         // if _output.needs_repaint {
//         //     *control_flow = ControlFlow::Poll;
//         // } else {
//         //     *control_flow = ControlFlow::Wait;
//         // }
//     }
// }