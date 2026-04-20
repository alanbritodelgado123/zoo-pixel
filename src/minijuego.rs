use macroquad::prelude::*;
use macroquad::rand::gen_range;
use crate::config;

// =====================================================================
//  PESCA (Acuario) - En P5, mismos animales que Z5-2
// =====================================================================
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FasePesca {
    Esperando,
    Picando,
    Resultado,
    InfoPez,
}

#[derive(Debug, Clone)]
pub struct PezInfo {
    pub nombre: String,
    pub cientifico: String,
    pub descripcion: String,
    pub peso_kg: f32,
}

pub struct MinijuegoPesca {
    pub activo: bool,
    pub fase: FasePesca,
    pub timer: f32,
    pub tiempo_espera: f32,
    pub tiempo_picada: f32,
    pub pez_actual: Option<PezInfo>,
    pub peces_atrapados: Vec<PezInfo>,
    pub intentos: usize,
    pub max_intentos: usize,
    pub exito: bool,
    pub texto_pos: usize,
    pub texto_timer: f32,
    pub texto_terminado: bool,
}

impl MinijuegoPesca {
    pub fn new() -> Self {
        Self {
            activo: false,
            fase: FasePesca::Esperando,
            timer: 0.0,
            tiempo_espera: 0.0,
            tiempo_picada: 0.0,
            pez_actual: None,
            peces_atrapados: Vec::new(),
            intentos: 0,
            max_intentos: 3,
            exito: false,
            texto_pos: 0,
            texto_timer: 0.0,
            texto_terminado: false,
        }
    }

    pub fn iniciar(&mut self) {
        self.activo = true;
        self.intentos = 0;
        self.peces_atrapados.clear();
        self.preparar_ronda();
    }

    fn preparar_ronda(&mut self) {
        self.fase = FasePesca::Esperando;
        self.timer = 0.0;
        self.tiempo_espera = gen_range(2.5, 5.0);
        self.tiempo_picada = gen_range(1.8, 3.5);
        self.pez_actual = Some(self.generar_pez());
        self.exito = false;
        self.texto_pos = 0;
        self.texto_timer = 0.0;
        self.texto_terminado = false;
    }

fn generar_pez(&self) -> PezInfo {
    // ✅ SOLO PECES REALES DE AGUAS VENEZOLANAS
    let peces = vec![
        PezInfo {
            nombre: "Pavón".into(),
            cientifico: "Cichla temensis".into(),
            descripcion: "Depredador de agua dulce, puede pesar hasta 15 kg. Muy combativo al ser pescado. Color verde-dorado con manchas oscuras características.".into(),
            peso_kg: gen_range(3.0, 15.0),
        },
        PezInfo {
            nombre: "Pez Loro".into(),
            cientifico: "Scarus guacamaia".into(),
            descripcion: "Pez loro más grande del Atlántico, hasta 1.2 metros. Se alimenta de coral y algas. Colorido con tonos verdes, azules y rosados.".into(),
            peso_kg: gen_range(5.0, 20.0),
        },
        PezInfo {
            nombre: "Mero".into(),
            cientifico: "Epinephelus itajara".into(),
            descripcion: "Pez de arrecife de gran tamaño, puede superar los 2 metros. Solitario y territorial. En peligro crítico de extinción.".into(),
            peso_kg: gen_range(50.0, 200.0),
        },
        PezInfo {
            nombre: "Pargo".into(),
            cientifico: "Lutjanus analis".into(),
            descripcion: "Pez de arrecife muy apreciado en pesca deportiva. Color plateado con aletas rojizas. Vive en cardúmenes.".into(),
            peso_kg: gen_range(2.0, 10.0),
        },
        PezInfo {
            nombre: "Jurel".into(),
            cientifico: "Caranx hippos".into(),
            descripcion: "Pez rápido y agresivo de aguas costeras. Color plateado con manchas oscuras. Muy común en Isla de Margarita.".into(),
            peso_kg: gen_range(3.0, 15.0),
        },
        PezInfo {
            nombre: "Corocoro".into(),
            cientifico: "Micropogonias furnieri".into(),
            descripcion: "Pez costero de la familia de los corvinas. Muy apreciado en la cocina venezolana. Color plateado con reflejos dorados.".into(),
            peso_kg: gen_range(1.0, 5.0),
        },
        PezInfo {
            nombre: "Lisa".into(),
            cientifico: "Mugil cephalus".into(),
            descripcion: "Pez costero que entra en lagunas y estuarios. Cuerpo alargado plateado. Muy común en la costa venezolana.".into(),
            peso_kg: gen_range(1.0, 4.0),
        },
        PezInfo {
            nombre: "Sardina".into(),
            cientifico: "Sardinella aurita".into(),
            descripcion: "Pez pequeño de cardumen, base de la pesca artesanal. Color plateado azulado. Importante para la economía local.".into(),
            peso_kg: gen_range(0.1, 0.3),
        },
    ];
    peces[gen_range(0, peces.len())].clone()
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
                    self.exito = false;
                    self.fase = FasePesca::Resultado;
                    self.timer = 0.0;
                }
            }
            FasePesca::InfoPez => {
                if !self.texto_terminado {
                    self.texto_timer += dt;
                    let chars = (self.texto_timer * config::TYPEWRITER_CPS) as usize;
                    if let Some(ref pez) = self.pez_actual {
                        let total = pez.descripcion.chars().count();
                        if chars >= total {
                            self.texto_pos = total;
                            self.texto_terminado = true;
                        } else {
                            self.texto_pos = chars;
                        }
                    }
                }
            }
            _ => {}
        }
    }

    pub fn tirar(&mut self) {
        if self.fase == FasePesca::Picando {
            self.exito = true;
            if let Some(ref pez) = self.pez_actual {
                self.peces_atrapados.push(pez.clone());
            }
            self.fase = FasePesca::InfoPez;
            self.texto_pos = 0;
            self.texto_timer = 0.0;
            self.texto_terminado = false;
        }
    }

    pub fn siguiente_o_salir(&mut self) {
        self.intentos += 1;
        if self.intentos >= self.max_intentos {
            self.cerrar();
        } else {
            self.preparar_ronda();
        }
    }

    pub fn cerrar(&mut self) {
        self.activo = false;
        self.fase = FasePesca::Esperando;
    }

    pub fn nombre_pez_visible(&self) -> Option<&str> {
        match self.fase {
            FasePesca::InfoPez => self.pez_actual.as_ref().map(|p| p.nombre.as_str()),
            FasePesca::Resultado if self.exito => self.pez_actual.as_ref().map(|p| p.nombre.as_str()),
            _ => None,
        }
    }
}

