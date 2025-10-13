use fltk::{app, button::Button, prelude::*, window::Window};
use wifiscanner::{self, scan};
use chrono::{self, Utc};

fn main() {
    let app = app::App::default()
        .with_scheme(app::Scheme::Gtk);
    let mut wind = Window::new(100, 100, 500, 400, "Signal Locate");
    let mut button = Button::default()
        .with_size(60, 30)
        .center_of(&wind)
        .with_label("Button");
    wind.make_resizable(true);
    wind.size_range(450, 350, 0, 0);
    wind.end();
    wind.show();
    button.set_callback(|_| println!("Button clicked."));
    app.run().unwrap();
}

fn get_networks() -> i32 {
    let time_format: &'static str = "%Y-%m-%d %H:%M:%S";
    let scanner_result = scan();
    let current_time = Utc::now().format(time_format);
    println!("Time: {}", current_time);
    match scanner_result {
        Ok(wifis) => {
            if wifis.get(0) != None {
                println!("\nDetected {} Networks at {}: ", wifis.len(), current_time);
                for single_wifi in &wifis {
                    println!("{}", single_wifi.ssid)
                }
                return 0;
            } else {
                println!("No Networks detected.");
                println!("Please check your WiFi Adapter.");
                return 1;
            }
        }
        Err(e) => {
            println!("Scan failed: {:?}", e);
            return 1;
        }
    }
}
