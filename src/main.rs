use std::{path::{Path, PathBuf}, sync::Arc};

use eframe::egui;
use egui::{Image, ImageButton};
use egui_file_dialog::FileDialog;
use egui_extras::install_image_loaders;

fn main() -> Result<(), eframe::Error> {
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
    img_path: Option<PathBuf>,
}

impl Default for SignalLocate {
    fn default() -> Self {
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
            img_path: None,
        }
    }
}

impl eframe::App for SignalLocate {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                if ui.button("Open Room Plan").clicked() {
                    self.open_dialog.pick_file();
                }

                ui.add_space(5.0);

                self.open_dialog.update(ctx);

                if let Some(path) = self.open_dialog.take_picked() {
                    self.img_path = Some(path.to_path_buf());
                }

                if let Some(image_path) = &self.img_path && image_path.exists() {
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
                        }
                    }
                }
            });
        });
    }
}