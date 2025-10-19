use fltk::{app, button::Button, dialog::{self, alert_default, message_default}, enums::Event, frame::Frame, image::{SharedImage}, menu::Choice, prelude::*, window::Window};
use std::{cell::RefCell, rc::Rc};
use image::RgbImage;
use libc;

use eframe::egui;

mod heatmap;
use heatmap::gen_heatmap;

mod wifitools;
use wifitools::{get_networks, strength_by_ssid};

#[derive(Clone)]
#[derive(Debug)]
struct WiFiMeasurement {
    ssid: String,
    strength: f64,
    prop_x: f64,
    prop_y: f64
}

fn main() {
    if std::env::consts::OS != "linux" {
        eprintln!("This program is designed to run on Linux. It may not work as expected.")
    }

    if !unsafe { libc::geteuid() == 0 } {
        eprintln!("Please run this program as root.");
        return;
    }

    let file_path: Rc<RefCell<Option<String>>> = Rc::new(RefCell::new(None));
    let cached_image: Rc<RefCell<Option<SharedImage>>> = Rc::new(RefCell::new(None));
    let measurement_points: Rc<RefCell<Vec<WiFiMeasurement>>> = Rc::new(RefCell::new(Vec::new()));

    let wifis = get_networks();
    if wifis.is_none() {
        println!("Exiting.");
        return;
    }

    let app = app::App::default()
        .with_scheme(app::Scheme::Gtk);
    let mut wind = Window::new(100, 100, 600, 700, "Signal Locate");
    let mut open_button = Button::default()
        .with_size(160, 30)
        .with_pos(wind.width() / 2 - 500 / 2, 20)
        .with_label("Open Room Plan");
    
    let wifi_choice = Rc::new(RefCell::new(Choice::new(
        wind.width() / 2 + 50, 20, 200, 30, "Select Network:"
    )));    
    wifi_choice.borrow_mut().add_choice("Select");
    for wifi_network in wifis.as_ref().unwrap() {
        wifi_choice.borrow_mut().add_choice(&wifi_network.ssid);
    }
    wifi_choice.borrow_mut().set_value(0);
    
    let image_frame = Rc::new(RefCell::new(Frame::new(0, 80, 500, 560, "")));

    let mut save_button = Button::default()
        .with_size(160, 30)
        .with_pos(wind.width() / 2 - 80, wind.height() - 50)
        .with_label("Create Heatmap");

    wind.make_resizable(true);
    wind.size_range(500, 400, 0, 0);
    wind.end();
    wind.show();

    let image_frame_resize = Rc::clone(&image_frame);
    let cached_img_resize = Rc::clone(&cached_image);
    wind.resize_callback(move |_, _, _, w, h| {
        image_frame_resize.borrow_mut().set_size(w, h - 140);
        if let Some(ref img) = *cached_img_resize.borrow() {
            let mut img = img.clone();
            img.scale(w, h - 140, true, true);
            image_frame_resize.borrow_mut().set_image(Some(img));
        }
    });

    // detect if open button has been pressed
    let image_frame_open_button = Rc::clone(&image_frame);
    let file_path_open_button = Rc::clone(&file_path);
    let cached_image_open_button = Rc::clone(&cached_image);
    open_button.set_callback(move |_| {
        let new_path = choose_file();
        println!("Loaded image: {}", new_path.clone().unwrap_or_default().to_string());
        *file_path_open_button.borrow_mut() = new_path;
        if let Some(path) = file_path_open_button.borrow().as_ref() {
            // load and cache
            if let Ok(img) = fltk::image::SharedImage::load(&path) {
                *cached_image_open_button.borrow_mut() = Some(img);
            } else {
                *cached_image_open_button.borrow_mut() = None;
            }
            // draw
            if let Some(ref img) = *cached_image_open_button.borrow() {
                let mut img = img.clone();
                let frame_w = image_frame_open_button.borrow().width();
                let frame_h = image_frame_open_button.borrow().height();
                img.scale(frame_w, frame_h, true, true);
                image_frame_open_button.borrow_mut().set_image(Some(img));
            }
        }
    });

    let measurements_points_save = Rc::clone(&measurement_points);
    let wifi_choice_save = Rc::clone(&wifi_choice);
    let file_path_save = Rc::clone(&file_path);
    save_button.set_callback(move |_| {
        if !measurements_points_save.borrow().is_empty() {
            let wifi_choice_str = &wifi_choice_save.borrow().choice().unwrap_or_default();
            println!("Measurements: {:?}", measurements_points_save.borrow());
            if let Some(save_path) = save_dialog(&wifi_choice_str) {
                println!("Save Path: {}", save_path);

                let temp_img = if let Some(ref path) = *file_path_save.borrow() {
                    image::open(path)
                } else {
                    eprintln!("No file selected.");
                    return;
                };

                let (img_width, img_height) = match temp_img {
                    Ok(ref img) => (img.width(), img.height()),
                    Err(_) => {
                        eprintln!("Failed to load image for heatmap.");
                        alert_default("Failed to load image for heatmap.");
                        return;
                    }
                };

                let filtered_measurements: Vec<_> = if !wifi_choice_str.is_empty() {
                    measurements_points_save
                        .borrow()
                        .iter()
                        .filter(|wifi| wifi.ssid == *wifi_choice_str)
                        .cloned()
                        .collect()
                } else {
                    measurements_points_save.borrow().clone()
                };

                let points: Vec<(f64, f64, f64)> = filtered_measurements
                    .iter()
                    .map(|m| (
                        m.prop_x * img_width as f64,
                        m.prop_y * img_height as f64,
                        m.strength
                    ))
                    .collect();

                let generated_heatmap = gen_heatmap(&points, img_width as usize, img_height as usize, (img_width as f64 * (1.0 / points.len() as f64 + 0.05)) as usize);
                if let Some(img_path) = &*file_path_save.borrow() {
                    let overlayed_heatmap = overlay_image(img_path, &generated_heatmap);

                    match overlayed_heatmap.save(save_path) {
                        Ok(_) => message_default("Successfully created heatmap."),
                        Err(e) => {
                            eprintln!("Failed to save heatmap: {}", e);
                            alert_default("Failed to save heatmap.");
                        }
                    }
                } else {
                    eprintln!("Could not load image for overlaying. Only saving heatmap");
                    alert_default("Could not load image for overlaying. Only saving heatmap.");
                    
                    match generated_heatmap.save(save_path) {
                        Ok(_) => message_default("Successfully created heatmap."),
                        Err(e) => {
                            eprintln!("Failed to save heatmap: {}", e);
                            alert_default("Failed to save heatmap.");
                        }
                    }
                }
            }
        } else {
            eprintln!("No measurements found. Alerting user.");
            alert_default("Please take some measurements first by clicking the image.");
        }
    });

    // detect if image frame has been clicked
    let image_frame_clicked = Rc::clone(&image_frame);
    let cached_image_clicked = Rc::clone(&cached_image);
    let wifi_choice_clicked = Rc::clone(&wifi_choice);
    let wifi_measurements_clicked = Rc::clone(&measurement_points);
    image_frame_clicked.borrow_mut().handle(move |f, ev: Event| {
        if ev == Event::Push {
            return handle_image_click(f, &cached_image_clicked.borrow(), &wifi_choice_clicked.borrow(), &mut wifi_measurements_clicked.borrow_mut());
        } else {
            return false;
        }
    });

    app.run().unwrap();
}

