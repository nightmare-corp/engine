use std::path::PathBuf;

use ne_math::Vec2;
use winit::{window::WindowId, event::MouseScrollDelta};
pub use winit::event::{ElementState, ScanCode, VirtualKeyCode, MouseButton};

//TODO honestly window id can be depreciated?

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
/// An event that is sent when winit eventloop is destroyed.
#[derive(Debug, Clone)]
pub struct ExitSequence;
/// An event that is sent whenever the operating systems requests that a window
/// be closed. This will be sent when the close button of the window is pressed.
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
/// The event is sent only if the cursor is over one of the application's windows.
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

//this?
/*
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
 */
pub struct OnKeyboardInput {
    pub key: VirtualKeyCode,
    pub state: ElementState,
}
//TODO is this alright?
impl OnKeyboardInput {
    pub fn new(key: VirtualKeyCode, state: ElementState) -> Self {
        Self { key, state }
    }
}

/* /// An event that indicates a window's OS-reported scale factor has changed.
#[derive(Debug, Clone)]
pub struct OnWindowBackendScaleFactorChanged {
    pub id: WindowId,
    pub scale_factor: f64,
} */
// #[derive(Debug, Clone)]
/// Reader in loop that will end the event loop.
pub struct ExitApp;
/// An event that is sent when a frame has been rendered
/// Inside of RedrawRequested in the eventloop
pub struct OnRedrawRequested;

/// An event reporting the change in physical position of a pointing device.
///
/// This represents raw, unfiltered physical motion.
/// It is the translated version of [`DeviceEvent::MouseMotion`] from the `winit` crate.
///
/// All pointing devices connected to a single machine at the same time can emit the event independently.
/// However, the event data does not make it possible to distinguish which device it is referring to.
///
/// [`DeviceEvent::MouseMotion`]: https://docs.rs/winit/latest/winit/event/enum.DeviceEvent.html#variant.MouseMotion
#[derive(Debug, Clone)]
pub struct OnMouseMotion {
    /// The change in the position of the pointing device since the last event was sent.
    pub delta: Vec2,
}
/// A mouse button input event.
///
/// ## Usage
///
/// The event is read inside of the [`mouse_button_input_system`](crate::mouse::mouse_button_input_system)
/// to update the [`Input<MouseButton>`](crate::Input<MouseButton>) resource.
#[derive(Debug, Clone)]
pub struct OnMouseButton {
    /// The pressed state of the button.
    pub state: ElementState,
    /// The mouse button assigned to the event.
    pub button: MouseButton,
}
//TODO scroll...

#[derive(Debug, Clone)]
pub struct OnMouseWheel {
    /// The pressed state of the button.
    pub delta: MouseScrollDelta,
}

