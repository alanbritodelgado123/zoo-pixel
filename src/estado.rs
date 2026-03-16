use std::collections::HashSet;
use serde::{Deserialize, Serialize};
use macroquad::rand::gen_range;

use crate::config;
use crate::db::{Animal, ZooDB};
use crate::escena::Escena;
use crate::input::Accion;

pub enum ModoVista {
    Normal,
    Seleccion {
        animales: Vec<Animal>,
        indice: usize,
    },
    ViendoAnimal {
        animal: Animal,
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

pub struct Transicion {
    pub destino: Escena,
    pub timer: f32,
    pub duracion: f32,
    pub cambiada: bool,
}

pub struct Estado {
    pub escena: Escena,
    pub visitadas: HashSet<Escena>,
    pub transicion: Option<Transicion>,
    pub duracion_transicion: f32,
    pub modo: ModoVista,
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize)]
pub struct SaveData {
    pub escena: Escena,
    pub visitadas: HashSet<Escena>,
}

impl Estado {
    pub fn new() -> Self {
        let mut visitadas = HashSet::new();
        visitadas.insert(Escena::Entrada);
        Self {
            escena: Escena::Entrada,
            visitadas,
            transicion: None,
            duracion_transicion: config::TRANSITION_SECS_FALLBACK,
            modo: ModoVista::Normal,
        }
    }

    pub fn update(&mut self, dt: f32) {
        if let Some(ref mut t) = self.transicion {
            t.timer += dt;
            if t.timer >= t.duracion / 2.0 && !t.cambiada {
                self.escena = t.destino;
                self.visitadas.insert(t.destino);
                t.cambiada = true;
            }
            if t.timer >= t.duracion {
                self.transicion = None;
            }
        }

        match &mut self.modo {
            ModoVista::ViendoAnimal { animal, texto_pos, timer, terminado } => {
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
            ModoVista::Foto { animales, indice_actual, texto_pos, timer, terminado, foto_tomada, .. } => {
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

    pub fn cambiar_escena(&mut self, destino: Escena) {
        self.transicion = Some(Transicion {
            destino,
            timer: 0.0,
            duracion: self.duracion_transicion,
            cambiada: false,
        });
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

    pub fn procesar_accion(&mut self, accion: Accion, db: &ZooDB) {
        if self.en_transicion() { return; }

        match &mut self.modo {
            ModoVista::Normal => {
                match accion {
                    Accion::BotonA => {
                        if self.escena == Escena::Entrada { return; }
                        let animales = db.animales_zona(&self.escena);
                        if animales.is_empty() { return; }

                        if self.escena == Escena::Aviario {
                            let idx = gen_range(0, animales.len());
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
                        } else {
                            self.modo = ModoVista::Seleccion { animales, indice: 0 };
                        }
                    }
                    Accion::BotonB => {
                        if self.escena != Escena::Entrada {
                            self.cambiar_escena(Escena::Entrada);
                        }
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
                    Accion::Arriba => {
                        if *indice > 0 { *indice -= 1; }
                    }
                    Accion::Abajo => {
                        if *indice + 1 < animales.len() { *indice += 1; }
                    }
                    Accion::BotonA => {
                        let animal = animales[*indice].clone();
                        self.modo = ModoVista::ViendoAnimal {
                            animal,
                            texto_pos: 0,
                            timer: 0.0,
                            terminado: false,
                        };
                    }
                    Accion::BotonB => {
                        self.modo = ModoVista::Normal;
                    }
                    _ => {}
                }
            }

            ModoVista::ViendoAnimal { animal, texto_pos, terminado, .. } => {
                if *terminado {
                    self.modo = ModoVista::Normal;
                } else {
                    *texto_pos = animal.descripcion.chars().count();
                    *terminado = true;
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
                            *timer = 0.0;
                            *texto_pos = 0;
                            *terminado = false;
                            ya_vistos.insert(*indice_actual);
                        }
                        Accion::BotonB => {
                            self.modo = ModoVista::Normal;
                            return;
                        }
                        _ => {}
                    }
                } else if *terminado {
                    let disponibles: Vec<usize> = (0..animales.len())
                        .filter(|i| !ya_vistos.contains(i))
                        .collect();
                    if disponibles.is_empty() {
                        self.modo = ModoVista::Normal;
                    } else {
                        *indice_actual = disponibles[gen_range(0, disponibles.len())];
                        *celda = gen_range(0, 4_usize);
                        *foto_tomada = false;
                        *texto_pos = 0;
                        *timer = 0.0;
                        *terminado = false;
                    }
                } else {
                    *texto_pos = animales[*indice_actual].descripcion.chars().count();
                    *terminado = true;
                }
            }
        }
    }
}