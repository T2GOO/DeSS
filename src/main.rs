mod app;
mod ui;
mod state;
mod backend;
mod utils;

use crate::utils::constants::{DIS_APP_NAME, DIM_WINDOW};

fn main() -> eframe::Result<()> {
    // For test ----
    use crate::backend::swarm::generate_swarm_key;

    let user_input = "test";
    let with_once = true;
    let (key, input) = generate_swarm_key(user_input, with_once).unwrap();
    println!("Generated key: {} / Input: {}", key, input);
    crate::backend::swarm::reinit_swarm_file(&key).unwrap();
    // For test ----

    return Ok(());
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