use std::collections::HashSet;
use macroquad::rand::gen_range;
use crate::config;
use crate::db::{Animal, ZooDB};
use crate::escena::Escena;
use crate::input::Accion;
use crate::guia::{self, SistemaDialogo};
use crate::ciclo_dia::{CicloDia, ModoCiclo};
use crate::libreta::Libreta;
use crate::minijuego::{MinijuegoPesca, MinijuegoMuseo, FasePesca, FaseMuseo};
use crate::plataforma::DetectorPlataforma;
use crate::save::SaveData;
use crate::eventos::SistemaEventos;

const LIBRETA_POR_PAGINA: usize = 5;

#[derive(PartialEq)]
pub enum Pantalla {
    Inicio,
    Intro,
    Juego,
    Config,
    MapaCompleto,
    LibretaCompleta,
}

pub enum ModoVista {
    Normal,
    Seleccion {
        animales: Vec<Animal>,
        indice: usize,
    },
    ViendoAnimal {
        animal: Animal,
        lista: Vec<Animal>,
        indice_lista: usize,
        texto_pos: usize,
        timer: f32,
        terminado: bool,
    },
    Foto {
        animales: Vec<Animal>,
        indice_actual: usize,
        celda: usize,
        foto_tomada: bool,
        texto_pos: usize,
        timer: f32,
        terminado: bool,
        ya_vistos: HashSet<usize>,
    },
}

pub struct MenuConfig {
    pub seleccion: usize,
    pub volumen_musica: f32,
    pub volumen_efectos: f32,
    pub crt: bool,
}

impl MenuConfig {
    pub fn new(save: &SaveData) -> Self {
        Self {
            seleccion: 0,
            volumen_musica: save.config.volumen_musica,
            volumen_efectos: save.config.volumen_efectos,
            crt: save.config.crt,
        }
    }
    pub const OPCIONES: &'static [&'static str] = &[
        "Volumen Música",
        "Volumen Efectos",
        "Filtro CRT",
        "Volver",
    ];
}

pub struct Transicion {
    pub destino: Escena,
    pub timer: f32,
    pub duracion: f32,
    pub cambiada: bool,
}

pub struct AnimalInfo {
    pub nombre_comun: String,
    pub nombre_cientifico: String,
    pub descripcion: String,
    pub texto_pos: usize,
    pub timer: f32,
    pub terminado: bool,
    pub categoria: String,
    pub hint_z: String,
}

impl AnimalInfo {
    pub fn from_animal(a: &Animal, hint_z: &str) -> Self {
        Self {
            nombre_comun: a.nombre_comun.clone(),
            nombre_cientifico: a.nombre_cientifico.clone(),
            descripcion: a.descripcion.clone(),
            texto_pos: 0,
            timer: 0.0,
            terminado: false,
            categoria: a.categoria.clone(),
            hint_z: hint_z.to_string(),
        }
    }

    pub fn from_libreta(e: &crate::libreta::EntradaLibreta) -> Self {
        Self {
            nombre_comun: e.nombre.clone(),
            nombre_cientifico: e.cientifico.clone(),
            descripcion: e.descripcion.clone(),
            texto_pos: 0,
            timer: 0.0,
            terminado: false,
            categoria: e.categoria.clone(),
            hint_z: String::new(),
        }
    }

    pub fn mensaje(titulo: &str, desc: &str) -> Self {
        Self {
            nombre_comun: titulo.to_string(),
            nombre_cientifico: String::new(),
            descripcion: desc.to_string(),
            texto_pos: 0,
            timer: 0.0,
            terminado: false,
            categoria: String::new(),
            hint_z: String::new(),
        }
    }

    pub fn update(&mut self, dt: f32) {
        if self.terminado { return; }
        self.timer += dt;
        let chars = (self.timer * config::TYPEWRITER_CPS) as usize;
        let total = self.descripcion.chars().count();
        if chars >= total {
            self.texto_pos = total;
            self.terminado = true;
        } else {
            self.texto_pos = chars;
        }
    }

    pub fn completar(&mut self) {
        self.texto_pos = self.descripcion.chars().count();
        self.terminado = true;
    }
}

