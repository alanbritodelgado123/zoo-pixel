use macroquad::prelude::*;

use crate::config;
use crate::escena::Escena;
use crate::estado::{Estado, ModoVista};
use crate::input::Accion;

// =====================================================================
//  BARRA INFERIOR
// =====================================================================

pub fn dibujar_barra(estado: &Estado, es_android: bool, font: &Font) {
    let sw = screen_width();
    let sh = screen_height();
    let h = config::bar_height();
    let y = sh - h;

    draw_line(0.0, y, sw, y, 1.0, config::COLOR_DIM);

    let fs = config::fs_bar();
    let cy = y + h / 2.0 + fs as f32 * 0.35;

    // === IZQUIERDA: flechas ===
    let con = estado.escena.conexiones();
    let mut flechas = String::new();
    if con[0].is_some() { flechas.push_str("^ "); }
    if con[1].is_some() { flechas.push_str("v "); }
    if con[2].is_some() { flechas.push_str("< "); }
    if con[3].is_some() { flechas.push_str("> "); }
    draw_text_ex(&flechas, 12.0, cy, TextParams {
        font: Some(font), font_size: fs, color: config::COLOR_TEXT, ..Default::default()
    });

    // === CENTRO: nombre ===
    let nombre = estado.escena.nombre();
    let nd = measure_text(nombre, Some(font), fs, 1.0);
    draw_text_ex(nombre, sw / 2.0 - nd.width / 2.0, cy, TextParams {
        font: Some(font), font_size: fs, color: config::COLOR_ACCENT, ..Default::default()
    });

    // === DERECHA: controles ===
    let hint = if es_android {
        match &estado.modo {
            ModoVista::Normal if estado.escena != Escena::Entrada => "[A] Animal  [B] Salir",
            ModoVista::Normal => "",
            ModoVista::Seleccion { .. } => "[A] Ver  [B] Volver",
            ModoVista::ViendoAnimal { terminado, .. } => {
                if *terminado { "[A/B] Cerrar" } else { "Pulsa para saltar" }
            }
            ModoVista::Foto { foto_tomada, terminado, .. } => {
                if !*foto_tomada { "[A] Foto  [B] Salir" }
                else if *terminado { "[A/B] Siguiente" }
                else { "Pulsa para saltar" }
            }
        }
    } else {
        match &estado.modo {
            ModoVista::Normal if estado.escena != Escena::Entrada => "[Z] Animal  [X] Salir",
            ModoVista::Normal => "",
            ModoVista::Seleccion { .. } => "[Z] Ver  [X] Volver  ^v Elegir",
            ModoVista::ViendoAnimal { terminado, .. } => {
                if *terminado { "[Z/X] Cerrar" } else { "Pulsa para saltar" }
            }
            ModoVista::Foto { foto_tomada, terminado, .. } => {
                if !*foto_tomada { "[Z] Foto  [X] Salir" }
                else if *terminado { "[Z/X] Siguiente" }
                else { "Pulsa para saltar" }
            }
        }
    };

    if !hint.is_empty() {
        let hfs = config::fs_hint();
        let hd = measure_text(hint, Some(font), hfs, 1.0);
        draw_text_ex(hint, sw - hd.width - 12.0, cy, TextParams {
            font: Some(font), font_size: hfs, color: config::COLOR_TEXT, ..Default::default()
        });
    }
}

// =====================================================================
//  MINIMAPA 3x4
// =====================================================================

