use egui::{Button, Response, Sense, TextEdit, Ui, Widget};

pub trait UiExtension {
    fn menu(&mut self) -> Response;

    fn square_button(&mut self, label: &str) -> Response;

    fn search_bar<'a>(&'a mut self, text: &'a mut String) -> Response;
}

impl UiExtension for Ui {
    fn menu(&mut self) -> Response {
        self.vertical(|ui| {
            _ = ui.square_button("Menu 1");
            _ = ui.square_button("Menu 2");
            _ = ui.square_button("Menu 3");
        }).response
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
}
