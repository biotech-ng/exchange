use crate::ui_model::{ChatMessage, PortalState};
use egui::{Align, Grid, Layout, Response, ScrollArea, Sense, TextEdit, TextStyle, Ui, Widget};
use crate::widgets::ui_extensions::UiExtension;

pub struct ChatCell<'a> {
    data: &'a ChatMessage,
}

impl<'a> ChatCell<'a> {
    pub fn new(data: &'a ChatMessage) -> Self {
        ChatCell { data }
    }
}

impl<'a> Widget for ChatCell<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        ui.scope(|ui| {
            ui.separator();
            ui.label(self.data.from());
            ui.label(self.data.message());
            ui.separator()
        })
        .response
        .interact(Sense::click())
    }
}

pub struct ChatTable<'a> {
    data: &'a mut PortalState,
    text_style: TextStyle,
}

impl<'a> ChatTable<'a> {
    pub fn new(data: &'a mut PortalState) -> Self {
        ChatTable { data, text_style: egui::TextStyle::Monospace }
    }
}

impl<'a> Widget for ChatTable<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let available_width = ui.available_width();
        ui.with_layout(Layout::bottom_up(Align::Max), |ui| {
            ui.scope(|ui| {
                let max_rows = 6;

                let row_height = ui.row_height_for_text_style(self.text_style.clone());
                let max_height = row_height * max_rows as f32 + row_height / 2.0;
                ui.set_max_height(max_height);

                ui.with_layout(Layout::top_down_justified(Align::Min), |ui| {
                    ScrollArea::vertical()
                        .id_source("chat table 1")
                        .stick_to_bottom(true)
                        .show(ui, |ui| {
                            TextEdit::multiline(&mut self.data.message_to_send)
                                .font(self.text_style)
                                .hint_text("Enter your message...")
                                .desired_width(f32::INFINITY)
                                .lock_focus(true)
                                .desired_rows(max_rows)
                                .ui(ui)
                        });
                });
            });

            ui.with_layout(Layout::top_down_justified(Align::Max), |ui| {
                ScrollArea::vertical()
                    .id_source("chat table")
                    .stick_to_bottom(true)
                    .show(ui, |ui| {
                        Grid::new("chat_table")
                            .min_col_width(available_width)
                            .show(ui, |ui| {
                                ui.vertical(|ui| {
                                    // Add table rows for each message
                                    // TODO do not unwrap, pass from client code an index
                                    let idx = self.data.selected_group_idx.unwrap_or(0);
                                    for message in self.data.chat_groups()[idx].messages().iter() {
                                        ChatCell::new(message).ui(ui);
                                    }
                                })
                            })
                    });
            })
        })
        .response
    }
}
