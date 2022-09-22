use ne_app::{Plugin, App};


pub mod events;

pub struct WindowEventPlugin;
impl Plugin for WindowEventPlugin {
    fn setup(&self, app: &mut App) {
        app
        .add_event::<events::OnWindowResized>()
        .add_event::<events::AppExit>()
        .add_event::<events::OnRedrawRequested>()
        .add_event::<events::OnWindowCloseRequested>()
        .add_event::<events::OnWindowResized>()
//      .add_event::<events::OnWindowMoved>()
        .add_event::<events::OnWindowScaleFactorChanged>()
        .add_event::<events::OnFileDragAndDrop>()
        .add_event::<events::OnCursorMoved>()
        .add_event::<events::OnCursorEntered>()
        .add_event::<events::OnCursorLeft>()
        .add_event::<events::OnReceivedCharacter>()
        .add_event::<events::OnWindowFocused>();
        //todo
/*     
        .add_event::events::<CreateWindow>()
        .add_event::events::<WindowCreated>()
        .add_event::events::<WindowClosed>()
        .add_event::events::<WindowBackendScaleFactorChanged>()
*/
    }
}