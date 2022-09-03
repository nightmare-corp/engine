mod events;

use events::WindowResized;
use instant::Instant;
use ne_app::{Plugin, App, Event, info, Events, warn};
use winit::{event_loop::{ControlFlow, EventLoop}, dpi::PhysicalSize, window::{Window, Fullscreen}, event::{self, WindowEvent, KeyboardInput, ElementState, VirtualKeyCode, DeviceEvent}};

struct WinitState
{
    
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
async fn init_renderer(mut app: App) {
    let mut last_render_time = instant::Instant::now();
    let event_loop = EventLoop::new();

    //the bevy way
    /*     let mut event_loop = app
        .world
        .remove_non_send_resource::<EventLoop<()>>()
        .unwrap(); */

    //TODO can this code be shrinked?
    let win_settings =  app.world.get_resource::<WindowSettings>()
        .cloned().unwrap_or_default();
    let window = create_window(&win_settings, &event_loop);

    //todo
    // let mut state = State::new(&window, win_settings).await;

    //I need to edit this eventloop to take custom events?

    /* match event {
            event::Event::WindowResized
    */
    event_loop.run(move |event, _, 
        control_flow| {
        *control_flow = ControlFlow::Poll;
        //update app
        app.update();
        match event {
            event::Event::MainEventsCleared => window.request_redraw(),
        event::Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => {
                let world = app.world.cell();

                if !state.input(event) {
                    match event {
                        WindowEvent::CloseRequested
                        | WindowEvent::KeyboardInput {
                                            //TODO replace by eventreader.
                            input:
                                KeyboardInput {
                                    state: ElementState::Pressed,
                                    virtual_keycode: Some(VirtualKeyCode::Escape),
                                    ..
                                },
                            ..
                        } => *control_flow = ControlFlow::Exit,
                        WindowEvent::Resized(physical_size) => {
                //TODO replace by eventreader.
                            //TODO MOVE TASK: decouple window and renderer
                            state.resize(*physical_size);
                            
                            //bevy
                            /* 
                            window.update_actual_size_from_backend(size.width, size.height);
                            let mut resize_events = world.resource_mut::<Events<WindowResized>>();
                            resize_events.send(WindowResized {
                                id: window_id,
                                width: window.width(),
                                height: window.height(),
                            });*/
                            let mut resize_events
                            = world.resource_mut::<Events<WindowResized>>();
                            resize_events.send(WindowResized {
                               id: window_id,
                               width: window.inner_size().width as f32,
                               height: window.inner_size().height as f32,
                           });
                        }
                        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                //TODO replace by eventreader.
                            state.resize(**new_inner_size);
                        }
                        _ => {}
                    }
                }
            }
        event::Event::DeviceEvent {
                event: DeviceEvent::MouseMotion{ delta, },
                .. // We're not using device_id currently
            } => if state.is_right_mouse_pressed {
                //TODO replace by eventreader.
                state.camera_controller.process_mouse(delta.0, delta.1)
            }
        event::Event::RedrawRequested(window_id) if window_id == window.id() => {
                //NPP
                //maybe replace by eventreader.

                let now = instant::Instant::now();
                //TODO move to global/ne_time (new crate) if it's ever needed.
                let delta_time:Duration = now - last_render_time;
                last_render_time = now;
                

                //TODO This is interesting... can we replace it by a placeholder to inject stuff in? Then again hard coded isn't bad..? 
                //But bevy_ecs does have something convenient here
                state.update(delta_time);

                //TODO replace by eventreader.
                match state.render() {
                    Ok(_) => {}
                    // Reconfigure the surface if it's lost or outdated
                    Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                        state.resize(state.size)
                    }
                    // The system is out of memory, we should probably quit
                    Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                    // We're ignoring timeouts
                    Err(wgpu::SurfaceError::Timeout) => warn!("Surface timeout"),
                }
            }
            _ => {}
        }
});
}

