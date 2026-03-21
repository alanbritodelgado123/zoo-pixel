// src/estado.rs
use std::collections::HashSet;
use macroquad::rand::gen_range;

use crate::config;
use crate::db::{Animal, ZooDB};
use crate::escena::Escena;
use crate::input::Accion;
use crate::guia::{self, SistemaDialogo};
use crate::eventos::SistemaEventos;
use crate::ciclo_dia::{CicloDia, ModoCiclo};
use crate::libreta::Libreta;
use crate::minijuego::{MinijuegoPesca, MinijuegoMuseo, FasePesca, FaseMuseo};
use crate::plataforma::DetectorPlataforma;
use crate::save::SaveData;

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
    Seleccion { animales: Vec<Animal>, indice: usize },
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
}

impl MenuConfig {
    pub fn new(save: &SaveData) -> Self {
        Self {
            seleccion: 0,
            volumen_musica: save.config.volumen_musica,
            volumen_efectos: save.config.volumen_efectos,
        }
    }
    pub const OPCIONES: &'static [&'static str] = &[
        "Volumen Musica",
        "Volumen Efectos",
        "Volver",
    ];
}

pub struct Transicion {
    pub destino: Escena,
    pub timer: f32,
    pub duracion: f32,
    pub cambiada: bool,
}

pub struct Estado {
    pub pantalla: Pantalla,
    pub escena: Escena,
    pub visitadas: HashSet<Escena>,
    pub transicion: Option<Transicion>,
    pub duracion_transicion: f32,
    pub modo: ModoVista,

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
}

impl Estado {
    pub fn new() -> Self {
        let save = SaveData::cargar();
        let mut visitadas = save.visitadas.clone();
        let escena = save.escena.unwrap_or(Escena::EntradaPrincipal);
        visitadas.insert(escena);
        let menu_config = MenuConfig::new(&save);

        Self {
            pantalla: Pantalla::Inicio,
            escena,
            visitadas,
            transicion: None,
            duracion_transicion: config::TRANSITION_SECS_FALLBACK,
            modo: ModoVista::Normal,

            dialogo: SistemaDialogo::new(),
            eventos: SistemaEventos::new(),
            ciclo: CicloDia::new(config::CICLO_DIA_SECS),
            libreta: Libreta::new(),
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
        }
    }

    fn eventos_deben_bloquearse(&self) -> bool {
        matches!(self.pantalla,
            Pantalla::MapaCompleto | Pantalla::LibretaCompleta | Pantalla::Config |
            Pantalla::Inicio | Pantalla::Intro
        ) || self.dialogo.activo
          || self.pesca.activo
          || self.museo.activo
          || !matches!(self.modo, ModoVista::Normal)
          || self.en_transicion()
    }

    pub fn update(&mut self, dt: f32) {
        self.plataforma.update();

        if self.indicador_z_pressed > 0.0 { self.indicador_z_pressed -= dt * 3.0; }
        if self.indicador_x_pressed > 0.0 { self.indicador_x_pressed -= dt * 3.0; }

        match self.pantalla {
            Pantalla::Inicio => {
                self.inicio_timer += dt;
            }
            Pantalla::Intro => {
                self.intro_timer += dt;
                self.dialogo.update(dt);
                if !self.dialogo.activo && self.dialogo.completado {
                    self.pantalla = Pantalla::Juego;
                    self.guardar_estado();
                }
            }
            Pantalla::Juego => {
                self.ciclo.update(dt);

                self.eventos.bloqueado = self.eventos_deben_bloquearse();
                self.eventos.update(dt, &self.escena);

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
                self.pesca.update(dt);
                self.museo.update(dt);
            }
            Pantalla::Config => {}
            Pantalla::MapaCompleto => {}
            Pantalla::LibretaCompleta => {}
        }
    }

