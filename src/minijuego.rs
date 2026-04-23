// src/minijuego.rs
use macroquad::prelude::*;
use macroquad::rand::gen_range;
//use crate::config;
use crate::db::{ZooDB, QuizPreguntaDB};

// =====================================================================
//  PESCA (Acuario) - En P5
// =====================================================================
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FasePesca {
    Esperando,
    Picando,
    Resultado,
}

#[derive(Debug, Clone)]
pub struct PezCapturado {
    pub nombre: String,
    pub cientifico: String,
    pub descripcion: String,
    pub categoria: String,
    pub peso_kg: f32,
}

pub struct MinijuegoPesca {
    pub activo: bool,
    pub fase: FasePesca,
    pub timer: f32,
    pub tiempo_espera: f32,
    pub tiempo_picada: f32,
    pub pez_capturado: Option<PezCapturado>,
    pub intentos: usize,
    pub max_intentos: usize,
    pub ultimo_exito: bool,
}

impl MinijuegoPesca {
    pub fn new() -> Self {
        Self {
            activo: false,
            fase: FasePesca::Esperando,
            timer: 0.0,
            tiempo_espera: 0.0,
            tiempo_picada: 0.0,
            pez_capturado: None,
            intentos: 0,
            max_intentos: 3,
            ultimo_exito: false,
        }
    }

    pub fn iniciar(&mut self, db: &ZooDB) {
        self.activo = true;
        self.intentos = 0;
        self.pez_capturado = None;
        self.preparar_ronda(db);
    }

    pub fn preparar_ronda(&mut self, db: &ZooDB) {
        self.fase = FasePesca::Esperando;
        self.timer = 0.0;
        self.tiempo_espera = gen_range(25, 50) as f32 / 10.0; // 2.5 - 5.0
        self.tiempo_picada = gen_range(18, 35) as f32 / 10.0; // 1.8 - 3.5
        self.ultimo_exito = false;
        // Precargar un pez aleatorio desde la DB
        self.pez_capturado = Some(self.generar_pez(db));
    }

    fn generar_pez(&self, db: &ZooDB) -> PezCapturado {
        let peces_db = db.animales_por_categoria("peces");
        if peces_db.is_empty() {
            return PezCapturado {
                nombre: "Pavón".into(),
                cientifico: "Cichla temensis".into(),
                descripcion: "Depredador de agua dulce, muy apreciado en pesca deportiva.".into(),
                categoria: "peces".into(),
                peso_kg: 5.0,
            };
        }
        let idx = gen_range(0, peces_db.len());
        let animal = &peces_db[idx];
        PezCapturado {
            nombre: animal.nombre_comun.clone(),
            cientifico: animal.nombre_cientifico.clone(),
            descripcion: animal.descripcion.clone(),
            categoria: animal.categoria.clone(),
            peso_kg: gen_range(20, 200) as f32 / 10.0, // 2.0 - 20.0 kg
        }
    }

    pub fn update(&mut self, dt: f32) {
        if !self.activo { return; }
        match self.fase {
            FasePesca::Esperando => {
                self.timer += dt;
                if self.timer >= self.tiempo_espera {
                    self.fase = FasePesca::Picando;
                    self.timer = 0.0;
                }
            }
            FasePesca::Picando => {
                self.timer += dt;
                if self.timer >= self.tiempo_picada {
                    // Se escapó
                    self.ultimo_exito = false;
                    self.fase = FasePesca::Resultado;
                    self.timer = 0.0;
                }
            }
            FasePesca::Resultado => {}
        }
    }

    /// Intenta tirar la caña. Retorna el pez si tuvo éxito.
    pub fn tirar(&mut self) -> Option<PezCapturado> {
        if self.fase == FasePesca::Picando {
            self.ultimo_exito = true;
            self.fase = FasePesca::Resultado;
            self.timer = 0.0;
            return self.pez_capturado.clone();
        }
        None
    }

    /// Avanza al siguiente intento o cierra si se agotaron.
    /// Retorna true si quedan más intentos.
    pub fn siguiente(&mut self, db: &ZooDB) -> bool {
        self.intentos += 1;
        if self.intentos >= self.max_intentos {
            self.cerrar();
            false
        } else {
            self.preparar_ronda(db);
            true
        }
    }

    pub fn cerrar(&mut self) {
        self.activo = false;
        self.fase = FasePesca::Esperando;
        self.pez_capturado = None;
    }

