use dess::config::MainConfig;
use super::daemon::Daemon;

// Function to star the deamon (server)
pub fn boot_server(config: MainConfig) -> Result<(), Box<dyn std::error::Error>> {
    check_components(config.clone())?;
    // Initialize the daemon
    let _daemon = Daemon::new(config.clone())?;
    Ok(())
}

// Function to check if all components are installed
pub fn check_components(_config: MainConfig) -> Result<(), Box<dyn std::error::Error>> {
    /*TODO : check if all components are installed
    -main.db
    -key.bin
    -all config components
    */
    Ok(())
}