pub struct Estado {
    pub pantalla: Pantalla,
    pub escena: Escena,
    pub visitadas: HashSet<Escena>,
    pub transicion: Option<Transicion>,
    pub duracion_transicion: f32,
    pub modo: ModoVista,
    pub mostrar_overlay: bool,
    pub dialogo: SistemaDialogo,
    pub eventos: SistemaEventos,
    pub ciclo: CicloDia,
    pub libreta: Libreta,
    pub pesca: MinijuegoPesca,
    pub museo: MinijuegoMuseo,
    pub plataforma: DetectorPlataforma,
    pub save: SaveData,
    pub menu_config: MenuConfig,
    pub intro_timer: f32,
    pub inicio_timer: f32,
    pub inicio_seleccion: usize,
    pub mapa_cursor: Escena,
    pub indicador_z_pressed: f32,
    pub indicador_x_pressed: f32,
    pub necesita_sonido_animal: bool,
    pub necesita_transicion_audio: Option<Escena>,
    pub libreta_seleccion: usize,
    pub libreta_info: Option<AnimalInfo>,
    pub info_overlay: Option<AnimalInfo>,
    pub ya_vio_intro: bool,
    pub dialogo_callejon_mostrado: HashSet<String>,
    pub dialogo_museo_ani_mostrado: bool,
    pub dialogo_pesca_mostrado: bool,
    pub dialogo_foto_mostrado: bool,
}

impl Estado {
    pub fn new(db: &ZooDB) -> Self {
        let save = SaveData::cargar();
        let mut visitadas = save.visitadas.clone();
        let escena = save.escena.unwrap_or(Escena::E);
        visitadas.insert(escena);

        let ya_vio_intro = !save.animales_vistos.is_empty() || save.visitadas.len() > 1;
        let menu_config = MenuConfig::new(&save);

        let mut libreta = Libreta::new();
        for nombre in &save.animales_vistos {
            if let Some(animal) = db.animal_por_nombre(nombre) {
                libreta.registrar_animal(&animal);
            }
        }

        Self {
            pantalla: if ya_vio_intro { Pantalla::Juego } else { Pantalla::Inicio },
            escena,
            visitadas,
            transicion: None,
            duracion_transicion: config::TRANSITION_SECS_FALLBACK,
            modo: ModoVista::Normal,
            mostrar_overlay: false,
            dialogo: SistemaDialogo::new(),
            eventos: SistemaEventos::new(),
            ciclo: CicloDia::new(config::CICLO_DIA_SECS),
            libreta,
            pesca: MinijuegoPesca::new(),
            museo: MinijuegoMuseo::new(),
            plataforma: DetectorPlataforma::new(),
            save,
            menu_config,
            intro_timer: 0.0,
            inicio_timer: 0.0,
            inicio_seleccion: 0,
            mapa_cursor: escena,
            indicador_z_pressed: 0.0,
            indicador_x_pressed: 0.0,
            necesita_sonido_animal: false,
            necesita_transicion_audio: None,
            libreta_seleccion: 0,
            libreta_info: None,
            info_overlay: None,
            ya_vio_intro,
            dialogo_callejon_mostrado: HashSet::new(),
            dialogo_museo_ani_mostrado: false,
            dialogo_pesca_mostrado: false,
            dialogo_foto_mostrado: false,
        }
    }

    pub fn en_pantalla_info(&self) -> bool {
        matches!(self.modo,
            ModoVista::ViendoAnimal { .. }
            | ModoVista::Foto { .. }
            | ModoVista::Seleccion { .. }
        ) || self.pesca.activo
          || self.museo.activo
          || self.libreta_info.is_some()
          || self.info_overlay.is_some()
    }

    pub fn en_transicion(&self) -> bool {
        self.transicion.is_some()
    }

    pub fn alpha_transicion(&self) -> f32 {
        if let Some(ref t) = self.transicion {
            let p = (t.timer / t.duracion).clamp(0.0, 1.0);
            if p < 0.5 { p * 2.0 } else { (1.0 - p) * 2.0 }
        } else {
            0.0
        }
    }

