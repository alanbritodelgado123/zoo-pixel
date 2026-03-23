// src/save.rs
use std::collections::HashSet;
use serde::{Deserialize, Serialize};
use crate::escena::Escena;

#[derive(Serialize, Deserialize, Default)]
pub struct SaveData {
    pub escena: Option<Escena>,
    pub visitadas: HashSet<Escena>,
    pub animales_vistos: HashSet<String>,
    pub config: ConfigGuardada,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ConfigGuardada {
    pub volumen_musica: f32,
    pub volumen_efectos: f32,
    pub crt: bool,
}

impl Default for ConfigGuardada {
    fn default() -> Self {
        Self {
            volumen_musica: 0.6,
            volumen_efectos: 0.8,
            crt: false,
        }
    }
}

const SAVE_FILE: &str = "zoo_save.json";

impl SaveData {
    pub fn cargar() -> Self {
        match std::fs::read_to_string(SAVE_FILE) {
            Ok(data) => serde_json::from_str(&data).unwrap_or_default(),
            Err(_) => Self::default(),
        }
    }

    pub fn guardar(&self) {
        if let Ok(json) = serde_json::to_string_pretty(self) {
            let _ = std::fs::write(SAVE_FILE, json);
        }
    }

    pub fn marcar_animal_visto(&mut self, nombre: &str) {
        self.animales_vistos.insert(nombre.to_string());
    }
}