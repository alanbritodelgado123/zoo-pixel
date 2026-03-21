// src/config.rs
use macroquad::prelude::*;

pub const TRANSITION_SECS_FALLBACK: f32 = 0.5;
pub const TRANSITION_MIN: f32 = 0.3;
pub const BTN_RATIO: f32 = 0.14;
pub const BTN_PAD: f32 = 15.0;
pub const BAR_HEIGHT_RATIO: f32 = 0.07;
pub const TYPEWRITER_CPS: f32 = 35.0;
pub const MAPA_COLS: usize = 6;
pub const MAPA_ROWS: usize = 6;
pub const CICLO_DIA_SECS: f32 = 300.0;

// =====================================================================
//  PALETA
// =====================================================================

pub const P_BLACK:        Color = Color::new(0.122, 0.098, 0.098, 1.0);
pub const P_DARK1:        Color = Color::new(0.130, 0.137, 0.122, 1.0);
pub const P_DARK2:        Color = Color::new(0.176, 0.157, 0.129, 1.0);
pub const P_DARK3:        Color = Color::new(0.243, 0.216, 0.192, 1.0);
pub const P_DARK_GREEN:   Color = Color::new(0.169, 0.239, 0.196, 1.0);
pub const P_DARK_BLUE:    Color = Color::new(0.153, 0.192, 0.227, 1.0);
pub const P_DARK_MAROON:  Color = Color::new(0.176, 0.122, 0.129, 1.0);
pub const P_OLIVE:        Color = Color::new(0.443, 0.416, 0.325, 1.0);
pub const P_GREY_GREEN:   Color = Color::new(0.380, 0.431, 0.392, 1.0);
pub const P_BROWN:        Color = Color::new(0.322, 0.239, 0.176, 1.0);
pub const P_MID_BROWN:    Color = Color::new(0.510, 0.298, 0.208, 1.0);
pub const P_WARM_BROWN:   Color = Color::new(0.557, 0.392, 0.263, 1.0);
pub const P_RUST:         Color = Color::new(0.584, 0.290, 0.204, 1.0);
pub const P_DARK_RUST:    Color = Color::new(0.502, 0.220, 0.176, 1.0);
pub const P_MAROON:       Color = Color::new(0.259, 0.153, 0.188, 1.0);
pub const P_PURPLE:       Color = Color::new(0.380, 0.184, 0.278, 1.0);
pub const P_MAUVE:        Color = Color::new(0.463, 0.271, 0.349, 1.0);
pub const P_DUSTY_ROSE:   Color = Color::new(0.569, 0.400, 0.451, 1.0);
pub const P_ROSE:         Color = Color::new(0.702, 0.592, 0.592, 1.0);
pub const P_GREEN:        Color = Color::new(0.243, 0.471, 0.231, 1.0);
pub const P_BRIGHT_GREEN: Color = Color::new(0.345, 0.569, 0.278, 1.0);
pub const P_FOREST:       Color = Color::new(0.216, 0.369, 0.239, 1.0);
pub const P_TEAL:         Color = Color::new(0.435, 0.718, 0.671, 1.0);
pub const P_DARK_TEAL:    Color = Color::new(0.325, 0.592, 0.600, 1.0);
pub const P_BLUE_TEAL:    Color = Color::new(0.247, 0.471, 0.529, 1.0);
pub const P_DARK_BLUE2:   Color = Color::new(0.184, 0.278, 0.380, 1.0);
pub const P_CREAM:        Color = Color::new(0.835, 0.827, 0.757, 1.0);
pub const P_BEIGE:        Color = Color::new(0.725, 0.710, 0.608, 1.0);
pub const P_TAN:          Color = Color::new(0.765, 0.678, 0.478, 1.0);
pub const P_GOLD:         Color = Color::new(0.804, 0.718, 0.373, 1.0);
pub const P_PALE_GOLD:    Color = Color::new(0.820, 0.782, 0.518, 1.0);
pub const P_LIME:         Color = Color::new(0.686, 0.757, 0.408, 1.0);
pub const P_YELLOW_GREEN: Color = Color::new(0.529, 0.678, 0.333, 1.0);
pub const P_PALE_GREEN:   Color = Color::new(0.631, 0.725, 0.655, 1.0);
pub const P_LIGHT_GREEN:  Color = Color::new(0.502, 0.545, 0.494, 1.0);
pub const P_AMBER:        Color = Color::new(0.733, 0.545, 0.298, 1.0);
pub const P_ORANGE:       Color = Color::new(0.655, 0.404, 0.243, 1.0);
pub const P_KHAKI:        Color = Color::new(0.616, 0.510, 0.341, 1.0);

// =====================================================================
//  COLORES SEMÁNTICOS
// =====================================================================

