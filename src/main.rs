use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    eframe::run_native(
        "Signal Locate", 
        eframe::NativeOptions::default(),
        Box::new(|_| Ok(Box::new(SignalLocate::default())))
    )
}

struct SignalLocate {
    img_path: String,
}

impl Default for SignalLocate {
    fn default() -> Self {
        Self {
            img_path: "".to_string()
        }
    }
}

impl eframe::App for SignalLocate {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Enter path to room plan: ");
            ui.text_edit_singleline(&mut self.img_path);

            if ui.button("Create Heatmap").clicked() {
                if !self.img_path.is_empty() {
                    println!("Image Path: {}.", self.img_path);
                } else {
                    println!("Image Path is empty.")
                }
            }
        });
    }
}