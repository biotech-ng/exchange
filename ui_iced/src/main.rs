use iced::{alignment, Element, Sandbox, Settings};
use iced::widget::{Button, Column, Container, Row, Text};

struct PortalApp {
    count: i32,
}

#[derive(Debug, Clone, Copy)]
enum UiMessage {
    Increment,
    Decrement,
}

impl PortalApp {
    fn search_elements(&self) -> Element<UiMessage> {
        let label = Text::new(format!("S-Count: {}", self.count));
        let incr = Button::new("S-Increment").on_press(UiMessage::Increment);
        let decr = Button::new("S-Decrement").on_press(UiMessage::Decrement);

        let col = Row::new()
            .push(incr)
            .push(label)
            .push(decr);

        Container::new(col)
            .align_x(alignment::Horizontal::Left)
            .align_y(alignment::Vertical::Top)
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .into()
    }

    fn chats_column(&self) -> Element<UiMessage> {
        let label = Text::new(format!("M-Count: {}", self.count));
        let incr = Button::new("M-Increment").on_press(UiMessage::Increment);
        let decr = Button::new("M-Decrement").on_press(UiMessage::Decrement);

        let col = Column::new()
            .push(self.search_elements())
            .push(incr)
            .push(label)
            .push(decr);

        Container::new(col)
            .align_x(alignment::Horizontal::Left)
            .align_y(alignment::Vertical::Top)
            .width(iced::Length::Fill)
            .into()
    }
}

impl Sandbox for PortalApp {
    type Message = UiMessage;

    fn new() -> Self {
        PortalApp { count: 0 }
    }

    fn title(&self) -> String {
        String::from("Counter app")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            UiMessage::Increment => self.count += 1,
            UiMessage::Decrement => self.count -= 1,
        }
    }

    fn view(&self) -> Element<Self::Message> {
        let label = Text::new(format!("R-Count: {}", self.count));
        let incr = Button::new("R-Increment").on_press(UiMessage::Increment);
        let decr = Button::new("R-Decrement").on_press(UiMessage::Decrement);
        let col = Row::new()
            .push(self.chats_column())
            .push(incr)
            .push(label)
            .push(decr);
        Container::new(col)
            .center_x()
            .center_y()
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .into()
    }
}

fn main() -> Result<(), iced::Error> {
    PortalApp::run(Settings::default())
}