    pub fn progreso_picada(&self) -> f32 {
        if self.tiempo_picada > 0.0 {
            1.0 - (self.timer / self.tiempo_picada).min(1.0)
        } else {
            0.0
        }
    }
}

// =====================================================================
//  MUSEO - Excavación 3x3 + Quiz desde DB
// =====================================================================
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FaseMuseo {
    Entrada,
    Explorando,
    Excavando,
    Quiz,
    QuizResultado,
}

#[derive(Debug, Clone)]
pub struct DinoInfo {
    pub nombre: String,
    pub cientifico: String,
    pub descripcion: String,
    pub era: String,
    pub categoria: String,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CeldaExcavacion {
    Roca(u8),
    Fosil,
    Vacio,
}

pub struct QuizActivo {
    pub pregunta: String,
    pub opciones: Vec<String>,
    pub correcta: usize,
    pub seleccion: usize,
    pub respondida: bool,
    pub fue_correcta: bool,
}

pub struct MinijuegoMuseo {
    pub activo: bool,
    pub fase: FaseMuseo,

    // Exhibiciones desde DB
    pub exhibiciones: Vec<DinoInfo>,
    pub indice: usize,

    // Excavación
    pub grilla: Vec<Vec<CeldaExcavacion>>,
    pub grilla_cols: usize,
    pub grilla_rows: usize,
    pub cursor_x: usize,
    pub cursor_y: usize,
    pub golpes_restantes: usize,
    pub max_golpes: usize,
    pub fosil_celdas: Vec<(usize, usize)>,
    pub fosil_reveladas: usize,
    pub fosil_encontrado: bool,
    pub dino_excavado: Option<DinoInfo>,

    // Quiz desde DB
    pub preguntas_db: Vec<QuizPreguntaDB>,
    pub quiz_activo: Option<QuizActivo>,
    pub quiz_indice: usize,
    pub quiz_total: usize,
    pub quiz_puntaje: usize,
}

impl MinijuegoMuseo {
    pub fn new() -> Self {
        Self {
            activo: false,
            fase: FaseMuseo::Entrada,
            exhibiciones: Vec::new(),
            indice: 0,
            grilla: Vec::new(),
            grilla_cols: 3,
            grilla_rows: 3,
            cursor_x: 1,
            cursor_y: 1,
            golpes_restantes: 0,
            max_golpes: 12,
            fosil_celdas: Vec::new(),
            fosil_reveladas: 0,
            fosil_encontrado: false,
            dino_excavado: None,
            preguntas_db: Vec::new(),
            quiz_activo: None,
            quiz_indice: 0,
            quiz_total: 3,
            quiz_puntaje: 0,
        }
    }

    /// Cargar exhibiciones y preguntas desde la DB
    pub fn iniciar(&mut self, db: &ZooDB) {
        self.activo = true;
        self.fase = FaseMuseo::Entrada;
        self.indice = 0;
        self.quiz_puntaje = 0;
        self.quiz_indice = 0;

        // Cargar fósiles desde DB
        let fosiles_db = db.animales_por_categoria("fosiles");
        self.exhibiciones = fosiles_db.iter().map(|a| DinoInfo {
            nombre: a.nombre_comun.clone(),
            cientifico: a.nombre_cientifico.clone(),
            descripcion: a.descripcion.clone(),
            era: String::new(), // Podría añadirse como campo en DB
            categoria: "fosiles".to_string(),
        }).collect();

        // Cargar preguntas del quiz desde DB
        self.preguntas_db = db.quiz_museo_preguntas();
    }

    pub fn entrar_explorando(&mut self) {
        self.fase = FaseMuseo::Explorando;
    }

