use crate::ui_model::{ChatGroup, PortalState};
use crate::widgets::selectable_widget::SelectableWidget;
use crate::widgets::ui_extensions::UiExtension;
use egui::{Grid, Response, ScrollArea, Sense, Ui, Widget};

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
    min_col_width: f32,
}

impl<'a> ChatGroupTable<'a> {
    pub fn new(data: &'a mut PortalState) -> Self {
        ChatGroupTable {
            data,
            min_col_width: 120.0,
        }
    }
}

impl<'a> Widget for ChatGroupTable<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        ui.vertical(|ui| {
            ui.search_bar(&mut self.data.search_text);

            ScrollArea::vertical().show(ui, |ui| {
                Grid::new("group_table")
                    .min_col_width(self.min_col_width)
                    .show(ui, |ui| {
                        ui.vertical(|ui| {
                            // Add table rows for each message
                            let mut selected_group_idx: Option<_> = None;
                            for (i, message) in self.data.chat_groups().iter().enumerate() {
                                let cell = ChatGroupCell::new(&message);
                                let response = SelectableWidget::new(cell).ui(ui);
                                if response.clicked() {
                                    selected_group_idx = Some(i)
                                }
                            }
                            if selected_group_idx.is_some() {
                                self.data.selected_group_idx = selected_group_idx
                            }
                        })
                    })
            });
        })
        .response
    }
}
