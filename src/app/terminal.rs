use eframe::egui::{self, CentralPanel, TextEdit};

// This is a simple terminal-like application using eframe and egui.
pub struct ToadTerminalApp {
    input: String,
    output: String,
}

// Implementing Default for ToadTerminalApp to initialize with default values.
impl Default for ToadTerminalApp {
    fn default() -> Self {
        Self {
            input: String::new(),
            output: "Welcome to Toad Terminal \nType a command below...".to_string(),
        }
    }
}

// Implementing the eframe::App trait for ToadTerminalApp to handle updates and rendering.
impl eframe::App for ToadTerminalApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.heading("Toad Terminal");

            ui.add_space(10.0);
            ui.label(&self.output);

            ui.add_space(10.0);
            ui.add(TextEdit::singleline(&mut self.input).hint_text("Type a command..."));
        });
    }
}