    fn update_typewriter(&mut self, dt: f32) {
        match &mut self.modo {
            ModoVista::ViendoAnimal { animal, texto_pos, timer, terminado, .. } => {
                if !*terminado {
                    *timer += dt;
                    let chars = (*timer * config::TYPEWRITER_CPS) as usize;
                    let total = animal.descripcion.chars().count();
                    if chars >= total { *texto_pos = total; *terminado = true; }
                    else { *texto_pos = chars; }
                }
            }
            ModoVista::Foto { animales, indice_actual, texto_pos, timer, terminado, foto_tomada, .. } => {
                if *foto_tomada && !*terminado {
                    *timer += dt;
                    let chars = (*timer * config::TYPEWRITER_CPS) as usize;
                    let total = animales[*indice_actual].descripcion.chars().count();
                    if chars >= total { *texto_pos = total; *terminado = true; }
                    else { *texto_pos = chars; }
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
        self.transicion = Some(Transicion {
            destino,
            timer: 0.0,
            duracion: self.duracion_transicion,
            cambiada: false,
        });
    }

    pub fn en_transicion(&self) -> bool { self.transicion.is_some() }

    pub fn alpha_transicion(&self) -> f32 {
        if let Some(ref t) = self.transicion {
            let p = (t.timer / t.duracion).clamp(0.0, 1.0);
            if p < 0.5 { p * 2.0 } else { (1.0 - p) * 2.0 }
        } else { 0.0 }
    }

    pub fn procesar_accion(&mut self, accion: Accion, db: &ZooDB) {
        if accion == Accion::BotonA { self.indicador_z_pressed = 1.0; }
        if accion == Accion::BotonB { self.indicador_x_pressed = 1.0; }

        match self.pantalla {
            Pantalla::Inicio => self.input_inicio(accion, db),
            Pantalla::Intro => self.input_intro(accion),
            Pantalla::Config => self.input_config(accion),
            Pantalla::MapaCompleto => self.input_mapa(accion),
            Pantalla::LibretaCompleta => self.input_libreta_completa(accion),
            Pantalla::Juego => {
                if self.dialogo.activo {
                    if accion == Accion::BotonA || accion == Accion::BotonB {
                        self.dialogo.avanzar();
                    }
                    return;
                }
                if self.eventos.hay_evento() {
                    match accion {
                        Accion::BotonA => self.eventos.interactuar(false),
                        Accion::BotonB => self.eventos.interactuar(true),
                        _ => {}
                    }
                    return;
                }
                if self.pesca.activo {
                    self.input_pesca(accion);
                    return;
                }
                if self.museo.activo {
                    self.input_museo(accion);
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
            Accion::BotonA => {
                match self.inicio_seleccion {
                    0 => {
                        self.ciclo.set_modo(ModoCiclo::Sistema);
                        self.iniciar_intro(db);
                    }
                    1 => {
                        self.ciclo.set_modo(ModoCiclo::DiaPermanente);
                        self.iniciar_intro(db);
                    }
                    2 => {
                        self.ciclo.set_modo(ModoCiclo::NochePermanente);
                        self.iniciar_intro(db);
                    }
                    3 => {
                        self.menu_config = MenuConfig::new(&self.save);
                        self.pantalla = Pantalla::Config;
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }

    fn iniciar_intro(&mut self, db: &ZooDB) {
        self.pantalla = Pantalla::Intro;
        self.intro_timer = 0.0;
        let dialogos = guia::dialogos_bienvenida_db(db, &self.plataforma);
        if !dialogos.is_empty() {
            self.dialogo.iniciar_desde_db(dialogos);
        } else {
            let fallback = guia::dialogos_bienvenida(&self.plataforma);
            self.dialogo.iniciar(fallback);
        }
    }

    fn input_intro(&mut self, accion: Accion) {
        if accion == Accion::BotonA || accion == Accion::BotonB {
            self.dialogo.avanzar();
        }
    }

    fn input_config(&mut self, accion: Accion) {
        let mc = &mut self.menu_config;
        match accion {
            Accion::Arriba => { if mc.seleccion > 0 { mc.seleccion -= 1; } }
            Accion::Abajo => { if mc.seleccion + 1 < MenuConfig::OPCIONES.len() { mc.seleccion += 1; } }
            Accion::Izquierda => {
                match mc.seleccion {
                    0 => mc.volumen_musica = (mc.volumen_musica - 0.1).max(0.0),
                    1 => mc.volumen_efectos = (mc.volumen_efectos - 0.1).max(0.0),
                    _ => {}
                }
            }
            Accion::Derecha => {
                match mc.seleccion {
                    0 => mc.volumen_musica = (mc.volumen_musica + 0.1).min(1.0),
                    1 => mc.volumen_efectos = (mc.volumen_efectos + 0.1).min(1.0),
                    _ => {}
                }
            }
            Accion::BotonA | Accion::BotonB => {
                if mc.seleccion == MenuConfig::OPCIONES.len() - 1 || accion == Accion::BotonB {
                    self.save.config.volumen_musica = mc.volumen_musica;
                    self.save.config.volumen_efectos = mc.volumen_efectos;
                    self.save.guardar();
                    self.pantalla = if self.dialogo.completado { Pantalla::Juego } else { Pantalla::Inicio };
                }
            }
            _ => {}
        }
    }

    fn input_mapa(&mut self, accion: Accion) {
        match accion {
            Accion::BotonB | Accion::Mapa => {
                self.pantalla = Pantalla::Juego;
            }
            Accion::Arriba | Accion::Abajo | Accion::Izquierda | Accion::Derecha => {
                let idx = match accion {
                    Accion::Arriba => 0, Accion::Abajo => 1,
                    Accion::Izquierda => 2, Accion::Derecha => 3,
                    _ => return,
                };
                if let Some(dest) = self.mapa_cursor.conexiones()[idx] {
                    self.mapa_cursor = dest;
                }
            }
            Accion::BotonA => {
                if self.visitadas.contains(&self.mapa_cursor) && self.mapa_cursor != self.escena {
                    self.cambiar_escena(self.mapa_cursor);
                    self.pantalla = Pantalla::Juego;
                }
            }
            _ => {}
        }
    }

    fn input_libreta_completa(&mut self, accion: Accion) {
        match accion {
            Accion::BotonB | Accion::Libreta => {
                self.pantalla = Pantalla::Juego;
            }
            Accion::Izquierda => self.libreta.pagina_anterior(),
            Accion::Derecha => self.libreta.pagina_siguiente(),
            _ => {}
        }
    }

    fn input_pesca(&mut self, accion: Accion) {
        match accion {
            Accion::BotonA => {
                match self.pesca.fase {
                    FasePesca::Picando => self.pesca.tirar(),
                    FasePesca::InfoPez => {
                        if self.pesca.texto_terminado {
                            self.pesca.siguiente_o_salir();
                        } else {
                            if let Some(ref pez) = self.pesca.pez_actual {
                                self.pesca.texto_pos = pez.descripcion.chars().count();
                                self.pesca.texto_terminado = true;
                            }
                        }
                    }
                    FasePesca::Resultado => self.pesca.siguiente_o_salir(),
                    _ => {}
                }
            }
            Accion::BotonB => {
                match self.pesca.fase {
                    FasePesca::InfoPez => self.pesca.siguiente_o_salir(),
                    _ => self.pesca.cerrar(),
                }
            }
            _ => {}
        }
    }

    fn input_museo(&mut self, accion: Accion) {
        match accion {
            Accion::BotonA => {
                match self.museo.fase {
                    FaseMuseo::Entrada => self.museo.entrar_explorando(),
                    FaseMuseo::Explorando => {
                        if self.museo.indice < self.museo.exhibiciones.len() {
                            self.museo.ver_exhibicion();
                        } else if self.museo.indice == self.museo.exhibiciones.len() {
                            self.museo.iniciar_excavacion();
                        } else {
                            self.museo.iniciar_quiz();
                        }
                    }
                    FaseMuseo::Excavando => {
                        self.museo.golpear();
                    }
                    FaseMuseo::FosilRevelado => {
                        if self.museo.terminado_texto {
                            self.museo.volver_explorar();
                        } else {
                            let desc = self.museo.dino_excavado.as_ref()
                                .map(|d| d.descripcion.chars().count()).unwrap_or(0);
                            self.museo.texto_pos = desc;
                            self.museo.terminado_texto = true;
                        }
                    }
                    FaseMuseo::ViendoExhibicion => {
                        if self.museo.terminado_texto {
                            self.museo.volver_explorar();
                        } else {
                            self.museo.texto_pos = self.museo.dino_actual().descripcion.chars().count();
                            self.museo.terminado_texto = true;
                        }
                    }
                    FaseMuseo::Quiz => {
                        if self.museo.quiz_respondida {
                            self.museo.siguiente_quiz();
                        } else {
                            self.museo.responder_quiz();
                        }
                    }
                }
            }
            Accion::BotonB => {
                match self.museo.fase {
                    FaseMuseo::Entrada => self.museo.cerrar(),
                    FaseMuseo::ViendoExhibicion => self.museo.volver_explorar(),
                    FaseMuseo::Explorando => self.museo.cerrar(),
                    FaseMuseo::Excavando => self.museo.volver_explorar(),
                    FaseMuseo::FosilRevelado => self.museo.volver_explorar(),
                    FaseMuseo::Quiz => {
                        if self.museo.quiz_respondida {
                            self.museo.siguiente_quiz();
                        } else {
                            self.museo.fase = FaseMuseo::Explorando;
                        }
                    }
                }
            }
            Accion::Arriba => {
                match self.museo.fase {
                    FaseMuseo::Explorando => {
                        if self.museo.indice > 0 { self.museo.indice -= 1; }
                    }
                    FaseMuseo::Excavando => {
                        self.museo.mover_cursor(0, -1);
                    }
                    FaseMuseo::Quiz if !self.museo.quiz_respondida => {
                        if self.museo.quiz_seleccion > 0 { self.museo.quiz_seleccion -= 1; }
                    }
                    _ => {}
                }
            }
            Accion::Abajo => {
                match self.museo.fase {
                    FaseMuseo::Explorando => {
                        let max = self.museo.exhibiciones.len() + 1;
                        if self.museo.indice < max {
                            self.museo.indice += 1;
                        }
                    }
                    FaseMuseo::Excavando => {
                        self.museo.mover_cursor(0, 1);
                    }
                    FaseMuseo::Quiz if !self.museo.quiz_respondida => {
                        if self.museo.quiz_seleccion + 1 < self.museo.quiz_opciones.len() {
                            self.museo.quiz_seleccion += 1;
                        }
                    }
                    _ => {}
                }
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
                        if self.escena == Escena::Acuario {
                            self.pesca.iniciar();
                            return;
                        }
                        if self.escena == Escena::Museo {
                            self.museo.iniciar();
                            return;
                        }
                        if self.escena.es_entrada() { return; }
                        let animales = db.animales_zona(&self.escena);
                        if animales.is_empty() { return; }
                        if self.escena.es_aviario() {
                            let idx = gen_range(0, animales.len());
                            let celda = gen_range(0, 4_usize);
                            self.modo = ModoVista::Foto {
                                animales, indice_actual: idx, celda,
                                foto_tomada: false, texto_pos: 0,
                                timer: 0.0, terminado: false,
                                ya_vistos: HashSet::new(),
                            };
                        } else {
                            self.modo = ModoVista::Seleccion { animales, indice: 0 };
                        }
                    }
                    Accion::BotonB => {
                        if !self.escena.es_entrada() {
                            self.cambiar_escena(Escena::EntradaPrincipal);
                        }
                    }
                    dir => {
                        let idx = match dir {
                            Accion::Arriba => 0, Accion::Abajo => 1,
                            Accion::Izquierda => 2, Accion::Derecha => 3,
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
                    Accion::Abajo => { if *indice + 1 < animales.len() { *indice += 1; } }
                    Accion::BotonA => {
                        let animal = animales[*indice].clone();
                        let lista = animales.clone();
                        let idx = *indice;
                        self.libreta.registrar_animal(&animal, &self.save);
                        self.save.marcar_animal_visto(&animal.nombre_comun);
                        self.save.guardar();
                        self.modo = ModoVista::ViendoAnimal {
                            animal, lista, indice_lista: idx,
                            texto_pos: 0, timer: 0.0, terminado: false,
                        };
                    }
                    Accion::BotonB => { self.modo = ModoVista::Normal; }
                    _ => {}
                }
            }

            ModoVista::ViendoAnimal { animal, lista, indice_lista, texto_pos, terminado, .. } => {
                match accion {
                    Accion::BotonB => {
                        let lista_c = lista.clone();
                        let idx = *indice_lista;
                        self.modo = ModoVista::Seleccion { animales: lista_c, indice: idx };
                    }
                    Accion::BotonA => {
                        if *terminado {
                            let lista_c = lista.clone();
                            let idx = *indice_lista;
                            self.modo = ModoVista::Seleccion { animales: lista_c, indice: idx };
                        } else {
                            *texto_pos = animal.descripcion.chars().count();
                            *terminado = true;
                        }
                    }
                    _ => {
                        if !*terminado {
                            *texto_pos = animal.descripcion.chars().count();
                            *terminado = true;
                        }
                    }
                }
            }

            ModoVista::Foto {
                animales, indice_actual, celda,
                foto_tomada, texto_pos, timer, terminado, ya_vistos
            } => {
                if !*foto_tomada {
                    match accion {
                        Accion::BotonA => {
                            *foto_tomada = true;
                            *timer = 0.0; *texto_pos = 0; *terminado = false;
                            ya_vistos.insert(*indice_actual);
                            let a = &animales[*indice_actual];
                            self.libreta.registrar_animal(a, &self.save);
                            self.save.marcar_animal_visto(&a.nombre_comun);
                            self.save.guardar();
                        }
                        Accion::BotonB => { self.modo = ModoVista::Normal; return; }
                        _ => {}
                    }
                } else if *terminado {
                    let disponibles: Vec<usize> = (0..animales.len())
                        .filter(|i| !ya_vistos.contains(i)).collect();
                    if disponibles.is_empty() {
                        self.modo = ModoVista::Normal;
                    } else {
                        *indice_actual = disponibles[gen_range(0, disponibles.len())];
                        *celda = gen_range(0, 4_usize);
                        *foto_tomada = false;
                        *texto_pos = 0; *timer = 0.0; *terminado = false;
                    }
                } else {
                    *texto_pos = animales[*indice_actual].descripcion.chars().count();
                    *terminado = true;
                }
            }
        }
    }
}