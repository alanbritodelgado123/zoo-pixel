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

// ✅ FUNCIÓN CORREGIDA: Usa quad-storage en Android
fn get_save_path() -> String {
    if cfg!(target_os = "android") {
        // En Android, usar nombre de archivo simple (quad-storage maneja la ruta)
        SAVE_FILE.to_string()
    } else {
        // En PC, guardar en directorio actual
        SAVE_FILE.to_string()
    }
}

impl SaveData {
    pub fn cargar() -> Self {
        let path = get_save_path();
        
        #[cfg(target_os = "android")]
        {
            // ✅ Android: Usar quad-storage
            match quad_storage::read(&path) {
                Ok(data) => serde_json::from_slice(&data).unwrap_or_default(),
                Err(_) => Self::default(),
            }
        }
        
        #[cfg(not(target_os = "android"))]
        {
            // ✅ PC: Usar std::fs
            match std::fs::read_to_string(&path) {
                Ok(data) => serde_json::from_str(&data).unwrap_or_default(),
                Err(_) => Self::default(),
            }
        }
    }
    
    pub fn guardar(&self) {
        if let Ok(json) = serde_json::to_string_pretty(self) {
            let path = get_save_path();
            
            #[cfg(target_os = "android")]
            {
                // ✅ Android: Usar quad-storage
                let _ = quad_storage::write(&path, json.as_bytes());
            }
            
            #[cfg(not(target_os = "android"))]
            {
                // ✅ PC: Usar std::fs
                let _ = std::fs::write(&path, json);
            }
        }
    }
    
    pub fn marcar_animal_visto(&mut self, nombre: &str) {
        self.animales_vistos.insert(nombre.to_string());
    }
}