fn overlay_image(file_path: &str, heatmap_img: &RgbImage) -> RgbImage {
    let mut base_img_raw = image::open(file_path)
        .expect("Failed to load base image")
        .to_rgb8();

    for (base_pixel, heat_pixel) in base_img_raw.pixels_mut().zip(heatmap_img.pixels()) {
        for i in 0..3 {
            base_pixel[i] = ((base_pixel[i] as f32) * 0.40 + (heat_pixel[i] as f32) * 0.60) as u8;
        }
    }

    base_img_raw
}

fn handle_image_click(f: &mut Frame, img: &Option<SharedImage>, wifi_choice: &Choice, wifi_measurements: &mut Vec<WiFiMeasurement>) -> bool {
    println!("User clicked on image frame.");
    let click_x = fltk::app::event_x() - f.x();
    let click_y = fltk::app::event_y() - f.y();
    let frame_w = f.width();
    let frame_h = f.height();

    if let Some(img) = img {
        // check if wifi is selected
        if wifi_choice.value() == 0 {
            eprintln!("No WiFi selected. Alerting user.");
            alert_default("Please select a WiFi Network first.");
            return false;
        }

        let img_w = img.width();
        let img_h = img.height();

        let offset_x = (frame_w - img_w) / 2;
        let offset_y = (frame_h - img_h) / 2;

        let rel_x = click_x - offset_x;
        let rel_y = click_y - offset_y;

        let prop_x: f64 = rel_x as f64 / img_w as f64;
        let prop_y: f64 = rel_y as f64 / img_h as f64;            

        println!("User clicked image at {}, {}", prop_x, prop_y);

        let ssid = wifi_choice.choice().unwrap_or_default();

        let signal_strength = strength_by_ssid(ssid.clone());

        let current_measurement = WiFiMeasurement {
            ssid: ssid.clone(),
            strength: signal_strength,
            prop_x: prop_x,
            prop_y: prop_y
        };

        println!("Measurement: {:?}", &current_measurement);

        wifi_measurements.push(current_measurement);

        message_default("WiFi signal measurement recorded.");

        return true;
    } else {
        return false;
    }
}

fn choose_file() -> Option<String> {
    let mut chooser = dialog::FileChooser::new(
        ".",
        "*.{png,jpg,jpeg,bmp,gif,svg}",
        dialog::FileChooserType::Single,
        "Select Room plan",
    );
    chooser.show();
    while chooser.shown() {
        app::wait();
    }
    return chooser.value(1);
}

fn save_dialog(wifi_name: &str) -> Option<String> {
    let mut chooser = dialog::FileChooser::new(
        ".",
        "*.png",
        dialog::FileChooserType::Create,
        "Save Heatmap",
    );
    chooser.set_filter("*.png");
    chooser.set_value(&format!("heatmap_{}.png", wifi_name));
    chooser.show();
    while chooser.shown() {
        app::wait();
    }
    chooser.value(1)
}