    pub fn iniciar_excavacion(&mut self) {
        self.fase = FaseMuseo::Excavando;
        self.golpes_restantes = self.max_golpes;
        self.fosil_encontrado = false;
        self.fosil_reveladas = 0;
        self.cursor_x = self.grilla_cols / 2;
        self.cursor_y = self.grilla_rows / 2;

        // Seleccionar fósil aleatorio
        if !self.exhibiciones.is_empty() {
            let idx = gen_range(0, self.exhibiciones.len());
            self.dino_excavado = Some(self.exhibiciones[idx].clone());
        }

        // Inicializar grilla
        self.grilla = vec![
            vec![CeldaExcavacion::Roca(3); self.grilla_cols];
            self.grilla_rows
        ];
        self.fosil_celdas.clear();

        // Colocar fósil aleatoriamente
        let cx = gen_range(0, self.grilla_cols);
        let cy = gen_range(0, self.grilla_rows);
        self.fosil_celdas.push((cx, cy));

        let extras = gen_range(1, 4);
        for _ in 0..extras {
            if let Some(&(bx, by)) = self.fosil_celdas.last() {
                let dx = gen_range(0, 3) as i32 - 1;
                let dy = gen_range(0, 3) as i32 - 1;
                let nx = (bx as i32 + dx).clamp(0, self.grilla_cols as i32 - 1) as usize;
                let ny = (by as i32 + dy).clamp(0, self.grilla_rows as i32 - 1) as usize;
                if !self.fosil_celdas.contains(&(nx, ny)) {
                    self.fosil_celdas.push((nx, ny));
                }
            }
        }
    }

    pub fn golpear(&mut self) -> bool {
        if self.golpes_restantes == 0 { return false; }
        let x = self.cursor_x;
        let y = self.cursor_y;
        match self.grilla[y][x] {
            CeldaExcavacion::Roca(capas) => {
                self.golpes_restantes -= 1;
                if capas <= 1 {
                    if self.fosil_celdas.contains(&(x, y)) {
                        self.grilla[y][x] = CeldaExcavacion::Fosil;
                        self.fosil_reveladas += 1;
                        if self.fosil_reveladas >= self.fosil_celdas.len() {
                            self.fosil_encontrado = true;
                            return true; // Señal: fósil completado
                        }
                    } else {
                        self.grilla[y][x] = CeldaExcavacion::Vacio;
                    }
                } else {
                    self.grilla[y][x] = CeldaExcavacion::Roca(capas - 1);
                }
            }
            _ => {}
        }
        // Si se agotaron los golpes
        if self.golpes_restantes == 0 {
            return true; // Señal: excavación terminada (con o sin fósil)
        }
        false
    }

    pub fn mover_cursor(&mut self, dx: i32, dy: i32) {
        self.cursor_x = (self.cursor_x as i32 + dx)
            .clamp(0, self.grilla_cols as i32 - 1) as usize;
        self.cursor_y = (self.cursor_y as i32 + dy)
            .clamp(0, self.grilla_rows as i32 - 1) as usize;
    }

    pub fn iniciar_quiz(&mut self) {
        self.quiz_indice = 0;
        self.quiz_puntaje = 0;
        self.cargar_pregunta_actual();
        self.fase = FaseMuseo::Quiz;
    }

    fn cargar_pregunta_actual(&mut self) {
        if self.preguntas_db.is_empty() {
            self.quiz_activo = None;
            return;
        }
        let idx = self.quiz_indice % self.preguntas_db.len();
        let p = &self.preguntas_db[idx];
        self.quiz_activo = Some(QuizActivo {
            pregunta: p.pregunta.clone(),
            opciones: vec![
                p.opcion_a.clone(),
                p.opcion_b.clone(),
                p.opcion_c.clone(),
                p.opcion_d.clone(),
            ],
            correcta: p.correcta,
            seleccion: 0,
            respondida: false,
            fue_correcta: false,
        });
    }

    pub fn responder_quiz(&mut self) {
        if let Some(ref mut q) = self.quiz_activo {
            if !q.respondida {
                q.fue_correcta = q.seleccion == q.correcta;
                q.respondida = true;
                if q.fue_correcta {
                    self.quiz_puntaje += 1;
                }
            }
        }
    }

    pub fn siguiente_pregunta(&mut self) -> bool {
        self.quiz_indice += 1;
        if self.quiz_indice >= self.quiz_total {
            self.fase = FaseMuseo::QuizResultado;
            self.quiz_activo = None;
            false
        } else {
            self.cargar_pregunta_actual();
            true
        }
    }

    pub fn volver_explorar(&mut self) {
        self.fase = FaseMuseo::Explorando;
    }

    pub fn cerrar(&mut self) {
        self.activo = false;
        self.fase = FaseMuseo::Entrada;
        self.quiz_activo = None;
    }

    pub fn total_opciones_menu(&self) -> usize {
        // Exhibiciones + "Excavar" + "Quiz"
        self.exhibiciones.len() + 2
    }
}