    pub fn update(&mut self, dt: f32, db: &ZooDB) {
        self.plataforma.update();
        if self.indicador_z_pressed > 0.0 { self.indicador_z_pressed -= dt * 3.0; }
        if self.indicador_x_pressed > 0.0 { self.indicador_x_pressed -= dt * 3.0; }
        self.mostrar_overlay = matches!(self.pantalla, Pantalla::Juego);

        match self.pantalla {
            Pantalla::Inicio => {
                self.inicio_timer += dt;
            }
            Pantalla::Intro => {
                self.intro_timer += dt;
                self.dialogo.update(dt);
                if !self.dialogo.activo && self.dialogo.completado {
                    self.pantalla = Pantalla::Juego;
                    self.ya_vio_intro = true;
                    self.guardar_estado();
                }
            }
            Pantalla::Juego => {
                self.ciclo.update(dt);
                self.eventos.update(dt);

                if self.dialogo.activo {
                    self.dialogo.update(dt);
                }

                self.verificar_dialogo_callejon(db);

                if let Some(ref mut t) = self.transicion {
                    t.timer += dt;
                    if t.timer >= t.duracion / 2.0 && !t.cambiada {
                        self.escena = t.destino;
                        self.visitadas.insert(t.destino);
                        t.cambiada = true;
                    }
                    if t.timer >= t.duracion {
                        self.transicion = None;
                        self.guardar_estado();
                    }
                }

                self.update_typewriter(dt);

                if let Some(ref mut info) = self.info_overlay {
                    info.update(dt);
                }

                self.pesca.update(dt);
            }
            Pantalla::Config => {}
            Pantalla::MapaCompleto => {}
            Pantalla::LibretaCompleta => {
                if let Some(ref mut info) = self.libreta_info {
                    info.update(dt);
                }
            }
        }
    }

    fn verificar_dialogo_callejon(&mut self, db: &ZooDB) {
        let escena_id = self.escena.db_id();
        // Zonas Zx_5: zona_05, zona_10, zona_15, zona_20, zona_25
        let es_callejon = matches!(escena_id,
            "zona_05" | "zona_10" | "zona_15" | "zona_20" | "zona_25"
        );
        if !es_callejon || self.dialogo.activo {
            return;
        }
        let clave = format!("callejon_{}", escena_id);
        if !self.dialogo_callejon_mostrado.contains(&clave) {
            let dialogos = guia::dialogos_callejon_db(db, escena_id);
            if !dialogos.is_empty() {
                self.dialogo.iniciar_desde_db(dialogos);
                self.dialogo_callejon_mostrado.insert(clave);
            }
        }
    }

    fn update_typewriter(&mut self, dt: f32) {
        match &mut self.modo {
            ModoVista::ViendoAnimal { animal, texto_pos, timer, terminado, .. } => {
                if !*terminado {
                    *timer += dt;
                    let chars = (*timer * config::TYPEWRITER_CPS) as usize;
                    let total = animal.descripcion.chars().count();
                    if chars >= total {
                        *texto_pos = total;
                        *terminado = true;
                    } else {
                        *texto_pos = chars;
                    }
                }
            }
            ModoVista::Foto {
                animales, indice_actual, foto_tomada,
                texto_pos, timer, terminado, ..
            } => {
                if *foto_tomada && !*terminado {
                    *timer += dt;
                    let chars = (*timer * config::TYPEWRITER_CPS) as usize;
                    let total = animales[*indice_actual].descripcion.chars().count();
                    if chars >= total {
                        *texto_pos = total;
                        *terminado = true;
                    } else {
                        *texto_pos = chars;
                    }
                }
            }
            _ => {}
        }
    }

    fn guardar_estado(&mut self) {
        self.save.escena = Some(self.escena);
        self.save.visitadas = self.visitadas.clone();
        self.save.guardar();
    }

