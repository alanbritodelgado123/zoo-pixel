// src/eventos.rs
use macroquad::prelude::*;
use macroquad::rand::gen_range;
use crate::escena::Escena;

#[derive(Debug, Clone)]
pub struct EventoAleatorio {
    pub texto: String,
    pub detalle: String,
    pub timer_visible: f32,
}

pub struct SistemaEventos {
    pub evento_actual: Option<EventoAleatorio>,
    pub cooldown: f32,
    pub mostrar_info: bool,
    intervalo_min: f32,
    intervalo_max: f32,
    pub bloqueado: bool,
}

impl SistemaEventos {
    pub fn new() -> Self {
        Self {
            evento_actual: None,
            cooldown: gen_range(45.0, 90.0),
            mostrar_info: false,
            intervalo_min: 60.0,
            intervalo_max: 120.0,
            bloqueado: false,
        }
    }

    pub fn update(&mut self, dt: f32, escena: &Escena) {
        if self.bloqueado { return; }
        // Solo eventos en entradas
        if !escena.es_entrada() {
            self.evento_actual = None;
            return;
        }

        if let Some(ref mut ev) = self.evento_actual {
            ev.timer_visible += dt;
            if ev.timer_visible > 30.0 && !self.mostrar_info {
                self.evento_actual = None;
                self.cooldown = gen_range(self.intervalo_min, self.intervalo_max);
            }
        } else {
            self.cooldown -= dt;
            if self.cooldown <= 0.0 {
                self.generar_evento();
            }
        }
    }

    fn generar_evento(&mut self) {
        self.evento_actual = Some(EventoAleatorio {
            texto: "Bienvenido al Zoo".to_string(),
            detalle: "Explora las zonas para descubrir animales. Usa Z para interactuar y X para volver.".to_string(),
            timer_visible: 0.0,
        });
        self.mostrar_info = false;
    }

    pub fn hay_evento(&self) -> bool {
        self.evento_actual.is_some()
    }

    pub fn interactuar(&mut self, cerrar: bool) {
        if cerrar {
            self.evento_actual = None;
            self.mostrar_info = false;
            self.cooldown = gen_range(self.intervalo_min, self.intervalo_max);
        } else {
            if self.mostrar_info {
                self.evento_actual = None;
                self.mostrar_info = false;
                self.cooldown = gen_range(self.intervalo_min, self.intervalo_max);
            } else {
                self.mostrar_info = true;
            }
        }
    }
}