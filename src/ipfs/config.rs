use std::fs::{self, File};
use std::io::{Write, Read};

use rand::RngCore;
use base16ct::lower;
use serde_json::{Value, Map};

use home;
use crate::utils::constants::{IPFS_CONFIG_PATH_SWARM_SUFF, IPFS_CONFIG_PATH_CONFIG_SUFF, 
    SWARM_KEY_PREFIX, SIZE_SWARM_KEY};

pub fn init_configuration_ipfs() -> std::io::Result<()> {
    let swarm_key_path = home::home_dir().unwrap().join(IPFS_CONFIG_PATH_SWARM_SUFF);
    let config_path = home::home_dir().unwrap().join(IPFS_CONFIG_PATH_CONFIG_SUFF);

    // 1. Generate swarm key
    if !swarm_key_path.exists() {
        let mut key = [0u8; SIZE_SWARM_KEY];
        rand::rng().fill_bytes(&mut key);
        let hex_key = lower::encode_string(&key);

        let swarm_key_content = format!(
            "{}{}\n",
            SWARM_KEY_PREFIX,
            hex_key
        );
        fs::write(&swarm_key_path, swarm_key_content)?;
        println!("🔐 swarm.key générée à {}", swarm_key_path.display());
    } else {
        println!("✅ swarm.key déjà présente, pas de régénération");
    }

    // === 2. Lecture config existante ===
    let mut config_file = File::open(&config_path)?;
    let mut config_data = String::new();
    config_file.read_to_string(&mut config_data)?;

    let mut config_json: Value = serde_json::from_str(&config_data)?;
    let obj = config_json.as_object_mut().expect("config.json must be an object");

    // === 3. Mise à jour ciblée ===

    // 3.1 Routing.Type = "dht"
    obj.insert("Routing".to_string(), Value::String("dht".to_string()));

    // 3.2 AutoTLS.Enabled = false
    obj.entry("AutoTLS")
        .or_insert(Value::Object(Map::new()))
        .as_object_mut()
        .unwrap()
        .insert("Enabled".to_string(), Value::Bool(false));

    // 3.3 Swarm.Transports.Network.Websocket = false
    let swarm = obj.entry("Swarm")
        .or_insert(Value::Object(Map::new()))
        .as_object_mut()
        .unwrap();

    let transports = swarm.entry("Transports")
        .or_insert(Value::Object(Map::new()))
        .as_object_mut()
        .unwrap();

    let network = transports.entry("Network")
        .or_insert(Value::Object(Map::new()))
        .as_object_mut()
        .unwrap();

    network.insert("Websocket".to_string(), Value::Bool(false));

    // 3.4 Supprimer les bootstrap nodes
    obj.insert("Bootstrap".to_string(), Value::Null);

    // === 4. Sauvegarde propre ===
    let formatted = serde_json::to_string_pretty(&config_json)?;
    let mut output = File::create(&config_path)?;
    output.write_all(formatted.as_bytes())?;

    println!("✅ Configuration IPFS privée mise à jour proprement");
    Ok(())
}