fn main_loop(mut app:App)
{
    pollster::block_on(init_renderer(app));
}
pub struct WindowPlugin;
impl Plugin for WindowPlugin {
    fn setup(&self, app: &mut App) {
        app
/*     
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
        .add_event::<WindowMoved>() */
        // .init_resource::<Windows>()
        
        .add_event::<WindowResized>()
        .set_runner(main_loop);
    }
}
/// ================================================================================================
/// Window functionality
/// ================================================================================================
/// Describes the information needed for creating a window.
///
/// This should be set up before adding the [`WindowPlugin`](crate::WindowPlugin).
/// Most of these settings can also later be configured through the [`Window`](crate::Window) resource.
///
/// See [`examples/window/window_settings.rs`] for usage.
///
/// [`examples/window/window_settings.rs`]: https://github.com/bevyengine/bevy/blob/latest/examples/window/window_settings.rs
#[derive(Debug, Clone)]
pub struct WindowSettings {
    /// Sets the title that displays on the window top bar, on the system task bar and other OS specific places.
    ///
    /// ## Platform-specific
    /// - Web: Unsupported.
    pub title: String,
    /// The requested logical width of the window's client area.
    /// May vary from the physical width due to different pixel density on different monitors.
    pub width: f32,
    /// The requested logical height of the window's client area.
    /// May vary from the physical height due to different pixel density on different monitors.
    pub height: f32,
    /// ## Platform-specific
    /// - iOS / Android / Web: Unsupported.
    /// - macOS X: Not working as expected.
    /// - Windows 11: Not working as expected
    /// macOS X transparent works with winit out of the box, so this issue might be related to: <https://github.com/gfx-rs/wgpu/issues/687>
    /// Windows 11 is related to <https://github.com/rust-windowing/winit/issues/2082>
    /// Sets whether the background of the window should be transparent.
    pub transparent: bool,
    /// - iOS / Android / Web: Unsupported.
    pub resizable: bool,
    /// Sets whether the window should have borders and bars.
    pub decorations: bool,
    /// Sets whether the cursor is visible when the window has focus.
    //TODO
    pub cursor_visible: bool,
    /// Sets whether the window locks the cursor inside its borders when the window has focus.
    //TODO
    pub cursor_locked: bool,
    /// Whether or not to fit the canvas element's size to its parent element's size.
    ///
    /// **Warning**: this will not behave as expected for parents that set their size according to the size of their
    /// children. This creates a "feedback loop" that will result in the canvas growing on each resize. When using this
    /// feature, ensure the parent's size is not affected by its children.
    ///
    /// This value has no effect on non-web platforms.
    //TODO
    pub fit_canvas_to_parent: bool,
    /// Controls when a frame is presented to the screen.
    #[doc(alias = "vsync")]
    /// The window's [`PresentMode`].
    /// Used to select whether or not VSync is used
    pub present_mode: wgpu::PresentMode,
    /// The position on the screen that the window will be placed at.
    pub position: WindowPosition,
    /// Sets minimum and maximum resize limits.
    pub resize_constraints: WindowResizeConstraints,
    /// Overrides the window's ratio of physical pixels to logical pixels.
    ///
    /// If there are some scaling problems on X11 try to set this option to `Some(1.0)`.
    pub scale_factor_override: Option<f64>,
    /// Sets whether the window is resizable.
    /// ## Platform-specific
    /// Sets the [`WindowMode`](crate::WindowMode).
    pub window_mode: WindowMode,
    /// The "html canvas" element selector.
    /// If set, this selector will be used to find a matching html canvas element,
    /// rather than creating a new one.
    /// Uses the [CSS selector format](https://developer.mozilla.org/en-US/docs/Web/API/Document/querySelector).
    /// This value has no effect on non-web platforms.
    pub canvas: Option<String>,
}
impl Default for WindowSettings {
    fn default() -> Self {
        WindowSettings {
            title: "app".to_string(),
            width: 1280.,
            height: 720.,

            resizable: true,
            decorations: true,
            cursor_locked: false,
            cursor_visible: true,
            transparent: false,
            fit_canvas_to_parent: false,
            present_mode: wgpu::PresentMode::Immediate,
                        
            //TODO THESE NEED TO BE ENABLED ON STARTUP
            position: WindowPosition::Automatic,
            resize_constraints: WindowResizeConstraints::default(),
            scale_factor_override: None,

            window_mode: WindowMode::Windowed,
            canvas: None,
        }
    }
}

//window
pub fn get_fitting_videomode(
    monitor: &winit::monitor::MonitorHandle,
    width: u32,
    height: u32,
) -> winit::monitor::VideoMode {
    let mut modes = monitor.video_modes().collect::<Vec<_>>();

    fn abs_diff(a: u32, b: u32) -> u32 {
        if a > b {
            return a - b;
        }
        b - a
    }

    modes.sort_by(|a, b| {
        use std::cmp::Ordering::*;
        match abs_diff(a.size().width, width).cmp(&abs_diff(b.size().width, width)) {
            Equal => {
                match abs_diff(a.size().height, height).cmp(&abs_diff(b.size().height, height)) {
                    Equal => b.refresh_rate().cmp(&a.refresh_rate()),
                    default => default,
                }
            }
            default => default,
        }
    });

    modes.first().unwrap().clone()
}

