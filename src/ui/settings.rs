use egui::{Ui, TextEdit};
use crate::backend::ipfs_config::{load_storage_max, save_storage_max};
use once_cell::sync::Lazy;
use std::sync::Mutex;

pub struct SettingsState {
    pub edited_storage: String,
    pub status: Option<String>,
}

impl SettingsState {
    pub fn new() -> Self {
        Self {
            edited_storage: load_storage_max().unwrap_or_else(|_| "0".to_string()),
            status: None,
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
        ui.colored_label(egui::Color32::from_rgb(220, 100, 100), msg);
    }

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

    if let Some(status) = &state.status {
        ui.label(status);
    }
}
