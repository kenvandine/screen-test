extern crate gtk4;
extern crate gio;
extern crate gdk4;

use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, DrawingArea};
use std::cell::Cell;
use std::rc::Rc;

fn main() {
    let application = Application::new(None::<String>, gio::ApplicationFlags::FLAGS_NONE);

    application.connect_activate(|app| {
        let window = ApplicationWindow::builder()
            .application(app)
            .title("Full-Screen Color Test")
            .default_width(800)
            .default_height(600)
            .build();

        // Create a DrawingArea widget
        let drawing_area = DrawingArea::new();
        window.set_child(Some(&drawing_area));

        // Color index: 0=white, 1=blue, 2=red, 3=black, 4=exit
        let color_index = Rc::new(Cell::new(0));

        // Draw canvas with current color
        let color_index_clone = color_index.clone();
        drawing_area.set_draw_func(move |_, cr, _, _| {
            let idx = color_index_clone.get();
            match idx {
                0 => cr.set_source_rgb(1.0, 1.0, 1.0), // White
                1 => cr.set_source_rgb(0.0, 0.0, 1.0), // Blue
                2 => cr.set_source_rgb(1.0, 0.0, 0.0), // Red
                3 => cr.set_source_rgb(0.0, 0.0, 0.0), // Black
                _ => cr.set_source_rgb(1.0, 1.0, 1.0), // Default to white
            }
            cr.paint().unwrap();
        });

        // Set the window to full screen
        window.fullscreen();

        // Handle key press events
        let key_controller = gtk4::EventControllerKey::new();
        let window_clone = window.clone();
        let drawing_area_clone = drawing_area.clone();
        let color_index_clone = color_index.clone();
        key_controller.connect_key_pressed(move |_, keyval, _, _| {
            println!("Key pressed: {:?}", keyval);
            let current = color_index_clone.get();
            if current >= 3 {
                window_clone.close();
            } else {
                color_index_clone.set(current + 1);
                drawing_area_clone.queue_draw();
            }
            glib::Propagation::Proceed
        });

        window.add_controller(key_controller);

        // Handle mouse click events (ignore touch screen)
        let click_controller = gtk4::GestureClick::new();
        let window_clone = window.clone();
        let drawing_area_clone = drawing_area.clone();
        let color_index_clone = color_index.clone();
        click_controller.connect_pressed(move |gesture, n_press, x, y| {
            // Check if this is a touch event and ignore it
            if let Some(device) = gesture.device() {
                if device.source() == gdk4::InputSource::Touchscreen {
                    println!("Touch event ignored at ({}, {})", x, y);
                    return;
                }
            }
            println!("Mouse clicked: {} times at ({}, {})", n_press, x, y);
            let current = color_index_clone.get();
            if current >= 3 {
                window_clone.close();
            } else {
                color_index_clone.set(current + 1);
                drawing_area_clone.queue_draw();
            }
        });

        window.add_controller(click_controller);

        window.show();
    });

    application.run();
}
