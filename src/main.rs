use adw::prelude::*;
use gtk::gio;
use gtk::glib;
use main_window::MainWindow;

const APP_ID: &str = "me.gtk-rs-test.test";

mod collection_object;
mod task_object;

mod main_window;

fn main() -> glib::ExitCode {
    gio::resources_register_include!("compiled.gresource").expect("Failed to register gresources");

    let app = adw::Application::builder().application_id(APP_ID).build();
    app.connect_activate(activate);
    app.run()
}

fn activate(app: &adw::Application) {
    MainWindow::builder().application(&app).build().present();
}
