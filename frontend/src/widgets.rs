use egui::{Button, Response, Sense, TextEdit, Ui, Widget};
use crate::ui_model::ChatGroup;

pub trait UiExtension {
    fn square_button(&mut self, label: &str) -> Response;

    fn search_bar<'a>(&'a mut self, text: &'a mut String) -> Response;
}

impl UiExtension for Ui {
    fn square_button(&mut self, label: &str) -> Response {
        let button_size = [50.0; 2]; // adjust the size to your liking

        Button::new(label)
            .sense(Sense::click())
            .fill(self.visuals().extreme_bg_color)
            .min_size(button_size.into())
            .ui(self)
    }

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
