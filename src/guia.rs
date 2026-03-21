// src/guia.rs
use crate::db::{ZooDB, DialogoDB};
use crate::plataforma::DetectorPlataforma;

#[derive(Clone)]
pub struct LineaDialogo {
    pub personaje: String,
    pub texto: String,
}

pub struct SistemaDialogo {
    pub activo: bool,
    pub completado: bool,
    pub lineas: Vec<LineaDialogo>,
    pub indice: usize,
    pub texto_pos: usize,
    pub timer: f32,
    pub terminado_linea: bool,
}

impl SistemaDialogo {
    pub fn new() -> Self {
        Self {
            activo: false,
            completado: false,
            lineas: Vec::new(),
            indice: 0,
            texto_pos: 0,
            timer: 0.0,
            terminado_linea: false,
        }
    }

    pub fn iniciar_desde_db(&mut self, dialogos: Vec<DialogoDB>) {
        self.lineas = dialogos.iter().map(|d| LineaDialogo {
            personaje: d.personaje.clone(),
            texto: d.texto.clone(),
        }).collect();
        self.activo = !self.lineas.is_empty();
        self.completado = false;
        self.indice = 0;
        self.texto_pos = 0;
        self.timer = 0.0;
        self.terminado_linea = false;
    }

    pub fn iniciar(&mut self, lineas: Vec<LineaDialogo>) {
        self.lineas = lineas;
        self.activo = !self.lineas.is_empty();
        self.completado = false;
        self.indice = 0;
        self.texto_pos = 0;
        self.timer = 0.0;
        self.terminado_linea = false;
    }

    pub fn update(&mut self, dt: f32) {
        if !self.activo { return; }
        if self.terminado_linea { return; }

        self.timer += dt;
        let cps = 35.0;
        let chars = (self.timer * cps) as usize;
        if let Some(linea) = self.lineas.get(self.indice) {
            let total = linea.texto.chars().count();
            if chars >= total {
                self.texto_pos = total;
                self.terminado_linea = true;
            } else {
                self.texto_pos = chars;
            }
        }
    }

    pub fn avanzar(&mut self) {
        if !self.activo { return; }

        if !self.terminado_linea {
            // Mostrar todo el texto de golpe
            if let Some(linea) = self.lineas.get(self.indice) {
                self.texto_pos = linea.texto.chars().count();
                self.terminado_linea = true;
            }
            return;
        }

        self.indice += 1;
        if self.indice >= self.lineas.len() {
            self.activo = false;
            self.completado = true;
        } else {
            self.texto_pos = 0;
            self.timer = 0.0;
            self.terminado_linea = false;
        }
    }

    pub fn texto_visible(&self) -> String {
        if let Some(linea) = self.lineas.get(self.indice) {
            linea.texto.chars().take(self.texto_pos).collect()
        } else {
            String::new()
        }
    }

    pub fn personaje_actual(&self) -> &str {
        self.lineas.get(self.indice)
            .map(|l| l.personaje.as_str())
            .unwrap_or("")
    }
}

/// Construir diálogos de bienvenida desde la DB
pub fn dialogos_bienvenida_db(db: &ZooDB, plataforma: &DetectorPlataforma) -> Vec<DialogoDB> {
    db.dialogos_bienvenida(plataforma.es_tactil())
}

/// Fallback: diálogos hardcoded (por si acaso)
pub fn dialogos_bienvenida(plataforma: &DetectorPlataforma) -> Vec<LineaDialogo> {
    let boton_a = if plataforma.es_tactil() { "botón A" } else { "Z" };
    let boton_b = if plataforma.es_tactil() { "botón B" } else { "X" };

    vec![
        LineaDialogo {
            personaje: "Guía Carlos".into(),
            texto: "¡Bienvenido al Zoológico Nacional! Soy Carlos, tu guía.".into(),
        },
        LineaDialogo {
            personaje: "Guía Carlos".into(),
            texto: "Aquí conocerás animales increíbles de Venezuela y el mundo.".into(),
        },
        LineaDialogo {
            personaje: "Guía Carlos".into(),
            texto: format!("Presiona {} para interactuar y {} para volver.", boton_a, boton_b),
        },
        LineaDialogo {
            personaje: "Guía Carlos".into(),
            texto: "¡Explora todas las zonas y completa tu libreta! ¡Buena suerte!".into(),
        },
    ]
}