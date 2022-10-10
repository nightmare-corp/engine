static mut A: i32 = 100;

pub(crate) struct ControlWindow {
    name: String,
}
pub(crate) struct DiagnosticsWindow {
    name: String,
    fps: u32,
}
// struct SceneWindow;
// struct FileWindow;
impl Default for ControlWindow {
    fn default() -> Self {
        Self {
            name: "Control Panel".to_owned(),
        }
    }
}
impl Default for DiagnosticsWindow {
    fn default() -> Self {
        Self { 
            name: "Diagnostics Window".to_owned(),
            fps: 10,
        }
    }
}
pub trait UserInterface {
    fn update(&mut self, ctx: &egui::Context);
}
impl UserInterface for ControlWindow {
    fn update(&mut self, ctx: &egui::Context) {
        let screen_size = ctx.input().screen_rect.size();
        let default_width = (screen_size.x - 20.0).min(400.0);

        egui::Window::new(&self.name)
        // .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
        .default_width(default_width)
        .default_height(ctx.available_rect().height() - 46.0)
        .vscroll(true)
        .open(&mut true)
        .resizable(true)
        .collapsible(true)
        .show(ctx, |ui| {
            ui.heading("My egui Application");
            ui.horizontal(|ui| {
                ui.label("Your name: ");
                ui.text_edit_singleline(&mut self.name);
            });
            ui.label(format!("Hello '{}'", self.name));
            unsafe {
                A+=1;
                ui.label(format!("Haaaello '{}'", A));
            }
        });
        egui::Window::new("B")
        // .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
        .default_width(default_width)
        .default_height(ctx.available_rect().height() - 46.0)
        .vscroll(true)
        .open(&mut true)
        .resizable(true)
        .collapsible(true)
        .show(ctx, |ui| {
            ui.heading("My egui Application");
            ui.horizontal(|ui| {
                ui.label("Your name: ");
                ui.text_edit_singleline(&mut self.name);
            });
            ui.label(format!("Hello '{}'", self.name));
            unsafe {
                A+=1;
                ui.label(format!("Haaaello '{}'", A));
            }

        });
    }
}

impl UserInterface for DiagnosticsWindow {
    fn update(&mut self, ctx: &egui::Context) {
        let screen_size = ctx.input().screen_rect.size();
        let default_width = (screen_size.x - 20.0).min(400.0);

        egui::Window::new(&self.name)
        // .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
        .default_width(default_width)
        .default_height(ctx.available_rect().height() - 46.0)
        .vscroll(true)
        .open(&mut true)
        .resizable(true)
        .collapsible(true)
        .show(ctx, |ui| {
            ui.heading("My egui Application");
            ui.horizontal(|ui| {
                ui.label("Your name: ");
                ui.text_edit_singleline(&mut self.name);
            });
            ui.label(format!("Hello '{}'", self.name));
            ui.label(format!("Hello '{}'", self.name));

        });
    }
}
