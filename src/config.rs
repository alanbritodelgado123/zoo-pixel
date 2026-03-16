use macroquad::prelude::*;

pub const TRANSITION_SECS_FALLBACK: f32 = 0.5;
pub const TRANSITION_MIN: f32 = 0.3;
pub const BTN_RATIO: f32 = 0.14;
pub const BTN_PAD: f32 = 15.0;
pub const BAR_HEIGHT: f32 = 42.0;
pub const TYPEWRITER_CPS: f32 = 35.0;

// Tamaños de fuente
pub const FONT_BAR: u16 = 24;
pub const FONT_SELECT_NAME: u16 = 28;
pub const FONT_SELECT_SCI: u16 = 18;
pub const FONT_ANIMAL_NAME: u16 = 32;
pub const FONT_ANIMAL_SCI: u16 = 22;
pub const FONT_ANIMAL_DESC: u16 = 24;
pub const FONT_HINT: u16 = 18;

// Colores de la paleta de 48
pub const COLOR_TEXT:      Color = Color::new(0.835, 0.827, 0.757, 1.0); // #d5d3c1
pub const COLOR_ACCENT:    Color = Color::new(0.765, 0.678, 0.478, 1.0); // #c3ad7a
pub const COLOR_BG_DARK:   Color = Color::new(0.122, 0.098, 0.098, 1.0); // #1f1919
pub const COLOR_HIGHLIGHT: Color = Color::new(0.435, 0.718, 0.671, 1.0); // #6fb7ab
pub const COLOR_GREEN:     Color = Color::new(0.243, 0.471, 0.231, 1.0); // #3e783b
pub const COLOR_DIM:       Color = Color::new(0.322, 0.290, 0.239, 1.0); // #524a3d
pub const COLOR_BORDER:    Color = Color::new(0.463, 0.271, 0.349, 1.0); // #764559
pub const COLOR_WARM:      Color = Color::new(0.510, 0.298, 0.208, 1.0); // #824c35