mod app;

use app::terminal::ToadTerminalApp;
use eframe::egui;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();

    // Set the initial window size
    eframe::run_native(
        "Toad Terminal",
        options,
        Box::new(|_cc| Box::new(ToadTerminalApp::default())),
    )
}
