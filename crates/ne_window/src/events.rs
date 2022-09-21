use std::path::PathBuf;

use ne_math::Vec2;
use winit::window::WindowId;


/// ================================================================================================
/// Events
/// ================================================================================================
/// A window event that is sent whenever a window's logical size has changed.
#[derive(Debug, Clone)]
pub struct OnWindowResized {
    pub id: winit::window::WindowId,
    /// The new logical width of the window.
    pub width: f32,
    /// The new logical height of the window.
    pub height: f32,
}
/// An event that is sent whenever a new window is created.
///
/// To create a new window, send a [`CreateWindow`] event - this
/// event will be sent in the handler for that event.
#[derive(Debug, Clone)]
pub struct OnWindowCreated {
    pub id: WindowId,
}
/// An event that is sent whenever the operating systems requests that a window
/// be closed. This will be sent when the close button of the window is pressed.
///
/// If the default [`WindowPlugin`] is used, these events are handled
/// by [closing] the corresponding [`Window`].  
/// To disable this behaviour, set `close_when_requested` on the [`WindowPlugin`]
/// to `false`.
///
/// [`WindowPlugin`]: crate::WindowPlugin
/// [`Window`]: crate::Window
/// [closing]: crate::Window::close
#[derive(Debug, Clone)]
pub struct OnWindowCloseRequested {
    pub id: WindowId,
}

/// An event that is sent whenever a window is closed. This will be sent by the
/// handler for [`Window::close`].
///
/// [`Window::close`]: crate::Window::close
#[derive(Debug, Clone)]
pub struct OnWindowClosed {
    pub id: WindowId,
}
/// An event reporting that the mouse cursor has moved on a window.
///
/// The event is sent only if the cursor is over one of the application's windows.
/// It is the translated version of [`WindowEvent::CursorMoved`] from the `winit` crate.
///
/// Not to be confused with the [`MouseMotion`] event from `bevy_input`.
///
/// [`WindowEvent::CursorMoved`]: https://docs.rs/winit/latest/winit/event/enum.WindowEvent.html#variant.CursorMoved
/// [`MouseMotion`]: bevy_input::mouse::MouseMotion
#[derive(Debug, Clone)]
pub struct OnCursorMoved {
    /// The identifier of the window the cursor has moved on.
    pub id: WindowId,

    /// The position of the cursor, in window coordinates.
    pub position: Vec2,
}
/// An event that is sent whenever the user's cursor enters a window.
#[derive(Debug, Clone)]
pub struct OnCursorEntered {
    pub id: WindowId,
}
/// An event that is sent whenever the user's cursor leaves a window.
#[derive(Debug, Clone)]
pub struct OnCursorLeft {
    pub id: WindowId,
}

/// An event that indicates a window has received or lost focus.
#[derive(Debug, Clone)]
pub struct OnWindowFocused {
    pub id: WindowId,
    pub focused: bool,
}

/// Events related to files being dragged and dropped on a window.
#[derive(Debug, Clone)]
pub enum OnFileDragAndDrop {
    DroppedFile { id: WindowId, path_buf: PathBuf },

    HoveredFile { id: WindowId, path_buf: PathBuf },

    HoveredFileCancelled { id: WindowId },
}
/// An event that is sent when a window is repositioned in physical pixels.
#[derive(Debug, Clone)]
pub struct OnWindowMoved {
    pub id: WindowId,
    pub position: Vec2,
}
//TODO implement these maybe:
/* /// An event that indicates that a new window should be created.
#[derive(Debug, Clone)]
pub struct OnCreateWindow {
    pub id: WindowId,
    pub descriptor: WindowDescriptor,
} */
/// An event that is sent whenever a window receives a character from the OS or underlying system.
#[derive(Debug, Clone)]
pub struct OnReceivedCharacter {
    pub id: WindowId,
    pub char: char,
}
/// An event that indicates a window's scale factor has changed.
#[derive(Debug, Clone)]
pub struct OnWindowScaleFactorChanged {
    pub id: WindowId,
    pub scale_factor: f64,
}
/* /// An event that indicates a window's OS-reported scale factor has changed.
#[derive(Debug, Clone)]
pub struct OnWindowBackendScaleFactorChanged {
    pub id: WindowId,
    pub scale_factor: f64,
} */
// #[derive(Debug, Clone)]
/// Reader in loop that will end the event loop.
pub struct AppExit;
/// An event that is sent when a frame has been rendered 
/// Inside of RedrawRequested in the eventloop
pub struct OnRedrawRequested;