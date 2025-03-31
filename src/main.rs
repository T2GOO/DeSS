mod app;
mod ui;
mod state;
mod backend;

use crate::ui::constants::{DIS_APP_NAME, DIM_WINDOW};

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions{
        viewport: egui::ViewportBuilder::default().with_inner_size(DIM_WINDOW),
        ..Default::default()
    };
    eframe::run_native(
        DIS_APP_NAME,
        options,
        Box::new(|_cc| Ok(Box::new(app::MainApp::default()))),
    )
}
//