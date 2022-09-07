
/// ================================================================================================
/// Events
/// ================================================================================================
/// A window event that is sent whenever a window's logical size has changed.
#[derive(Debug, Clone)]
pub struct WindowResized {
    pub id: winit::window::WindowId,
    /// The new logical width of the window.
    pub width: f32,
    /// The new logical height of the window.
    pub height: f32,
}