pub fn dibujar_minimapa(estado: &Estado, font: &Font) {
    let size = config::mini_size();
    let gap = config::mini_gap();
    let step = size + gap;
    let half = size / 2.0;
    let sh = screen_height();
    let base_x = 8.0;
    let base_y = sh - config::bar_height() - (4.0 * step) - 8.0;
    let mfs = config::fs_mini();

    let lc = Color::new(config::COLOR_DIM.r, config::COLOR_DIM.g, config::COLOR_DIM.b, 0.35);

    // Horizontales
    draw_line(base_x + size, base_y + half, base_x + step, base_y + half, 1.0, lc);
    draw_line(base_x + step + size, base_y + half, base_x + 2.0 * step, base_y + half, 1.0, lc);
    draw_line(base_x + size, base_y + step + half, base_x + step, base_y + step + half, 1.0, lc);
    draw_line(base_x + step + size, base_y + step + half, base_x + 2.0 * step, base_y + step + half, 1.0, lc);
    draw_line(base_x + size, base_y + 2.0 * step + half, base_x + step, base_y + 2.0 * step + half, 1.0, lc);
    draw_line(base_x + step + size, base_y + 2.0 * step + half, base_x + 2.0 * step, base_y + 2.0 * step + half, 1.0, lc);
    // Verticales
    draw_line(base_x + half, base_y + size, base_x + half, base_y + step, 1.0, lc);
    draw_line(base_x + half, base_y + step + size, base_x + half, base_y + 2.0 * step, 1.0, lc);
    draw_line(base_x + step + half, base_y + size, base_x + step + half, base_y + step, 1.0, lc);
    draw_line(base_x + step + half, base_y + step + size, base_x + step + half, base_y + 2.0 * step, 1.0, lc);
    draw_line(base_x + step + half, base_y + 2.0 * step + size, base_x + step + half, base_y + 3.0 * step, 1.0, lc);
    draw_line(base_x + 2.0 * step + half, base_y + size, base_x + 2.0 * step + half, base_y + step, 1.0, lc);
    draw_line(base_x + 2.0 * step + half, base_y + step + size, base_x + 2.0 * step + half, base_y + 2.0 * step, 1.0, lc);

    for escena in Escena::TODAS {
        let (col, row) = escena.pos_mapa();
        let x = base_x + col as f32 * step;
        let y = base_y + row as f32 * step;
        let actual = *escena == estado.escena;
        let visitada = estado.visitadas.contains(escena);

        let relleno = if actual {
            config::COLOR_HIGHLIGHT
        } else if visitada {
            Color::new(config::COLOR_DIM.r, config::COLOR_DIM.g, config::COLOR_DIM.b, 0.6)
        } else {
            Color::new(0.1, 0.1, 0.1, 0.25)
        };
        let texto = if actual {
            config::COLOR_BG_DARK
        } else if visitada {
            config::COLOR_TEXT
        } else {
            Color::new(0.3, 0.3, 0.3, 0.35)
        };
        let borde = if visitada {
            Color::new(config::COLOR_ACCENT.r, config::COLOR_ACCENT.g, config::COLOR_ACCENT.b, 0.5)
        } else {
            Color::new(0.2, 0.2, 0.2, 0.25)
        };

        draw_rectangle(x, y, size, size, relleno);
        draw_rectangle_lines(x, y, size, size, 1.0, borde);
        draw_text_ex(escena.letra(), x + 2.0, y + size * 0.77, TextParams {
            font: Some(font), font_size: mfs, color: texto, ..Default::default()
        });
    }
}

// =====================================================================
//  SELECCIÓN DE ANIMAL
// =====================================================================

