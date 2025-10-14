use fltk::{app, button::Button, dialog, enums::Event, frame::Frame, image::SharedImage, prelude::*, window::Window};
use wifiscanner::{self, scan, Wifi};
use chrono::{self, Utc};
use std::{rc::Rc, cell::RefCell};

fn main() {
    let file_path: Rc<RefCell<Option<String>>> = Rc::new(RefCell::new(None));
    let cached_image: Rc<RefCell<Option<SharedImage>>> = Rc::new(RefCell::new(None));

    let app = app::App::default()
        .with_scheme(app::Scheme::Gtk);
    let mut wind = Window::new(100, 100, 600, 600, "Signal Locate");
    let mut button = Button::default()
        .with_size(80, 30)
        .with_pos(wind.width() / 2 - 80 / 2, 20)
        .with_label("Open File");
    let image_frame = Rc::new(RefCell::new(Frame::new(0, 60, 500, 500, "")));
    wind.make_resizable(true);
    wind.size_range(450, 350, 0, 0);
    wind.end();
    wind.show();

    let image_frame_resize = Rc::clone(&image_frame);
    let cached_img_resize = Rc::clone(&cached_image);
    wind.resize_callback(move |_, _, _, w, h| {
        image_frame_resize.borrow_mut().set_size(w, h - 100);
        if let Some(ref img) = *cached_img_resize.borrow() {
            let mut img = img.clone();
            img.scale(w, h - 100, true, true);
            image_frame_resize.borrow_mut().set_image(Some(img));
        }
    });

    // detect if button has been pressed
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

    // detect if image frame has been clicked
    let image_frame_clicked = Rc::clone(&image_frame);
    let cached_image_clicked = Rc::clone(&cached_image);
    image_frame_clicked.borrow_mut().handle(move |f, ev: Event| {
        return get_click_coords(f, ev, &cached_image_clicked.borrow());
    });

    app.run().unwrap();
}

fn get_click_coords(f: &mut Frame, ev: Event, img: &Option<SharedImage>) -> bool {
    if ev == Event::Push {
        println!("Pushed Image.");
        let click_x = fltk::app::event_x() - f.x();
        let click_y = fltk::app::event_y() - f.y();
        let frame_w = f.width();
        let frame_h = f.height();
        
        if let Some(img) = img {
            let img_w = img.width();
            let img_h = img.height();

            let offset_x = (frame_w - img_w) / 2;
            let offset_y = (frame_h - img_h) / 2;

            let rel_x = click_x - offset_x;
            let rel_y = click_y - offset_y;

            let prop_x: f64 = rel_x as f64 / img_w as f64;
            let prop_y: f64 = rel_y as f64 / img_w as f64;

            println!("Clicked image at {}, {}", prop_x, prop_y);

            return true;
        } else {
            return false;
        }
    } else {
        false
    }
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
