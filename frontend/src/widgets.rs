pub mod ui_extensions;
mod selectable_widget;

use crate::ui_model::{ChatGroup, PortalState};
use crate::widgets::ui_extensions::UiExtension;
use egui::{Grid, Response, ScrollArea, Sense, Ui, Widget};
use crate::widgets::selectable_widget::SelectableWidget;

pub struct ChatGroupCell<'a> {
    data: &'a ChatGroup,
}

impl<'a> ChatGroupCell<'a> {
    pub fn new(data: &'a ChatGroup) -> Self {
        ChatGroupCell { data }
    }
}

impl<'a> Widget for ChatGroupCell<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        ui.scope(|ui| {
            ui.separator();
            ui.label(self.data.name());
            ui.label(self.data.last_message());
            ui.separator()
        })
        .response
        .interact(Sense::click())
    }
}

pub struct ChatGroupTable<'a> {
    data: &'a mut PortalState,
}

impl<'a> ChatGroupTable<'a> {
    pub fn new(data: &'a mut PortalState) -> Self {
        ChatGroupTable { data }
    }
}

impl<'a> Widget for ChatGroupTable<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        ui.vertical(|ui| {
            ui.search_bar(&mut self.data.search_text);

            ScrollArea::vertical()
                .stick_to_bottom(true)
                .show(ui, |ui| {
                    Grid::new("chat_table")
                        .striped(true)
                        .min_col_width(120.0)
                        .show(ui, |ui| {
                            ui.vertical(|ui| {
                                // Add table rows for each message
                                let mut selected_group_idx: Option<u16> = None;
                                for (i, message) in self.data.chat_groups().iter().enumerate() {
                                    let cell = ChatGroupCell::new(&message);
                                    let response = SelectableWidget::new(cell).ui(ui);
                                    if response.clicked() {
                                        selected_group_idx = Some(i as u16)
                                    }
                                }
                                self.data.selected_group_idx = selected_group_idx
                            })
                        })
                });
        })
        .response
    }
}