    pub fn cambiar_escena(&mut self, destino: Escena) {
        if destino == self.escena { return; }

        if self.escena.es_pesca() && !destino.es_pesca() {
            self.pesca.cerrar();
        }
        if self.escena.es_museo() && !destino.es_museo() {
            self.museo.cerrar();
        }

        // Resetear flags de diálogo al cambiar de zona
        if destino.es_pesca() && destino != self.escena {
            // No resetear: el diálogo de pesca se muestra una sola vez por sesión
        }

        self.modo = ModoVista::Normal;
        self.info_overlay = None;

        self.transicion = Some(Transicion {
            destino,
            timer: 0.0,
            duracion: self.duracion_transicion,
            cambiada: false,
        });
        self.necesita_transicion_audio = Some(destino);
    }

    pub fn procesar_accion(&mut self, accion: Accion, db: &ZooDB) {
        if accion == Accion::BotonA { self.indicador_z_pressed = 1.0; }
        if accion == Accion::BotonB { self.indicador_x_pressed = 1.0; }

        match self.pantalla {
            Pantalla::Inicio      => self.input_inicio(accion, db),
            Pantalla::Intro       => self.input_intro(accion),
            Pantalla::Config      => self.input_config(accion),
            Pantalla::MapaCompleto    => self.input_mapa(accion),
            Pantalla::LibretaCompleta => self.input_libreta(accion),
            Pantalla::Juego => {
                if self.dialogo.activo {
                    if matches!(accion, Accion::BotonA | Accion::BotonB) {
                        self.dialogo.avanzar();
                        if !self.dialogo.activo && self.dialogo.completado
                            && !self.ya_vio_intro
                        {
                            self.pantalla = Pantalla::Juego;
                            self.ya_vio_intro = true;
                            self.guardar_estado();
                        }
                    }
                    return;
                }

                if let Some(ref mut info) = self.info_overlay {
                    match accion {
                        Accion::BotonA => {
                            if info.terminado {
                                self.info_overlay = None;
                            } else {
                                info.completar();
                            }
                        }
                        Accion::BotonB => {
                            self.info_overlay = None;
                        }
                        _ => {}
                    }
                    return;
                }

                if self.pesca.activo {
                    self.input_pesca(accion, db);
                    return;
                }
                if self.museo.activo {
                    self.input_museo(accion, db);
                    return;
                }

                if accion == Accion::Mapa {
                    self.mapa_cursor = self.escena;
                    self.pantalla = Pantalla::MapaCompleto;
                    return;
                }
                if accion == Accion::Menu {
                    self.menu_config = MenuConfig::new(&self.save);
                    self.pantalla = Pantalla::Config;
                    return;
                }
                if accion == Accion::Libreta {
                    self.libreta_seleccion = 0;
                    self.libreta.pagina = 0;
                    self.libreta_info = None;
                    self.pantalla = Pantalla::LibretaCompleta;
                    return;
                }

                self.input_juego(accion, db);
            }
        }
    }

    fn input_inicio(&mut self, accion: Accion, db: &ZooDB) {
        match accion {
            Accion::Arriba => {
                if self.inicio_seleccion > 0 { self.inicio_seleccion -= 1; }
            }
            Accion::Abajo => {
                if self.inicio_seleccion < 3 { self.inicio_seleccion += 1; }
            }
            Accion::BotonA => match self.inicio_seleccion {
                0 => { self.ciclo.set_modo(ModoCiclo::Sistema);         self.entrar_juego(db); }
                1 => { self.ciclo.set_modo(ModoCiclo::DiaPermanente);   self.entrar_juego(db); }
                2 => { self.ciclo.set_modo(ModoCiclo::NochePermanente); self.entrar_juego(db); }
                3 => { self.menu_config = MenuConfig::new(&self.save);  self.pantalla = Pantalla::Config; }
                _ => {}
            }
            _ => {}
        }
    }

    fn entrar_juego(&mut self, db: &ZooDB) {
        if self.ya_vio_intro {
            self.pantalla = Pantalla::Juego;
        } else {
            self.iniciar_intro(db);
        }
    }

    fn iniciar_intro(&mut self, db: &ZooDB) {
        self.pantalla = Pantalla::Intro;
        self.intro_timer = 0.0;
        let dialogos = guia::dialogos_bienvenida_db(db, &self.plataforma);
        if !dialogos.is_empty() {
            self.dialogo.iniciar_desde_db(dialogos);
        } else {
            self.dialogo.iniciar(guia::dialogos_bienvenida_fallback(&self.plataforma));
        }
    }

