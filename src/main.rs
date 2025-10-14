use fltk::{app, button::Button, dialog, frame::{self, Frame}, image::{JpegImage, PngImage, SvgImage, SharedImage}, prelude::*, window::Window};
use wifiscanner::{self, scan, Wifi};
use chrono::{self, Utc};
use std::{rc::Rc, cell::RefCell};

fn main() {
    let file_path: Rc<RefCell<Option<String>>> = Rc::new(RefCell::new(None));
    let cached_image: Rc<RefCell<Option<SharedImage>>> = Rc::new(RefCell::new(None));

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
    let cached_img_resize = Rc::clone(&cached_image);
    wind.resize_callback(move |_, _, _, w, h| {
        image_frame_resize.borrow_mut().set_size(w, h - 60);
        if let Some(ref img) = *cached_img_resize.borrow() {
            let mut img = img.clone();
            img.scale(w, h -60, true, true);
            image_frame_resize.borrow_mut().set_image(Some(img));
        }
    });

    let image_frame_button = Rc::clone(&image_frame);
    let file_path_button = Rc::clone(&file_path);
    let cached_image_button = Rc::clone(&cached_image);
    button.set_callback(move |_| {
        let new_path = choose_file();
        println!("{:?}", new_path);
        *file_path_button.borrow_mut() = new_path;
        if let Some(path) = file_path_button.borrow().as_ref() {
            // load and cache
            if let Ok(img) = fltk::image::SharedImage::load(&path) {
                *cached_image_button.borrow_mut() = Some(img);
            } else {
                *cached_image_button.borrow_mut() = None;
            }
            // draw
            if let Some(ref img) = *cached_image_button.borrow() {
                let mut img = img.clone();
                let frame_w = image_frame_button.borrow().width();
                let frame_h = image_frame_button.borrow().height();
                img.scale(frame_w, frame_h, true, true);
                image_frame_button.borrow_mut().set_image(Some(img));
            }
        }
    });
    app.run().unwrap();
}

fn choose_file() -> Option<String>{
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
    let frame_w = image_frame.width();
    let frame_h = image_frame.height();

    if image_path.ends_with(".png") {
        match PngImage::load(image_path) {
            Ok(mut img) => {
                img.scale(frame_w, frame_h, true, true);
                image_frame.set_image(Some(img));
            }
            Err(e) => {
                println!("An error occurred: {:?}", e)
            }
        }
    } else if image_path.ends_with(".jpg") || image_path.ends_with(".jpeg") {
        match JpegImage::load(image_path) {
            Ok(mut img) => {
                img.scale(frame_w, frame_h, true, true);
                image_frame.set_image(Some(img));
            }
            Err(e) => {
                println!("An error occurred: {:?}", e)
            }
        }
    } else if image_path.ends_with(".svg") {
        match SvgImage::load(image_path) {
            Ok(mut img) => {
                img.scale(frame_w, frame_h, true, true);
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
