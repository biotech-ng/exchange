#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod ui_model;
mod widgets;

use crate::ui_model::PortalState;
use crate::widgets::ui_extensions::UiExtension;
use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(640.0, 640.0)),
        ..Default::default()
    };
    eframe::run_native(
        "Biotech exchange",
        options,
        Box::new(|_cc| Box::new(PortalState::default())),
    )
}

impl eframe::App for PortalState {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let available_height = ui.available_height();
            ui.horizontal(|ui| {
                ui.menu(available_height);

                ui.separator();

                ui.chat_group_table(self);

                ui.separator();

                ui.chat_table(self)
            })
        });
    }
}
