// use std::thread;

// use ne_app::Plugin;
pub mod basic_window;
use ne_app::{App, Plugin};
use ne_gui::NUserInterface;
use ne_render::{WindowSettings};

use self::basic_window::BasicWindow;

// UserInterface::new(size)

//TODO do once instead of system...
// fn main_system(world: &mut World) {
//     let nui = world.get_non_send_resource::<NUserInterface>().unwrap();
// }

// fn render_ui(world:&World,mut frame_event: EventReader<FrameEvent>)
// {
//     for _ in frame_event.iter().rev() {
//         println!("FRAME UNO");

//         let ui = 

//         // let ctx = ui.draw();
//     }
// }
fn init_res()
{

}
pub struct EditorPlugin;
impl Plugin for EditorPlugin {
    fn setup(&self, app: &mut App) {

        let w = app.world.get_resource::<WindowSettings>().unwrap();
        //insert NUserInterface
        app.insert_non_send_resource::<NUserInterface>(NUserInterface::new(w.width, w.height));
        //insert basic window
        let a = BasicWindow::new(&mut app.world);
        app.insert_non_send_resource::<BasicWindow>(a);
        //add loop on main thread
        // app.add_system(render_ui.exclusive_system());
    }
}
