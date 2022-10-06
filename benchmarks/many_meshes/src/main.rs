// I don't know how to write tests, this will do for now!

use bevy_ecs::prelude::EventReader;
use bevy_ecs::prelude::{EventWriter};
use ne_app::{get_time_passed, App};
use ne_render::{ RenderPlugin, WindowSettings};
use ne_window::events::{OnWindowCloseRequested, OnRedrawRequested, AppExit};

//TOOD modularly add different types of camera and input events...
//1) orbit camera, with predetermined rotation and look at point.
//2) WASD flying first person camera
fn main() {
    // std::env::set_var("RUST_BACKTRACE", "1");
    // vulkan, metal, dx12, dx11, or gl
    // std::env::set_var("WGPU_BACKEND", "vulkan");
    std::env::set_var("neprint", "true");
    const WIDTH: f32 = 1600.0;
    const HEIGHT: f32 = 900.0;

    ne::L::init_log!(tracing::Level::ERROR);
    App::new()
        .add_startup_system(setup_once)
        .insert_resource(WindowSettings {
            title: "Nightmare_Editor".to_string(),
            window_mode: ne_render::WindowMode::Windowed,
            width: WIDTH,
            height: HEIGHT,
            ..WindowSettings::default()
        })
        .add_plugin(RenderPlugin)
        .add_system(bench)
        .add_system(exit_window)
        .run();
}
fn setup_once() 
{
    println!("hey so this is supposed to be the first message of app ");
    /*         .insert_resource(SceneLoader {
            
        }) */
}
fn exit_window(mut window_close_requested: EventReader<OnWindowCloseRequested>) {
    for _ in window_close_requested.iter().rev() {
        //TODO GUI would you like to save? Yes, No, Cancel.
        ne::log!("Would you like to save?");
        ne::log!("exiting program");
        //Doesn't call any destructors, maybe a bad idea?
        std::process::exit(0);
    }
}

/// exit after max is reached
pub static mut TOTAL_TIME: Option<instant::Duration> = None;
static mut FRAME_COUNT: u32 = 0;
fn bench(mut frame_event: EventReader<OnRedrawRequested>, mut exit: EventWriter<AppExit>) {
    unsafe {
        for _ in frame_event.iter().rev() {
            FRAME_COUNT += 1;
            const MAX_FRAME_COUNT: u32 = 25_000;
            if FRAME_COUNT > MAX_FRAME_COUNT {
                let t = get_time_passed(ne_app::FIRST_FRAME_TIME);
                ne::log!("to render: {} frames took: {:?}", MAX_FRAME_COUNT, t);
                exit.send(AppExit);
            }
        }
    }
}
