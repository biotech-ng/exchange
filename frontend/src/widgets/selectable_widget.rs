use egui::{Color32, Response, Stroke, Ui, Widget};

pub struct SelectableWidget<W: Widget> {
    widget: W,
    selected: bool,
}

impl<W: Widget> SelectableWidget<W> {
    pub fn new(widget: W) -> Self {
        Self { widget, selected: false }
    }

    pub fn ui(mut self, ui: &mut Ui) -> Response {
        let response = self.widget.ui(ui);

        if response.clicked[0] {
            self.selected = !self.selected;
        }

        if response.hovered || self.selected {
            let visuals = ui.style().interact(&response);

            let color = if self.selected {
                Color32::WHITE
            } else {
                visuals.bg_stroke.color
            };

            ui.painter().rect_stroke(
                response.rect,
                3.0,
                Stroke {
                    color,
                    ..visuals.bg_stroke
                }
            );
        }

        response
    }
}
