// src/eventos.rs
use macroquad::prelude::*;
use macroquad::rand::gen_range;
use crate::escena::Escena;

#[derive(Debug, Clone)]
pub struct EventoAleatorio {
    pub tipo: TipoEvento,
    pub texto: String,
    pub detalle: String,
    pub timer_visible: f32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TipoEvento {
    Cria,
    Alimentacion,
    Enriquecimiento,
    DatoCurioso,
    Lluvia,
}

pub struct SistemaEventos {
    pub evento_actual: Option<EventoAleatorio>,
    pub cooldown: f32,
    pub mostrar_info: bool,
    intervalo_min: f32,
    intervalo_max: f32,
    /// Si es true, el evento NO debe mostrarse
    pub bloqueado: bool,
}

impl SistemaEventos {
    pub fn new() -> Self {
        Self {
            evento_actual: None,
            cooldown: gen_range(30.0, 60.0),
            mostrar_info: false,
            intervalo_min: 30.0,
            intervalo_max: 90.0,
            bloqueado: false,
        }
    }

    pub fn update(&mut self, dt: f32, _escena: &Escena) {
        // Si está bloqueado, no hacer nada (no generar eventos)
        if self.bloqueado {
            return;
        }

        if let Some(ref mut ev) = self.evento_actual {
            ev.timer_visible += dt;
            // El evento se queda visible hasta que el usuario interactúe
            // NO auto-abrir info - se quita solo si el usuario lo decide
            // Timeout largo para que no se quede infinitamente
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
        let tipo = match gen_range(0, 5) {
            0 => TipoEvento::Cria,
            1 => TipoEvento::Alimentacion,
            2 => TipoEvento::Enriquecimiento,
            3 => TipoEvento::DatoCurioso,
            _ => TipoEvento::Lluvia,
        };

        let (texto, detalle) = match tipo {
            TipoEvento::Cria => (
                "¡Nueva cría!".to_string(),
                "Ha nacido una cría en esta zona. La madre la protege con mucho cuidado.".to_string(),
            ),
            TipoEvento::Alimentacion => (
                "Hora de comer".to_string(),
                "Es hora de la alimentación. Los animales ya saben que es su momento favorito.".to_string(),
            ),
            TipoEvento::Enriquecimiento => (
                "Enriquecimiento ambiental".to_string(),
                "Los cuidadores colocan juguetes y retos para mantener activos a los animales.".to_string(),
            ),
            TipoEvento::DatoCurioso => (
                "¿Sabías que...?".to_string(),
                "Venezuela tiene más de 1.400 especies de aves. ¡Uno de los países con mayor biodiversidad!".to_string(),
            ),
            TipoEvento::Lluvia => (
                "Lluvia tropical".to_string(),
                "Empieza a llover. Muchos animales se refugian, pero otros disfrutan la lluvia fresca.".to_string(),
            ),
        };

        self.evento_actual = Some(EventoAleatorio {
            tipo,
            texto,
            detalle,
            timer_visible: 0.0,
        });
        self.mostrar_info = false;
    }

    pub fn hay_evento(&self) -> bool {
        self.evento_actual.is_some()
    }

    /// Llamado cuando el usuario presiona Z (ver info) o X (cerrar)
    pub fn interactuar(&mut self, cerrar: bool) {
        if cerrar {
            // X: cerrar evento sin ver info
            self.evento_actual = None;
            self.mostrar_info = false;
            self.cooldown = gen_range(self.intervalo_min, self.intervalo_max);
        } else {
            // Z: si no está mostrando info, mostrarla; si ya la muestra, cerrar
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