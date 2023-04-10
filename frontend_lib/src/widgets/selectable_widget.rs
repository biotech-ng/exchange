use egui::{Color32, Response, Stroke, Ui, Widget};

pub struct SelectableWidget<W: Widget> {
    widget: W,
}

impl<W: Widget> SelectableWidget<W> {
    pub fn new(widget: W) -> Self {
        Self { widget }
    }

    pub fn ui(self, ui: &mut Ui) -> Response {
        let response = self.widget.ui(ui);

        let selected = response.clicked.iter().any(|x| *x);

        if response.hovered || selected {
            let visuals = ui.style().interact(&response);

            let color = if selected {
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
                },
            );
        }

        response
    }
}
