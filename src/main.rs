extern crate gtk4;
extern crate gio;
extern crate gdk4;

use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, DrawingArea};
use gtk4::gdk::Key;

fn main() {
    let application = Application::new(None::<String>, gio::ApplicationFlags::FLAGS_NONE);

    application.connect_activate(|app| {
        let window = ApplicationWindow::builder()
            .application(app)
            .title("Full-Screen White Canvas")
            .default_width(800)
            .default_height(600)
            .build();

        // Create a DrawingArea widget
        let drawing_area = DrawingArea::new();
        window.set_child(Some(&drawing_area));

        // Draw a white canvas
        drawing_area.set_draw_func(|_, cr, _, _| {
            cr.set_source_rgb(1.0, 1.0, 1.0); // White color
            cr.paint().unwrap();
        });

        // Set the window to full screen
        window.fullscreen();

        // Handle key press events
        let key_controller = gtk4::EventControllerKey::new();
        // Clone the window variable before moving into the closure
        let window_clone = window.clone();
        key_controller.connect_key_pressed(move |_, keyval, _, _| {
            // Print the value of keyval
            println!("Key pressed: {:?}", keyval);
            if keyval == Key::Escape {
                window_clone.close();
            }

            glib::Propagation::Proceed
        });

        window.add_controller(key_controller);

        // Handle mouse click events
        let click_controller = gtk4::GestureClick::new();
        let window_clone = window.clone();
        click_controller.connect_pressed(move |_, n_press, x, y| {
            println!("Mouse clicked: {} times at ({}, {})", n_press, x, y);
            window_clone.close();
        });

        window.add_controller(click_controller);

        window.show();
    });

    application.run();
}
