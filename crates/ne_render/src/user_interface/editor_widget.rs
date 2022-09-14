// use egui::{ScrollArea, Context};
// use egui_demo_lib::is_mobile;


// /// A menu bar in which you can select different demo windows to show.
// #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
// #[cfg_attr(feature = "serde", serde(default))]
// pub struct DemoWindows {
//     about_is_open: bool,
//     about: About,
//     demos: Demos,
//     tests: Tests,
// }

// impl Default for DemoWindows {
//     fn default() -> Self {
//         Self {
//             about_is_open: true,
//         }
//     }
// }

// impl DemoWindows {
//     /// Show the app ui (menu bar and windows).
//     pub fn ui(&mut self, ctx: &Context) {
//         if is_mobile(ctx) {
//             self.mobile_ui(ctx);
//         } else {
//             self.desktop_ui(ctx);
//         }
//     }

//     fn mobile_ui(&mut self, ctx: &Context) {
//         if self.about_is_open {
//             let screen_size = ctx.input().screen_rect.size();
//             let default_width = (screen_size.x - 20.0).min(400.0);

//             let mut close = false;
//             egui::Window::new(self.about.name())
//                 .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
//                 .default_width(default_width)
//                 .default_height(ctx.available_rect().height() - 46.0)
//                 .vscroll(true)
//                 .open(&mut self.about_is_open)
//                 .resizable(false)
//                 .collapsible(false)
//                 .show(ctx, |ui| {
//                     self.about.ui(ui);
//                     ui.add_space(12.0);
//                     ui.vertical_centered_justified(|ui| {
//                         if ui
//                             .button(egui::RichText::new("Continue to the demo!").size(24.0))
//                             .clicked()
//                         {
//                             close = true;
//                         }
//                     });
//                 });
//             self.about_is_open &= !close;
//         } else {
//             self.mobile_top_bar(ctx);
//             self.show_windows(ctx);
//         }
//     }

//     fn mobile_top_bar(&mut self, ctx: &Context) {
//         egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
//             egui::menu::bar(ui, |ui| {
//                 let font_size = 20.0;

//                 ui.menu_button(egui::RichText::new("⏷ demos").size(font_size), |ui| {
//                     ui.set_style(ui.ctx().style()); // ignore the "menu" style set by `menu_button`.
//                     self.demo_list_ui(ui);
//                     if ui.ui_contains_pointer() && ui.input().pointer.any_click() {
//                         ui.close_menu();
//                     }
//                 });

//                 ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
//                     use egui::special_emojis::{GITHUB, TWITTER};
//                     ui.hyperlink_to(
//                         egui::RichText::new(TWITTER).size(font_size),
//                         "https://twitter.com/ernerfeldt",
//                     );
//                     ui.hyperlink_to(
//                         egui::RichText::new(GITHUB).size(font_size),
//                         "https://github.com/emilk/egui",
//                     );
//                 });
//             });
//         });
//     }

//     fn desktop_ui(&mut self, ctx: &Context) {
//         egui::SidePanel::right("egui_demo_panel")
//             .resizable(false)
//             .default_width(145.0)
//             .show(ctx, |ui| {
//                 egui::trace!(ui);
//                 ui.vertical_centered(|ui| {
//                     ui.heading("✒ egui demos");
//                 });

//                 ui.separator();

//                 use egui::special_emojis::{GITHUB, TWITTER};
//                 ui.hyperlink_to(
//                     format!("{} egui on GitHub", GITHUB),
//                     "https://github.com/emilk/egui",
//                 );
//                 ui.hyperlink_to(
//                     format!("{} @ernerfeldt", TWITTER),
//                     "https://twitter.com/ernerfeldt",
//                 );

//                 ui.separator();

//                 self.demo_list_ui(ui);
//             });

//         egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
//             egui::menu::bar(ui, |ui| {
//                 file_menu_button(ui);
//             });
//         });

//         self.show_windows(ctx);
//     }

//     /// Show the open windows.
//     fn show_windows(&mut self, ctx: &Context) {
//         self.about.show(ctx, &mut self.about_is_open);
//         self.demos.windows(ctx);
//         self.tests.windows(ctx);
//     }

