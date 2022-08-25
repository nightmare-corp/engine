// pub fn add(left: usize, right: usize) -> usize {
//     left + right
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }

// let mut winit_window_builder = winit::window::WindowBuilder::new();

// winit_window_builder = match window_descriptor.mode {
//     WindowMode::BorderlessFullscreen => winit_window_builder.with_fullscreen(Some(
//         winit::window::Fullscreen::Borderless(event_loop.primary_monitor()),
//     )),
//     WindowMode::Fullscreen => {
//         winit_window_builder.with_fullscreen(Some(winit::window::Fullscreen::Exclusive(
//             get_best_videomode(&event_loop.primary_monitor().unwrap()),
//         )))
//     }
//     WindowMode::SizedFullscreen => winit_window_builder.with_fullscreen(Some(
//         winit::window::Fullscreen::Exclusive(get_fitting_videomode(
//             &event_loop.primary_monitor().unwrap(),
//             window_descriptor.width as u32,
//             window_descriptor.height as u32,
//         )),
//     )),
//     _ => {
//         let WindowDescriptor {
//             width,
//             height,
//             position,
//             scale_factor_override,
//             ..
//         } = window_descriptor;

//         use bevy_window::WindowPosition::*;
//         match position {
//             Automatic => { /* Window manager will handle position */ }
//             Centered(monitor_selection) => {
//                 use bevy_window::MonitorSelection::*;
//                 let maybe_monitor = match monitor_selection {
//                     Current => {
//                         warn!("Can't select current monitor on window creation!");
//                         None
//                     }
//                     Primary => event_loop.primary_monitor(),
//                     Number(n) => event_loop.available_monitors().nth(*n),
//                 };

//                 if let Some(monitor) = maybe_monitor {
//                     let screen_size = monitor.size();

//                     let scale_factor = scale_factor_override.unwrap_or(1.0);

//                     // Logical to physical window size
//                     let (width, height): (u32, u32) = LogicalSize::new(*width, *height)
//                         .to_physical::<u32>(scale_factor)
//                         .into();

//                     let position = PhysicalPosition {
//                         x: screen_size.width.saturating_sub(width) as f64 / 2.
//                             + monitor.position().x as f64,
//                         y: screen_size.height.saturating_sub(height) as f64 / 2.
//                             + monitor.position().y as f64,
//                     };

//                     winit_window_builder = winit_window_builder.with_position(position);
//                 } else {
//                     warn!("Couldn't get monitor selected with: {monitor_selection:?}");
//                 }
//             }
//             At(position) => {
//                 if let Some(sf) = scale_factor_override {
//                     winit_window_builder = winit_window_builder.with_position(
//                         LogicalPosition::new(position[0] as f64, position[1] as f64)
//                             .to_physical::<f64>(*sf),
//                     );
//                 } else {
//                     winit_window_builder = winit_window_builder.with_position(
//                         LogicalPosition::new(position[0] as f64, position[1] as f64),
//                     );
//                 }
//             }
//         }

//         if let Some(sf) = scale_factor_override {
//             winit_window_builder
//                 .with_inner_size(LogicalSize::new(*width, *height).to_physical::<f64>(*sf))
//         } else {
//             winit_window_builder.with_inner_size(LogicalSize::new(*width, *height))
//         }
//     }
//     .with_resizable(window_descriptor.resizable)
//     .with_decorations(window_descriptor.decorations)
//     .with_transparent(window_descriptor.transparent),
// };