// =====================================================================
//  MUSEO - Excavación 3x3 + Quiz con puntaje
// =====================================================================
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FaseMuseo {
    Entrada,
    Explorando,
    Excavando,
    FosilRevelado,
    ViendoExhibicion,
    Quiz,
    QuizResultado,
}

#[derive(Debug, Clone)]
pub struct DinoInfo {
    pub nombre: String,
    pub cientifico: String,
    pub descripcion: String,
    pub era: String,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CeldaExcavacion {
    Roca(u8),
    Fosil,
    Vacio,
}

pub struct MinijuegoMuseo {
    pub activo: bool,
    pub fase: FaseMuseo,
    pub exhibiciones: Vec<DinoInfo>,
    pub indice: usize,
    pub texto_pos: usize,
    pub texto_timer: f32,
    pub terminado_texto: bool,
    // Quiz con puntaje
    pub quiz_pregunta: String,
    pub quiz_opciones: Vec<String>,
    pub quiz_correcta: usize,
    pub quiz_seleccion: usize,
    pub quiz_respondida: bool,
    pub quiz_correcta_resp: bool,
    pub quiz_indice: usize,
    pub quiz_puntaje: usize,
    pub quiz_total: usize,
    // Excavación 3x3
    pub grilla: Vec<Vec<CeldaExcavacion>>,
    pub grilla_cols: usize,
    pub grilla_rows: usize,
    pub cursor_x: usize,
    pub cursor_y: usize,
    pub golpes_restantes: usize,
    pub max_golpes: usize,
    pub fosil_encontrado: bool,
    pub fosil_celdas: Vec<(usize, usize)>,
    pub fosil_reveladas: usize,
    pub dino_excavado: Option<DinoInfo>,
}

impl MinijuegoMuseo {
    pub fn new() -> Self {
        Self {
            activo: false,
            fase: FaseMuseo::Entrada,
            exhibiciones: Self::crear_exhibiciones(),
            indice: 0,
            texto_pos: 0,
            texto_timer: 0.0,
            terminado_texto: false,
            quiz_pregunta: String::new(),
            quiz_opciones: Vec::new(),
            quiz_correcta: 0,
            quiz_seleccion: 0,
            quiz_respondida: false,
            quiz_correcta_resp: false,
            quiz_indice: 0,
            quiz_puntaje: 0,
            quiz_total: 3,
            grilla: Vec::new(),
            grilla_cols: 3,  // 3x3 grid
            grilla_rows: 3,
            cursor_x: 1,
            cursor_y: 1,
            golpes_restantes: 0,
            max_golpes: 12,
            fosil_encontrado: false,
            fosil_celdas: Vec::new(),
            fosil_reveladas: 0,
            dino_excavado: None,
        }
    }