pub fn dibujar_seleccion(estado: &Estado, font: &Font) {
    if let ModoVista::Seleccion { animales, indice } = &estado.modo {
        let sw = screen_width();
        let sh = screen_height();
        let s = config::scale();

        draw_rectangle(0.0, 0.0, sw, sh, Color::new(0.0, 0.0, 0.0, 0.88));

        let tfs = config::fs_sel_title();
        let titulo = estado.escena.nombre();
        let td = measure_text(titulo, Some(font), tfs, 1.0);
        draw_text_ex(titulo, sw / 2.0 - td.width / 2.0, 50.0 * s, TextParams {
            font: Some(font), font_size: tfs, color: config::COLOR_ACCENT, ..Default::default()
        });

        let sfs = config::fs_sel_sub();
        let subtitulo = "Selecciona un animal";
        let sd = measure_text(subtitulo, Some(font), sfs, 1.0);
        draw_text_ex(subtitulo, sw / 2.0 - sd.width / 2.0, 78.0 * s, TextParams {
            font: Some(font), font_size: sfs, color: config::COLOR_DIM, ..Default::default()
        });

        let item_h = 64.0 * s;
        let list_total_h = animales.len() as f32 * item_h;
        let area_top = 100.0 * s;
        let area_bottom = sh - config::bar_height() - 10.0;
        let area_h = area_bottom - area_top;
        let start_y = area_top + (area_h - list_total_h).max(0.0) / 2.0;

        let name_fs = config::fs_sel_name();
        let sci_fs = config::fs_sel_sci();

        for (i, animal) in animales.iter().enumerate() {
            let y = start_y + i as f32 * item_h;
            let selected = i == *indice;

            if selected {
                draw_rectangle(20.0, y - 2.0, sw - 40.0, item_h - 4.0,
                    Color::new(config::COLOR_HIGHLIGHT.r, config::COLOR_HIGHLIGHT.g, config::COLOR_HIGHLIGHT.b, 0.12));
                draw_rectangle_lines(20.0, y - 2.0, sw - 40.0, item_h - 4.0, 1.5, config::COLOR_HIGHLIGHT);
                draw_text_ex(">", 28.0, y + item_h * 0.42, TextParams {
                    font: Some(font), font_size: name_fs, color: config::COLOR_HIGHLIGHT, ..Default::default()
                });
            }

            let name_color = if selected { config::COLOR_TEXT } else { config::COLOR_DIM };
            let sci_color = if selected { config::COLOR_HIGHLIGHT } else {
                Color::new(config::COLOR_DIM.r, config::COLOR_DIM.g, config::COLOR_DIM.b, 0.5)
            };

            draw_text_ex(&animal.nombre_comun, 56.0, y + item_h * 0.4, TextParams {
                font: Some(font), font_size: name_fs, color: name_color, ..Default::default()
            });
            draw_text_ex(&animal.nombre_cientifico, 56.0, y + item_h * 0.78, TextParams {
                font: Some(font), font_size: sci_fs, color: sci_color, ..Default::default()
            });
        }
    }
}

// =====================================================================
//  VISTA DE ANIMAL
// =====================================================================

pub fn dibujar_animal(estado: &Estado, font: &Font) {
    if let ModoVista::ViendoAnimal { animal, texto_pos, terminado, .. } = &estado.modo {
        let sw = screen_width();
        let sh = screen_height();
        let s = config::scale();

        draw_rectangle(0.0, 0.0, sw, sh, config::COLOR_BG_DARK);

        let margin = 24.0 * s;
        let panel_split = sw * 0.38;
        let area_h = sh - config::bar_height();

        let name_fs = config::fs_anim_name();
        let sci_fs = config::fs_anim_sci();
        let desc_fs = config::fs_anim_desc();
        let init_fs = config::fs_anim_init();

        // ===== PANEL IZQUIERDO =====
        let img_size = (panel_split - margin * 2.0).min(180.0 * s);
        let gap_img_name = 24.0 * s;
        let nd = measure_text(&animal.nombre_comun, Some(font), name_fs, 1.0);
        let gap_name_sci = 8.0 * s;
        let sd = measure_text(&animal.nombre_cientifico, Some(font), sci_fs, 1.0);

        let left_total_h = img_size + gap_img_name + nd.height + gap_name_sci + sd.height;
        let left_start_y = (area_h - left_total_h) / 2.0;
        let panel_cx = panel_split / 2.0;

        let img_x = panel_cx - img_size / 2.0;
        let img_y = left_start_y;

        draw_rectangle(img_x, img_y, img_size, img_size,
            Color::new(config::COLOR_DIM.r, config::COLOR_DIM.g, config::COLOR_DIM.b, 0.3));
        draw_rectangle_lines(img_x, img_y, img_size, img_size, 2.0, config::COLOR_BORDER);

        let initial = &animal.nombre_comun[..animal.nombre_comun.char_indices()
            .nth(1).map(|(i, _)| i).unwrap_or(animal.nombre_comun.len())];
        let id = measure_text(initial, Some(font), init_fs, 1.0);
        draw_text_ex(initial,
            img_x + img_size / 2.0 - id.width / 2.0,
            img_y + img_size / 2.0 + id.height / 2.0,
            TextParams { font: Some(font), font_size: init_fs, color: config::COLOR_DIM, ..Default::default() });

        let name_y = img_y + img_size + gap_img_name + nd.height;
        draw_text_ex(&animal.nombre_comun,
            panel_cx - nd.width / 2.0, name_y,
            TextParams { font: Some(font), font_size: name_fs, color: config::COLOR_ACCENT, ..Default::default() });

        let sci_y = name_y + gap_name_sci + sd.height;
        draw_text_ex(&animal.nombre_cientifico,
            panel_cx - sd.width / 2.0, sci_y,
            TextParams { font: Some(font), font_size: sci_fs, color: config::COLOR_HIGHLIGHT, ..Default::default() });

        // ===== SEPARADOR =====
        draw_line(panel_split, margin, panel_split, area_h - margin, 1.0, config::COLOR_BORDER);

        // ===== PANEL DERECHO =====
        let text_x = panel_split + margin;
        let max_w = sw - panel_split - margin * 2.0;
        let line_h = desc_fs as f32 * 1.3;

        let visible: String = animal.descripcion.chars().take(*texto_pos).collect();
        let lines = wrap_text(&visible, font, desc_fs, max_w);
        let total_h = lines.len() as f32 * line_h;
        let text_start_y = (area_h - total_h) / 2.0 + desc_fs as f32;

        for (i, line) in lines.iter().enumerate() {
            draw_text_ex(line, text_x, text_start_y + i as f32 * line_h, TextParams {
                font: Some(font), font_size: desc_fs, color: config::COLOR_GREEN, ..Default::default()
            });
        }

        if !*terminado {
            let last_line = lines.last().map(|s| s.as_str()).unwrap_or("");
            let lw = measure_text(last_line, Some(font), desc_fs, 1.0).width;
            let cursor_y = text_start_y + (lines.len().saturating_sub(1)) as f32 * line_h;
            if (get_time() * 4.0).sin() > 0.0 {
                draw_text_ex("_", text_x + lw, cursor_y, TextParams {
                    font: Some(font), font_size: desc_fs, color: config::COLOR_GREEN, ..Default::default()
                });
            }
        }
    }
}

