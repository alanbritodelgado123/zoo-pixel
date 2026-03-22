// src/ciclo_dia.rs
use macroquad::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FaseDia {
    Dia,
    Noche,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ModoCiclo {
    Sistema,
    DiaPermanente,
    NochePermanente,
}

pub struct CicloDia {
    modo: ModoCiclo,
}

impl CicloDia {
    pub fn new(_ciclo_secs: f32) -> Self {
        Self { modo: ModoCiclo::Sistema }
    }

    pub fn set_modo(&mut self, modo: ModoCiclo) {
        self.modo = modo;
    }

    pub fn modo(&self) -> ModoCiclo {
        self.modo
    }

    pub fn update(&mut self, _dt: f32) {}

    pub fn progreso(&self) -> f32 {
        match self.modo {
            ModoCiclo::DiaPermanente => 0.25,
            ModoCiclo::NochePermanente => 0.75,
            ModoCiclo::Sistema => self.progreso_desde_sistema(),
        }
    }

    fn progreso_desde_sistema(&self) -> f32 {
        let ahora = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        let secs_en_dia = (ahora % 86400) as f32;
        let hora = secs_en_dia / 3600.0;
        // Dia: 6am-18pm, Noche: 18pm-6am
        if hora >= 6.0 && hora < 18.0 { 0.25 } else { 0.75 }
    }

    pub fn fase(&self) -> FaseDia {
        if self.progreso() < 0.5 { FaseDia::Dia } else { FaseDia::Noche }
    }

    pub fn es_noche(&self) -> bool {
        self.fase() == FaseDia::Noche
    }

    pub fn tinte(&self) -> Color {
        match self.fase() {
            FaseDia::Dia => WHITE,
            FaseDia::Noche => Color::new(0.4, 0.4, 0.6, 1.0),
        }
    }

    pub fn overlay_alpha(&self) -> f32 {
        match self.fase() {
            FaseDia::Noche => 0.25,
            FaseDia::Dia => 0.0,
        }
    }

    pub fn nombre_fase(&self) -> &'static str {
        match self.fase() {
            FaseDia::Dia => "Dia",
            FaseDia::Noche => "Noche",
        }
    }

    pub fn nombre_modo(&self) -> &'static str {
        match self.modo {
            ModoCiclo::Sistema => "Automatico",
            ModoCiclo::DiaPermanente => "Dia fijo",
            ModoCiclo::NochePermanente => "Noche fija",
        }
    }
}