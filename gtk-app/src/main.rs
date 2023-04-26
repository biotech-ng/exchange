use std::cell::RefCell;
use std::rc::Rc;
use gtk4::{glib, Application,  ApplicationWindow, Button, Orientation};
use gtk4::glib::{clone, Object};
use gtk4::prelude::{ApplicationExt, ApplicationExtManual, BoxExt, ButtonExt, GtkWindowExt};

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
    use gtk4::subclass::button::ButtonImpl;
    use gtk4::subclass::prelude::{ObjectImpl, ObjectSubclass, WidgetImpl};

    // Object holding the state
    #[derive(Default)]
    pub struct CustomButton;

    // The central trait for subclassing a GObject
    #[glib::object_subclass]
    impl ObjectSubclass for CustomButton {
        const NAME: &'static str = "gtk-appCustomButton";

        type Type = super::CustomButton;
        type ParentType = gtk4::Button;
    }

    // Trait shared by all GObjects
    impl ObjectImpl for CustomButton {}

    // Trait shared by all widgets
    impl WidgetImpl for CustomButton {}

    // Trait shared by all buttons
    impl ButtonImpl for CustomButton {}
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

fn build_ui(application: &Application) {
    // Create two buttons
    let button_increase = Button::builder()
        .label("Increase")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();
    let button_decrease = Button::builder()
        .label("Decrease")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    // A mutable integer
    let number = Rc::new(RefCell::new(0));

    // Connect callbacks
    // When a button is clicked, `number` should be changed
    button_increase.connect_clicked(clone!(@weak number, @weak button_decrease =>
        move |_| {
            *number.borrow_mut() += 1;
            button_decrease.set_label(&number.borrow().to_string());
    }));
    button_decrease.connect_clicked(clone!(@strong button_increase =>
        move |_| {
            *number.borrow_mut() -= 1;
            button_increase.set_label(&number.borrow().to_string());
    }));

    // Add buttons to `gtk_box`
    let gtk_box = gtk4::Box::builder()
        .orientation(Orientation::Vertical)
        .build();
    gtk_box.append(&button_increase);
    gtk_box.append(&button_decrease);

    // Create a window
    let window = ApplicationWindow::builder()
        .application(application)
        .title("My GTK App")
        .child(&gtk_box)
        .resizable(true)
        .build();

    // Present the window
    window.present();
}
