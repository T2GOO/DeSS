// Manage swarm keys

use sha2::{Sha256, Digest};
use rand::{distr::Alphanumeric, Rng};
use std::fs::OpenOptions;
use std::io::Write;
use home;

use crate::utils::constants::IPFS_CONFIG_PATH_MAIN_SUFF;

pub fn generate_swarm_key(user_input: &str, with_once: bool) -> Result<(String, String), Box<dyn std::error::Error>> {
    let mut hasher = Sha256::new();
    let mut str_input : String = user_input.to_string();
    if with_once || user_input.is_empty() {
        let once: String = rand::rng()
            .sample_iter(&Alphanumeric)
            .take(30)
            .map(char::from)
            .collect();
        str_input.push_str(&"_");
        str_input.push_str(&once);
    }
    hasher.update(&str_input);
    Ok((hasher.finalize()
        .iter()
        .map(|byte| format!("{:02x}", byte))
        .collect::<String>(),
        str_input))
}

pub fn set_swarm_key (swarm_id : &str) -> Result<(), Box<dyn std::error::Error>> {
    let file_path = home::home_dir().unwrap()
        .join(IPFS_CONFIG_PATH_MAIN_SUFF)
        .join("swarm.key");


    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(file_path)?;

    writeln!(file, "/key/swarm/psk/1.0.0/")?;
    writeln!(file, "/base16/")?;
    writeln!(file, "{}", swarm_id)?;

    Ok(())
}
