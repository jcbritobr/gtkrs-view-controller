use gtk::{
    prelude::{ContainerExt, WidgetExt},
    Application, ApplicationWindow, Button,
};

pub struct Ui {
    pub window: ApplicationWindow,
    pub button: Button,
}

impl Ui {
    pub fn new(app: &Application) -> Self {
        let window = ApplicationWindow::builder()
            .application(app)
            .default_width(320)
            .default_height(200)
            .title("Hello, World!")
            .build();

        let button = Button::with_label("Click me");
        window.add(&button);
        window.show_all();

        Ui { window, button }
    }
}
