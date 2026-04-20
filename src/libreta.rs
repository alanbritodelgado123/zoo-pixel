use crate::db::Animal;

#[derive(Debug, Clone)]
pub struct EntradaLibreta {
    pub nombre: String,
    pub cientifico: String,
    pub zona: String,
    pub descripcion: String,
    pub categoria: String,  // ✅ NUEVO: Necesario para mostrar iconos
}

pub struct Libreta {
    pub entradas: Vec<EntradaLibreta>,
    pub pagina: usize,
}

impl Libreta {
    pub fn new() -> Self {
        Self { 
            entradas: Vec::new(), 
            pagina: 0 
        }
    }

    pub fn registrar_animal(&mut self, animal: &Animal) {
        // Evitar duplicados
        if self.entradas.iter().any(|e| e.nombre == animal.nombre_comun) { 
            return; 
        }
        self.entradas.push(EntradaLibreta {
            nombre: animal.nombre_comun.clone(),
            cientifico: animal.nombre_cientifico.clone(),
            zona: animal.zona_id.clone(),
            descripcion: animal.descripcion.clone(),
            categoria: animal.categoria.clone(),  // ✅ NUEVO: Copiar categoría del animal
        });
        self.ordenar();
    }

    pub fn ordenar(&mut self) {
        // Ordenar alfabéticamente por nombre común
        self.entradas.sort_by(|a, b| a.nombre.cmp(&b.nombre));
    }

    pub fn pagina_anterior(&mut self) {
        if self.pagina > 0 { 
            self.pagina -= 1; 
        }
    }

    pub fn pagina_siguiente(&mut self) {
        let por_pagina = 5;
        let total_paginas = (self.entradas.len() + por_pagina - 1) / por_pagina;
        if self.pagina + 1 < total_paginas { 
            self.pagina += 1; 
        }
    }

    pub fn total_entradas(&self) -> usize {
        self.entradas.len()
    }

    pub fn total_paginas(&self) -> usize {
        let por_pagina = 5;
        (self.entradas.len() + por_pagina - 1) / por_pagina
    }
}