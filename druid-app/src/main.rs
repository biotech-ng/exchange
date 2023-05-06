use druid::widget::{Button, Flex, Label};
use druid::{AppLauncher, LocalizedString, PlatformError, Widget, WidgetExt, WidgetId, WindowDesc};

fn main() -> Result<(), PlatformError> {
    let main_window = WindowDesc::new(ui_builder());
    let data: u32 = 0_u32;
    AppLauncher::with_window(main_window)
        .log_to_console()
        .launch(data)
}

fn custom_widget() -> impl Widget<u32> {
    let text =
        LocalizedString::new("hello-counter").with_arg("count", |data: &u32, _env| (*data).into());

    let label = Label::new(text).padding(5.0).center();
    let button = Button::new("increment")
        .on_click(|_ctx, data: &mut u32, _env| *data += 1)
        .padding(5.0);

    Flex::column().with_child(label).with_child(button)
}

fn ui_builder() -> impl Widget<u32> {
    // The label text will be computed dynamically based on the current locale and count
    let text =
        LocalizedString::new("hello-counter").with_arg("count", |data: &u32, _env| (*data).into());

    let label2 = Label::new(text.clone()).padding(5.0).center();
    let label3 = Label::new(text).padding(5.0).center();

    Flex::row()
        .with_child(custom_widget())
        .with_child(Flex::column().with_child(label2))
        .with_child(Flex::column().with_child(label3))

    // Flex::column().with_child(label).with_child(button)
}
