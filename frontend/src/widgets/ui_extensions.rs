use crate::ui_model::PortalState;
use crate::widgets::chat_group_table::ChatGroupTable;
use crate::widgets::chat_table::ChatTable;
use egui::{Button, Response, Sense, TextEdit, Ui, Widget};

pub trait UiExtension {
    fn menu(&mut self, available_height: f32) -> Response;

    fn square_button(&mut self, label: &str) -> Response;

    fn search_bar<'a>(&'a mut self, text: &'a mut String) -> Response;

    fn chat_group_table(&mut self, state: &mut PortalState) -> Response;

    fn chat_table(&mut self, state: &mut PortalState) -> Response;
}

impl UiExtension for Ui {
    fn menu(&mut self, available_height: f32) -> Response {
        self.vertical(|ui| {
            ui.set_min_height(available_height);
            _ = ui.square_button("Menu 1");
            _ = ui.square_button("Menu 2");
            _ = ui.square_button("Menu 3");
        })
        .response
    }

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

    fn chat_group_table(&mut self, state: &mut PortalState) -> Response {
        ChatGroupTable::new(state).ui(self)
    }

    fn chat_table(&mut self, state: &mut PortalState) -> Response {
        ChatTable::new(state).ui(self)
    }
}
