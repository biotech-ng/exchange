use iced::{alignment, Alignment, Element, Sandbox, Settings, Theme, theme};
use iced::widget::{Button, Column, Container, Row, Text, text_input};

struct PortalApp {
    count: i32,
    search_input: String
}

#[derive(Debug, Clone)]
enum UiMessage {
    DoNothing,
    Increment,
    Decrement,
    InputChanged(String), // TODO search input changed
    CreateTask, // TODO do search
}

impl PortalApp {
    // Row with menu button and search input
    fn search_elements(&self) -> Element<UiMessage> {
        let menu_btn = Button::new("Menu").on_press(UiMessage::DoNothing);
        let search = text_input("What needs to be done?", self.search_input.as_ref())
            .on_input(UiMessage::InputChanged)
            //.on_submit(UiMessage::CreateTask)
            .padding(5)
            .size(20);

        let col = Row::new()
            .push(menu_btn)
            .push(search)
            .align_items(Alignment::Start);

        Container::new(col)
            .align_x(alignment::Horizontal::Left)
            .align_y(alignment::Vertical::Top)
            .max_width(200.0)
            .width(iced::Length::Shrink)
            .height(iced::Length::Shrink)
            .style(theme::Container::Box)
            .into()
    }

    // Column with search and table
    fn chats_column(&self) -> Element<UiMessage> {
        let label = Text::new(format!("M-Count: {}", self.count));
        let incr = Button::new("M-Increment").on_press(UiMessage::Increment);
        let decr = Button::new("M-Decrement").on_press(UiMessage::Decrement);

        let col = Column::new()
            .push(self.search_elements())
            .push(incr)
            .push(label)
            .push(decr)
            .align_items(Alignment::Start);

        Container::new(col)
            .align_x(alignment::Horizontal::Left)
            .align_y(alignment::Vertical::Top)
            .width(iced::Length::Shrink)
            .height(iced::Length::Fill)
            .style(theme::Container::Box)
            .into()
    }
}

impl Sandbox for PortalApp {
    type Message = UiMessage;

    fn new() -> Self {
        PortalApp { count: 0, search_input: String::new() }
    }

    fn title(&self) -> String {
        String::from("Counter app")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            UiMessage::Increment => self.count += 1,
            UiMessage::Decrement => self.count -= 1,
            UiMessage::DoNothing => (),
            UiMessage::InputChanged(new_input) => self.search_input = new_input,
            UiMessage::CreateTask => self.search_input = "".to_string(),
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
            .push(decr)
            .align_items(Alignment::Start);
        Container::new(col)
            .align_x(alignment::Horizontal::Left)
            .align_y(alignment::Vertical::Top)
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .into()
    }
}

fn main() -> Result<(), iced::Error> {
    PortalApp::run(Settings::default())
}
