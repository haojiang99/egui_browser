// src/main.rs
mod app;
mod html_renderer;
mod style;
mod ui_components;

use app::EguiBrowser;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([900.0, 700.0]),
        ..Default::default()
    };
    
    eframe::run_native(
        "Browser",
        options,
        Box::new(|_cc| Box::new(EguiBrowser::default())),
    )
}
