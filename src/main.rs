use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, ListBox, WindowType, Label, ListBoxRow};

use rusty_tft_helper::champion::ChampionPool;

fn main() {
    let application = Application::builder()
        .application_id("com.example.FirstGtkApp")
        .build();

    application.connect_activate(|app| {
        let champion_pool = ChampionPool::new().expect("Unable to load champion pool.");

        let window = ApplicationWindow::builder()
            .application(app)
            .title("Rusty TFT Helper")
            .default_width(1280)
            .default_height(720)
            .build();

        champion_pool.render(&window);

        window.show_all();
    });

    application.run();
}
