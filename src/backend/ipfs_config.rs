use std::fs;
use std::path::PathBuf;
use serde_json::Value;
use anyhow::{Result, Context};
use home;

pub fn get_config_path() -> PathBuf {
    home::home_dir().unwrap().join(".ipfs/config")
}

pub fn load_storage_max() -> Result<String> {
    let content = fs::read_to_string(get_config_path())
        .context("Impossible de lire le fichier de config IPFS")?;

    let json: Value = serde_json::from_str(&content)
        .context("JSON invalide dans le fichier de config IPFS")?;

    let storage = json["Datastore"]["StorageMax"]
        .as_str()
        .ok_or_else(|| anyhow::anyhow!("Champ StorageMax manquant ou invalide"))?;

    Ok(storage.to_string())
}

pub fn save_storage_max(new_value: &str) -> Result<()> {
    let path = get_config_path();
    let content = fs::read_to_string(&path)
        .context("Impossible de lire le fichier de config IPFS")?;

    let mut json: Value = serde_json::from_str(&content)
        .context("JSON invalide dans le fichier de config IPFS")?;

    json["Datastore"]["StorageMax"] = Value::String(new_value.to_string());

    let new_content = serde_json::to_string_pretty(&json)?;
    fs::write(path, new_content).context("Impossible d’écrire la config mise à jour")?;
    Ok(())
}
