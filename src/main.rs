use gtk::{Application,ApplicationWindow, Button};
use gtk::prelude::*;

//main function
fn main() {
    let app = Application::builder().application_id("fr.testgtk4.com").build();

    app.connect_startup(|_| load_css());
    app.connect_activate(build_ui);

    app.run();
}

//I just copied that xD
fn load_css() {
    // Load the CSS file and add it to the provider
    let provider = gtk::CssProvider::new();
    provider.load_from_data(include_bytes!("style.css"));

    // Add the provider to the default screen
    gtk::StyleContext::add_provider_for_display(
        &gtk::gdk::Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

//Build the UI
fn build_ui(app: &Application){
    
    let main_button = Button::builder()
        .label("Hello World")
        .margin_start(20)
        .margin_top(20)
        .margin_end(20)
        .margin_bottom(20)
        .build();
    let main_window = ApplicationWindow::builder()
        .application(app)
        .title("Test Rust with GTK4")
        .child(&main_button)
        .build();
    main_window.set_css_classes(&["window"]);
    // main_window.set_resizable(false);
    main_window.present();
}