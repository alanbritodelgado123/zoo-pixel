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
    pub mostrar_info: bool,
    // Añadimos el campo bloqueado
    pub bloqueado: bool,
    // Otros campos existentes...
    ya_mostrado_en_entrada: bool, // O el nombre que tenga
    cooldown: f32, // Ejemplo de otro campo
    intervalo_min: f32,
    intervalo_max: f32,
}

impl SistemaEventos {
    pub fn new() -> Self {
        Self {
            evento_actual: None,
            mostrar_info: false,
            bloqueado: false, // Inicializado en false
            ya_mostrado_en_entrada: false,
            cooldown: 0.0, // Inicializar según sea necesario
            intervalo_min: 10.0, // Ejemplo
            intervalo_max: 30.0, // Ejemplo
        }
    }

    pub fn update(&mut self, dt: f32, escena: &Escena) {
        // Aplicar la lógica de bloqueo aquí
        if self.bloqueado {
            // Si está bloqueado, no hacemos nada
            return;
        }

        // Resto de la lógica de update...
        // (Tu lógica existente aquí, por ejemplo)
        if !escena.es_entrada() {
            self.ya_mostrado_en_entrada = false;
            self.evento_actual = None;
            return;
        }

        if self.ya_mostrado_en_entrada {
            if let Some(ref mut ev) = self.evento_actual {
                ev.timer_visible += dt;
                if ev.timer_visible > 30.0 && !self.mostrar_info {
                    self.evento_actual = None;
                    self.cooldown = gen_range(self.intervalo_min as i32, self.intervalo_max as i32) as f32;
                }
            }
            return;
        }

        if let Some(ref mut ev) = self.evento_actual {
            ev.timer_visible += dt;
            if ev.timer_visible > 30.0 && !self.mostrar_info {
                self.evento_actual = None;
                self.cooldown = gen_range(self.intervalo_min as i32, self.intervalo_max as i32) as f32;
            }
        } else {
            self.cooldown -= dt;
            if self.cooldown <= 0.0 {
                self.generar_evento();
            }
        }
    }

    pub fn generar_evento(&mut self) {
        // Ejemplo de generación de evento
        self.evento_actual = Some(EventoAleatorio {
            texto: "Evento Aleatorio".to_string(),
            detalle: "Detalles del evento.".to_string(),
            timer_visible: 0.0,
        });
        self.mostrar_info = false;
        self.ya_mostrado_en_entrada = true; // Marcamos que ya se mostró en esta visita a la entrada
    }

    pub fn hay_evento(&self) -> bool {
        self.evento_actual.is_some()
    }

    pub fn interactuar(&mut self, cerrar: bool) {
        if cerrar {
            self.evento_actual = None;
            self.mostrar_info = false;
            // Reiniciar cooldown si se cierra
            self.cooldown = gen_range(self.intervalo_min as i32, self.intervalo_max as i32) as f32;
        } else {
            if self.mostrar_info {
                self.evento_actual = None;
                self.mostrar_info = false;
                // Reiniciar cooldown si se cierra después de mostrar info
                 self.cooldown = gen_range(self.intervalo_min as i32, self.intervalo_max as i32) as f32;
            } else {
                self.mostrar_info = true;
            }
        }
    }
}
