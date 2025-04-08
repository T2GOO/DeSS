mod app;
mod ui;
mod state;
mod backend;
mod utils;
mod ipfs;

use crate::utils::constants::{DIS_APP_NAME, DIM_WINDOW};
use crate::ipfs::daemon_manager::{start_ipfs_daemon, stop_ipfs_daemon};

// eframe::Result<()>
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // For test ----
    let mut child = start_ipfs_daemon()?;
    std::thread::sleep(std::time::Duration::from_secs(10));
    stop_ipfs_daemon(&mut child)?;
    // For test ----

    return Ok(());
    /*
    let options = eframe::NativeOptions{
        viewport: egui::ViewportBuilder::default().with_inner_size(DIM_WINDOW),
        ..Default::default()
    };
    eframe::run_native(
        DIS_APP_NAME,
        options,
        Box::new(|_cc| Ok(Box::new(app::MainApp::default()))),
    );
    */
}
//cd