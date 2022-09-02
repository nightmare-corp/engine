/* use ne_app::{Plugin, App};

mod events;
mod window;
mod windows;

pub use events::*;
pub use windows::*;


pub struct WindowPlugin;
impl Plugin for WindowPlugin {
    fn setup(&self, app: &mut App) {
        app.add_event::<WindowResized>()
            .add_event::<CreateWindow>()
            .add_event::<WindowCreated>()
            .add_event::<WindowClosed>()
            .add_event::<WindowCloseRequested>()
            .add_event::<RequestRedraw>()
            .add_event::<CursorMoved>()
            .add_event::<CursorEntered>()
            .add_event::<CursorLeft>()
            .add_event::<ReceivedCharacter>()
            .add_event::<WindowFocused>()
            .add_event::<WindowScaleFactorChanged>()
            .add_event::<WindowBackendScaleFactorChanged>()
            .add_event::<FileDragAndDrop>()
            .add_event::<WindowMoved>()
            .init_resource::<Windows>();

        let settings = app
            .world
            .get_resource::<WindowSettings>()
            .cloned()
            .unwrap_or_default();

        if settings.add_primary_window {
            let window_descriptor = app
                .world
                .get_resource::<WindowDescriptor>()
                .cloned()
                .unwrap_or_default();
            let mut create_window_event = app.world.resource_mut::<Events<CreateWindow>>();
            create_window_event.send(CreateWindow {
                id: WindowId::primary(),
                descriptor: window_descriptor,
            });
        }

        if settings.exit_on_all_closed {
            app.add_system_to_stage(
                CoreStage::PostUpdate,
                exit_on_all_closed.after(ModifiesWindows),
            );
        }
        if settings.close_when_requested {
            app.add_system(close_when_requested);
        }
    }
}
 */