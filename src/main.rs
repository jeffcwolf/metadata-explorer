mod app;
mod data;
mod prefs;
mod ui;

use eframe::egui;
use app::BiblioAnalyzerApp;

impl eframe::App for BiblioAnalyzerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ui::render_main_ui(self, ctx);
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_title("Bibliographic Metadata Analyzer"),
        ..Default::default()
    };

    eframe::run_native(
        "Bibliographic Metadata Analyzer",
        options,
        Box::new(|cc| Ok(Box::new(BiblioAnalyzerApp::new(cc)))),
    )
}