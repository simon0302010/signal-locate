use std::sync::mpsc::{self, Receiver, Sender};
use std::{path::{Path, PathBuf}, sync::Arc};
use std::time::Duration;

use eframe::egui;
use egui::{ComboBox, Image, ImageButton};
use egui_file_dialog::FileDialog;
use egui_extras::install_image_loaders;
use egui_notify::Toasts;

mod wifitools;
use wifitools::{get_networks, strength_by_ssid};

#[derive(Clone)]
#[derive(Debug)]
struct WiFiMeasurement {
    ssid: String,
    strength: f64,
    prop_x: f32,
    prop_y: f32
}

fn main() -> Result<(), eframe::Error> {
    if std::env::consts::OS != "linux" {
        eprintln!("This program is designed to run on Linux. It may not work as expected.")
    }

    if !unsafe { libc::geteuid() == 0 } {
        eprintln!("Please run this program as root.");
        std::process::exit(1);
    }

    eframe::run_native(
        "Signal Locate", 
        eframe::NativeOptions::default(),
        Box::new(|cc| {
            install_image_loaders(&cc.egui_ctx);
            Ok(Box::new(SignalLocate::default()))
        }),
    )
}

struct SignalLocate {
    open_dialog: FileDialog,
    save_dialog: FileDialog,
    img_path: Option<PathBuf>,
    wifi_names: Vec<String>,
    selected_wifi: Option<usize>,
    toasts: Toasts,
    measurement_points: Vec<WiFiMeasurement>,
    measurement_sender: Option<Sender<WiFiMeasurement>>,
    measurement_receiver: Option<Receiver<WiFiMeasurement>>,
}

impl Default for SignalLocate {
    fn default() -> Self {
        let wifis = get_networks();
        if wifis.is_none() {
            eprintln!("Exiting.");
            std::process::exit(1)
        }

        let wifis_names = wifis.as_ref().unwrap().iter().map(|n| n.ssid.clone()).collect::<Vec<String>>();

        let (tx, rx) = mpsc::channel::<WiFiMeasurement>();

        Self {
            open_dialog: FileDialog::new()
                .show_new_folder_button(false)
                .add_file_filter(
                    "Images", 
                    Arc::new(|p: &Path| {
                        p.extension()
                         .and_then(|s| s.to_str())
                         .map(|ext| {
                             let ext = ext.to_ascii_lowercase();
                             ext == "png" || ext == "jpg" || ext == "jpeg"
                         })
                         .unwrap_or(false)
                    })
                ),
            save_dialog: FileDialog::new()
                .show_new_folder_button(true)
                .add_save_extension("PNG files", "png")
                .default_save_extension("PNG files"),
            img_path: None,
            wifi_names: wifis_names,
            selected_wifi: None,
            toasts: Toasts::default(),
            measurement_points: Vec::new(),
            measurement_sender: Some(tx),
            measurement_receiver: Some(rx),
        }
    }
}

impl eframe::App for SignalLocate {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if let Some(receiver) = &self.measurement_receiver {
            while let Ok(measurement) = receiver.try_recv() {
                self.measurement_points.push(measurement);
                self.toasts.info("WiFi signal measurement recorded.").duration(Duration::from_secs(4));
            }
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.horizontal(|ui| {
                    if ui.button("Open Room Plan").clicked() {
                        self.open_dialog.pick_file();
                    }

                    ComboBox::from_label("Select Network")
                        .selected_text(
                            self.selected_wifi
                                .and_then(|i| self.wifi_names.get(i))
                                .unwrap_or(&"Choose...".to_string())
                        )
                        .show_ui(ui, |ui| {
                            for (i, name) in self.wifi_names.iter().enumerate() {
                                ui.selectable_value(&mut self.selected_wifi, Some(i), name);
                            }
                        })

                });

                ui.add_space(5.0);

                self.open_dialog.update(ctx);

                if let Some(path) = self.open_dialog.take_picked() {
                    self.img_path = Some(path.to_path_buf());
                }

                if let Some(image_path) = &self.img_path {
                    if image_path.exists() {
                        let uri = format!("file://{}", image_path.to_string_lossy());
                        let image_element = ui.add(ImageButton::new(Image::new(uri)));
                        if image_element.clicked() {
                            println!("Clicked Image.");
                            if let Some(pos) = ui.input(|i| i.pointer.interact_pos()) {
                                let local_x = pos.x - image_element.rect.min.x;
                                let local_y = pos.y - image_element.rect.min.y;
                                let prop_x = (local_x / image_element.rect.width()).clamp(0.0, 1.0);
                                let prop_y = (local_y / image_element.rect.height()).clamp(0.0, 1.0);
                                println!("Clicked image at ({}, {})", prop_x, prop_y);
                                if let Some(selected_index) = self.selected_wifi {
                                    if let (Some(sender), Some(ssid)) = (&self.measurement_sender, self.wifi_names.get(selected_index)) {
                                        let sender = sender.clone();
                                        let ssid = ssid.clone();
                                        std::thread::spawn(move || {
                                            let signal_strength = strength_by_ssid(ssid.clone());
                                            let measurement = WiFiMeasurement {
                                                ssid,
                                                strength: signal_strength,
                                                prop_x,
                                                prop_y,
                                            };
                                            let _ = sender.send(measurement);
                                        });

                                        self.toasts.info("Please wait for the WiFi signal to be measured.").duration(Duration::from_secs(4));
                                    }
                                } else {
                                    self.toasts.warning("Please select a WiFi Network first.").duration(Duration::from_secs(5));
                                }
                            }
                        }

                        if ui.button("Create Heatmap").clicked() {
                            self.save_dialog.save_file();
                        }

                        self.save_dialog.update(ctx);

                        if let Some(save_path) = self.save_dialog.take_picked() {
                            println!("User saved to: {:?}", save_path.to_str());
                            println!("Measurement Points: {:?}", self.measurement_points);
                        }
                    }
                }

            self.toasts.show(ctx);
            });
        });
    }
}