fn create_window(win_settings: &WindowSettings, event_loop: &EventLoop<()>) -> Window
{
    let mut wind = winit::window::WindowBuilder::new()
    .with_title(win_settings.title.clone())
    .with_inner_size(PhysicalSize::new(win_settings.width, win_settings.height))
    .with_transparent(win_settings.transparent)
    .with_resizable(win_settings.resizable)
    .with_decorations(win_settings.decorations);

    match win_settings.window_mode
    {
        //incomplete 
        WindowMode::Windowed => {},
        WindowMode::BorderlessFullscreen => {
            // let mut monitor_index = 0; //todo
            let mut monitor = event_loop
            .available_monitors()
            .next()
            .expect("no monitor found!");
            let fullscreen = Some(Fullscreen::Borderless(Some(monitor.clone())));
            info!("Setting mode: {:?}", fullscreen);
            wind = wind.with_fullscreen(fullscreen);
        },
        WindowMode::SizedFullscreen => todo!(),
        WindowMode::Fullscreen => {
            // let mut monitor_index = 0; //todo
            let mut monitor = event_loop
            .available_monitors()
            .next()
            .expect("no monitor found!");
            let fullscreen = Some(Fullscreen::Exclusive(get_fitting_videomode(
                &monitor,
                win_settings.width as u32,
                win_settings.height as u32,
            )),);
            info!("Setting mode: {:?}", fullscreen);
            wind = wind.with_fullscreen(fullscreen);
        },
    }

    //TODO ...
    // match (win_settings.mode)
    // {
    //     WindowMode::Windowed => (),
    //     WindowMode::BorderlessFullscreen => wind.with_fullscreen(),
    //     WindowMode::SizedFullscreen => todo!(),
    //     WindowMode::Fullscreen => wind.with_fullscreen(),
    // }

    wind
    .build(&event_loop)
    .unwrap()
}
/// Defines the way a window is displayed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowMode {
    /// Creates a window that uses the given size.
    Windowed,
    /// Creates a borderless window that uses the full size of the screen.
    BorderlessFullscreen,
    /// Creates a fullscreen window that will render at desktop resolution.
    ///
    /// The app will use the closest supported size from the given size and scale it to fit the screen.
    SizedFullscreen,
    /// Creates a fullscreen window that uses the maximum supported size.
    Fullscreen,
}

/// Defines where window should be placed at on creation.
#[derive(Debug, Clone, Copy)]
pub enum WindowPosition {
    /// Position will be set by the window manager
    Automatic,
    /// Window will be centered on the selected monitor
    ///
    /// Note that this does not account for window decorations.
    Centered(MonitorSelection),
    /// The window's top-left corner will be placed at the specified position (in pixels)
    ///
    /// (0,0) represents top-left corner of screen space.
    At(ne_math::Vec2),
}

/// Defines which monitor to use.
#[derive(Debug, Clone, Copy)]
pub enum MonitorSelection {
    /// Uses current monitor of the window.
    Current,
    /// Uses primary monitor of the system.
    Primary,
    /// Uses monitor with the specified index.
    Number(usize),
}

//Needed?
/// The size limits on a window.
///
/// These values are measured in logical pixels, so the user's
/// scale factor does affect the size limits on the window.
/// Please note that if the window is resizable, then when the window is
/// maximized it may have a size outside of these limits. The functionality
/// required to disable maximizing is not yet exposed by winit.
#[derive(Debug, Clone, Copy)]
pub struct WindowResizeConstraints {
    pub min_width: f32,
    pub min_height: f32,
    pub max_width: f32,
    pub max_height: f32,
}

impl Default for WindowResizeConstraints {
    fn default() -> Self {
        Self {
            min_width: 180.,
            min_height: 120.,
            max_width: f32::INFINITY,
            max_height: f32::INFINITY,
        }
    }
}

impl WindowResizeConstraints {
    #[must_use]
    pub fn check_constraints(&self) -> Self {
        let WindowResizeConstraints {
            mut min_width,
            mut min_height,
            mut max_width,
            mut max_height,
        } = self;
        min_width = min_width.max(1.);
        min_height = min_height.max(1.);
        if max_width < min_width {
            warn!(
                "The given maximum width {} is smaller than the minimum width {}",
                max_width, min_width
            );
            max_width = min_width;
        }
        if max_height < min_height {
            warn!(
                "The given maximum height {} is smaller than the minimum height {}",
                max_height, min_height
            );
            max_height = min_height;
        }
        WindowResizeConstraints {
            min_width,
            min_height,
            max_width,
            max_height,
        }
    }
}