// =====================================================================
//  MODO FOTO
// =====================================================================

pub fn dibujar_foto(estado: &Estado, font: &Font) {
    if let ModoVista::Foto {
        animales, indice_actual, celda, foto_tomada,
        texto_pos, terminado, ya_vistos, timer, ..
    } = &estado.modo {
        let sw = screen_width();
        let sh = screen_height();
        let s = config::scale();
        let area_h = sh - config::bar_height();

        // Viñeta
        let vig_w = 40.0 * s;
        let vignette = Color::new(0.0, 0.0, 0.0, 0.4);
        draw_rectangle(0.0, 0.0, vig_w, area_h, vignette);
        draw_rectangle(sw - vig_w, 0.0, vig_w, area_h, vignette);

        // Marco
        let m = 30.0 * s;
        let cam_border = Color::new(config::COLOR_TEXT.r, config::COLOR_TEXT.g, config::COLOR_TEXT.b, 0.5);
        draw_rectangle_lines(m, 10.0, sw - m * 2.0, area_h - 20.0, 2.0, cam_border);

        // Esquinas
        let cl_len = 30.0 * s;
        let cc = config::COLOR_HIGHLIGHT;
        let cl = m; let ct = 10.0; let cr = sw - m; let cb = area_h - 10.0;
        draw_line(cl, ct, cl + cl_len, ct, 2.0, cc);
        draw_line(cl, ct, cl, ct + cl_len, 2.0, cc);
        draw_line(cr, ct, cr - cl_len, ct, 2.0, cc);
        draw_line(cr, ct, cr, ct + cl_len, 2.0, cc);
        draw_line(cl, cb, cl + cl_len, cb, 2.0, cc);
        draw_line(cl, cb, cl, cb - cl_len, 2.0, cc);
        draw_line(cr, cb, cr - cl_len, cb, 2.0, cc);
        draw_line(cr, cb, cr, cb - cl_len, 2.0, cc);

        // REC
        let rec_fs = config::fs_foto_rec();
        if (get_time() * 2.0).sin() > 0.0 {
            draw_circle(sw - 60.0 * s, 30.0 * s, 6.0 * s, Color::new(0.9, 0.15, 0.15, 0.9));
        }
        draw_text_ex("REC", sw - 50.0 * s, 35.0 * s, TextParams {
            font: Some(font), font_size: rec_fs, color: config::COLOR_TEXT, ..Default::default()
        });

        // Contador
        let cnt_fs = config::fs_foto_count();
        let counter = format!("{}/{}", ya_vistos.len(), animales.len());
        draw_text_ex(&counter, 50.0 * s, 35.0 * s, TextParams {
            font: Some(font), font_size: cnt_fs, color: config::COLOR_ACCENT, ..Default::default()
        });

        // Cuadrícula
        let gm = 60.0 * s;
        let gw = sw - gm * 2.0;
        let gh = area_h - gm * 2.0;
        let cw = gw / 2.0;
        let ch = gh / 2.0;
        let gx = gm;
        let gy = gm;

        let grid_c = Color::new(config::COLOR_TEXT.r, config::COLOR_TEXT.g, config::COLOR_TEXT.b, 0.2);
        draw_line(gx, gy + ch, gx + gw, gy + ch, 1.0, grid_c);
        draw_line(gx + cw, gy, gx + cw, gy + gh, 1.0, grid_c);

        let third_c = Color::new(config::COLOR_DIM.r, config::COLOR_DIM.g, config::COLOR_DIM.b, 0.12);
        for i in 1..6 {
            draw_line(gx, gy + gh * i as f32 / 6.0, gx + gw, gy + gh * i as f32 / 6.0, 1.0, third_c);
            draw_line(gx + gw * i as f32 / 6.0, gy, gx + gw * i as f32 / 6.0, gy + gh, 1.0, third_c);
        }

        if !*foto_tomada {
            let col = *celda % 2;
            let row = *celda / 2;
            let cx = gx + col as f32 * cw + cw / 2.0;
            let cy = gy + row as f32 * ch + ch / 2.0;

            let bird_r = cw.min(ch) * 0.25;
            draw_circle(cx, cy, bird_r, Color::new(
                config::COLOR_HIGHLIGHT.r, config::COLOR_HIGHLIGHT.g,
                config::COLOR_HIGHLIGHT.b, 0.2));
            draw_circle_lines(cx, cy, bird_r, 2.0, config::COLOR_HIGHLIGHT);

            let animal = &animales[*indice_actual];
            let initial = &animal.nombre_comun[..animal.nombre_comun.char_indices()
                .nth(1).map(|(i, _)| i).unwrap_or(animal.nombre_comun.len())];
            let bfs = config::fs_foto_bird();
            let id = measure_text(initial, Some(font), bfs, 1.0);
            draw_text_ex(initial, cx - id.width / 2.0, cy + id.height / 2.0, TextParams {
                font: Some(font), font_size: bfs, color: config::COLOR_HIGHLIGHT, ..Default::default()
            });

            let nfs = config::fs_foto_name();
            let nd = measure_text(&animal.nombre_comun, Some(font), nfs, 1.0);
            draw_text_ex(&animal.nombre_comun, cx - nd.width / 2.0, cy + bird_r + 22.0 * s, TextParams {
                font: Some(font), font_size: nfs, color: config::COLOR_ACCENT, ..Default::default()
            });

            // Cruz central
            let cross = 15.0 * s;
            let cross_c = Color::new(config::COLOR_TEXT.r, config::COLOR_TEXT.g, config::COLOR_TEXT.b, 0.35);
            let scx = sw / 2.0;
            let scy = area_h / 2.0;
            draw_line(scx - cross, scy, scx + cross, scy, 1.0, cross_c);
            draw_line(scx, scy - cross, scx, scy + cross, 1.0, cross_c);
        } else {
            let animal = &animales[*indice_actual];

            // Flash
            if *timer < 0.15 {
                let flash_a = (1.0 - *timer / 0.15) * 0.6;
                draw_rectangle(0.0, 0.0, sw, area_h, Color::new(1.0, 1.0, 1.0, flash_a));
            }

            draw_rectangle(0.0, 0.0, sw, area_h, Color::new(0.0, 0.0, 0.0, 0.75));

            let name_fs = config::fs_anim_name();
            let sci_fs = config::fs_anim_sci();
            let desc_fs = config::fs_anim_desc();
            let line_h = desc_fs as f32 * 1.3;
            let margin = 30.0 * s;

            let nd = measure_text(&animal.nombre_comun, Some(font), name_fs, 1.0);
            let sd = measure_text(&animal.nombre_cientifico, Some(font), sci_fs, 1.0);

            let header_y = 40.0 * s;
            draw_text_ex(&animal.nombre_comun,
                sw / 2.0 - nd.width / 2.0, header_y + nd.height,
                TextParams { font: Some(font), font_size: name_fs, color: config::COLOR_ACCENT, ..Default::default() });
            draw_text_ex(&animal.nombre_cientifico,
                sw / 2.0 - sd.width / 2.0, header_y + nd.height + 12.0 * s + sd.height,
                TextParams { font: Some(font), font_size: sci_fs, color: config::COLOR_HIGHLIGHT, ..Default::default() });

            let sep_y = header_y + nd.height + 12.0 * s + sd.height + 20.0 * s;
            draw_line(margin, sep_y, sw - margin, sep_y, 1.0, config::COLOR_BORDER);

            let visible: String = animal.descripcion.chars().take(*texto_pos).collect();
            let max_w = sw - margin * 2.0;
            let lines = wrap_text(&visible, font, desc_fs, max_w);
            let total_text_h = lines.len() as f32 * line_h;
            let desc_top = sep_y + 15.0 * s;
            let desc_bot = area_h - 10.0;
            let desc_h = desc_bot - desc_top;
            let text_start_y = desc_top + (desc_h - total_text_h).max(0.0) / 2.0 + desc_fs as f32;

            for (i, line) in lines.iter().enumerate() {
                let ld = measure_text(line, Some(font), desc_fs, 1.0);
                draw_text_ex(line,
                    sw / 2.0 - ld.width / 2.0,
                    text_start_y + i as f32 * line_h,
                    TextParams { font: Some(font), font_size: desc_fs, color: config::COLOR_GREEN, ..Default::default() });
            }

            if !*terminado && (get_time() * 4.0).sin() > 0.0 {
                let last_line = lines.last().map(|s| s.as_str()).unwrap_or("");
                let lw = measure_text(last_line, Some(font), desc_fs, 1.0).width;
                let cy = text_start_y + (lines.len().saturating_sub(1)) as f32 * line_h;
                draw_text_ex("_", sw / 2.0 + lw / 2.0, cy, TextParams {
                    font: Some(font), font_size: desc_fs, color: config::COLOR_GREEN, ..Default::default()
                });
            }
        }
    }
}

