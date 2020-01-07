extern crate gio;
extern crate gtk;

use gio::prelude::*;
use gtk::prelude::*;

use std::env::args;
fn i32_to_binary_string(number: i32) -> String {
    format!("{:b}", number).chars().collect()
}

fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);

    window.set_title("Unit conversion");
    window.set_border_width(10);
    window.set_position(gtk::WindowPosition::Center);
    window.set_default_size(350, 100);
    let main_window_layout = gtk::Box::new(gtk::Orientation::Vertical, 9);
    let top_result_field = gtk::Box::new(gtk::Orientation::Horizontal, 9);
    let bottom_result_field = gtk::Box::new(gtk::Orientation::Horizontal, 9);

    let button = gtk::Button::new_with_label("Click me!");
    let edit = gtk::Entry::new();
    let label = gtk::Label::new(Some("0"));
    let edit2 = edit.clone();
    let label2 = label.clone();

    button.connect_clicked(move |_| {
        let u: Option<String> = Some(edit2.get_text().unwrap().to_string());
        match u {
            Some(value) => {
                match value.parse::<i32>() {
                    Ok(t) => {                       
                        label2.set_text(i32_to_binary_string(t).as_str());
                    }
                    Err(_) => {
                        label2.set_text("Err, enter valid digits");
                    }
                };
            }
            None => {
                label2.set_text("Err: enter valid digits");
            }
        };
    });

    main_window_layout.pack_start(&top_result_field, true, true, 5);
    main_window_layout.pack_start(&bottom_result_field, true, true, 5);
    top_result_field.pack_start(&edit, true, true, 5);
    top_result_field.pack_start(&button, true, true, 5);
    bottom_result_field.pack_start(&label, true, true, 5);
    window.add(&main_window_layout);
    window.show_all();
}

fn main() {
    let application =
        gtk::Application::new(Some("com.github.gtk-rs.examples.basic"), Default::default())
            .expect("Initialization failed...");

    application.connect_activate(|app| {
        build_ui(app);
    });
    application.run(&args().collect::<Vec<_>>());
}
