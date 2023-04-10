#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use frontend_lib::ui_model::PortalState;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(640.0, 640.0)),
        ..Default::default()
    };
    eframe::run_native(
        "Biotech exchange",
        options,
        Box::new(|_cc| Box::<PortalState>::default()),
    )
}
