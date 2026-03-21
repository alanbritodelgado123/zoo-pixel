use macroquad::prelude::*;
use serde::{Deserialize, Serialize};

/// Todas las zonas del nuevo mapa del zoológico.
/// Convención: filas 0 (arriba) a 5 (abajo), columnas 0 a 5.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Escena {
    EntradaPrincipal,
    EntradaSecundaria,
    AreaCentral,
    A1, A2, A3,
    B1, B2, B3, B4, B5,
    C1, C2, C3,
    Acuario,
    Museo,
    Aviario,
    D1, D2, D3, D4,
}

/// Tipo lógico de la zona (para UI / lógica especial)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TipoZona {
    Entrada,
    AreaEspecial,
    Zona,
    AtraccionEspecial,
}

impl Escena {
    pub const TODAS: &'static [Escena] = &[
        Escena::EntradaPrincipal,
        Escena::EntradaSecundaria,
        Escena::AreaCentral,
        Escena::A1, Escena::A2, Escena::A3,
        Escena::B1, Escena::B2, Escena::B3, Escena::B4, Escena::B5,
        Escena::C1, Escena::C2, Escena::C3,
        Escena::Acuario,
        Escena::Museo,
        Escena::Aviario,
        Escena::D1, Escena::D2, Escena::D3, Escena::D4,
    ];

    pub fn nombre(&self) -> &'static str {
        match self {
            Self::EntradaPrincipal  => "Entrada Principal",
            Self::EntradaSecundaria => "Entrada Secundaria",
            Self::AreaCentral       => "Área Central",
            Self::A1 => "Zona A1",
            Self::A2 => "Zona A2",
            Self::A3 => "Zona A3",
            Self::B1 => "Zona B1",
            Self::B2 => "Zona B2",
            Self::B3 => "Zona B3",
            Self::B4 => "Zona B4",
            Self::B5 => "Zona B5",
            Self::C1 => "Zona C1",
            Self::C2 => "Zona C2",
            Self::C3 => "Zona C3",
            Self::Acuario => "Acuario",
            Self::Museo   => "Museo",
            Self::Aviario  => "Aviario",
            Self::D1 => "Zona D1",
            Self::D2 => "Zona D2",
            Self::D3 => "Zona D3",
            Self::D4 => "Zona D4",
        }
    }

    /// ID para la base de datos (snake_case, sin tildes)
    pub fn db_id(&self) -> &'static str {
        match self {
            Self::EntradaPrincipal  => "entrada_principal",
            Self::EntradaSecundaria => "entrada_secundaria",
            Self::AreaCentral       => "area_central",
            Self::A1 => "a1", Self::A2 => "a2", Self::A3 => "a3",
            Self::B1 => "b1", Self::B2 => "b2", Self::B3 => "b3",
            Self::B4 => "b4", Self::B5 => "b5",
            Self::C1 => "c1", Self::C2 => "c2", Self::C3 => "c3",
            Self::Acuario => "acuario",
            Self::Museo   => "museo",
            Self::Aviario  => "aviario",
            Self::D1 => "d1", Self::D2 => "d2", Self::D3 => "d3", Self::D4 => "d4",
        }
    }

    /// Letra corta para el minimapa
    pub fn letra(&self) -> &'static str {
        match self {
            Self::EntradaPrincipal  => "EP",
            Self::EntradaSecundaria => "ES",
            Self::AreaCentral       => "AC",
            Self::A1 => "A1", Self::A2 => "A2", Self::A3 => "A3",
            Self::B1 => "B1", Self::B2 => "B2", Self::B3 => "B3",
            Self::B4 => "B4", Self::B5 => "B5",
            Self::C1 => "C1", Self::C2 => "C2", Self::C3 => "C3",
            Self::Acuario => "Aq",
            Self::Museo   => "Mu",
            Self::Aviario  => "Av",
            Self::D1 => "D1", Self::D2 => "D2", Self::D3 => "D3", Self::D4 => "D4",
        }
    }

    pub fn tipo(&self) -> TipoZona {
        match self {
            Self::EntradaPrincipal | Self::EntradaSecundaria => TipoZona::Entrada,
            Self::AreaCentral => TipoZona::AreaEspecial,
            Self::Acuario | Self::Museo | Self::Aviario => TipoZona::AtraccionEspecial,
            _ => TipoZona::Zona,
        }
    }

    /// Posición en la grilla del mapa: (columna, fila)
    /// Grilla de 6 columnas × 6 filas
    pub fn pos_mapa(&self) -> (usize, usize) {
        match self {
            Self::EntradaPrincipal  => (2, 5),
            Self::EntradaSecundaria => (3, 0),
            Self::AreaCentral       => (2, 4),
            Self::A1 => (1, 4),
            Self::A2 => (3, 4),
            Self::A3 => (4, 4),
            Self::B1 => (0, 3),
            Self::B2 => (1, 3),
            Self::B3 => (2, 3),
            Self::B4 => (3, 3),
            Self::B5 => (5, 3),
            Self::C1 => (0, 2),
            Self::Acuario => (1, 2),
            Self::Museo   => (2, 2),
            Self::C2 => (3, 2),
            Self::C3 => (4, 2),
            Self::Aviario => (4, 3),
            Self::D1 => (0, 1),
            Self::D2 => (1, 1),
            Self::D3 => (2, 1),
            Self::D4 => (3, 1),
        }
    }

    /// Conexiones: [Arriba, Abajo, Izquierda, Derecha]
    /// Derivadas del grafo bidireccional según posiciones relativas.
    pub fn conexiones(&self) -> [Option<Escena>; 4] {
        use Escena::*;
        match self {
            //                         Arriba              Abajo                 Izquierda          Derecha
            EntradaPrincipal  => [Some(AreaCentral),       None,                 None,              None             ],
            EntradaSecundaria => [None,                    Some(D4),             None,              None             ],
            AreaCentral       => [Some(B3),                Some(EntradaPrincipal),Some(A1),         Some(A2)         ],

            A1 => [Some(B2),     None,            None,            Some(AreaCentral)],
            A2 => [None,         None,            Some(AreaCentral), Some(A3)       ],
            A3 => [None,         None,            Some(A2),        None             ],

            B1 => [Some(C1),     None,            None,            Some(B2)         ],
            B2 => [Some(Acuario),Some(A1),        Some(B1),        Some(B3)         ],
            B3 => [Some(Museo),  Some(AreaCentral),Some(B2),       Some(B4)         ],
            B4 => [Some(C2),     None,            Some(B3),        Some(Aviario)    ],
            B5 => [None,         None,            Some(Aviario),   None             ],

            C1 => [Some(D1),     Some(B1),        None,            Some(Acuario)    ],
            Acuario => [Some(D2),Some(B2),        Some(C1),        None             ],
            Museo   => [Some(D3),Some(B3),        None,            None             ],
            C2 => [Some(D4),     Some(B4),        None,            Some(C3)         ],
            C3 => [None,         Some(Aviario),   Some(C2),        None             ],
            Aviario => [Some(C3),None,            Some(B4),        Some(B5)         ],

            D1 => [None,         Some(C1),        None,            Some(D2)         ],
            D2 => [None,         Some(Acuario),   Some(D1),        Some(D3)         ],
            D3 => [None,         Some(Museo),     Some(D2),        Some(D4)         ],
            D4 => [Some(EntradaSecundaria), Some(C2), Some(D3),    None             ],
        }
    }

    /// Color de fondo fallback cuando no hay textura
    pub fn color_fondo(&self) -> Color {
        match self.tipo() {
            TipoZona::Entrada         => Color::new(0.05, 0.05, 0.08, 1.0),
            TipoZona::AreaEspecial     => Color::new(0.07, 0.06, 0.04, 1.0),
            TipoZona::AtraccionEspecial => Color::new(0.03, 0.06, 0.08, 1.0),
            TipoZona::Zona => {
                // Variar un poco según hash del db_id
                let h = self.db_id().bytes().fold(0u32, |a, b| a.wrapping_mul(31).wrapping_add(b as u32));
                let r = 0.03 + (h % 5) as f32 * 0.01;
                let g = 0.04 + ((h / 5) % 5) as f32 * 0.01;
                let b = 0.03 + ((h / 25) % 5) as f32 * 0.01;
                Color::new(r, g, b, 1.0)
            }
        }
    }

    /// ¿Es una entrada donde no se muestran animales?
    pub fn es_entrada(&self) -> bool {
        matches!(self, Self::EntradaPrincipal | Self::EntradaSecundaria | Self::AreaCentral)
    }

    /// ¿Es el aviario (modo foto)?
    pub fn es_aviario(&self) -> bool {
        matches!(self, Self::Aviario)
    }
}