// src/audio.rs
use std::collections::HashMap;
use macroquad::audio::*;
use crate::escena::Escena;

enum EstadoAudio {
    Sonando,
    FadeOut { vol_actual: f32 },
    EsperandoTransicion,
    Silencio,
}

enum AccionAudio {
    TerminarFade,
    AplicarVol(f32),
    IniciarPendiente,
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

    // ── Carga ────────────────────────────────────────────────────────

    pub async fn set_fallback(&mut self, bytes: &[u8]) {
        match load_sound_from_bytes(bytes).await {
            Ok(sound) => {
                self.fallback = Some(sound);
                println!("✅ Fallback de audio cargado");
            }
            Err(e) => println!("❌ Error cargando fallback: {:?}", e),
        }
    }

    pub async fn agregar_ambiente(&mut self, escena: Escena, bytes: &[u8]) {
        match load_sound_from_bytes(bytes).await {
            Ok(sound) => {
                self.ambientes.insert(escena, sound);
            }
            Err(e) => println!("❌ Error ambiente {:?}: {:?}", escena, e),
        }
    }

    pub async fn agregar_efecto(&mut self, nombre: &str, bytes: &[u8]) {
        match load_sound_from_bytes(bytes).await {
            Ok(sound) => {
                if let Some(dur) = duracion_wav(bytes) {
                    self.duraciones.insert(nombre.to_string(), dur);
                }
                self.efectos.insert(nombre.to_string(), sound);
                println!("  ✓ Efecto cargado: {}", nombre);
            }
            Err(e) => println!("❌ Error efecto {}: {:?}", nombre, e),
        }
    }

    // ── Volumen ──────────────────────────────────────────────────────

    pub fn set_volumen_musica(&mut self, vol: f32) {
        self.volumen_musica = vol.clamp(0.0, 1.0);
        self.aplicar_volumen_actual();
    }

    pub fn set_volumen_efectos(&mut self, vol: f32) {
        self.volumen_efectos = vol.clamp(0.0, 1.0);
    }

    fn aplicar_volumen_actual(&self) {
        self.aplicar_volumen(self.volumen_musica);
    }

    fn aplicar_volumen(&self, vol: f32) {
        if let Some(escena) = &self.sonido_actual {
            if let Some(sound) = self.ambientes.get(escena) {
                set_sound_volume(sound, vol);
                return;
            }
        }
        if let Some(ref fb) = self.fallback {
            set_sound_volume(fb, vol);
        }
    }

    // ── Update ───────────────────────────────────────────────────────

    pub fn update(&mut self, dt: f32) {
        let accion = match &mut self.estado {
            EstadoAudio::FadeOut { vol_actual } => {
                *vol_actual -= self.fade_speed * dt;
                if *vol_actual <= 0.0 {
                    Some(AccionAudio::TerminarFade)
                } else {
                    Some(AccionAudio::AplicarVol(*vol_actual))
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

        if let Some(accion) = accion {
            match accion {
                AccionAudio::TerminarFade => {
                    self.parar_actual();
                    self.reproducir_efecto_interno("transicion");
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

    // ── Transición de escena ─────────────────────────────────────────

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
                self.reproducir_efecto_interno("transicion");
                self.estado = EstadoAudio::EsperandoTransicion;
                self.transicion_timer = 0.0;
            }
            _ => {}
        }
    }

    pub fn iniciar_ambiente(&mut self, escena: Escena) {
        self.iniciar_ambiente_interno(escena);
        self.estado = EstadoAudio::Sonando;
    }

    fn iniciar_ambiente_interno(&mut self, escena: Escena) {
        let sound_ref = if let Some(s) = self.ambientes.get(&escena) {
            Some(s as *const Sound)
        } else if let Some(ref fb) = self.fallback {
            Some(fb as *const Sound)
        } else {
            None
        };

        if let Some(ptr) = sound_ref {
            let sound = unsafe { &*ptr };
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

    fn parar_actual(&mut self) {
        if let Some(escena) = self.sonido_actual.take() {
            if let Some(sound) = self.ambientes.get(&escena) {
                stop_sound(sound);
                return;
            }
        }
        if let Some(ref fb) = self.fallback {
            stop_sound(fb);
        }
    }

    // ── Efectos ──────────────────────────────────────────────────────

    fn reproducir_efecto_interno(&self, nombre: &str) {
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

    pub fn efecto(&self, nombre: &str) {
        self.reproducir_efecto_interno(nombre);
    }

    pub fn efecto_unico(&self, nombre: &str) {
        if let Some(sound) = self.efectos.get(nombre) {
            stop_sound(sound);
            play_sound(
                sound,
                PlaySoundParams {
                    looped: false,
                    volume: self.volumen_efectos,
                },
            );
        }
    }

    /// ✅ Reproduce el grito de una categoría de animal (40% volumen)
    pub fn reproducir_grito_categoria(&self, categoria: &str) {
        let nombre = format!("grito_{}", categoria.to_lowercase());
        if let Some(sound) = self.efectos.get(&nombre) {
            play_sound(
                sound,
                PlaySoundParams {
                    looped: false,
                    volume: self.volumen_efectos * 0.9,
                },
            );
        }
    }

    // ── Consultas ────────────────────────────────────────────────────

    pub fn duracion_efecto(&self, nombre: &str) -> f32 {
        self.duraciones.get(nombre).copied().unwrap_or(0.0)
    }

    pub fn duracion_transicion(&self) -> f32 {
        let dur = self.duracion_efecto("transicion");
        if dur > 0.1 { dur } else { 0.5 }
    }

    pub fn en_transicion(&self) -> bool {
        matches!(
            self.estado,
            EstadoAudio::FadeOut { .. } | EstadoAudio::EsperandoTransicion
        )
    }
}

// ── Utilidad: duración de WAV desde bytes ────────────────────────────

fn duracion_wav(bytes: &[u8]) -> Option<f32> {
    if bytes.len() < 44 { return None; }
    if &bytes[0..4] != b"RIFF" || &bytes[8..12] != b"WAVE" { return None; }
    let byte_rate = u32::from_le_bytes([
        bytes[28], bytes[29], bytes[30], bytes[31],
    ]);
    if byte_rate == 0 { return None; }
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
        pos += 8 + chunk_size as usize;
        if pos % 2 != 0 { pos += 1; }
    }
    None
}