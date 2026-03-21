use std::collections::HashSet;
use serde::{Deserialize, Serialize};
use crate::escena::Escena;

#[derive(Serialize, Deserialize, Default)]
pub struct SaveData {
    pub escena: Option<Escena>,
    pub visitadas: HashSet<Escena>,
    pub libreta: Vec<NotaGuardada>,
    pub animales_vistos: HashSet<String>,
    pub eventos_vistos: Vec<String>,
    pub config: ConfigGuardada,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct NotaGuardada {
    pub animal_id: String,
    pub texto: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ConfigGuardada {
    pub volumen_musica: f32,
    pub volumen_efectos: f32,
    pub idioma: String,
}

impl Default for ConfigGuardada {
    fn default() -> Self {
        Self {
            volumen_musica: 0.6,
            volumen_efectos: 0.8,
            idioma: "es".to_string(),
        }
    }
}

const SAVE_FILE: &str = "zoo_save.json";

impl SaveData {
    pub fn cargar() -> Self {
        // Intentar leer del filesystem
        // En Android: los assets internos; en PC: directorio actual
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

    pub fn agregar_nota(&mut self, animal_id: &str, texto: &str) {
        // Reemplazar si ya existe
        self.libreta.retain(|n| n.animal_id != animal_id);
        self.libreta.push(NotaGuardada {
            animal_id: animal_id.to_string(),
            texto: texto.to_string(),
        });
    }

    pub fn nota_de(&self, animal_id: &str) -> Option<&str> {
        self.libreta.iter()
            .find(|n| n.animal_id == animal_id)
            .map(|n| n.texto.as_str())
    }
}