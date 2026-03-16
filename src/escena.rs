use macroquad::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Escena {
    Entrada,
    Sabana,
    Laguna,
    Aviario,
    Felinos,
    Reptiliario,
    Primates,
    Montana,
    Humedal,
    Nocturario,
}

impl Escena {
    pub const TODAS: &'static [Escena] = &[
        Escena::Entrada,   Escena::Sabana,    Escena::Laguna,
        Escena::Aviario,   Escena::Felinos,   Escena::Reptiliario,
        Escena::Primates,  Escena::Montana,   Escena::Humedal,
        Escena::Nocturario,
    ];

    pub fn nombre(&self) -> &'static str {
        match self {
            Self::Entrada     => "Entrada",
            Self::Sabana      => "Sabana",
            Self::Laguna      => "Laguna",
            Self::Aviario     => "Aviario",
            Self::Felinos     => "Felinos",
            Self::Reptiliario => "Reptiliario",
            Self::Primates    => "Primates",
            Self::Montana     => "Montaña",
            Self::Humedal     => "Humedal",
            Self::Nocturario  => "Nocturario",
        }
    }

    pub fn db_id(&self) -> &'static str {
        match self {
            Self::Entrada     => "entrada",
            Self::Sabana      => "sabana",
            Self::Laguna      => "laguna",
            Self::Aviario     => "aviario",
            Self::Felinos     => "felinos",
            Self::Reptiliario => "reptiliario",
            Self::Primates    => "primates",
            Self::Montana     => "montana",
            Self::Humedal     => "humedal",
            Self::Nocturario  => "nocturario",
        }
    }

    pub fn color_fondo(&self) -> Color {
        match self {
            Self::Entrada     => Color::new(0.05, 0.05, 0.08, 1.0),
            Self::Sabana      => Color::new(0.08, 0.06, 0.02, 1.0),
            Self::Laguna      => Color::new(0.02, 0.06, 0.08, 1.0),
            Self::Aviario     => Color::new(0.02, 0.08, 0.04, 1.0),
            Self::Felinos     => Color::new(0.04, 0.06, 0.02, 1.0),
            Self::Reptiliario => Color::new(0.06, 0.04, 0.02, 1.0),
            Self::Primates    => Color::new(0.04, 0.05, 0.02, 1.0),
            Self::Montana     => Color::new(0.04, 0.05, 0.07, 1.0),
            Self::Humedal     => Color::new(0.03, 0.05, 0.04, 1.0),
            Self::Nocturario  => Color::new(0.03, 0.02, 0.05, 1.0),
        }
    }

    pub fn conexiones(&self) -> [Option<Escena>; 4] {
        use Escena::*;
        match self {
            Entrada     => [Some(Sabana),      None,              None,               None             ],
            Sabana      => [Some(Felinos),     Some(Entrada),     Some(Laguna),       Some(Aviario)    ],
            Laguna      => [Some(Reptiliario), None,              None,               Some(Sabana)     ],
            Aviario     => [Some(Primates),    None,              Some(Sabana),       None             ],
            Felinos     => [Some(Montana),     Some(Sabana),      Some(Reptiliario),  Some(Primates)   ],
            Reptiliario => [Some(Humedal),     Some(Laguna),      None,               Some(Felinos)    ],
            Primates    => [Some(Nocturario),  Some(Aviario),     Some(Felinos),      None             ],
            Montana     => [None,              Some(Felinos),     Some(Humedal),      Some(Nocturario) ],
            Humedal     => [None,              Some(Reptiliario), None,               Some(Montana)    ],
            Nocturario  => [None,              Some(Primates),    Some(Montana),      None             ],
        }
    }

    pub fn letra(&self) -> &'static str {
        match self {
            Self::Entrada     => "E",
            Self::Sabana      => "S",
            Self::Laguna      => "L",
            Self::Aviario     => "A",
            Self::Felinos     => "F",
            Self::Reptiliario => "R",
            Self::Primates    => "P",
            Self::Montana     => "M",
            Self::Humedal     => "H",
            Self::Nocturario  => "N",
        }
    }

    pub fn pos_mapa(&self) -> (usize, usize) {
        match self {
            Self::Humedal     => (0, 0),
            Self::Montana     => (1, 0),
            Self::Nocturario  => (2, 0),
            Self::Reptiliario => (0, 1),
            Self::Felinos     => (1, 1),
            Self::Primates    => (2, 1),
            Self::Laguna      => (0, 2),
            Self::Sabana      => (1, 2),
            Self::Aviario     => (2, 2),
            Self::Entrada     => (1, 3),
        }
    }
}