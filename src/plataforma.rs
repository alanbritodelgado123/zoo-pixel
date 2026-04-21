// src/plataforma.rs
use macroquad::prelude::*;

// =====================================================================
//  TIPOS
// =====================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Plataforma {
    PC,
    Movil,
}

// =====================================================================
//  DETECTOR DE PLATAFORMA
// =====================================================================

pub struct DetectorPlataforma {
    plataforma: Plataforma,
    ultimo_touch: f64,
    ultima_tecla: f64,
}

impl DetectorPlataforma {
    pub fn new() -> Self {
        // En Android/iOS empezamos como móvil, en PC empezamos como PC
        let inicial = if cfg!(target_os = "android") || cfg!(target_os = "ios") {
            Plataforma::Movil
        } else {
            Plataforma::PC
        };
        Self {
            plataforma: inicial,
            ultimo_touch: -100.0,
            ultima_tecla: -100.0,
        }
    }

    /// Detecta dinámicamente si el usuario usa táctil o teclado
    pub fn update(&mut self) {
        let t = get_time();

        // Detectar touch
        if !touches().is_empty() {
            self.ultimo_touch = t;
        }

        // Detectar teclas relevantes
        let teclas = [
            KeyCode::Up,
            KeyCode::Down,
            KeyCode::Left,
            KeyCode::Right,
            KeyCode::Z,
            KeyCode::X,
            KeyCode::Escape,
            KeyCode::Enter,
            KeyCode::M,
            KeyCode::L,
        ];
        for k in &teclas {
            if is_key_pressed(*k) {
                self.ultima_tecla = t;
                break;
            }
        }

        // Cambiar plataforma si hay señal clara (0.5s de margen)
        if self.ultimo_touch > self.ultima_tecla + 0.5 {
            self.plataforma = Plataforma::Movil;
        } else if self.ultima_tecla > self.ultimo_touch + 0.5 {
            self.plataforma = Plataforma::PC;
        }
    }

    // ── Consultas ────────────────────────────────────────────────────

    pub fn actual(&self) -> Plataforma {
        self.plataforma
    }

    pub fn es_movil(&self) -> bool {
        self.plataforma == Plataforma::Movil
    }

    pub fn es_tactil(&self) -> bool {
        self.plataforma == Plataforma::Movil
    }

    pub fn es_pc(&self) -> bool {
        self.plataforma == Plataforma::PC
    }

    // ── Nombres de botones adaptativos ───────────────────────────────

    pub fn nombre_boton_a(&self) -> &'static str {
        match self.plataforma {
            Plataforma::PC    => "Z",
            Plataforma::Movil => "A",
        }
    }

    pub fn nombre_boton_b(&self) -> &'static str {
        match self.plataforma {
            Plataforma::PC    => "X",
            Plataforma::Movil => "B",
        }
    }

    pub fn nombre_boton_mapa(&self) -> &'static str {
        match self.plataforma {
            Plataforma::PC    => "M",
            Plataforma::Movil => "Mapa",
        }
    }

    pub fn nombre_boton_libreta(&self) -> &'static str {
        match self.plataforma {
            Plataforma::PC    => "L",
            Plataforma::Movil => "Libreta",
        }
    }
}