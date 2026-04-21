// src/eventos.rs - Sistema de eventos eliminado (causaba bloqueos en P5)
// Mantenemos el módulo como stub vacío para no romper imports

pub struct SistemaEventos;

impl SistemaEventos {
    pub fn new() -> Self {
        Self
    }

    pub fn update(&mut self, _dt: f32) {}

    pub fn hay_evento(&self) -> bool {
        false
    }

    pub fn interactuar(&mut self, _cerrar: bool) {}
}