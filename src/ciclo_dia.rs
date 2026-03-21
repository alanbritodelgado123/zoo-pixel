// src/ciclo_dia.rs
use macroquad::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FaseDia {
    Amanecer,
    Dia,
    Atardecer,
    Noche,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ModoCiclo {
    /// Usa el reloj del sistema
    Sistema,
    /// Forzado a día permanente
    DiaPermanente,
    /// Forzado a noche permanente
    NochePermanente,
}

pub struct CicloDia {
    /// Tiempo acumulado en segundos (solo para modo manual/legacy)
    tiempo: f32,
    /// Duración total de un ciclo completo (día entero) en segundos
    duracion_ciclo: f32,
    /// Modo del ciclo
    modo: ModoCiclo,
}

impl CicloDia {
    pub fn new(ciclo_secs: f32) -> Self {
        Self {
            tiempo: ciclo_secs * 0.25,
            duracion_ciclo: ciclo_secs,
            modo: ModoCiclo::Sistema,
        }
    }

    pub fn set_modo(&mut self, modo: ModoCiclo) {
        self.modo = modo;
    }

    pub fn modo(&self) -> ModoCiclo {
        self.modo
    }

    pub fn update(&mut self, _dt: f32) {
        // En modo sistema no necesitamos actualizar timer interno
        // En modos forzados tampoco
    }

    /// Progreso normalizado 0.0 .. 1.0
    pub fn progreso(&self) -> f32 {
        match self.modo {
            ModoCiclo::DiaPermanente => 0.35,   // Mitad del día
            ModoCiclo::NochePermanente => 0.85,  // Mitad de la noche
            ModoCiclo::Sistema => {
                self.progreso_desde_sistema()
            }
        }
    }

    fn progreso_desde_sistema(&self) -> f32 {
        // Obtener hora del sistema usando macroquad
        // macroquad no tiene get_time como hora del día,
        // así que usamos std::time
        let ahora = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        // Segundos desde medianoche (UTC, pero funciona para el ciclo)
        // Para zona horaria local usamos un approach simple
        let secs_en_dia = (ahora % 86400) as f32;

        // Mapear: 0=medianoche, 0.25=6am, 0.5=mediodía, 0.75=6pm
        // Pero nuestras fases son:
        // 0.0-0.2 = Amanecer (5am-9am aprox)
        // 0.2-0.5 = Día (9am-3pm)
        // 0.5-0.7 = Atardecer (3pm-7pm)
        // 0.7-1.0 = Noche (7pm-5am)

        // Hora decimal (0-24)
        let hora = secs_en_dia / 3600.0;

        // Mapear horas a progreso del ciclo:
        // 5:00 = 0.0 (inicio amanecer)
        // 9:00 = 0.2 (inicio día)
        // 15:00 = 0.5 (inicio atardecer)
        // 19:00 = 0.7 (inicio noche)
        // 5:00 (siguiente) = 1.0

        let p = if hora >= 5.0 && hora < 9.0 {
            (hora - 5.0) / 4.0 * 0.2
        } else if hora >= 9.0 && hora < 15.0 {
            0.2 + (hora - 9.0) / 6.0 * 0.3
        } else if hora >= 15.0 && hora < 19.0 {
            0.5 + (hora - 15.0) / 4.0 * 0.2
        } else if hora >= 19.0 {
            0.7 + (hora - 19.0) / 10.0 * 0.3
        } else {
            // 0:00 - 5:00
            0.7 + (hora + 5.0) / 10.0 * 0.3
        };

        p.clamp(0.0, 0.999)
    }

    pub fn fase(&self) -> FaseDia {
        let p = self.progreso();
        if p < 0.2 { FaseDia::Amanecer }
        else if p < 0.5 { FaseDia::Dia }
        else if p < 0.7 { FaseDia::Atardecer }
        else { FaseDia::Noche }
    }

    pub fn es_noche(&self) -> bool {
        self.fase() == FaseDia::Noche
    }

    /// Tinte de color para aplicar al fondo según la hora
    pub fn tinte(&self) -> Color {
        let p = self.progreso();
        match self.fase() {
            FaseDia::Amanecer => {
                let t = p / 0.2;
                Color::new(
                    0.7 + 0.3 * t,
                    0.6 + 0.4 * t,
                    0.5 + 0.5 * t,
                    1.0,
                )
            }
            FaseDia::Dia => WHITE,
            FaseDia::Atardecer => {
                let t = (p - 0.5) / 0.2;
                Color::new(
                    1.0 - 0.3 * t,
                    0.85 - 0.25 * t,
                    0.7 - 0.3 * t,
                    1.0,
                )
            }
            FaseDia::Noche => {
                Color::new(0.3, 0.3, 0.5, 1.0)
            }
        }
    }

    /// Overlay oscuro para la noche
    pub fn overlay_alpha(&self) -> f32 {
        match self.fase() {
            FaseDia::Noche => {
                let p = self.progreso();
                let t = (p - 0.7) / 0.3;
                0.15 + 0.2 * (t * std::f32::consts::PI).sin()
            }
            FaseDia::Atardecer => {
                let t = (self.progreso() - 0.5) / 0.2;
                t * 0.15
            }
            _ => 0.0,
        }
    }

    pub fn nombre_fase(&self) -> &'static str {
        match self.fase() {
            FaseDia::Amanecer => "Amanecer",
            FaseDia::Dia => "Día",
            FaseDia::Atardecer => "Atardecer",
            FaseDia::Noche => "Noche",
        }
    }

    pub fn nombre_modo(&self) -> &'static str {
        match self.modo {
            ModoCiclo::Sistema => "Automático",
            ModoCiclo::DiaPermanente => "Día fijo",
            ModoCiclo::NochePermanente => "Noche fija",
        }
    }
}