    fn input_intro(&mut self, accion: Accion) {
        if matches!(accion, Accion::BotonA | Accion::BotonB) {
            self.dialogo.avanzar();
        }
    }

    fn input_config(&mut self, accion: Accion) {
        let mc = &mut self.menu_config;
        let total = MenuConfig::OPCIONES.len();
        match accion {
            Accion::Arriba    => { if mc.seleccion > 0 { mc.seleccion -= 1; } }
            Accion::Abajo     => { if mc.seleccion + 1 < total { mc.seleccion += 1; } }
            Accion::Izquierda => match mc.seleccion {
                0 => mc.volumen_musica  = (mc.volumen_musica  - 0.1).max(0.0),
                1 => mc.volumen_efectos = (mc.volumen_efectos - 0.1).max(0.0),
                2 => mc.crt = !mc.crt,
                _ => {}
            }
            Accion::Derecha => match mc.seleccion {
                0 => mc.volumen_musica  = (mc.volumen_musica  + 0.1).min(1.0),
                1 => mc.volumen_efectos = (mc.volumen_efectos + 0.1).min(1.0),
                2 => mc.crt = !mc.crt,
                _ => {}
            }
            Accion::BotonA => {
                if mc.seleccion == 2 {
                    mc.crt = !mc.crt;
                } else if mc.seleccion == total - 1 {
                    self.guardar_config();
                }
            }
            Accion::BotonB => { self.guardar_config(); }
            _ => {}
        }
    }

    fn guardar_config(&mut self) {
        self.save.config.volumen_musica  = self.menu_config.volumen_musica;
        self.save.config.volumen_efectos = self.menu_config.volumen_efectos;
        self.save.config.crt             = self.menu_config.crt;
        self.save.guardar();
        self.pantalla = if self.ya_vio_intro { Pantalla::Juego } else { Pantalla::Inicio };
    }

    fn input_mapa(&mut self, accion: Accion) {
        match accion {
            Accion::BotonB | Accion::Mapa => { self.pantalla = Pantalla::Juego; }
            Accion::Arriba | Accion::Abajo | Accion::Izquierda | Accion::Derecha => {
                let idx = match accion {
                    Accion::Arriba    => 0,
                    Accion::Abajo     => 1,
                    Accion::Izquierda => 2,
                    Accion::Derecha   => 3,
                    _ => return,
                };
                if let Some(dest) = self.mapa_cursor.conexiones()[idx] {
                    self.mapa_cursor = dest;
                }
            }
            Accion::BotonA => {
                if self.mapa_cursor != self.escena
                    && self.visitadas.contains(&self.mapa_cursor)
                {
                    self.pantalla = Pantalla::Juego;
                    self.cambiar_escena(self.mapa_cursor);
                }
            }
            _ => {}
        }
    }

    fn input_libreta(&mut self, accion: Accion) {
        if let Some(ref mut info) = self.libreta_info {
            match accion {
                Accion::BotonB => { self.libreta_info = None; }
                Accion::BotonA => {
                    if info.terminado { self.libreta_info = None; }
                    else { info.completar(); }
                }
                _ => {}
            }
            return;
        }
        let total = self.libreta.entradas.len();
        match accion {
            Accion::BotonB | Accion::Libreta => { self.pantalla = Pantalla::Juego; }
            Accion::Arriba => {
                if self.libreta_seleccion > 0 {
                    self.libreta_seleccion -= 1;
                    self.libreta.pagina = self.libreta_seleccion / LIBRETA_POR_PAGINA;
                }
            }
            Accion::Abajo => {
                if total > 0 && self.libreta_seleccion + 1 < total {
                    self.libreta_seleccion += 1;
                    self.libreta.pagina = self.libreta_seleccion / LIBRETA_POR_PAGINA;
                }
            }
            Accion::BotonA => {
                if total > 0 && self.libreta_seleccion < total {
                    self.libreta_info = Some(AnimalInfo::from_libreta(
                        &self.libreta.entradas[self.libreta_seleccion],
                    ));
                }
            }
            _ => {}
        }
    }

