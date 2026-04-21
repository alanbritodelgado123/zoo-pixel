// src/save.rs
use std::collections::HashSet;
use serde::{Deserialize, Serialize};
use crate::escena::Escena;

// =====================================================================
//  ESTRUCTURAS DE DATOS
// =====================================================================

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

// =====================================================================
//  CONSTANTES
// =====================================================================

const SAVE_FILE: &str = "zoo_save.json";

// =====================================================================
//  IMPLEMENTACIÓN
// =====================================================================

impl SaveData {
    pub fn cargar() -> Self {
        #[cfg(target_os = "android")]
        {
            match quad_storage::read(SAVE_FILE) {
                Ok(data) => {
                    serde_json::from_slice(&data).unwrap_or_default()
                }
                Err(_) => {
                    println!("📂 No hay guardado previo (Android), iniciando nuevo.");
                    Self::default()
                }
            }
        }

        #[cfg(not(target_os = "android"))]
        {
            match std::fs::read_to_string(SAVE_FILE) {
                Ok(data) => {
                    serde_json::from_str(&data).unwrap_or_else(|e| {
                        println!("⚠️ Error leyendo guardado: {}. Iniciando nuevo.", e);
                        Self::default()
                    })
                }
                Err(_) => {
                    println!("📂 No hay guardado previo (PC), iniciando nuevo.");
                    Self::default()
                }
            }
        }
    }

    pub fn guardar(&self) {
        match serde_json::to_string_pretty(self) {
            Ok(json) => {
                #[cfg(target_os = "android")]
                {
                    if let Err(e) = quad_storage::write(SAVE_FILE, json.as_bytes()) {
                        println!("❌ Error guardando (Android): {:?}", e);
                    }
                }

                #[cfg(not(target_os = "android"))]
                {
                    if let Err(e) = std::fs::write(SAVE_FILE, &json) {
                        println!("❌ Error guardando (PC): {}", e);
                    }
                }
            }
            Err(e) => println!("❌ Error serializando guardado: {}", e),
        }
    }

    pub fn marcar_animal_visto(&mut self, nombre: &str) {
        self.animales_vistos.insert(nombre.to_string());
    }

    pub fn total_vistos(&self) -> usize {
        self.animales_vistos.len()
    }

    pub fn total_zonas_visitadas(&self) -> usize {
        self.visitadas.len()
    }

    pub fn porcentaje_completado(&self, total_animales: usize) -> f32 {
        if total_animales == 0 {
            return 0.0;
        }
        (self.animales_vistos.len() as f32 / total_animales as f32 * 100.0)
            .min(100.0)
    }
}