// =====================================================================
//  WORD WRAP
// =====================================================================

fn wrap_text(text: &str, font: &Font, font_size: u16, max_w: f32) -> Vec<String> {
    let mut lines = Vec::new();
    let mut current_line = String::new();

    for word in text.split_whitespace() {
        let test = if current_line.is_empty() {
            word.to_string()
        } else {
            format!("{} {}", current_line, word)
        };

        let w = measure_text(&test, Some(font), font_size, 1.0).width;

        if w > max_w && !current_line.is_empty() {
            lines.push(current_line);
            current_line = word.to_string();
        } else {
            current_line = test;
        }
    }

    if !current_line.is_empty() {
        lines.push(current_line);
    }

    if lines.is_empty() {
        lines.push(String::new());
    }

    lines
}

// =====================================================================
//  PLACEHOLDER
// =====================================================================

pub fn dibujar_placeholder(escena: &Escena, area_h: f32, font: &Font) {
    let sw = screen_width();
    draw_rectangle(0.0, 0.0, sw, area_h, escena.color_fondo());

    let pulse = (get_time() as f32 * 2.0).sin() * 0.1 + 0.9;
    let color = Color::new(
        config::COLOR_TEXT.r * pulse,
        config::COLOR_TEXT.g * pulse,
        config::COLOR_TEXT.b * pulse,
        0.6,
    );

    let pfs = config::fs_place();
    let nombre = escena.nombre();
    let dims = measure_text(nombre, Some(font), pfs, 1.0);
    draw_text_ex(nombre, sw / 2.0 - dims.width / 2.0, area_h / 2.0, TextParams {
        font: Some(font), font_size: pfs, color, ..Default::default()
    });
}