    fn crear_exhibiciones() -> Vec<DinoInfo> {
        vec![
            DinoInfo {
                nombre: "Carnotauro".into(),
                cientifico: "Carnotaurus sastrei".into(),
                descripcion: "Dinosaurio terópodo del Cretácico Superior. Tenía cuernos sobre los ojos y brazos extremadamente reducidos. Bípedo y carnívoro. Descubierto en Argentina en 1984.".into(),
                era: "Cretácico Superior".into(),
            },
            DinoInfo {
                nombre: "Pteranodon".into(),
                cientifico: "Pteranodon longiceps".into(),
                descripcion: "Reptil volador del Cretácico con envergadura de hasta 7 metros. No tenía dientes. Su cresta ósea servía como timón en vuelo. Se alimentaba de peces.".into(),
                era: "Cretácico Superior".into(),
            },
            DinoInfo {
                nombre: "Ammonite".into(),
                cientifico: "Ammonoidea".into(),
                descripcion: "Molusco marino extinto emparentado con el nautilus actual. Su concha espiral podía medir desde 1 cm hasta 2 metros. Vivió desde el Devónico hasta el Cretácico.".into(),
                era: "Devónico - Cretácico".into(),
            },
            DinoInfo {
                nombre: "Trilobite".into(),
                cientifico: "Trilobita".into(),
                descripcion: "Artrópodo marino que vivió durante 300 millones de años. Tenía ojos compuestos de cristal de calcita. Uno de los fósiles más comunes y reconocibles del mundo.".into(),
                era: "Cámbrico - Pérmico".into(),
            },
            DinoInfo {
                nombre: "Megalodon".into(),
                cientifico: "Otodus megalodon".into(),
                descripcion: "El tiburón más grande que existió, hasta 18 metros de largo. Sus dientes medían 17 cm. Dominó los océanos durante millones de años antes de extinguirse.".into(),
                era: "Mioceno - Plioceno".into(),
            },
        ]
    }

    pub fn iniciar(&mut self) {
        self.activo = true;
        self.fase = FaseMuseo::Entrada;
        self.indice = 0;
        self.quiz_puntaje = 0;
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

        let dino_idx = gen_range(0, self.exhibiciones.len());
        self.dino_excavado = Some(self.exhibiciones[dino_idx].clone());

        self.grilla = vec![vec![CeldaExcavacion::Roca(3); self.grilla_cols]; self.grilla_rows];

        // Fósil en 3-5 celdas para 3x3
        self.fosil_celdas.clear();
        let cx = gen_range(1, self.grilla_cols - 1);
        let cy = gen_range(1, self.grilla_rows - 1);
        let base_celdas = vec![(cx, cy)];
        for (fx, fy) in &base_celdas {
            self.fosil_celdas.push((*fx, *fy));
        }
        let extras = gen_range(2, 5);
        for _ in 0..extras {
            if let Some(&(bx, by)) = self.fosil_celdas.last() {
                let dx: i32 = gen_range(-1, 2);
                let dy: i32 = gen_range(-1, 2);
                let nx = (bx as i32 + dx).clamp(0, self.grilla_cols as i32 - 1) as usize;
                let ny = (by as i32 + dy).clamp(0, self.grilla_rows as i32 - 1) as usize;
                if !self.fosil_celdas.contains(&(nx, ny)) {
                    self.fosil_celdas.push((nx, ny));
                }
            }
        }
    }

