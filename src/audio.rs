use std::collections::HashMap;
use macroquad::audio::*;
use crate::escena::Escena;

pub struct AudioManager {
    ambientes: HashMap<Escena, Sound>,
    efectos: HashMap<String, Sound>,
    duraciones: HashMap<String, f32>,
    sonido_actual: Option<Escena>,
    volumen_musica: f32,
    volumen_efectos: f32,
}

impl AudioManager {
    pub fn new() -> Self {
        Self {
            ambientes: HashMap::new(),
            efectos: HashMap::new(),
            duraciones: HashMap::new(),
            sonido_actual: None,
            volumen_musica: 0.6,
            volumen_efectos: 0.8,
        }
    }

    pub async fn agregar_ambiente(&mut self, escena: Escena, bytes: &[u8]) {
        if let Ok(sound) = load_sound_from_bytes(bytes).await {
            self.ambientes.insert(escena, sound);
        }
    }

    pub async fn agregar_efecto(&mut self, nombre: &str, bytes: &[u8]) {
        if let Ok(sound) = load_sound_from_bytes(bytes).await {
            self.efectos.insert(nombre.to_string(), sound);
            // Intentar detectar duración del WAV
            if let Some(dur) = duracion_wav(bytes) {
                self.duraciones.insert(nombre.to_string(), dur);
            }
        }
    }

    /// Duración de un efecto en segundos (0.0 si no se pudo detectar)
    pub fn duracion_efecto(&self, nombre: &str) -> f32 {
        self.duraciones.get(nombre).copied().unwrap_or(0.0)
    }

    /// Parar el ambiente actual inmediatamente
    pub fn parar_ambiente(&mut self) {
        if let Some(escena) = self.sonido_actual.take() {
            if let Some(sound) = self.ambientes.get(&escena) {
                stop_sound(sound);
            }
        }
    }

    /// Iniciar el ambiente de una escena (sin parar nada antes)
    pub fn iniciar_ambiente(&mut self, escena: Escena) {
        if let Some(sound) = self.ambientes.get(&escena) {
            play_sound(
                sound,
                PlaySoundParams {
                    looped: true,
                    volume: self.volumen_musica,
                },
            );
        }
        self.sonido_actual = Some(escena);
    }

    /// Reproducir efecto puntual
    pub fn efecto(&self, nombre: &str) {
        if let Some(sound) = self.efectos.get(nombre) {
            play_sound(
                sound,
                PlaySoundParams {
                    looped: false,
                    volume: self.volumen_efectos,
                },
            );
        }
    }
}

// =====================================================================
//  Parser de duración WAV (desde bytes crudos)
// =====================================================================

fn duracion_wav(bytes: &[u8]) -> Option<f32> {
    // Mínimo 44 bytes para un WAV válido
    if bytes.len() < 44 {
        return None;
    }

    // Verificar header RIFF + WAVE
    if &bytes[0..4] != b"RIFF" || &bytes[8..12] != b"WAVE" {
        return None;
    }

    // Byte rate está en offset 28 (4 bytes, little endian)
    let byte_rate = u32::from_le_bytes([
        bytes[28], bytes[29], bytes[30], bytes[31],
    ]);
    if byte_rate == 0 {
        return None;
    }

    // Buscar el chunk "data" para obtener su tamaño
    let mut pos = 12;
    while pos + 8 <= bytes.len() {
        let chunk_id = &bytes[pos..pos + 4];
        let chunk_size = u32::from_le_bytes([
            bytes[pos + 4],
            bytes[pos + 5],
            bytes[pos + 6],
            bytes[pos + 7],
        ]);

        if chunk_id == b"data" {
            return Some(chunk_size as f32 / byte_rate as f32);
        }

        // Avanzar al siguiente chunk
        pos += 8 + chunk_size as usize;
        // Alinear a 2 bytes (padding WAV)
        if pos % 2 != 0 {
            pos += 1;
        }
    }

    None
}