    fn input_pesca(&mut self, accion: Accion, db: &ZooDB) {
        match accion {
            Accion::BotonA => {
                match self.pesca.fase {
                    FasePesca::Picando => {
                        if let Some(pez) = self.pesca.tirar() {
                            if let Some(animal) = db.animal_por_nombre(&pez.nombre) {
                                self.libreta.registrar_animal(&animal);
                                self.save.marcar_animal_visto(&pez.nombre);
                                self.save.guardar();
                            }
                            self.info_overlay = Some(AnimalInfo {
                                nombre_comun: pez.nombre,
                                nombre_cientifico: pez.cientifico,
                                descripcion: format!(
                                    "{}\n\nPeso capturado: {:.1} kg",
                                    pez.descripcion, pez.peso_kg
                                ),
                                texto_pos: 0,
                                timer: 0.0,
                                terminado: false,
                                categoria: pez.categoria,
                                hint_z: "Z: Siguiente pesca".to_string(),
                            });
                            self.pesca.siguiente(db);
                        }
                    }
                    FasePesca::Resultado => {
                        self.pesca.siguiente(db);
                    }
                    _ => {}
                }
            }
            Accion::BotonB => {
                self.pesca.cerrar();
            }
            _ => {}
        }
    }

    fn input_museo(&mut self, accion: Accion, db: &ZooDB) {
        match accion {
            Accion::BotonA => match self.museo.fase {
                FaseMuseo::Entrada => {
                    self.museo.entrar_explorando();
                }
                FaseMuseo::Explorando => {
                    let total_exhibiciones = self.museo.exhibiciones.len();
                    let idx_excavar = total_exhibiciones;
                    let idx_quiz    = total_exhibiciones + 1;

                    if self.museo.indice < total_exhibiciones {
                        let dino = self.museo.exhibiciones[self.museo.indice].clone();
                        self.info_overlay = Some(AnimalInfo {
                            nombre_comun: dino.nombre,
                            nombre_cientifico: dino.cientifico,
                            descripcion: dino.descripcion,
                            texto_pos: 0,
                            timer: 0.0,
                            terminado: false,
                            categoria: dino.categoria,
                            hint_z: String::new(),
                        });
                    } else if self.museo.indice == idx_excavar {
                        self.museo.iniciar_excavacion();
                    } else if self.museo.indice == idx_quiz {
                        self.museo.iniciar_quiz();
                    }
                }
                FaseMuseo::Excavando => {
                    let terminada = self.museo.golpear();
                    if terminada {
                        if self.museo.fosil_encontrado {
                            if let Some(ref dino) = self.museo.dino_excavado {
                                if let Some(animal) = db.animal_por_nombre(&dino.nombre) {
                                    self.libreta.registrar_animal(&animal);
                                    self.save.marcar_animal_visto(&dino.nombre);
                                    self.save.guardar();
                                }
                                self.info_overlay = Some(AnimalInfo {
                                    nombre_comun: dino.nombre.clone(),
                                    nombre_cientifico: dino.cientifico.clone(),
                                    descripcion: dino.descripcion.clone(),
                                    texto_pos: 0,
                                    timer: 0.0,
                                    terminado: false,
                                    categoria: dino.categoria.clone(),
                                    hint_z: String::new(),
                                });
                            }
                        } else {
                            self.info_overlay = Some(AnimalInfo::mensaje(
                                "Excavación incompleta",
                                "Se agotaron los golpes sin revelar el fósil completo. Puedes intentarlo de nuevo.",
                            ));
                        }
                        self.museo.volver_explorar();
                    }
                }
                FaseMuseo::Quiz => {
                    if let Some(ref q) = self.museo.quiz_activo {
                        if q.respondida {
                            self.museo.siguiente_pregunta();
                        } else {
                            self.museo.responder_quiz();
                        }
                    }
                }
                FaseMuseo::QuizResultado => {
                    self.museo.volver_explorar();
                }
            }

            Accion::BotonB => match self.museo.fase {
                FaseMuseo::Entrada | FaseMuseo::Explorando => {
                    self.museo.cerrar();
                }
                FaseMuseo::Excavando => {
                    self.museo.volver_explorar();
                }
                FaseMuseo::Quiz => {
                    if let Some(ref q) = self.museo.quiz_activo {
                        if q.respondida {
                            self.museo.siguiente_pregunta();
                        } else {
                            self.museo.fase = FaseMuseo::Explorando;
                        }
                    }
                }
                FaseMuseo::QuizResultado => {
                    self.museo.volver_explorar();
                }
            }

            Accion::Arriba => match self.museo.fase {
                FaseMuseo::Explorando => {
                    if self.museo.indice > 0 { self.museo.indice -= 1; }
                }
                FaseMuseo::Excavando => {
                    self.museo.mover_cursor(0, -1);
                }
                FaseMuseo::Quiz => {
                    if let Some(ref mut q) = self.museo.quiz_activo {
                        if !q.respondida && q.seleccion > 0 {
                            q.seleccion -= 1;
                        }
                    }
                }
                _ => {}
            }

            Accion::Abajo => match self.museo.fase {
                FaseMuseo::Explorando => {
                    let max = self.museo.total_opciones_menu();
                    if self.museo.indice + 1 < max {
                        self.museo.indice += 1;
                    }
                }
                FaseMuseo::Excavando => {
                    self.museo.mover_cursor(0, 1);
                }
                FaseMuseo::Quiz => {
                    if let Some(ref mut q) = self.museo.quiz_activo {
                        if !q.respondida && q.seleccion + 1 < q.opciones.len() {
                            q.seleccion += 1;
                        }
                    }
                }
                _ => {}
            }

            Accion::Izquierda => {
                if self.museo.fase == FaseMuseo::Excavando {
                    self.museo.mover_cursor(-1, 0);
                }
            }
            Accion::Derecha => {
                if self.museo.fase == FaseMuseo::Excavando {
                    self.museo.mover_cursor(1, 0);
                }
            }
            _ => {}
        }
    }

