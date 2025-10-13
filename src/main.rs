use fltk::{app, button::Button, prelude::*, window::Window, dialog};
use wifiscanner::{self, scan, Wifi};
use chrono::{self, Utc};

fn main() {
    let app = app::App::default()
        .with_scheme(app::Scheme::Gtk);
    let mut wind = Window::new(100, 100, 500, 400, "Signal Locate");
    let mut button = Button::default()
        .with_size(80, 30)
        .center_of(&wind)
        .with_label("Open File");
    wind.make_resizable(true);
    wind.size_range(450, 350, 0, 0);
    wind.end();
    wind.show();
    button.set_callback(|_| {
        choose_file();
    });
    app.run().unwrap();
}

fn choose_file() -> Option<String> {
    let mut chooser = dialog::FileChooser::new(
        ".",
        "*.{png,jpg}",
        dialog::FileChooserType::Multi,
        "Select Room plan",
    );
    chooser.show();
    chooser.window().set_pos(300, 300);
    while chooser.shown() {
        app::wait();
    }
    return chooser.value(1);
}

fn get_networks() -> Option<Vec<Wifi>> {
    let time_format: &'static str = "%Y-%m-%d %H:%M:%S";
    let scanner_result = scan();
    let current_time = Utc::now().format(time_format);
    println!("Time: {}", current_time);
    match scanner_result {
        Ok(wifis) => {
            if wifis.get(0) != None {
                println!("\nDetected {} Networks at {}: ", wifis.len(), current_time);
                return Some(wifis);
            } else {
                println!("No Networks detected.");
                println!("Please check your WiFi Adapter.");
                return None;
            }
        }
        Err(e) => {
            println!("Scan failed: {:?}", e);
            return None;
        }
    }
}
