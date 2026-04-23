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
            Self::E    => "Entrada Principal",
            Self::P1   => "Pasillo 1",
            Self::P2   => "Pasillo 2",
            Self::P3   => "Pasillo 3",
            Self::P4   => "Pasillo 4",
            Self::P5   => "Pasillo 5 - Acuario",
            Self::Z1_1 => "Zona 1: Llanos Centrales",
            Self::Z1_2 => "Zona 2: Cordillera de los Andes",
            Self::Z1_3 => "Zona 3: Selva Amazonica",
            Self::Z1_4 => "Zona 4: Sierra de Perija",
            Self::Z1_5 => "Zona 5: Peninsula de Paria",
            Self::Z2_1 => "Zona 6: Isla de Margarita",
            Self::Z2_2 => "Zona 7: Costa Caribe Oriental",
            Self::Z2_3 => "Zona 8: Delta del Orinoco",
            Self::Z2_4 => "Zona 9: Parque Nacional Canaima",
            Self::Z2_5 => "Zona 10: Serrania de la Neblina",
            Self::Z3_1 => "Zona 11: Cordillera de la Costa",
            Self::Z3_2 => "Zona 12: Lago de Maracaibo",
            Self::Z3_3 => "Zona 13: Sierra de San Luis",
            Self::Z3_4 => "Zona 14: Los Roques",
            Self::Z3_5 => "Zona 15: Rio Orinoco",
            Self::Z4_1 => "Zona 16: Selva de Imataca",
            Self::Z4_2 => "Zona 17: Paramos de Merida",
            Self::Z4_3 => "Zona 18: Llanos Occidentales",
            Self::Z4_4 => "Zona 19: Peninsula de la Guajira",
            Self::Z4_5 => "Zona 20: Cerro Yapacana",
            Self::Z5_1 => "Zona 21: Museo Paleontologia",
            Self::Z5_2 => "Zona 22: Peces de Pesca",
            Self::Z5_3 => "Zona 23: Reptiles y Anfibios",
            Self::Z5_4 => "Zona 24: Insectos",
            Self::Z5_5 => "Zona 25: Aves muy llamativas de Venezuela",
        }
    }

    pub fn db_id(&self) -> &'static str {
        match self {
            Self::E    => "entrada",
            Self::P1   => "pasillo_01",
            Self::P2   => "pasillo_02",
            Self::P3   => "pasillo_03",
            Self::P4   => "pasillo_04",
            Self::P5   => "pasillo_05",
            Self::Z1_1 => "zona_01",
            Self::Z1_2 => "zona_02",
            Self::Z1_3 => "zona_03",
            Self::Z1_4 => "zona_04",
            Self::Z1_5 => "zona_05",
            Self::Z2_1 => "zona_06",
            Self::Z2_2 => "zona_07",
            Self::Z2_3 => "zona_08",
            Self::Z2_4 => "zona_09",
            Self::Z2_5 => "zona_10",
            Self::Z3_1 => "zona_11",
            Self::Z3_2 => "zona_12",
            Self::Z3_3 => "zona_13",
            Self::Z3_4 => "zona_14",
            Self::Z3_5 => "zona_15",
            Self::Z4_1 => "zona_16",
            Self::Z4_2 => "zona_17",
            Self::Z4_3 => "zona_18",
            Self::Z4_4 => "zona_19",
            Self::Z4_5 => "zona_20",
            Self::Z5_1 => "zona_21",
            Self::Z5_2 => "zona_22",
            Self::Z5_3 => "zona_23",
            Self::Z5_4 => "zona_24",
            Self::Z5_5 => "zona_25",
        }
    }

    pub fn letra(&self) -> &'static str {
        match self {
            Self::E    => "E",
            Self::P1   => "P1", Self::P2  => "P2", Self::P3  => "P3",
            Self::P4   => "P4", Self::P5  => "P5",
            Self::Z1_1 => "1",  Self::Z1_2 => "2",  Self::Z1_3 => "3",
            Self::Z1_4 => "4",  Self::Z1_5 => "5",
            Self::Z2_1 => "6",  Self::Z2_2 => "7",  Self::Z2_3 => "8",
            Self::Z2_4 => "9",  Self::Z2_5 => "10",
            Self::Z3_1 => "11", Self::Z3_2 => "12", Self::Z3_3 => "13",
            Self::Z3_4 => "14", Self::Z3_5 => "15",
            Self::Z4_1 => "16", Self::Z4_2 => "17", Self::Z4_3 => "18",
            Self::Z4_4 => "19", Self::Z4_5 => "20",
            Self::Z5_1 => "21", Self::Z5_2 => "22", Self::Z5_3 => "23",
            Self::Z5_4 => "24", Self::Z5_5 => "25",
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
            Self::E    => (2, 0),
            Self::P1   => (0, 1), Self::P2 => (1, 1), Self::P3 => (2, 1),
            Self::P4   => (3, 1), Self::P5 => (4, 1),
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
            E  => [Some(P3), None, None, None],
            P1 => [Some(Z1_1), None, None, Some(P2)],
            P2 => [Some(Z2_1), None, Some(P1), Some(P3)],
            P3 => [Some(Z3_1), Some(E), Some(P2), Some(P4)],
            P4 => [Some(Z4_1), None, Some(P3), Some(P5)],
            P5 => [Some(Z5_1), None, Some(P4), None],

            Z1_1 => [Some(Z1_2), Some(P1), None, None],
            Z1_2 => [Some(Z1_3), Some(Z1_1), None, None],
            Z1_3 => [Some(Z1_4), Some(Z1_2), None, None],
            Z1_4 => [Some(Z1_5), Some(Z1_3), None, None],
            Z1_5 => [None,       Some(Z1_4), None, None],

            Z2_1 => [Some(Z2_2), Some(P2), None, None],
            Z2_2 => [Some(Z2_3), Some(Z2_1), None, None],
            Z2_3 => [Some(Z2_4), Some(Z2_2), None, None],
            Z2_4 => [Some(Z2_5), Some(Z2_3), None, None],
            Z2_5 => [None,       Some(Z2_4), None, None],

            Z3_1 => [Some(Z3_2), Some(P3), None, None],
            Z3_2 => [Some(Z3_3), Some(Z3_1), None, None],
            Z3_3 => [Some(Z3_4), Some(Z3_2), None, None],
            Z3_4 => [Some(Z3_5), Some(Z3_3), None, None],
            Z3_5 => [None,       Some(Z3_4), None, None],

            Z4_1 => [Some(Z4_2), Some(P4), None, None],
            Z4_2 => [Some(Z4_3), Some(Z4_1), None, None],
            Z4_3 => [Some(Z4_4), Some(Z4_2), None, None],
            Z4_4 => [Some(Z4_5), Some(Z4_3), None, None],
            Z4_5 => [None,       Some(Z4_4), None, None],

            Z5_1 => [Some(Z5_2), Some(P5), None, None],
            Z5_2 => [Some(Z5_3), Some(Z5_1), None, None],
            Z5_3 => [Some(Z5_4), Some(Z5_2), None, None],
            Z5_4 => [Some(Z5_5), Some(Z5_3), None, None],
            Z5_5 => [None,       Some(Z5_4), None, None],
        }
    }

    pub fn color_fondo(&self) -> Color {
        match self.tipo() {
            TipoZona::Entrada  => Color::new(0.05, 0.05, 0.08, 1.0),
            TipoZona::Pasillo  => Color::new(0.07, 0.06, 0.04, 1.0),
            TipoZona::Zona => {
                let h = self.db_id().bytes()
                    .fold(0u32, |a, b| a.wrapping_mul(31).wrapping_add(b as u32));
                let r = 0.03 + (h % 5) as f32 * 0.01;
                let g = 0.04 + ((h / 5) % 5) as f32 * 0.01;
                let b = 0.03 + ((h / 25) % 5) as f32 * 0.01;
                Color::new(r, g, b, 1.0)
            }
        }
    }

    pub fn es_solo_entrada(&self) -> bool {
        *self == Escena::E
    }

    pub fn es_entrada(&self) -> bool {
        matches!(self.tipo(), TipoZona::Entrada)
    }

    pub fn es_pasillo(&self) -> bool {
        matches!(self.tipo(), TipoZona::Pasillo)
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

    pub fn sin_exploracion(&self) -> bool {
        match self {
            Escena::E => true,
            Escena::P1 | Escena::P2 | Escena::P3 | Escena::P4 => true,
            _ => false,
        }
    }

    pub fn icono_categoria(&self) -> &'static str {
        match self {
            Self::Z5_1                          => "fosiles",
            Self::Z2_4 | Self::Z4_5            => "anfibios",
            Self::Z5_4                          => "insectos",
            Self::Z1_4 | Self::Z2_3
            | Self::Z3_1 | Self::Z3_4          => "primates",
            Self::Z2_3 | Self::Z2_1            => "mamiferos",
            Self::Z5_5                          => "reptiles",
            Self::Z5_3 | Self::Z3_5            => "peces",
            Self::Z5_2 | Self::Z2_2 | Self::Z2_5
            | Self::Z3_3 | Self::Z3_2          => "aves",
            _                                   => "mamiferos",
        }
    }
}