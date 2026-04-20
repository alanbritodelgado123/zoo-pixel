use macroquad::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Escena {
    E,
    P1, P2, P3, P4, P5,
    Z1_1, Z1_2, Z1_3, Z1_4, Z1_5,
    Z2_1, Z2_2, Z2_3, Z2_4, Z2_5,
    Z3_1, Z3_2, Z3_3, Z3_4, Z3_5,
    Z4_1, Z4_2, Z4_3, Z4_4, Z4_5,
    Z5_1, Z5_2, Z5_3, Z5_4, Z5_5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TipoZona {
    Entrada,
    Pasillo,
    Zona,
}

impl Escena {
    pub const TODAS: &'static [Escena] = &[
        Escena::E,
        Escena::P1, Escena::P2, Escena::P3, Escena::P4, Escena::P5,
        Escena::Z1_1, Escena::Z1_2, Escena::Z1_3, Escena::Z1_4, Escena::Z1_5,
        Escena::Z2_1, Escena::Z2_2, Escena::Z2_3, Escena::Z2_4, Escena::Z2_5,
        Escena::Z3_1, Escena::Z3_2, Escena::Z3_3, Escena::Z3_4, Escena::Z3_5,
        Escena::Z4_1, Escena::Z4_2, Escena::Z4_3, Escena::Z4_4, Escena::Z4_5,
        Escena::Z5_1, Escena::Z5_2, Escena::Z5_3, Escena::Z5_4, Escena::Z5_5,
    ];

    pub fn nombre(&self) -> &'static str {
        match self {
            Self::E => "Entrada Principal",
            Self::P1 => "Pasillo 1",
            Self::P2 => "Pasillo 2",
            Self::P3 => "Pasillo 3",
            Self::P4 => "Pasillo 4",
            Self::P5 => "Pasillo 5 - Acuario",
            Self::Z1_1 => "Llanos Centrales - Chigüire",
            Self::Z1_2 => "Llanos Centrales - Aves",
            Self::Z1_3 => "Llanos Centrales - Peces",
            Self::Z1_4 => "Llanos Centrales - Reptiles",
            Self::Z1_5 => "Llanos Occidentales",
            Self::Z2_1 => "Cordillera de los Andes",
            Self::Z2_2 => "Andes - Aves",
            Self::Z2_3 => "Cordillera de la Costa",
            Self::Z2_4 => "Páramos de Mérida",
            Self::Z2_5 => "Sierra de San Luis",
            Self::Z3_1 => "Selva Amazónica",
            Self::Z3_2 => "Amazonas - Insectos",
            Self::Z3_3 => "Serranía de la Neblina",
            Self::Z3_4 => "Selva de Imataca",
            Self::Z3_5 => "Cerro Yapacana",
            Self::Z4_1 => "Sierra de Perijá",
            Self::Z4_2 => "Parque Nacional Canaima",
            Self::Z4_3 => "Delta del Orinoco I",
            Self::Z4_4 => "Delta del Orinoco II",
            Self::Z4_5 => "Río Orinoco",
            Self::Z5_1 => "Península de Paria - Museo",
            Self::Z5_2 => "Isla de Margarita",
            Self::Z5_3 => "Costa Caribe Oriental",
            Self::Z5_4 => "Los Roques",
            Self::Z5_5 => "Lago de Maracaibo - Aviario",
        }
    }

    pub fn db_id(&self) -> &'static str {
        match self {
            Self::E => "entrada",
            Self::P1 => "p1", Self::P2 => "p2", Self::P3 => "p3",
            Self::P4 => "p4", Self::P5 => "p5",
            Self::Z1_1 => "z1_1", Self::Z1_2 => "z1_2", Self::Z1_3 => "z1_3",
            Self::Z1_4 => "z1_4", Self::Z1_5 => "z1_5",
            Self::Z2_1 => "z2_1", Self::Z2_2 => "z2_2", Self::Z2_3 => "z2_3",
            Self::Z2_4 => "z2_4", Self::Z2_5 => "z2_5",
            Self::Z3_1 => "z3_1", Self::Z3_2 => "z3_2", Self::Z3_3 => "z3_3",
            Self::Z3_4 => "z3_4", Self::Z3_5 => "z3_5",
            Self::Z4_1 => "z4_1", Self::Z4_2 => "z4_2", Self::Z4_3 => "z4_3",
            Self::Z4_4 => "z4_4", Self::Z4_5 => "z4_5",
            Self::Z5_1 => "z5_1", Self::Z5_2 => "z5_2", Self::Z5_3 => "z5_3",
            Self::Z5_4 => "z5_4", Self::Z5_5 => "z5_5",
        }
    }

    pub fn letra(&self) -> &'static str {
        match self {
            Self::E => "E",
            Self::P1 => "P1", Self::P2 => "P2", Self::P3 => "P3",
            Self::P4 => "P4", Self::P5 => "P5",
            Self::Z1_1 => "L1", Self::Z1_2 => "L2", Self::Z1_3 => "L3",
            Self::Z1_4 => "L4", Self::Z1_5 => "L5",
            Self::Z2_1 => "A1", Self::Z2_2 => "A2", Self::Z2_3 => "A3",
            Self::Z2_4 => "A4", Self::Z2_5 => "A5",
            Self::Z3_1 => "S1", Self::Z3_2 => "S2", Self::Z3_3 => "S3",
            Self::Z3_4 => "S4", Self::Z3_5 => "S5",
            Self::Z4_1 => "P1", Self::Z4_2 => "P2", Self::Z4_3 => "P3",
            Self::Z4_4 => "P4", Self::Z4_5 => "P5",
            Self::Z5_1 => "C1", Self::Z5_2 => "C2", Self::Z5_3 => "C3",
            Self::Z5_4 => "C4", Self::Z5_5 => "C5",
        }
    }

    pub fn tipo(&self) -> TipoZona {
        match self {
            Self::E => TipoZona::Entrada,
            Self::P1 | Self::P2 | Self::P3 | Self::P4 | Self::P5 => TipoZona::Pasillo,
            _ => TipoZona::Zona,
        }
    }

    pub fn pos_mapa(&self) -> (usize, usize) {
        match self {
            Self::E => (2, 0),
            Self::P1 => (0, 1), Self::P2 => (1, 1), Self::P3 => (2, 1),
            Self::P4 => (3, 1), Self::P5 => (4, 1),
            Self::Z1_1 => (0, 2), Self::Z1_2 => (0, 3), Self::Z1_3 => (0, 4),
            Self::Z1_4 => (0, 5), Self::Z1_5 => (0, 6),
            Self::Z2_1 => (1, 2), Self::Z2_2 => (1, 3), Self::Z2_3 => (1, 4),
            Self::Z2_4 => (1, 5), Self::Z2_5 => (1, 6),
            Self::Z3_1 => (2, 2), Self::Z3_2 => (2, 3), Self::Z3_3 => (2, 4),
            Self::Z3_4 => (2, 5), Self::Z3_5 => (2, 6),
            Self::Z4_1 => (3, 2), Self::Z4_2 => (3, 3), Self::Z4_3 => (3, 4),
            Self::Z4_4 => (3, 5), Self::Z4_5 => (3, 6),
            Self::Z5_1 => (4, 2), Self::Z5_2 => (4, 3), Self::Z5_3 => (4, 4),
            Self::Z5_4 => (4, 5), Self::Z5_5 => (4, 6),
        }
    }

    pub fn conexiones(&self) -> [Option<Escena>; 4] {
        use Escena::*;
        match self {
            // E solo conecta con P3 (arriba)
            E => [Some(P3), None, None, None],
            
            // ✅ CORREGIDO: Solo P3 conecta con E
            // [Arriba, Abajo, Izquierda, Derecha]
            P1 => [Some(Z1_1), None, None, Some(P2)],
            P2 => [Some(Z2_1), None, Some(P1), Some(P3)],
            P3 => [Some(Z3_1), Some(E), Some(P2), Some(P4)],  // ← ÚNICO con acceso a E
            P4 => [Some(Z4_1), None, Some(P3), Some(P5)],
            P5 => [Some(Z5_1), None, Some(P4), None],
            
            // Zonas (todas igual)
            Z1_1 => [Some(Z1_2), Some(P1), None, None],
            Z1_2 => [Some(Z1_3), Some(Z1_1), None, None],
            Z1_3 => [Some(Z1_4), Some(Z1_2), None, None],
            Z1_4 => [Some(Z1_5), Some(Z1_3), None, None],
            Z1_5 => [None, Some(Z1_4), None, None],
            Z2_1 => [Some(Z2_2), Some(P2), None, None],
            Z2_2 => [Some(Z2_3), Some(Z2_1), None, None],
            Z2_3 => [Some(Z2_4), Some(Z2_2), None, None],
            Z2_4 => [Some(Z2_5), Some(Z2_3), None, None],
            Z2_5 => [None, Some(Z2_4), None, None],
            Z3_1 => [Some(Z3_2), Some(P3), None, None],
            Z3_2 => [Some(Z3_3), Some(Z3_1), None, None],
            Z3_3 => [Some(Z3_4), Some(Z3_2), None, None],
            Z3_4 => [Some(Z3_5), Some(Z3_3), None, None],
            Z3_5 => [None, Some(Z3_4), None, None],
            Z4_1 => [Some(Z4_2), Some(P4), None, None],
            Z4_2 => [Some(Z4_3), Some(Z4_1), None, None],
            Z4_3 => [Some(Z4_4), Some(Z4_2), None, None],
            Z4_4 => [Some(Z4_5), Some(Z4_3), None, None],
            Z4_5 => [None, Some(Z4_4), None, None],
            Z5_1 => [Some(Z5_2), Some(P5), None, None],
            Z5_2 => [Some(Z5_3), Some(Z5_1), None, None],
            Z5_3 => [Some(Z5_4), Some(Z5_2), None, None],
            Z5_4 => [Some(Z5_5), Some(Z5_3), None, None],
            Z5_5 => [None, Some(Z5_4), None, None],
        }
    }

    pub fn color_fondo(&self) -> Color {
        match self.tipo() {
            TipoZona::Entrada => Color::new(0.05, 0.05, 0.08, 1.0),
            TipoZona::Pasillo => Color::new(0.07, 0.06, 0.04, 1.0),
            TipoZona::Zona => {
                let h = self.db_id().bytes().fold(0u32, |a, b| a.wrapping_mul(31).wrapping_add(b as u32));
                let r = 0.03 + (h % 5) as f32 * 0.01;
                let g = 0.04 + ((h / 5) % 5) as f32 * 0.01;
                let b = 0.03 + ((h / 25) % 5) as f32 * 0.01;
                Color::new(r, g, b, 1.0)
            }
        }
    }

    pub fn es_entrada(&self) -> bool {
        matches!(self.tipo(), TipoZona::Entrada | TipoZona::Pasillo)
    }

    pub fn es_pesca(&self) -> bool {
        *self == Escena::P5
    }

    pub fn es_museo(&self) -> bool {
        *self == Escena::Z5_1
    }

    pub fn es_foto(&self) -> bool {
        *self == Escena::Z5_5
    }



pub fn icono_categoria(&self) -> &'static str {
    match self {
        // ✅ ESPECÍFICOS PRIMERO (antes que los generales)
        Self::Z5_1 => "fosiles",
        Self::Z3_5 | Self::Z4_2 => "anfibios",
        Self::Z3_2 => "insectos",
        Self::Z2_3 | Self::Z3_1 | Self::Z3_4 | Self::Z4_1 | Self::Z4_3 => "primates",
        
        // ✅ LUEGO LOS GENERALES
        Self::Z1_4 | Self::Z5_4 => "reptiles",
        Self::Z1_3 | Self::Z4_5 => "peces",
        Self::Z1_2 | Self::Z2_2 | Self::Z3_3 | Self::Z4_4 | Self::Z5_3 | Self::Z5_5 => "aves",
        
        // ✅ DEFAULT AL FINAL
        Self::Z1_1 | Self::Z1_5 | Self::Z2_1 | Self::Z2_5 | 
        Self::Z4_2 | Self::Z4_3 | Self::Z5_2 => "mamiferos",
        
        _ => "mamiferos",
    }
}
}