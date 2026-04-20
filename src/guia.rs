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
        self.lineas = dialogos
            .iter()
            .map(|d| LineaDialogo {
                personaje: d.personaje.clone(),
                texto: d.texto.clone(),
            })
            .collect();
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
        if !self.activo || self.terminado_linea {
            return;
        }
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
        if !self.activo {
            return;
        }
        if !self.terminado_linea {
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
        self.lineas
            .get(self.indice)
            .map(|l| l.personaje.as_str())
            .unwrap_or("")
    }
}

pub fn dialogos_bienvenida_db(db: &ZooDB, plataforma: &DetectorPlataforma) -> Vec<DialogoDB> {
    db.dialogos_bienvenida(plataforma.es_tactil())
}

pub fn dialogos_bienvenida(plataforma: &DetectorPlataforma) -> Vec<LineaDialogo> {
    let a = plataforma.nombre_boton_a();
    let b = plataforma.nombre_boton_b();
    vec![
        LineaDialogo {
            personaje: "Guía Eli".into(),
            texto: "¡Bienvenido al Zoológico Nacional! Soy Eli, tu guía personal.".into(),
        },
        LineaDialogo {
            personaje: "Guía Eli".into(),
            texto: "Aquí explorarás 25 zonas que representan los ecosistemas de Venezuela.".into(),
        },
        LineaDialogo {
            personaje: "Guía Eli".into(),
            texto: format!("Usa {} para interactuar y {} para volver o cancelar.", a, b),
        },
        LineaDialogo {
            personaje: "Guía Eli".into(),
            texto: "Presiona M para ver el mapa y L para abrir tu libreta de campo.".into(),
        },
        LineaDialogo {
            personaje: "Guía Eli".into(),
            texto: "En la Península de Paria (Z5-1) está el Museo Paleontológico.".into(),
        },
        LineaDialogo {
            personaje: "Guía Eli".into(),
            texto: "En el Pasillo 5 (P5) está el Acuario: ¡prepara tu caña!".into(),
        },
        LineaDialogo {
            personaje: "Guía Eli".into(),
            texto: "¡Completa tu libreta con cada animal que descubras! ¡Buena expedición!".into(),
        },
    ]
}

pub fn dialogos_museo_db(db: &ZooDB) -> Vec<DialogoDB> {
    db.dialogos_museo()
}

// ✅ Diálogos de Ani para museo (primera vez)
pub fn dialogos_museo_ani_db(db: &ZooDB) -> Vec<DialogoDB> {
    db.dialogos_por_contexto("museo_bienvenida")
}

// ✅ Diálogos de callejones (Zx-5)
pub fn dialogos_callejon_db(db: &ZooDB, escena_id: &str) -> Vec<DialogoDB> {
    let contexto = format!("callejon_{}", escena_id);
    db.dialogos_por_contexto(&contexto)
}