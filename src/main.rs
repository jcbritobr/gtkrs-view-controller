mod controller;
mod ui;

use controller::UiController;
use gtk::{
    prelude::{ApplicationExt, ApplicationExtManual},
    Application,
};
use ui::Ui;

fn main() {
    let app = Application::builder()
        .application_id("org.example.hello")
        .build();

    app.connect_activate(|app| {
        let ui = Ui::new(&app);
        UiController::setup(&ui);
    });

    app.run();
}
