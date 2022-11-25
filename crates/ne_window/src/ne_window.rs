use ne_app::{App, Plugin};
pub mod events;

pub struct WindowEventPlugin;
impl Plugin for WindowEventPlugin {
    fn setup(&self, app: &mut App) {
        //this is bad atm.
        app.add_event::<events::OnWindowResized>()
            .add_event::<events::ExitApp>()
            .add_event::<events::OnRedrawRequested>()
            .add_event::<events::OnWindowCloseRequested>()
            .add_event::<events::ExitSequence>()
            .add_event::<events::OnWindowResized>()
            //      .add_event::<events::OnWindowMoved>()
            .add_event::<events::OnWindowScaleFactorChanged>()
            .add_event::<events::OnFileDragAndDrop>()
            .add_event::<events::OnCursorMoved>()
            .add_event::<events::OnCursorEntered>()
            .add_event::<events::OnCursorLeft>()
            .add_event::<events::OnReceivedCharacter>()
            .add_event::<events::OnWindowFocused>()
            .add_event::<events::OnKeyboardInput>()
            
            .add_event::<events::OnMouseMotion>()
            .add_event::<events::OnMouseWheel>()
            .add_event::<events::OnMouseButton>();
        //todo
        /*
                .add_event::events::<CreateWindow>()
                .add_event::events::<WindowCreated>()
                .add_event::events::<WindowClosed>()
                .add_event::events::<WindowBackendScaleFactorChanged>()
        */
    }
}
