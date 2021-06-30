use gtk::prelude::{ButtonExt, GtkWindowExt};

use crate::ui::Ui;

pub struct UiController {}

impl UiController {
    pub fn setup(ui: &Ui) {
        close_window(&ui);
    }
}

fn close_window(ui: &Ui) {
    let button = ui.button.clone();
    let window = ui.window.clone();
    button.connect_clicked(move |_| window.close());
}
