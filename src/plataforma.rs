// src/plataforma.rs
use macroquad::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Plataforma {
    PC,
    Movil,
}

pub struct DetectorPlataforma {
    plataforma: Plataforma,
    ultimo_touch: f64,
    ultima_tecla: f64,
}

impl DetectorPlataforma {
    pub fn new() -> Self {
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

    pub fn update(&mut self) {
        let t = get_time();

        if !touches().is_empty() {
            self.ultimo_touch = t;
        }

        let teclas = [
            KeyCode::Up, KeyCode::Down, KeyCode::Left, KeyCode::Right,
            KeyCode::Z, KeyCode::X, KeyCode::Escape, KeyCode::Enter,
        ];
        for k in &teclas {
            if is_key_pressed(*k) {
                self.ultima_tecla = t;
            }
        }

        if self.ultimo_touch > self.ultima_tecla + 0.5 {
            self.plataforma = Plataforma::Movil;
        } else if self.ultima_tecla > self.ultimo_touch + 0.5 {
            self.plataforma = Plataforma::PC;
        }
    }

    pub fn actual(&self) -> Plataforma {
        self.plataforma
    }

    pub fn es_movil(&self) -> bool {
        self.plataforma == Plataforma::Movil
    }

    pub fn es_tactil(&self) -> bool {
        self.plataforma == Plataforma::Movil
    }

    pub fn nombre_boton_a(&self) -> &'static str {
        match self.plataforma {
            Plataforma::PC => "Z",
            Plataforma::Movil => "A",
        }
    }

    pub fn nombre_boton_b(&self) -> &'static str {
        match self.plataforma {
            Plataforma::PC => "X",
            Plataforma::Movil => "B",
        }
    }
}