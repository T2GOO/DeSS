#![allow(dead_code)] 
#![allow(unused_variables)]

pub mod storage;
pub mod registry;
pub mod security;
pub mod core;

use core::boot::boot_server;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get config
    let config = dess::config::MainConfig::load()?;
    // Start the daemon
    boot_server(config)?;
    Ok(())
}
