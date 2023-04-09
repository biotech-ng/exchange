pub mod ui_extensions;

use egui::{Response, Sense, Ui, Widget};
use crate::ui_model::ChatGroup;

pub struct ChatGroupCell<'a> {
    data: &'a ChatGroup
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
            ui.label(self.data.last_message())
        }).response.interact(Sense::click())
    }
}
