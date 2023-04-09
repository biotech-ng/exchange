#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod ui_model;

use eframe::egui;
use egui::{Color32, Direction, Grid, Label, Response, ScrollArea, Sense, TextEdit, Ui, Widget, widgets::Button};
use crate::ui_model::MyApp;

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

fn square_button(ui: &mut Ui, label: &str) -> Response {
    let button_size = [50.0; 2]; // adjust the size to your liking

    Button::new(label)
        .sense(Sense::click())
        .fill(ui.visuals().extreme_bg_color)
        .min_size(button_size.into())
        .ui(ui)
}

fn search_bar(text: &mut String) -> TextEdit {
    let hint_text = "Search...";

    TextEdit::singleline(text)
        .hint_text(hint_text)
        .desired_width(120.0)
}

trait UiExtension {
    fn search_bar<'a>(&'a mut self, text: &'a mut String) -> Response;
}

impl UiExtension for Ui {
    fn search_bar<'a>(&'a mut self, text: &'a mut String) -> Response {
        let hint_text = "Search...";

        let result = TextEdit::singleline(text)
            .hint_text(hint_text)
            .desired_width(120.0)
            .ui(self);

        let id = result.id;
        if !self.memory(|x| x.has_focus(id)) {
            text.clear();
        }

        result
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                // Menu
                ui.vertical(|ui| {
                    _ = square_button(ui, "Menu 1");
                    _ = square_button(ui, "Menu 2");
                    _ = square_button(ui, "Menu 3");
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
                                    for message in self.chat_groups() {
                                        ui.separator();
                                        ui.label(message.name());
                                        ui.label(message.last_message());
                                    }
                                })
                            })
                    });
                });
            })
        });
    }
}