//     fn demo_list_ui(&mut self, ui: &mut egui::Ui) {
//         ScrollArea::vertical().show(ui, |ui| {
//             ui.with_layout(egui::Layout::top_down_justified(egui::Align::LEFT), |ui| {
//                 ui.toggle_value(&mut self.about_is_open, self.about.name());

//                 ui.separator();
//                 self.demos.checkboxes(ui);
//                 ui.separator();
//                 self.tests.checkboxes(ui);
//                 ui.separator();

//                 if ui.button("Organize windows").clicked() {
//                     ui.ctx().memory().reset_areas();
//                 }
//             });
//         });
//     }
// }

// //
// //
// //

// #[derive(Default)]
// #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
// #[cfg_attr(feature = "serde", serde(default))]
// pub struct About {}

// impl Demo for About {
//     fn name(&self) -> &'static str {
//         "About egui"
//     }

//     fn show(&mut self, ctx: &egui::Context, open: &mut bool) {
//         egui::Window::new(self.name())
//             .default_width(320.0)
//             .open(open)
//             .show(ctx, |ui| {
//                 use super::View as _;
//                 self.ui(ui);
//             });
//     }
// }

// impl super::View for About {
//     fn ui(&mut self, ui: &mut egui::Ui) {
//         use egui::special_emojis::{OS_APPLE, OS_LINUX, OS_WINDOWS};

//         ui.heading("egui");
//         ui.label(format!(
//             "egui is an immediate mode GUI library written in Rust. egui runs both on the web and natively on {}{}{}. \
//             On the web it is compiled to WebAssembly and rendered with WebGL.{}",
//             OS_APPLE, OS_LINUX, OS_WINDOWS,
//             if cfg!(target_arch = "wasm32") {
//                 " Everything you see is rendered as textured triangles. There is no DOM, HTML, JS or CSS. Just Rust."
//             } else {""}
//         ));
//         ui.label("egui is designed to be easy to use, portable, and fast.");

//         ui.add_space(12.0); // ui.separator();
//         ui.heading("Immediate mode");
//         about_immediate_mode(ui);

//         ui.add_space(12.0); // ui.separator();
//         ui.heading("Links");
//         links(ui);
//     }
// }

// fn about_immediate_mode(ui: &mut egui::Ui) {
//     use crate::syntax_highlighting::code_view_ui;
//     ui.style_mut().spacing.interact_size.y = 0.0; // hack to make `horizontal_wrapped` work better with text.

//     ui.horizontal_wrapped(|ui| {
//             ui.spacing_mut().item_spacing.x = 0.0;
//             ui.label("Immediate mode is a GUI paradigm that lets you create a GUI with less code and simpler control flow. For example, this is how you create a ");
//             let _ = ui.small_button("button");
//             ui.label(" in egui:");
//         });

//     ui.add_space(8.0);
//     code_view_ui(
//         ui,
//         r#"
//   if ui.button("Save").clicked() {
//       my_state.save();
//   }"#
//         .trim_start_matches('\n'),
//     );
//     ui.add_space(8.0);

//     ui.label("Note how there are no callbacks or messages, and no button state to store.");

//     ui.label("Immediate mode has its roots in gaming, where everything on the screen is painted at the display refresh rate, i.e. at 60+ frames per second. \
//         In immediate mode GUIs, the entire interface is layed out and painted at the same high rate. \
//         This makes immediate mode GUIs especially well suited for highly interactive applications.");

//     ui.horizontal_wrapped(|ui| {
//         ui.spacing_mut().item_spacing.x = 0.0;
//         ui.label("More about immediate mode ");
//         ui.hyperlink_to("here", "https://github.com/emilk/egui#why-immediate-mode");
//         ui.label(".");
//     });
// }

// fn links(ui: &mut egui::Ui) {
//     use egui::special_emojis::{GITHUB, TWITTER};
//     ui.hyperlink_to(
//         format!("{} egui on GitHub", GITHUB),
//         "https://github.com/emilk/egui",
//     );
//     ui.hyperlink_to(
//         format!("{} @ernerfeldt", TWITTER),
//         "https://twitter.com/ernerfeldt",
//     );
//     ui.hyperlink_to("egui documentation", "https://docs.rs/egui/");
// }

