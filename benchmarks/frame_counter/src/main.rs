// I don't know how to write tests, this will do for now!

// use core::time;
use bevy_ecs::prelude::{EventWriter, EventReader};
use nightmare_engine::*;

use ne_app::{App, get_time_passed};
use ne_render::{RenderPlugin, WindowSettings, AppExit, FrameEvent};

fn gui_event_system()
{

}
pub static mut TOTAL_TIME:Option<instant::Duration> = None;
static mut frame_count: u32 = 0;
fn bench(mut frame_event: EventReader<FrameEvent>,
  mut exit: EventWriter<AppExit>)
{
  unsafe {
    for _ in frame_event.iter().rev() {
      frame_count+=1;
      // println!("frame count: {}", frame_count);
      const MAX:u32 = 25_000;
      if frame_count > MAX
      {
        let t = get_time_passed(ne_app::FIRST_FRAME_TIME);
        println!("to render: {} frames took: {:?}",MAX, t); //write to results/*
        //TODO this is messed up
        //I want it to write to afile these details, together with:
        //window settings that impact performance like resolution and quality 

        exit.send(AppExit);
      }
  }
  }
}

//TOOD modularly add different types of camera and input events...
//1) orbit camera, with predetermined rotation and look at point.
//2) WASD flying first person camera
fn main() {
    // env::set_var("RUST_BACKTRACE", "1");
  L::init_log!(tracing::Level::ERROR);
    App::new()
        .insert_resource(WindowSettings {
            title: "Nightmare_Editor".to_string(),
            window_mode: ne_render::WindowMode::Windowed,
            ..WindowSettings::default()
        })
        .add_plugin(RenderPlugin)
        .add_system(bench)
        .run();
}

//random conclusions:
//fps counter has minimal performance impact on --release but significant on debug.
//window down and window focused cost the same, this still has to be optimized maybe

//questions still:
//what do events cost?
//are globals cheaper?
//how much does resolution affect performance.

//But all of this should be with a spinning camera instead of 
//a frozen camera.


//without events 
//1)26.574561s  | fps:990.68756     fps | avg:940.8561      fps | 1%LOW:990.68756 fps
//2)26.1475108s | fps:1125.9993     fps | avg:956.24023     fps | 1%LOW:1125.9993 fps
//3)26.3261607s | fps:1141.0315     fps | avg:949.733       fps | 1%LOW:582.479   fps
//This is with events added:
//1)
//2)
//3)