// =====================================================================
//  TRANSICIÓN
// =====================================================================

pub fn dibujar_transicion(estado: &Estado) {
    let alpha = estado.alpha_transicion();
    if alpha < 0.01 { return; }

    draw_rectangle(0.0, 0.0, screen_width(), screen_height(),
        Color::new(0.0, 0.0, 0.0, alpha));

    if alpha > 0.3 {
        let line_a = (alpha - 0.3) * 0.3;
        let mut y = 0.0;
        while y < screen_height() {
            draw_line(0.0, y, screen_width(), y, 1.0,
                Color::new(config::COLOR_DIM.r, config::COLOR_DIM.g, config::COLOR_DIM.b, line_a));
            y += 3.0;
        }
    }
}

// =====================================================================
//  BOTONES TÁCTILES
// =====================================================================

pub struct Botones {
    pub up: Rect,
    pub down: Rect,
    pub left: Rect,
    pub right: Rect,
    pub a: Rect,
    pub b: Rect,
}

impl Botones {
    pub fn calcular() -> Self {
        let sw = screen_width();
        let sh = screen_height() - config::bar_height();
        let sz = sw * config::BTN_RATIO;
        let pad = config::BTN_PAD;
        Self {
            up:    Rect::new(pad + sz,       sh - sz * 3.2, sz, sz),
            down:  Rect::new(pad + sz,       sh - sz * 1.2, sz, sz),
            left:  Rect::new(pad,            sh - sz * 2.2, sz, sz),
            right: Rect::new(pad + sz * 2.0, sh - sz * 2.2, sz, sz),
            a:     Rect::new(sw - sz * 1.2 - pad, sh - sz * 2.2, sz, sz),
            b:     Rect::new(sw - sz * 2.5 - pad, sh - sz * 1.2, sz, sz),
        }
    }
}