pub const COLOR_BG_DARK:   Color = P_BLACK;
pub const COLOR_BG_ALT:    Color = P_DARK2;
pub const COLOR_TEXT:       Color = P_CREAM;
pub const COLOR_TEXT_DIM:   Color = P_BEIGE;
pub const COLOR_ACCENT:    Color = P_GOLD;
pub const COLOR_HIGHLIGHT: Color = P_TEAL;
pub const COLOR_GREEN:     Color = P_BRIGHT_GREEN;
pub const COLOR_DIM:       Color = P_OLIVE;
pub const COLOR_OLIVE:     Color = P_OLIVE;
pub const COLOR_BORDER:    Color = P_MAUVE;
pub const COLOR_WARM:      Color = P_AMBER;
pub const COLOR_SPECIAL:   Color = P_PURPLE;
pub const COLOR_DANGER:    Color = P_RUST;
pub const COLOR_SUCCESS:   Color = P_FOREST;
pub const COLOR_BAR_BG:    Color = P_DARK1;
pub const COLOR_DIALOG_BG: Color = P_DARK_MAROON;
pub const COLOR_ROSE:      Color = P_DUSTY_ROSE;

pub fn bar_height() -> f32 {
    (screen_height() * BAR_HEIGHT_RATIO).max(36.0)
}

pub fn scale() -> f32 {
    (screen_height() / 600.0).clamp(0.5, 2.5)
}

pub fn safe_top() -> f32 {
    if cfg!(target_os = "android") { 24.0 * scale() } else { 0.0 }
}

/// Altura del overlay de botones en la parte inferior (para móvil o PC)
/// Devuelve la altura que ocupan los controles overlay desde abajo
pub fn overlay_height() -> f32 {
    let s = scale();
    let btn_size = (screen_height() * BTN_RATIO).max(40.0);
    // El overlay tiene dpad + botones AB + algo de padding
    // Estimamos: botón más grande + padding inferior
    if cfg!(target_os = "android") {
        btn_size * 3.0 + BTN_PAD * 2.0
    } else {
        // En PC el overlay de teclas es más pequeño
        let indicator_h = 28.0 * s + 8.0;
        indicator_h + BTN_PAD
    }
}

/// Área segura inferior: desde dónde podemos dibujar sin tapar overlay
pub fn safe_bottom() -> f32 {
    screen_height() - overlay_height()
}

pub fn fs_bar() -> u16        { (24.0 * scale()) as u16 }
pub fn fs_hint() -> u16       { (18.0 * scale()) as u16 }
pub fn fs_sel_name() -> u16   { (28.0 * scale()) as u16 }
pub fn fs_sel_sci() -> u16    { (18.0 * scale()) as u16 }
pub fn fs_sel_title() -> u16  { (34.0 * scale()) as u16 }
pub fn fs_sel_sub() -> u16    { (20.0 * scale()) as u16 }
pub fn fs_anim_name() -> u16  { (32.0 * scale()) as u16 }
pub fn fs_anim_sci() -> u16   { (22.0 * scale()) as u16 }
pub fn fs_anim_desc() -> u16  { (28.0 * scale()) as u16 }  // Más grande
pub fn fs_anim_init() -> u16  { (72.0 * scale()) as u16 }
pub fn fs_place() -> u16      { (40.0 * scale()) as u16 }
pub fn fs_mini() -> u16       { (9.0 * scale()) as u16 }
pub fn fs_btn() -> u16        { (22.0 * scale()) as u16 }
pub fn fs_foto_count() -> u16 { (16.0 * scale()) as u16 }
pub fn fs_foto_rec() -> u16   { (14.0 * scale()) as u16 }
pub fn fs_foto_bird() -> u16  { (48.0 * scale()) as u16 }
pub fn fs_foto_name() -> u16  { (18.0 * scale()) as u16 }
pub fn fs_titulo() -> u16     { (48.0 * scale()) as u16 }
pub fn fs_subtitulo() -> u16  { (20.0 * scale()) as u16 }
pub fn fs_dialogo() -> u16    { (22.0 * scale()) as u16 }
pub fn fs_menu() -> u16       { (26.0 * scale()) as u16 }
pub fn fs_libreta() -> u16    { (20.0 * scale()) as u16 }
pub fn fs_pesca() -> u16      { (28.0 * scale()) as u16 }
pub fn fs_evento() -> u16     { (18.0 * scale()) as u16 }
pub fn fs_quiz() -> u16       { (22.0 * scale()) as u16 }
pub fn fs_indicador() -> u16  { (20.0 * scale()) as u16 }
pub fn fs_fase_dia() -> u16   { (14.0 * scale()) as u16 }
pub fn fs_config() -> u16     { (24.0 * scale()) as u16 }
pub fn fs_config_val() -> u16 { (20.0 * scale()) as u16 }

pub fn mini_size() -> f32     { (12.0 * scale()).max(7.0) }
pub fn mini_gap() -> f32      { (2.0 * scale()).max(1.0) }

pub fn fs_adaptativo(texto: &str, font: &Font, max_fs: u16, max_w: f32) -> u16 {
    let mut fs = max_fs;
    loop {
        let w = measure_text(texto, Some(font), fs, 1.0).width;
        if w <= max_w || fs <= 8 { return fs; }
        fs -= 1;
    }
}

/// Calcula un font size que haga que el texto quepa en el ancho dado,
/// pero nunca mayor que max_fs ni menor que min_fs
pub fn fs_fit(texto: &str, font: &Font, max_fs: u16, min_fs: u16, max_w: f32) -> u16 {
    let mut fs = max_fs;
    loop {
        let w = measure_text(texto, Some(font), fs, 1.0).width;
        if w <= max_w || fs <= min_fs { return fs; }
        fs -= 1;
    }
}