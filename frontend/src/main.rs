#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod ui_model;
mod widgets;

use eframe::egui;
use egui::{Grid, ScrollArea, Widget};
use crate::ui_model::MyApp;
use crate::widgets::ChatGroupCell;
use crate::widgets::ui_extensions::UiExtension;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(640.0, 640.0)),
        ..Default::default()
    };
    eframe::run_native(
        "Biotech exchange",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    )
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                // Menu
                ui.vertical(|ui| {
                    _ = ui.square_button("Menu 1");
                    _ = ui.square_button("Menu 2");
                    _ = ui.square_button("Menu 3");
                });

                // Chats
                ui.vertical(|ui| {
                    ui.search_bar(&mut self.search_text);

                    ScrollArea::vertical()
                        .min_scrolled_height(640.0)
                        .show(ui, |ui| {
                        Grid::new("chat_table")
                            .striped(true)
                            // .min_col_width()
                            .min_col_width(120.0)
                            .show(ui, |ui| {
                                ui.vertical(|ui| {
                                    // Add table rows for each message
                                    for (i, message) in self.chat_groups().iter().enumerate() {
                                        let response = ChatGroupCell::new(&message).ui(ui);
                                        if response.clicked() {
                                            println!("clicked2: {}", i + 1)
                                        }
                                    }
                                })
                            })
                    });
                });
            })
        });
    }
}