pub fn dibujar_controles(estado: &Estado, botones: &Botones, font: &Font) -> Vec<Accion> {
    let mut acciones = Vec::new();
    let con = estado.escena.conexiones();
    let nav = matches!(estado.modo, ModoVista::Normal) && !estado.en_transicion();
    let puede = !estado.en_transicion();

    if boton_tactil(botones.up,    "^", nav && con[0].is_some(), font) { acciones.push(Accion::Arriba); }
    if boton_tactil(botones.down,  "v", nav && con[1].is_some(), font) { acciones.push(Accion::Abajo); }
    if boton_tactil(botones.left,  "<", nav && con[2].is_some(), font) { acciones.push(Accion::Izquierda); }
    if boton_tactil(botones.right, ">", nav && con[3].is_some(), font) { acciones.push(Accion::Derecha); }
    if boton_tactil(botones.a,     "Z", puede, font)                   { acciones.push(Accion::BotonA); }
    if boton_tactil(botones.b,     "X", puede, font)                   { acciones.push(Accion::BotonB); }

    acciones
}

fn boton_tactil(rect: Rect, label: &str, activo: bool, font: &Font) -> bool {
    let mut pulsado = false;

    if activo {
        if is_mouse_button_pressed(MouseButton::Left) {
            let (mx, my) = mouse_position();
            if rect.contains(vec2(mx, my)) { pulsado = true; }
        }
        for touch in touches() {
            if touch.phase == TouchPhase::Started && rect.contains(touch.position) {
                pulsado = true;
            }
        }
    }

    let bg = if pulsado {
        Color::new(config::COLOR_HIGHLIGHT.r, config::COLOR_HIGHLIGHT.g, config::COLOR_HIGHLIGHT.b, 0.4)
    } else if activo {
        Color::new(config::COLOR_DIM.r, config::COLOR_DIM.g, config::COLOR_DIM.b, 0.15)
    } else {
        Color::new(0.1, 0.1, 0.1, 0.06)
    };

    let borde = if activo { config::COLOR_BORDER } else {
        Color::new(0.15, 0.15, 0.15, 0.12)
    };
    let texto_c = if activo { config::COLOR_TEXT } else {
        Color::new(0.3, 0.3, 0.3, 0.25)
    };

    draw_rectangle(rect.x, rect.y, rect.w, rect.h, bg);
    draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 1.0, borde);

    let bfs = config::fs_btn();
    let dims = measure_text(label, Some(font), bfs, 1.0);
    draw_text_ex(label,
        rect.x + (rect.w - dims.width) / 2.0,
        rect.y + rect.h / 2.0 + dims.height / 2.0,
        TextParams { font: Some(font), font_size: bfs, color: texto_c, ..Default::default() });

    pulsado
}