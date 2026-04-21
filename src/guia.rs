// src/guia.rs
use crate::db::{ZooDB, DialogoDB};
use crate::plataforma::DetectorPlataforma;

// =====================================================================
//  LÍNEA DE DIÁLOGO
// =====================================================================
#[derive(Clone)]
pub struct LineaDialogo {
    pub personaje: String,
    pub texto: String,
}

impl LineaDialogo {
    pub fn new(personaje: &str, texto: &str) -> Self {
        Self {
            personaje: personaje.to_string(),
            texto: texto.to_string(),
        }
    }
}

// =====================================================================
//  SISTEMA DE DIÁLOGO UNIFICADO
// =====================================================================
pub struct SistemaDialogo {
    pub activo: bool,
    pub completado: bool,
    lineas: Vec<LineaDialogo>,
    indice: usize,
    texto_pos: usize,
    timer: f32,
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

    // ── Inicialización ───────────────────────────────────────────────

    pub fn iniciar(&mut self, lineas: Vec<LineaDialogo>) {
        if lineas.is_empty() {
            return;
        }
        self.lineas = lineas;
        self.reiniciar_estado();
    }

    pub fn iniciar_desde_db(&mut self, dialogos: Vec<DialogoDB>) {
        if dialogos.is_empty() {
            return;
        }
        self.lineas = dialogos.iter().map(|d| LineaDialogo {
            personaje: d.personaje.clone(),
            texto: d.texto.clone(),
        }).collect();
        self.reiniciar_estado();
    }

    fn reiniciar_estado(&mut self) {
        self.activo = true;
        self.completado = false;
        self.indice = 0;
        // ✅ Arrancar con 1 carácter visible para evitar caja vacía
        self.texto_pos = 1;
        self.timer = 1.0 / crate::config::TYPEWRITER_CPS;
        self.terminado_linea = false;
    }

    // ── Update (typewriter) ──────────────────────────────────────────

    pub fn update(&mut self, dt: f32) {
        if !self.activo || self.terminado_linea {
            return;
        }
        self.timer += dt;
        let cps = crate::config::TYPEWRITER_CPS;
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

    // ── Avance ──────────────────────────────────────────────────────

    pub fn avanzar(&mut self) {
        if !self.activo {
            return;
        }
        // Si la línea no terminó, completarla primero
        if !self.terminado_linea {
            if let Some(linea) = self.lineas.get(self.indice) {
                self.texto_pos = linea.texto.chars().count();
                self.terminado_linea = true;
            }
            return;
        }
        // Avanzar a la siguiente línea
        self.indice += 1;
        if self.indice >= self.lineas.len() {
            self.activo = false;
            self.completado = true;
        } else {
            // ✅ Nueva línea también arranca con 1 carácter visible
            self.texto_pos = 1;
            self.timer = 1.0 / crate::config::TYPEWRITER_CPS;
            self.terminado_linea = false;
        }
    }

    // ── Consultas ────────────────────────────────────────────────────

    pub fn texto_visible(&self) -> String {
        self.lineas
            .get(self.indice)
            .map(|l| l.texto.chars().take(self.texto_pos).collect())
            .unwrap_or_default()
    }

    pub fn personaje_actual(&self) -> &str {
        self.lineas
            .get(self.indice)
            .map(|l| l.personaje.as_str())
            .unwrap_or("")
    }

    pub fn progreso(&self) -> (usize, usize) {
        (self.indice + 1, self.lineas.len())
    }
}

// =====================================================================
//  FUNCIONES DE FÁBRICA DE DIÁLOGOS
// =====================================================================

/// Diálogo de bienvenida desde DB
pub fn dialogos_bienvenida_db(db: &ZooDB, plataforma: &DetectorPlataforma) -> Vec<DialogoDB> {
    db.dialogos_bienvenida(plataforma.es_tactil())
}

/// Diálogo de bienvenida hardcodeado como fallback
pub fn dialogos_bienvenida_fallback(plataforma: &DetectorPlataforma) -> Vec<LineaDialogo> {
    let a = plataforma.nombre_boton_a();
    let b = plataforma.nombre_boton_b();
    vec![
        LineaDialogo::new("Guía Eli",
            "¡Bienvenido al Zoológico Nacional! Soy Eli, tu guía personal."),
        LineaDialogo::new("Guía Eli",
            "Aquí explorarás 25 zonas que representan los ecosistemas de Venezuela."),
        LineaDialogo::new("Guía Eli",
            &format!("Usa {} para interactuar y {} para volver o cancelar.", a, b)),
        LineaDialogo::new("Guía Eli",
            "Presiona M para ver el mapa y L para abrir tu libreta de campo."),
        LineaDialogo::new("Guía Eli",
            "En la Península de Paria (Z5-1) está el Museo Paleontológico."),
        LineaDialogo::new("Guía Eli",
            "En el Pasillo 5 (P5) está el Acuario: ¡prepara tu caña de pescar!"),
        LineaDialogo::new("Guía Eli",
            "¡Completa tu libreta con cada animal que descubras! ¡Buena expedición!"),
    ]
}

/// Diálogo de bienvenida del museo (Guía Ani) desde DB
pub fn dialogos_museo_ani_db(db: &ZooDB) -> Vec<DialogoDB> {
    db.dialogos_por_contexto("museo_bienvenida")
}

/// Diálogos de callejones (Zx-5) desde DB
pub fn dialogos_callejon_db(db: &ZooDB, escena_id: &str) -> Vec<DialogoDB> {
    let contexto = format!("callejon_{}", escena_id);
    db.dialogos_por_contexto(&contexto)
}