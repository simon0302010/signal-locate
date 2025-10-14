use fltk::{app, button::Button, dialog, frame::{self, Frame}, image::{JpegImage, PngImage, SvgImage}, prelude::*, window::Window};
use wifiscanner::{self, scan, Wifi};
use chrono::{self, Utc};
use std::{rc::Rc, cell::RefCell};

fn main() {
    let app = app::App::default()
        .with_scheme(app::Scheme::Gtk);
    let mut wind = Window::new(100, 100, 500, 400, "Signal Locate");
    let mut button = Button::default()
        .with_size(80, 30)
        .with_pos(210, 20)
        .with_label("Open File");
    let image_frame = Rc::new(RefCell::new(Frame::new(0, 60, 500, 440, "")));
    wind.make_resizable(true);
    wind.size_range(450, 350, 0, 0);
    wind.end();
    wind.show();

    let image_frame_resize = Rc::clone(&image_frame);
    wind.resize_callback(move |_, _, _, w, h| {
        image_frame_resize.borrow_mut().set_size(w, h - 60);
    });

    let image_frame_button = Rc::clone(&image_frame);
    button.set_callback(move |_| {
        let file_path = choose_file();
        println!("{:?}", file_path);
        if let Some(path) = file_path {
            set_image(&path, &mut image_frame_button.borrow_mut());
        }
    });
    app.run().unwrap();
}

fn choose_file() -> Option<String> {
    let mut chooser = dialog::FileChooser::new(
        ".",
        "*.{png,jpg,svg}",
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

fn set_image(image_path: &str, image_frame: &mut frame::Frame) {
    if image_path.ends_with(".png") {
        match PngImage::load(image_path) {
            Ok(img) => {
                image_frame.set_image(Some(img));
            }
            Err(e) => {
                println!("An error occurred: {:?}", e)
            }
        }
    } else if image_path.ends_with(".jpg") || image_path.ends_with(".jpeg") {
        match JpegImage::load(image_path) {
            Ok(img) => {
                image_frame.set_image(Some(img));
            }
            Err(e) => {
                println!("An error occurred: {:?}", e)
            }
        }
    } else if image_path.ends_with(".svg") {
        match SvgImage::load(image_path) {
            Ok(img) => {
                image_frame.set_image(Some(img));
            }
            Err(e) => {
                println!("An error occurred: {:?}", e)
            }
        }
    } else {
        println!("Unsupported image format.")
    }
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
