// src/libreta.rs
use crate::db::Animal;

// =====================================================================
//  ENTRADA DE LIBRETA
// =====================================================================

#[derive(Debug, Clone)]
pub struct EntradaLibreta {
    pub nombre: String,
    pub cientifico: String,
    pub zona: String,
    pub descripcion: String,
    pub categoria: String,
}

impl EntradaLibreta {
    pub fn desde_animal(animal: &Animal) -> Self {
        Self {
            nombre: animal.nombre_comun.clone(),
            cientifico: animal.nombre_cientifico.clone(),
            zona: animal.zona_id.clone(),
            descripcion: animal.descripcion.clone(),
            categoria: animal.categoria.clone(),
        }
    }
}

// =====================================================================
//  LIBRETA DE CAMPO
// =====================================================================

pub struct Libreta {
    pub entradas: Vec<EntradaLibreta>,
    pub pagina: usize,
}

impl Libreta {
    pub fn new() -> Self {
        Self {
            entradas: Vec::new(),
            pagina: 0,
        }
    }

    /// Registra un animal si no existe ya (sin duplicados)
    pub fn registrar_animal(&mut self, animal: &Animal) {
        if self.entradas.iter().any(|e| e.nombre == animal.nombre_comun) {
            return;
        }
        self.entradas.push(EntradaLibreta::desde_animal(animal));
        self.ordenar();
    }

    /// Ordena alfabéticamente por nombre común
    fn ordenar(&mut self) {
        self.entradas.sort_by(|a, b| a.nombre.cmp(&b.nombre));
    }

    // ── Paginación ───────────────────────────────────────────────────

    pub fn por_pagina() -> usize {
        5
    }

    pub fn total_paginas(&self) -> usize {
        let pp = Self::por_pagina();
        if self.entradas.is_empty() {
            1
        } else {
            (self.entradas.len() + pp - 1) / pp
        }
    }

    pub fn pagina_anterior(&mut self) {
        if self.pagina > 0 {
            self.pagina -= 1;
        }
    }

    pub fn pagina_siguiente(&mut self) {
        if self.pagina + 1 < self.total_paginas() {
            self.pagina += 1;
        }
    }

    pub fn entradas_pagina_actual(&self) -> &[EntradaLibreta] {
        let pp = Self::por_pagina();
        let inicio = self.pagina * pp;
        let fin = (inicio + pp).min(self.entradas.len());
        &self.entradas[inicio..fin]
    }

    // ── Consultas ────────────────────────────────────────────────────

    pub fn total_entradas(&self) -> usize {
        self.entradas.len()
    }

    pub fn contiene(&self, nombre: &str) -> bool {
        self.entradas.iter().any(|e| e.nombre == nombre)
    }

    /// Filtra entradas por categoría
    pub fn por_categoria(&self, categoria: &str) -> Vec<&EntradaLibreta> {
        self.entradas
            .iter()
            .filter(|e| e.categoria == categoria)
            .collect()
    }

    /// Estadísticas por categoría
    pub fn conteo_por_categoria(&self) -> Vec<(String, usize)> {
        let categorias = [
            "mamiferos", "aves", "reptiles", "peces",
            "anfibios", "primates", "insectos", "fosiles",
        ];
        categorias
            .iter()
            .map(|cat| {
                let count = self.entradas
                    .iter()
                    .filter(|e| e.categoria.as_str() == *cat)
                    .count();
                (cat.to_string(), count)
            })
            .filter(|(_, count)| *count > 0)
            .collect()
    }
}