    pub fn golpear(&mut self) {
        if self.golpes_restantes == 0 { return; }
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
                            self.fase = FaseMuseo::FosilRevelado;
                            self.texto_pos = 0;
                            self.texto_timer = 0.0;
                            self.terminado_texto = false;
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
        if self.golpes_restantes == 0 && !self.fosil_encontrado {
            self.fase = FaseMuseo::FosilRevelado;
            self.texto_pos = 0;
            self.texto_timer = 0.0;
            self.terminado_texto = false;
        }
    }

    pub fn mover_cursor(&mut self, dx: i32, dy: i32) {
        self.cursor_x = (self.cursor_x as i32 + dx).clamp(0, self.grilla_cols as i32 - 1) as usize;
        self.cursor_y = (self.cursor_y as i32 + dy).clamp(0, self.grilla_rows as i32 - 1) as usize;
    }

    pub fn ver_exhibicion(&mut self) {
        self.fase = FaseMuseo::ViendoExhibicion;
        self.texto_pos = 0;
        self.texto_timer = 0.0;
        self.terminado_texto = false;
    }

    pub fn volver_explorar(&mut self) {
        self.fase = FaseMuseo::Explorando;
    }

    pub fn dino_actual(&self) -> &DinoInfo {
        &self.exhibiciones[self.indice]
    }

    pub fn iniciar_quiz(&mut self) {
        self.quiz_indice = 0;
        self.quiz_puntaje = 0;
        self.generar_quiz();
        self.fase = FaseMuseo::Quiz;
    }

    fn generar_quiz(&mut self) {
        let quizes: Vec<(&str, Vec<&str>, usize)> = vec![
            ("¿En qué era vivió el Carnotauro?",
             vec!["Jurásico", "Cretácico Superior", "Triásico", "Pérmico"], 1),
            ("¿Qué característica tenía el Pteranodon?",
             vec!["Cuernos", "Cola larga", "No tenía dientes", "Plumas"], 2),
            ("¿Cuánto podía medir un Megalodon?",
             vec!["5 metros", "10 metros", "18 metros", "25 metros"], 2),
            ("¿Los ammonites estaban emparentados con...?",
             vec!["Las estrellas de mar", "El nautilus", "Los corales", "Las medusas"], 1),
            ("¿De qué estaban hechos los ojos de los trilobites?",
             vec!["Queratina", "Cristal de calcita", "Cartílago", "Sílice"], 1),
        ];
        let idx = self.quiz_indice % quizes.len();
        let (preg, opts, correcta) = &quizes[idx];
        self.quiz_pregunta = preg.to_string();
        self.quiz_opciones = opts.iter().map(|s| s.to_string()).collect();
        self.quiz_correcta = *correcta;
        self.quiz_seleccion = 0;
        self.quiz_respondida = false;
        self.quiz_correcta_resp = false;
    }

    pub fn responder_quiz(&mut self) {
        self.quiz_respondida = true;
        self.quiz_correcta_resp = self.quiz_seleccion == self.quiz_correcta;
        if self.quiz_correcta_resp {
            self.quiz_puntaje += 1;
        }
    }

    pub fn siguiente_quiz(&mut self) {
        self.quiz_indice += 1;
        if self.quiz_indice >= self.quiz_total {
            self.fase = FaseMuseo::QuizResultado;
        } else {
            self.generar_quiz();
        }
    }

    pub fn cerrar(&mut self) {
        self.activo = false;
    }

    pub fn update(&mut self, dt: f32) {
        if !self.activo { return; }
        match self.fase {
            FaseMuseo::ViendoExhibicion | FaseMuseo::FosilRevelado => {
                if !self.terminado_texto {
                    self.texto_timer += dt;
                    let chars = (self.texto_timer * config::TYPEWRITER_CPS) as usize;
                    let desc = if self.fase == FaseMuseo::FosilRevelado {
                        self.dino_excavado.as_ref().map(|d| d.descripcion.as_str()).unwrap_or("")
                    } else {
                        &self.exhibiciones[self.indice].descripcion
                    };
                    let total = desc.chars().count();
                    if chars >= total {
                        self.texto_pos = total;
                        self.terminado_texto = true;
                    } else {
                        self.texto_pos = chars;
                    }
                }
            }
            _ => {}
        }
    }
}