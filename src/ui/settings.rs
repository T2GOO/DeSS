use egui::{Ui, TextEdit};
use crate::backend::ipfs_config::{load_storage_max, save_storage_max};
use once_cell::sync::Lazy;
use std::sync::Mutex;
use crate::backend::swarm::{generate_swarm_key, set_swarm_key};

pub struct SettingsState {
    pub edited_storage: String,
    pub new_key : String,
    pub status: Option<String>,
    pub gen_state: Option<String>,
}

impl SettingsState {
    pub fn new() -> Self {
        Self {
            edited_storage: load_storage_max().unwrap_or_else(|_| "0".to_string()),
            new_key: String::new(),
            status: None,
            gen_state: None,
        }
    }
}

// Utilisation thread-safe, initialisée à la première frame
static SETTINGS_STATE: Lazy<Mutex<SettingsState>> = Lazy::new(|| {
    Mutex::new(SettingsState::new())
});

pub fn show(ui: &mut Ui) {
    let mut state = SETTINGS_STATE.lock().unwrap();

    ui.heading("Paramètres IPFS");
    ui.separator();
    if let Some(msg) = &state.status {
        ui.colored_label(egui::Color32::from_rgb(100, 250, 100), msg);
    }
    ui.separator();
    ui.label("Capacité maximale de stockage (ex: 10GB, 500MB):");
    ui.add(TextEdit::singleline(&mut state.edited_storage).hint_text("Ex: 10GB"));

    ui.horizontal(|ui| {
        if ui.button("Sauvegarder").clicked() {
            let new_value = state.edited_storage.clone();

            match save_storage_max(&new_value) {
                Ok(_) => state.status = Some("Sauvegardé avec succès".to_string()),
                Err(e) => state.status = Some(format!("Erreur: {}", e)),
            }
        }

        if ui.button("↩️ Annuler").clicked() {
            match load_storage_max() {
                Ok(value) => {
                    state.edited_storage = value;
                    state.status = Some("Modifications annulées".into());
                }
                Err(e) => {
                    state.status = Some(format!("Erreur lors de l'annulation: {}", e));
                }
            }
        }
    });
    ui.separator();
    ui.label("Swarm key");
    ui.horizontal(|ui| {
        if ui.button("New key (auto)").clicked() {
            match generate_swarm_key("_", true) {
                Ok((key, _)) => {
                    set_swarm_key(&key).unwrap();
                    state.status = Some(format!("Clé générée: {}", key));
                    state.gen_state = Some(String::from("Key generated: ") + key.as_str());
                    state.new_key = "".to_string();
                }
                Err(e) => {
                    state.status = Some(format!("Imossible to set the key: {}", e));
                    state.gen_state = Some(String::from("No key generated"));
                }
            }
        }
        if let Some(msg) = &state.gen_state {
            ui.label(msg);
        }
    });
    ui.horizontal(|ui| {
        ui.label("New key: ");
        ui.add(TextEdit::singleline(&mut state.new_key).hint_text("0123466789abcdef"));
        if ui.button("Set key").clicked() {
            match set_swarm_key(&state.new_key) {
                Ok(()) => {
                    state.status = Some(format!("Set"));
                    state.gen_state = None;
                }
                Err(e) => {
                    state.status = Some(format!("Imossible to set the key: {}", e));
                }
            }
        }
    });
}
