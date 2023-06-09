use gtk4::glib::Object;
use gtk4::prelude::{
    ApplicationExt, ApplicationExtManual, GtkWindowExt, WidgetExt,
};
use gtk4::{glib, Application, ApplicationWindow, Orientation};

const APP_ID: &str = "org.gtk_rs.HelloWorld2";

fn main() -> glib::ExitCode {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}

mod imp {
    use gtk4::glib;
    use gtk4::prelude::ButtonExt;
    use gtk4::subclass::button::ButtonImpl;
    use gtk4::subclass::prelude::{
        ObjectImpl, ObjectImplExt, ObjectSubclass, ObjectSubclassExt, WidgetImpl,
    };
    use std::cell::Cell;

    // Object holding the state
    #[derive(Default)]
    pub struct CustomButton {
        number: Cell<i32>,
    }

    // The central trait for subclassing a GObject
    #[glib::object_subclass]
    impl ObjectSubclass for CustomButton {
        const NAME: &'static str = "gtk-appCustomButton";

        type Type = super::CustomButton;
        type ParentType = gtk4::Button;
    }

    // Trait shared by all GObjects
    impl ObjectImpl for CustomButton {
        fn constructed(&self) {
            self.parent_constructed();
            self.obj().set_label(&self.number.get().to_string());
        }
    }

    // Trait shared by all widgets
    impl WidgetImpl for CustomButton {}

    // Trait shared by all buttons
    impl ButtonImpl for CustomButton {
        fn clicked(&self) {
            self.number.set(self.number.get() + 1);
            self.obj().set_label(&self.number.get().to_string())
        }
    }
}

glib::wrapper! {
    pub struct CustomButton(ObjectSubclass<imp::CustomButton>)
        @extends gtk4::Button, gtk4::Widget,
        @implements gtk4::Accessible, gtk4::Actionable, gtk4::Buildable, gtk4::ConstraintTarget;
}

impl CustomButton {
    pub fn new() -> Self {
        Object::builder().build()
    }

    pub fn with_label(label: &str) -> Self {
        Object::builder().property("label", label).build()
    }
}

fn build_ui(app: &Application) {
    // Create a button
    let button = CustomButton::new();
    button.set_margin_top(12);
    button.set_margin_bottom(12);
    button.set_margin_start(12);
    button.set_margin_end(12);

    // Create a window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .child(&button)
        .build();

    // Present window
    window.present();
}
