use macroquad::prelude::*;

pub const TRANSITION_SECS_FALLBACK: f32 = 0.5;
pub const TRANSITION_MIN: f32 = 0.3;
pub const BTN_RATIO: f32 = 0.14;
pub const BTN_PAD: f32 = 15.0;
pub const BAR_HEIGHT_RATIO: f32 = 0.07;
pub const TYPEWRITER_CPS: f32 = 35.0;

pub const COLOR_TEXT:      Color = Color::new(0.835, 0.827, 0.757, 1.0);
pub const COLOR_ACCENT:    Color = Color::new(0.765, 0.678, 0.478, 1.0);
pub const COLOR_BG_DARK:   Color = Color::new(0.122, 0.098, 0.098, 1.0);
pub const COLOR_HIGHLIGHT: Color = Color::new(0.435, 0.718, 0.671, 1.0);
pub const COLOR_GREEN:     Color = Color::new(0.243, 0.471, 0.231, 1.0);
pub const COLOR_DIM:       Color = Color::new(0.322, 0.290, 0.239, 1.0);
pub const COLOR_BORDER:    Color = Color::new(0.463, 0.271, 0.349, 1.0);
pub const COLOR_WARM:      Color = Color::new(0.510, 0.298, 0.208, 1.0);

pub fn bar_height() -> f32 {
    (screen_height() * BAR_HEIGHT_RATIO).max(36.0)
}

pub fn scale() -> f32 {
    (screen_height() / 600.0).clamp(0.5, 2.5)
}

pub fn fs_bar() -> u16        { (24.0 * scale()) as u16 }
pub fn fs_hint() -> u16       { (18.0 * scale()) as u16 }
pub fn fs_sel_name() -> u16   { (28.0 * scale()) as u16 }
pub fn fs_sel_sci() -> u16    { (18.0 * scale()) as u16 }
pub fn fs_sel_title() -> u16  { (34.0 * scale()) as u16 }
pub fn fs_sel_sub() -> u16    { (20.0 * scale()) as u16 }
pub fn fs_anim_name() -> u16  { (32.0 * scale()) as u16 }
pub fn fs_anim_sci() -> u16   { (22.0 * scale()) as u16 }
pub fn fs_anim_desc() -> u16  { (24.0 * scale()) as u16 }
pub fn fs_anim_init() -> u16  { (72.0 * scale()) as u16 }
pub fn fs_place() -> u16      { (40.0 * scale()) as u16 }
pub fn fs_mini() -> u16       { (11.0 * scale()) as u16 }
pub fn fs_btn() -> u16        { (22.0 * scale()) as u16 }
pub fn fs_foto_count() -> u16 { (16.0 * scale()) as u16 }
pub fn fs_foto_rec() -> u16   { (14.0 * scale()) as u16 }
pub fn fs_foto_bird() -> u16  { (48.0 * scale()) as u16 }
pub fn fs_foto_name() -> u16  { (18.0 * scale()) as u16 }

pub fn mini_size() -> f32     { (13.0 * scale()).max(8.0) }
pub fn mini_gap() -> f32      { (2.0 * scale()).max(1.0) }