    fn input_juego(&mut self, accion: Accion, db: &ZooDB) {
        if self.en_transicion() { return; }

        match &mut self.modo {
            ModoVista::Normal => {
                match accion {
                    Accion::BotonA => {
                        // Museo
                        if self.escena.es_museo() {
                            if !self.museo.activo {
                                if !self.dialogo_museo_ani_mostrado {
                                    let dialogos = guia::dialogos_museo_ani_db(db);
                                    if !dialogos.is_empty() {
                                        self.dialogo.iniciar_desde_db(dialogos);
                                        self.dialogo_museo_ani_mostrado = true;
                                        return;
                                    }
                                }
                                self.museo.iniciar(db);
                            }
                            return;
                        }

                        // Pesca
                        if self.escena.es_pesca() {
                            if !self.pesca.activo {
                                if !self.dialogo_pesca_mostrado {
                                    let dialogos = db.dialogos_por_contexto("pesca_bienvenida");
                                    if !dialogos.is_empty() {
                                        self.dialogo.iniciar_desde_db(dialogos);
                                        self.dialogo_pesca_mostrado = true;
                                        return;
                                    }
                                }
                                self.pesca.iniciar(db);
                            }
                            return;
                        }

                        // Zonas sin exploración
                        if self.escena.sin_exploracion() { return; }

// Foto (Z5_5 = zona_25: Aves Llamativas)
if self.escena.es_foto() {
    if !self.dialogo_foto_mostrado {
        let dialogos = db.dialogos_por_contexto("foto_bienvenida");
        if !dialogos.is_empty() {
            self.dialogo.iniciar_desde_db(dialogos);
            self.dialogo_foto_mostrado = true;
            return;
        }
    }
    // ✅ FILTRAR: Solo animales de categoría "aves"
    let todos = db.animales_zona(&self.escena);
    let animales: Vec<Animal> = todos
        .into_iter()
        .filter(|a| a.categoria == "aves")
        .collect();
    
    if animales.is_empty() { return; }
    let idx   = gen_range(0, animales.len());
    let celda = gen_range(0, 4_usize);
    self.modo = ModoVista::Foto {
        animales,
        indice_actual: idx,
        celda,
        foto_tomada: false,
        texto_pos: 0,
        timer: 0.0,
        terminado: false,
        ya_vistos: HashSet::new(),
    };
    return;
}

                        // Exploración normal
                        let animales = db.animales_zona(&self.escena);
                        if !animales.is_empty() {
                            self.modo = ModoVista::Seleccion { animales, indice: 0 };
                        }
                    }
                    Accion::BotonB => {
                        self.pantalla = Pantalla::Inicio;
                        self.inicio_seleccion = 0;
                    }
                    dir => {
                        let idx = match dir {
                            Accion::Arriba    => 0,
                            Accion::Abajo     => 1,
                            Accion::Izquierda => 2,
                            Accion::Derecha   => 3,
                            _ => return,
                        };
                        if let Some(destino) = self.escena.conexiones()[idx] {
                            self.cambiar_escena(destino);
                        }
                    }
                }
            }

            ModoVista::Seleccion { animales, indice } => {
                match accion {
                    Accion::Arriba => { if *indice > 0 { *indice -= 1; } }
                    Accion::Abajo  => { if *indice + 1 < animales.len() { *indice += 1; } }
                    Accion::BotonA => {
                        let animal = animales[*indice].clone();
                        let lista  = animales.clone();
                        let idx    = *indice;
                        self.libreta.registrar_animal(&animal);
                        self.save.marcar_animal_visto(&animal.nombre_comun);
                        self.save.guardar();
                        self.necesita_sonido_animal = true;
                        self.modo = ModoVista::ViendoAnimal {
                            animal, lista, indice_lista: idx,
                            texto_pos: 0, timer: 0.0, terminado: false,
                        };
                    }
                    Accion::BotonB => { self.modo = ModoVista::Normal; }
                    _ => {}
                }
            }

            ModoVista::ViendoAnimal {
                animal, lista, indice_lista, texto_pos, terminado, ..
            } => {
                match accion {
                    Accion::BotonB => {
                        let lista_c = lista.clone();
                        let idx = *indice_lista;
                        self.modo = ModoVista::Seleccion {
                            animales: lista_c,
                            indice: idx,
                        };
                    }
                    Accion::BotonA => {
                        if *terminado {
                            self.necesita_sonido_animal = true;
                        } else {
                            *texto_pos = animal.descripcion.chars().count();
                            *terminado = true;
                        }
                    }
                    _ => {}
                }
            }

            ModoVista::Foto {
                animales, indice_actual, celda, foto_tomada,
                texto_pos, timer, terminado, ya_vistos,
            } => {
                if !*foto_tomada {
                    match accion {
                        Accion::BotonA => {
                            *foto_tomada = true;
                            *timer = 0.0;
                            *texto_pos = 0;
                            *terminado = false;
                            ya_vistos.insert(*indice_actual);
                            let a = &animales[*indice_actual];
                            self.libreta.registrar_animal(a);
                            self.save.marcar_animal_visto(&a.nombre_comun);
                            self.save.guardar();
                            self.necesita_sonido_animal = true;
                        }
                        Accion::BotonB => {
                            self.modo = ModoVista::Normal;
                        }
                        _ => {}
                    }
                } else if *terminado {
                    let disponibles: Vec<usize> = (0..animales.len())
                        .filter(|i| !ya_vistos.contains(i))
                        .collect();
                    if disponibles.is_empty() {
                        self.info_overlay = Some(AnimalInfo::mensaje(
                            "¡Zona completada!",
                            &format!(
                                "Has explorado los {} animales de esta zona. ¡Excelente trabajo de campo!",
                                animales.len()
                            ),
                        ));
                        self.modo = ModoVista::Normal;
                    } else {
                        *indice_actual = disponibles[gen_range(0, disponibles.len())];
                        *celda        = gen_range(0, 4_usize);
                        *foto_tomada  = false;
                        *texto_pos    = 0;
                        *timer        = 0.0;
                        *terminado    = false;
                    }
                } else {
                    *texto_pos = animales[*indice_actual].descripcion.chars().count();
                    *terminado = true;
                }
            }
        }
    }
}