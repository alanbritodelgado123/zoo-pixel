// src/libreta.rs
use crate::db::Animal;
use crate::save::SaveData;

#[derive(Debug, Clone)]
pub struct EntradaLibreta {
    pub nombre: String,
    pub cientifico: String,
    pub zona: String,
}

pub struct Libreta {
    pub entradas: Vec<EntradaLibreta>,
    pub pagina: usize,
    pub abierta: bool, // Kept for compatibility but unused now
}

impl Libreta {
    pub fn new() -> Self {
        Self {
            entradas: Vec::new(),
            pagina: 0,
            abierta: false,
        }
    }

    pub fn registrar_animal(&mut self, animal: &Animal, _save: &SaveData) {
        // No duplicar
        if self.entradas.iter().any(|e| e.nombre == animal.nombre_comun) {
            return;
        }
        self.entradas.push(EntradaLibreta {
            nombre: animal.nombre_comun.clone(),
            cientifico: animal.nombre_cientifico.clone(),
            zona: animal.zona_id.clone(),
        });
    }

    pub fn toggle(&mut self) {
        self.abierta = !self.abierta;
    }

    pub fn pagina_anterior(&mut self) {
        if self.pagina > 0 { self.pagina -= 1; }
    }

    pub fn pagina_siguiente(&mut self) {
        let por_pagina = 6;
        let total_paginas = (self.entradas.len() + por_pagina - 1) / por_pagina;
        if self.pagina + 1 < total_paginas {
            self.pagina += 1;
        }
    }
}