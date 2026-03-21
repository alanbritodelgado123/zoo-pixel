use std::collections::HashMap;
use macroquad::audio::*;
use crate::escena::Escena;

enum EstadoAudio {
    Sonando,
    FadeOut { vol_actual: f32 },
    EsperandoTransicion,
    Silencio,
}

pub struct AudioManager {
    ambientes: HashMap<Escena, Sound>,
    efectos: HashMap<String, Sound>,
    duraciones: HashMap<String, f32>,
    sonido_actual: Option<Escena>,
    estado: EstadoAudio,
    volumen_musica: f32,
    volumen_efectos: f32,
    fallback: Option<Sound>,
    pendiente: Option<Escena>,
    transicion_timer: f32,
    transicion_duracion: f32,
    fade_speed: f32,
}

impl AudioManager {
    pub fn new() -> Self {
        Self {
            ambientes: HashMap::new(),
            efectos: HashMap::new(),
            duraciones: HashMap::new(),
            sonido_actual: None,
            estado: EstadoAudio::Silencio,
            volumen_musica: 0.6,
            volumen_efectos: 0.8,
            fallback: None,
            pendiente: None,
            transicion_timer: 0.0,
            transicion_duracion: 0.0,
            fade_speed: 5.0,
        }
    }

    pub async fn set_fallback(&mut self, bytes: &[u8]) {
        if let Ok(sound) = load_sound_from_bytes(bytes).await {
            self.fallback = Some(sound);
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
            if let Some(dur) = duracion_wav(bytes) {
                self.duraciones.insert(nombre.to_string(), dur);
            }
        }
    }

    pub fn duracion_efecto(&self, nombre: &str) -> f32 {
        self.duraciones.get(nombre).copied().unwrap_or(0.0)
    }

    /// Duración que debe tener la transición visual (basada en el audio)
    pub fn duracion_transicion(&self) -> f32 {
        let dur = self.duracion_efecto("transicion");
        if dur > 0.1 { dur } else { 0.5 }
    }

    #[allow(dead_code)]
    pub fn set_volumen_musica(&mut self, vol: f32) {
        self.volumen_musica = vol;
    }

    #[allow(dead_code)]
    pub fn set_volumen_efectos(&mut self, vol: f32) {
        self.volumen_efectos = vol;
    }

    pub fn update(&mut self, dt: f32) {
        // Extraer el estado actual para evitar problemas de borrow
        let accion = match &mut self.estado {
            EstadoAudio::FadeOut { vol_actual } => {
                *vol_actual -= self.fade_speed * dt;
                if *vol_actual <= 0.0 {
                    Some(AccionAudio::TerminarFade)
                } else {
                    let v = *vol_actual;
                    Some(AccionAudio::AplicarVol(v))
                }
            }
            EstadoAudio::EsperandoTransicion => {
                self.transicion_timer += dt;
                if self.transicion_timer >= self.transicion_duracion {
                    Some(AccionAudio::IniciarPendiente)
                } else {
                    None
                }
            }
            _ => None,
        };

        // Ejecutar la acción fuera del match
        if let Some(accion) = accion {
            match accion {
                AccionAudio::TerminarFade => {
                    self.parar_actual();
                    self.reproducir_efecto_transicion();
                    self.estado = EstadoAudio::EsperandoTransicion;
                    self.transicion_timer = 0.0;
                }
                AccionAudio::AplicarVol(v) => {
                    self.aplicar_volumen(v);
                }
                AccionAudio::IniciarPendiente => {
                    if let Some(escena) = self.pendiente.take() {
                        self.iniciar_ambiente_interno(escena);
                    }
                    self.estado = EstadoAudio::Sonando;
                }
            }
        }
    }

    pub fn transicionar_a(&mut self, destino: Escena) {
        self.pendiente = Some(destino);
        self.transicion_duracion = self.duracion_transicion();

        match self.estado {
            EstadoAudio::Sonando => {
                self.estado = EstadoAudio::FadeOut {
                    vol_actual: self.volumen_musica,
                };
            }
            EstadoAudio::Silencio => {
                self.reproducir_efecto_transicion();
                self.estado = EstadoAudio::EsperandoTransicion;
                self.transicion_timer = 0.0;
            }
            _ => {
                // Ya en transición, solo actualizamos destino
            }
        }
    }

    pub fn iniciar_ambiente(&mut self, escena: Escena) {
        self.iniciar_ambiente_interno(escena);
        self.estado = EstadoAudio::Sonando;
    }

    fn iniciar_ambiente_interno(&mut self, escena: Escena) {
        let sound = if let Some(s) = self.ambientes.get(&escena) {
            s
        } else if let Some(ref fb) = self.fallback {
            fb
        } else {
            self.sonido_actual = Some(escena);
            return;
        };
        play_sound(sound, PlaySoundParams {
            looped: true,
            volume: self.volumen_musica,
        });
        self.sonido_actual = Some(escena);
    }

    fn parar_actual(&mut self) {
        if let Some(escena) = self.sonido_actual.take() {
            if let Some(sound) = self.ambientes.get(&escena) {
                stop_sound(sound);
            } else if let Some(ref fb) = self.fallback {
                stop_sound(fb);
            }
        }
    }

    fn aplicar_volumen(&self, vol: f32) {
        if let Some(escena) = &self.sonido_actual {
            if let Some(sound) = self.ambientes.get(escena) {
                set_sound_volume(sound, vol);
            } else if let Some(ref fb) = self.fallback {
                set_sound_volume(fb, vol);
            }
        }
    }

    fn reproducir_efecto_transicion(&self) {
        if let Some(sound) = self.efectos.get("transicion") {
            play_sound(sound, PlaySoundParams {
                looped: false,
                volume: self.volumen_efectos,
            });
        }
    }

    #[allow(dead_code)]
    pub fn efecto(&self, nombre: &str) {
        if let Some(sound) = self.efectos.get(nombre) {
            play_sound(sound, PlaySoundParams {
                looped: false,
                volume: self.volumen_efectos,
            });
        }
    }

    #[allow(dead_code)]
    pub fn en_transicion(&self) -> bool {
        matches!(self.estado, EstadoAudio::FadeOut { .. } | EstadoAudio::EsperandoTransicion)
    }
}

enum AccionAudio {
    TerminarFade,
    AplicarVol(f32),
    IniciarPendiente,
}

fn duracion_wav(bytes: &[u8]) -> Option<f32> {
    if bytes.len() < 44 { return None; }
    if &bytes[0..4] != b"RIFF" || &bytes[8..12] != b"WAVE" { return None; }
    let byte_rate = u32::from_le_bytes([bytes[28], bytes[29], bytes[30], bytes[31]]);
    if byte_rate == 0 { return None; }
    let mut pos = 12;
    while pos + 8 <= bytes.len() {
        let chunk_id = &bytes[pos..pos + 4];
        let chunk_size = u32::from_le_bytes([
            bytes[pos + 4], bytes[pos + 5], bytes[pos + 6], bytes[pos + 7],
        ]);
        if chunk_id == b"data" {
            return Some(chunk_size as f32 / byte_rate as f32);
        }
        pos += 8 + chunk_size as usize;
        if pos % 2 != 0 { pos += 1; }
    }
    None
}