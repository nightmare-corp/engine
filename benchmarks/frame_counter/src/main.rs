// I don't know how to write tests, this will do for now!

// use core::time;
use bevy_ecs::prelude::{EventReader, EventWriter};
use nightmare_engine::*;

use ne_app::{get_time_passed, App};
use ne_render::{AppExit, OnRedrawRequested, OnWindowCloseRequested, RenderPlugin, WindowSettings};

pub static mut TOTAL_TIME: Option<instant::Duration> = None;
static mut frame_count: u32 = 0;
fn bench(mut frame_event: EventReader<OnRedrawRequested>, mut exit: EventWriter<AppExit>) {
    unsafe {
        for _ in frame_event.iter().rev() {
            frame_count += 1;
            const MAX: u32 = 25_000;
            if frame_count > MAX {
                let t = get_time_passed(ne_app::FIRST_FRAME_TIME);
                println!("to render: {} frames took: {:?}", MAX, t); //write to results/*
                                                                     //TODO this is messed up
                                                                     //I want it to write to afile these details, together with:
                                                                     //window settings that impact performance like resolution and quality

                exit.send(AppExit);
            }
        }
    }
}
fn gui_event_system(mut frame_event: EventReader<OnRedrawRequested>) {
    for _ in frame_event.iter().rev() {
        //reference 
        // data.platform.update_time(data.start_time.elapsed().as_secs_f64());
        // data.platform.begin_frame();

        // // Insert egui commands here
        // let ctx = data.platform.context();
        // egui::Window::new("Change color").resizable(true).show(&ctx, |ui| {
        //     ui.label("Change the color of the cube");
        //     if ui.color_edit_button_rgba_unmultiplied(&mut data.color).changed() {
        //         renderer.update_material(
        //             &data.material_handle.clone(),
        //             rend3_routine::pbr::PbrMaterial {
        //                 albedo: rend3_routine::pbr::AlbedoComponent::Value(glam::Vec4::from(data.color)),
        //                 transparency: rend3_routine::pbr::Transparency::Blend,
        //                 ..rend3_routine::pbr::PbrMaterial::default()
        //             },
        //         );
        //     }
        //     ui.label("Want to get rusty?");
        //     if ui
        //         .add(egui::widgets::ImageButton::new(self.rust_logo, egui::Vec2::splat(64.0)))
        //         .clicked()
        //     {
        //         webbrowser::open("https://www.rust-lang.org").expect("failed to open URL");
        //     }
        // });

        // // End the UI frame. Now let's draw the UI with our Backend, we could also
        // // handle the output here
        // let egui::FullOutput {
        //     shapes, textures_delta, ..
        // } = data.platform.end_frame(Some(window));
        // let paint_jobs = data.platform.context().tessellate(shapes);

        // let input = rend3_egui::Input {
        //     clipped_meshes: &paint_jobs,
        //     textures_delta,
        //     context: data.platform.context(),
        // };

        // // Get a frame
        // let frame = rend3::util::output::OutputFrame::Surface {
        //     surface: Arc::clone(surface.unwrap()),
        // };

        // // Ready up the renderer
        // let (cmd_bufs, ready) = renderer.ready();

        // // Lock the routines
        // let pbr_routine = rend3_framework::lock(&routines.pbr);
        // let tonemapping_routine = rend3_framework::lock(&routines.tonemapping);

        // // Build a rendergraph
        // let mut graph = rend3::graph::RenderGraph::new();

        // // Add the default rendergraph without a skybox
        // base_rendergraph.add_to_graph(
        //     &mut graph,
        //     &ready,
        //     &pbr_routine,
        //     None,
        //     &tonemapping_routine,
        //     resolution,
        //     SAMPLE_COUNT,
        //     glam::Vec4::ZERO,
        //     glam::Vec4::new(0.10, 0.05, 0.10, 1.0), // Nice scene-referred purple
        // );

        // // Add egui on top of all the other passes
        // let surface = graph.add_surface_texture();
        // data.egui_routine.add_to_graph(&mut graph, input, surface);

        // // Dispatch a render using the built up rendergraph!
        // graph.execute(renderer, frame, cmd_bufs, &ready);

        // control_flow(winit::event_loop::ControlFlow::Poll);


    }

}

//TOOD modularly add different types of camera and input events...
//1) orbit camera, with predetermined rotation and look at point.
//2) WASD flying first person camera
fn main() {
    // std::env::set_var("RUST_BACKTRACE", "1");
    // vulkan, metal, dx12, dx11, or gl
    // std::env::set_var("WGPU_BACKEND", "vulkan");

    const width: f32 = 1600.0;
    const height: f32 = 900.0;

    L::init_log!(tracing::Level::ERROR);
    App::new()
        .insert_resource(WindowSettings {
            title: "Nightmare_Editor".to_string(),
            window_mode: ne_render::WindowMode::Windowed,
            width: width,
            height: height,
            ..WindowSettings::default()
        })
        .add_plugin(RenderPlugin)
        .add_system(bench)
        .add_system(exit_window)
        .run();
}
fn exit_window(mut window_close_requested: EventReader<OnWindowCloseRequested>) {
    for event in window_close_requested.iter().rev() {
        //TODO GUI would you like to save? Yes, No, Cancel.
        println!("Would you like to save?");
        println!("exiting program");
        //Doesn't call any destructors, maybe a bad idea?
        std::process::exit(0);
    }
}
//random conclusions:
//fps counter has minimal performance impact on --release but significant on debug.
//window down and window focused cost the same, except on lower resolutions. This still has to be optimized ofcourse

//But all of this should be with a obitting/spinning camera instead of
//a frozen camera.

//To measure:
// 1) measure overhead of OnRedrawRequested
// 2) measure function vs #[inline] vs macro 
// ^^Both of these with egui ui. 

//resolution 100x100
//1)17.4363505s | fps:1751.9271     fps | avg:1433.9448     fps | 1%LOW:1751.9271 fps
//2)21.6829268s | fps:1740.0382     fps | avg:1153.1024     fps | 1%LOW:1730.1039 fps //window down
//3)19.0872487s | fps:1006.13745    fps | avg:1309.9307     fps | 1%LOW:1006.13745fps
//resolution 1000x1000
//1)25.4664988s  | fps:987.55676     fps | avg:981.798       fps | 1%LOW:987.55676 fps
//2)27.1185229s  | fps:1555.21       fps | avg:921.9817      fps | 1%LOW:983.76776 fps //window down
//resolution 2000x2000
//1)55.4715823s  | fps:348.94272     fps | avg:450.73666     fps | 1%LOW:348.94272 fps
//2)55.4501635s  | fps:554.0473      fps | avg:450.90796     fps | 1%LOW:508.41428 fps //window down
//this test was not meant to be accurate just to detect a significant differences in performance if present
//conclusion: resolution has a big impact on performance, suprisingly focusing a small window improves performance greatly


//dx12 vs vulkan
//vulkan: to render: 25000 frames took: 18.8669649s
// fps:1349.5277     fps | avg:1325.2115     fps | 1%LOW:1349.5277 fps
//
//dx12: to render: 25000 frames took: 35.5932863s
// fps:588.3737      fps | avg:702.4591      fps | 1%LOW:588.3737  fps
//
//default: to render: 25000 frames took: 18.8244332s
// fps:1527.6505     fps | avg:1328.2094     fps | 1%LOW:1527.6505 fps

//HMMM IS THIS RIGHT?