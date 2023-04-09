use eframe::epaint::text::LayoutJob;
use crate::ui_model::{ChatMessage, PortalState};
use egui::{Align, Color32, FontId, Grid, Layout, Response, ScrollArea, Sense, TextEdit, TextStyle, Ui, Widget};

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
}

impl<'a> ChatTable<'a> {
    pub fn new(data: &'a mut PortalState) -> Self {
        ChatTable { data }
    }
}

impl<'a> Widget for ChatTable<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let available_width = ui.available_width();
        ui.with_layout(Layout::bottom_up(Align::Max), |ui| {
            ui.scope(|ui| {
                let max_rows = 6;

                ui.set_max_height(90.0);

                ui.with_layout(Layout::top_down_justified(Align::Max), |ui| {
                    ScrollArea::vertical()
                        .id_source("chat table 1")
                        .stick_to_bottom(true)
                        .show(ui, |ui| {
                            TextEdit::multiline(&mut self.data.message_to_send)
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
