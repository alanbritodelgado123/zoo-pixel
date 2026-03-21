// src/ui.rs
use macroquad::prelude::*;
use crate::config::*;
use crate::estado::*;
use crate::escena::Escena;
use crate::fondo::Fondos;
use crate::minijuego::{FasePesca, FaseMuseo, CeldaExcavacion, MinijuegoMuseo};
use crate::ciclo_dia::ModoCiclo;

pub struct UiRenderer {
    pub font: Font,
}

impl UiRenderer {
    pub fn new(font: Font) -> Self {
        Self { font }
    }

    pub fn render(&self, estado: &Estado, fondos: &Fondos) {
        match estado.pantalla {
            Pantalla::Inicio => self.render_inicio(estado),
            Pantalla::Intro => self.render_intro(estado),
            Pantalla::Config => self.render_config(estado),
            Pantalla::MapaCompleto => self.render_mapa_completo(estado),
            Pantalla::LibretaCompleta => self.render_libreta_completa(estado),
            Pantalla::Juego => self.render_juego(estado, fondos),
        }
    }

    fn render_inicio(&self, estado: &Estado) {
        clear_background(COLOR_BG_DARK);
        let sw = screen_width();
        let sh = screen_height();

        // Titulo
        let titulo = "Zoo Pixel";
        let fs_t = fs_adaptativo(titulo, &self.font, fs_pct(0.08), sw * 0.85);
        let tw = measure_text(titulo, Some(&self.font), fs_t, 1.0).width;
        draw_text_ex(titulo, (sw - tw) / 2.0, sh * 0.22, TextParams {
            font: Some(&self.font), font_size: fs_t, color: COLOR_ACCENT, ..Default::default()
        });

        // Subtitulo
        let sub = "Fauna venezolana";
        let fs_sub = fs_adaptativo(sub, &self.font, fs_pct(0.03), sw * 0.8);
        let sw2 = measure_text(sub, Some(&self.font), fs_sub, 1.0).width;
        let th = text_height(&self.font, fs_t);
        draw_text_ex(sub, (sw - sw2) / 2.0, sh * 0.22 + th * 0.8, TextParams {
            font: Some(&self.font), font_size: fs_sub, color: COLOR_TEXT_DIM, ..Default::default()
        });

        // Opciones
        let opciones = ["Explorar", "Modo Dia", "Modo Noche", "Configuracion"];
        let fs_m = fs_adaptativo("Configuracion", &self.font, fs_pct(0.035), sw * 0.7);
        let item_h = text_height(&self.font, fs_m) * 2.0;
        let y_base = sh * 0.42;

        for (i, opt) in opciones.iter().enumerate() {
            let color = if i == estado.inicio_seleccion { COLOR_ACCENT } else { COLOR_TEXT };
            let prefix = if i == estado.inicio_seleccion { "> " } else { "  " };
            let texto = format!("{}{}", prefix, opt);
            let tw = measure_text(&texto, Some(&self.font), fs_m, 1.0).width;
            draw_text_ex(&texto, (sw - tw) / 2.0, y_base + i as f32 * item_h, TextParams {
                font: Some(&self.font), font_size: fs_m, color, ..Default::default()
            });
        }
    }

    fn render_intro(&self, estado: &Estado) {
        clear_background(COLOR_BG_DARK);
        if estado.dialogo.activo {
            self.render_dialogo(estado);
        }
    }

    fn render_config(&self, estado: &Estado) {
        clear_background(COLOR_BG_DARK);
        let sw = screen_width();
        let sh = screen_height();
        let mc = &estado.menu_config;

        let titulo = "Configuracion";
        let fs_t = fs_adaptativo(titulo, &self.font, fs_pct(0.05), sw * 0.9);
        let tw = measure_text(titulo, Some(&self.font), fs_t, 1.0).width;
        let th_t = text_height(&self.font, fs_t);
        draw_text_ex(titulo, (sw - tw) / 2.0, sh * 0.12 + th_t, TextParams {
            font: Some(&self.font), font_size: fs_t, color: COLOR_ACCENT, ..Default::default()
        });

        let fs_c = fs_adaptativo("Volumen Efectos", &self.font, fs_pct(0.032), sw * 0.8);
        let th_c = text_height(&self.font, fs_c);
        let fs_v = fs_pct(0.025);
        let th_v = text_height(&self.font, fs_v);
        let bar_h_px = sh * 0.018;
        // Cada item: label + gap + bar + gap + pct
        let item_h = th_c + 8.0 + bar_h_px + 8.0 + th_v + 16.0;
        let y_base = sh * 0.25;

        for (i, opt) in MenuConfig::OPCIONES.iter().enumerate() {
            let selected = i == mc.seleccion;
            let color = if selected { COLOR_ACCENT } else { COLOR_TEXT };
            let prefix = if selected { "> " } else { "  " };
            let texto = format!("{}{}", prefix, opt);
            let y = y_base + i as f32 * item_h;

            let ttw = measure_text(&texto, Some(&self.font), fs_c, 1.0).width;
            draw_text_ex(&texto, (sw - ttw) / 2.0, y + th_c, TextParams {
                font: Some(&self.font), font_size: fs_c, color, ..Default::default()
            });

            match i {
                0 => {
                    let bar_y = y + th_c + 8.0;
                    self.render_barra_volumen_layout(sw, bar_y, bar_h_px, mc.volumen_musica, selected, fs_v, th_v);
                }
                1 => {
                    let bar_y = y + th_c + 8.0;
                    self.render_barra_volumen_layout(sw, bar_y, bar_h_px, mc.volumen_efectos, selected, fs_v, th_v);
                }
                _ => {}
            }
        }

        let hint = "Flechas: Ajustar  Z/X: Volver";
        let fs_h = fs_adaptativo(hint, &self.font, fs_pct(0.025), sw * 0.9);
        let htw = measure_text(hint, Some(&self.font), fs_h, 1.0).width;
        draw_text_ex(hint, (sw - htw) / 2.0, sh * 0.90, TextParams {
            font: Some(&self.font), font_size: fs_h, color: COLOR_DIM, ..Default::default()
        });
    }

    fn render_barra_volumen_layout(&self, sw: f32, y: f32, bar_h: f32, valor: f32, selected: bool, fs_v: u16, _th_v: f32) {
        let bar_w = (sw * 0.5).min(sw - 60.0);
        let x = (sw - bar_w) / 2.0;

        draw_rectangle(x, y, bar_w, bar_h, COLOR_BAR_BG);
        let fill_color = if selected { COLOR_HIGHLIGHT } else { COLOR_DIM };
        draw_rectangle(x, y, bar_w * valor, bar_h, fill_color);
        draw_rectangle_lines(x, y, bar_w, bar_h, 1.0, COLOR_BORDER);

        let pct = format!("{}%", (valor * 100.0) as i32);
        let ptw = measure_text(&pct, Some(&self.font), fs_v, 1.0).width;
        draw_text_ex(&pct, (sw - ptw) / 2.0, y + bar_h + 6.0 + text_height(&self.font, fs_v), TextParams {
            font: Some(&self.font), font_size: fs_v, color: COLOR_TEXT_DIM, ..Default::default()
        });
    }

    fn render_mapa_completo(&self, estado: &Estado) {
        clear_background(COLOR_BG_DARK);
        let sw = screen_width();
        let sh = screen_height();
        let st = safe_top();

        let titulo = "Mapa";
        let fs_t = fs_adaptativo(titulo, &self.font, fs_pct(0.04), sw * 0.8);
        let th_t = text_height(&self.font, fs_t);
        let title_y = st + 10.0 + th_t;
        let tw = measure_text(titulo, Some(&self.font), fs_t, 1.0).width;
        draw_text_ex(titulo, (sw - tw) / 2.0, title_y, TextParams {
            font: Some(&self.font), font_size: fs_t, color: COLOR_ACCENT, ..Default::default()
        });

        let mapa_top = title_y + 12.0;
        let mapa_bottom = sh - 55.0;
        let mapa_h = mapa_bottom - mapa_top;
        let cell = (mapa_h / MAPA_ROWS as f32).min((sw * 0.88) / MAPA_COLS as f32);
        let mapa_w = cell * MAPA_COLS as f32;
        let ox = (sw - mapa_w) / 2.0;
        let oy = mapa_top;

        // Conexiones
        for escena in Escena::TODAS {
            let (c, r) = escena.pos_mapa();
            let cx = ox + c as f32 * cell + cell / 2.0;
            let cy = oy + r as f32 * cell + cell / 2.0;
            for conexion in escena.conexiones().iter().flatten() {
                let (c2, r2) = conexion.pos_mapa();
                let cx2 = ox + c2 as f32 * cell + cell / 2.0;
                let cy2 = oy + r2 as f32 * cell + cell / 2.0;
                let color = if estado.visitadas.contains(escena) && estado.visitadas.contains(conexion) { COLOR_DIM } else { Color::new(0.2, 0.2, 0.2, 0.5) };
                draw_line(cx, cy, cx2, cy2, 2.0, color);
            }
        }

        let fs_n = (cell * 0.22).max(8.0) as u16;
        for escena in Escena::TODAS {
            let (c, r) = escena.pos_mapa();
            let x = ox + c as f32 * cell;
            let y = oy + r as f32 * cell;
            let size = cell * 0.65;
            let nx = x + (cell - size) / 2.0;
            let ny = y + (cell - size) / 2.0;

            let (bg, border) = if *escena == estado.mapa_cursor { (COLOR_HIGHLIGHT, COLOR_ACCENT) }
                else if *escena == estado.escena { (COLOR_GREEN, COLOR_ACCENT) }
                else if estado.visitadas.contains(escena) { (COLOR_BG_ALT, COLOR_BORDER) }
                else { (Color::new(0.15, 0.15, 0.15, 0.8), Color::new(0.3, 0.3, 0.3, 0.5)) };

            draw_rectangle(nx, ny, size, size, bg);
            draw_rectangle_lines(nx, ny, size, size, 2.0, border);

            let letra = escena.letra();
            let ltw = measure_text(letra, Some(&self.font), fs_n, 1.0).width;
            draw_text_ex(letra, nx + (size - ltw) / 2.0, ny + size * 0.62, TextParams {
                font: Some(&self.font), font_size: fs_n, color: COLOR_TEXT, ..Default::default()
            });
        }

        // Info cursor
        let cursor_name = estado.mapa_cursor.nombre();
        let fs_info = fs_adaptativo(cursor_name, &self.font, fs_pct(0.03), sw * 0.9);
        let inf_tw = measure_text(cursor_name, Some(&self.font), fs_info, 1.0).width;
        draw_text_ex(cursor_name, (sw - inf_tw) / 2.0, sh - 30.0, TextParams {
            font: Some(&self.font), font_size: fs_info, color: COLOR_TEXT, ..Default::default()
        });

        let hint = "Flechas: mover  Z: ir  X: cerrar";
        let fs_h = fs_adaptativo(hint, &self.font, fs_pct(0.022), sw * 0.95);
        let htw = measure_text(hint, Some(&self.font), fs_h, 1.0).width;
        draw_text_ex(hint, (sw - htw) / 2.0, sh - 8.0, TextParams {
            font: Some(&self.font), font_size: fs_h, color: COLOR_DIM, ..Default::default()
        });
    }

    fn render_libreta_completa(&self, estado: &Estado) {
        clear_background(COLOR_BG_DARK);
        let sw = screen_width();
        let sh = screen_height();
        let st = safe_top();
        let lib = &estado.libreta;

        let titulo = "Libreta de Campo";
        let fs_t = fs_adaptativo(titulo, &self.font, fs_pct(0.04), sw * 0.9);
        let th_t = text_height(&self.font, fs_t);
        let title_y = st + 10.0 + th_t;
        let tw = measure_text(titulo, Some(&self.font), fs_t, 1.0).width;
        draw_text_ex(titulo, (sw - tw) / 2.0, title_y, TextParams {
            font: Some(&self.font), font_size: fs_t, color: COLOR_ACCENT, ..Default::default()
        });

        let total = lib.entradas.len();
        if total == 0 {
            let msg = "Aun no has descubierto animales.";
            let fs_m = fs_adaptativo(msg, &self.font, fs_pct(0.028), sw * 0.9);
            let mtw = measure_text(msg, Some(&self.font), fs_m, 1.0).width;
            draw_text_ex(msg, (sw - mtw) / 2.0, sh * 0.5, TextParams {
                font: Some(&self.font), font_size: fs_m, color: COLOR_TEXT_DIM, ..Default::default()
            });
        } else {
            let por_pagina = 5;
            let pagina = lib.pagina;
            let inicio = pagina * por_pagina;
            let fin = (inicio + por_pagina).min(total);

            let y_start = title_y + 20.0;
            let available_h = sh - y_start - 60.0;
            let item_h = available_h / por_pagina as f32;
            let fs_name = fs_adaptativo("Nombre largo animal", &self.font, fs_pct(0.028), sw * 0.7);
            let th_name = text_height(&self.font, fs_name);

            for (i, idx) in (inicio..fin).enumerate() {
                let entry = &lib.entradas[idx];
                let y = y_start + i as f32 * item_h;

                // Inicial del animal
                let inicial = entry.nombre.chars().next().unwrap_or('?').to_uppercase().to_string();
                let fs_ini = fs_name;
                draw_text_ex(&inicial, 15.0, y + th_name, TextParams {
                    font: Some(&self.font), font_size: fs_ini, color: COLOR_GREEN, ..Default::default()
                });

                let name_fs = fs_adaptativo(&entry.nombre, &self.font, fs_name, sw - 60.0);
                draw_text_ex(&entry.nombre, 40.0, y + th_name, TextParams {
                    font: Some(&self.font), font_size: name_fs, color: COLOR_TEXT, ..Default::default()
                });

                // Separador
                let sep_y = y + item_h - 4.0;
                draw_line(15.0, sep_y, sw - 15.0, sep_y, 1.0, Color::new(0.3, 0.3, 0.3, 0.4));
            }

            let total_paginas = (total + por_pagina - 1) / por_pagina;
            let pag_texto = format!("Pagina {} / {}", pagina + 1, total_paginas);
            let fs_p = fs_pct(0.022);
            let ptw = measure_text(&pag_texto, Some(&self.font), fs_p, 1.0).width;
            draw_text_ex(&pag_texto, (sw - ptw) / 2.0, sh - 35.0, TextParams {
                font: Some(&self.font), font_size: fs_p, color: COLOR_TEXT_DIM, ..Default::default()
            });
        }

        let hint = "Flechas: Pagina  X: Cerrar";
        let fs_h = fs_adaptativo(hint, &self.font, fs_pct(0.022), sw * 0.9);
        let htw = measure_text(hint, Some(&self.font), fs_h, 1.0).width;
        draw_text_ex(hint, (sw - htw) / 2.0, sh - 10.0, TextParams {
            font: Some(&self.font), font_size: fs_h, color: COLOR_DIM, ..Default::default()
        });
    }

    fn render_juego(&self, estado: &Estado, fondos: &Fondos) {
        let sw = screen_width();
        let sh = screen_height();
        let bar_h = bar_height();
        let st = safe_top();
        let sb = safe_bottom();

        let content_top = st + bar_h;
        let content_bottom = sb;

        // Fondo
        let tinte = estado.ciclo.tinte();
        fondos.draw(&estado.escena, tinte, content_top, content_bottom);

        // Barra superior
        self.render_barra_superior(estado, sw, bar_h, st);

        match &estado.modo {
            ModoVista::Normal => self.render_normal(estado, content_top, content_bottom),
            ModoVista::Seleccion { animales, indice } => self.render_seleccion(animales, *indice, content_top, content_bottom),
            ModoVista::ViendoAnimal { animal, texto_pos, terminado, .. } => {
                self.render_animal_info(animal, *texto_pos, *terminado, content_top, content_bottom);
            }
            ModoVista::Foto { animales, indice_actual, celda, foto_tomada, texto_pos, terminado, ya_vistos, .. } => {
                self.render_foto(animales, *indice_actual, *celda, *foto_tomada, *texto_pos, *terminado, ya_vistos, content_top, content_bottom);
            }
        }

        if estado.pesca.activo { self.render_pesca(estado, content_top, content_bottom); }
        if estado.museo.activo { self.render_museo(estado, content_top, content_bottom); }
        if estado.dialogo.activo { self.render_dialogo(estado); }

        // Eventos (solo si no hay minimapa tapando)
        if let Some(ref evento) = estado.eventos.evento_actual {
            self.render_evento(evento, estado.eventos.mostrar_info, content_bottom);
        }

        // Overlay dia/noche
        let alpha = estado.ciclo.overlay_alpha();
        if alpha > 0.0 { draw_rectangle(0.0, 0.0, sw, sh, Color::new(0.0, 0.0, 0.15, alpha)); }

        // Transicion
        if estado.en_transicion() {
            let a = estado.alpha_transicion();
            draw_rectangle(0.0, 0.0, sw, sh, Color::new(0.0, 0.0, 0.0, a));
        }

        // Minimapa solo si no hay evento
        if estado.eventos.evento_actual.is_none() {
            self.render_minimapa(estado, content_bottom);
        }
    }

    fn render_barra_superior(&self, estado: &Estado, sw: f32, bar_h: f32, st: f32) {
        draw_rectangle(0.0, st, sw, bar_h, COLOR_BAR_BG);
        let fs = fs_adaptativo(estado.escena.nombre(), &self.font, fs_pct(0.03), sw * 0.65);

        draw_text_ex(estado.escena.nombre(), 8.0, st + bar_h * 0.72, TextParams {
            font: Some(&self.font), font_size: fs, color: COLOR_TEXT, ..Default::default()
        });

        // Fase del dia a la derecha
        let fase = estado.ciclo.nombre_fase();
        let fs_f = fs_pct(0.02);
        let ftw = measure_text(fase, Some(&self.font), fs_f, 1.0).width;
        draw_text_ex(fase, sw - ftw - 8.0, st + bar_h * 0.72, TextParams {
            font: Some(&self.font), font_size: fs_f, color: COLOR_DIM, ..Default::default()
        });
    }

    fn render_normal(&self, estado: &Estado, content_top: f32, content_bottom: f32) {
        let sw = screen_width();
        let mid_y = (content_top + content_bottom) / 2.0;
        let conns = estado.escena.conexiones();
        let fs_arrow = fs_pct(0.05);

        if conns[0].is_some() {
            let a = "^"; let aw = measure_text(a, Some(&self.font), fs_arrow, 1.0).width;
            draw_text_ex(a, (sw - aw) / 2.0, content_top + 50.0, TextParams { font: Some(&self.font), font_size: fs_arrow, color: COLOR_HIGHLIGHT, ..Default::default() });
        }
        if conns[1].is_some() {
            let a = "v"; let aw = measure_text(a, Some(&self.font), fs_arrow, 1.0).width;
            draw_text_ex(a, (sw - aw) / 2.0, content_bottom - 20.0, TextParams { font: Some(&self.font), font_size: fs_arrow, color: COLOR_HIGHLIGHT, ..Default::default() });
        }
        if conns[2].is_some() {
            draw_text_ex("<", 10.0, mid_y, TextParams { font: Some(&self.font), font_size: fs_arrow, color: COLOR_HIGHLIGHT, ..Default::default() });
        }
        if conns[3].is_some() {
            let a = ">"; let aw = measure_text(a, Some(&self.font), fs_arrow, 1.0).width;
            draw_text_ex(a, sw - aw - 10.0, mid_y, TextParams { font: Some(&self.font), font_size: fs_arrow, color: COLOR_HIGHLIGHT, ..Default::default() });
        }

        if !estado.escena.es_entrada() {
            let hint = "Z: Explorar";
            let fs_h = fs_pct(0.025);
            let htw = measure_text(hint, Some(&self.font), fs_h, 1.0).width;
            draw_text_ex(hint, (sw - htw) / 2.0, content_bottom - 5.0, TextParams { font: Some(&self.font), font_size: fs_h, color: COLOR_DIM, ..Default::default() });
        }
    }

    fn render_seleccion(&self, animales: &[crate::db::Animal], indice: usize, content_top: f32, content_bottom: f32) {
        let sw = screen_width();
        draw_rectangle(0.0, content_top, sw, content_bottom - content_top, Color::new(0.0, 0.0, 0.0, 0.8));

        let fs_name = fs_adaptativo("Nombre Largo", &self.font, fs_pct(0.035), sw * 0.75);
        let th = text_height(&self.font, fs_name);
        let item_h = th * 2.2;
        let y_start = content_top + 20.0;

        for (i, animal) in animales.iter().enumerate() {
            let selected = i == indice;
            let color = if selected { COLOR_ACCENT } else { COLOR_TEXT };
            let y = y_start + i as f32 * item_h;

            // Inicial del animal
            let inicial = animal.nombre_comun.chars().next().unwrap_or('?').to_uppercase().to_string();
            draw_text_ex(&inicial, 15.0, y + th, TextParams {
                font: Some(&self.font), font_size: fs_name, color: if selected { COLOR_WARM } else { COLOR_DIM }, ..Default::default()
            });

            let prefix = if selected { "> " } else { "  " };
            let texto = format!("{}{}", prefix, animal.nombre_comun);
            let txt_fs = fs_adaptativo(&texto, &self.font, fs_name, sw - 50.0);
            draw_text_ex(&texto, 40.0, y + th, TextParams {
                font: Some(&self.font), font_size: txt_fs, color, ..Default::default()
            });
        }

        let hint = "Z: Ver  X: Volver";
        let fs_h = fs_pct(0.022);
        let htw = measure_text(hint, Some(&self.font), fs_h, 1.0).width;
        draw_text_ex(hint, (sw - htw) / 2.0, content_bottom - 8.0, TextParams { font: Some(&self.font), font_size: fs_h, color: COLOR_DIM, ..Default::default() });
    }

    fn render_animal_info(&self, animal: &crate::db::Animal, texto_pos: usize, terminado: bool, content_top: f32, content_bottom: f32) {
        let sw = screen_width();
        draw_rectangle(0.0, content_top, sw, content_bottom - content_top, Color::new(0.0, 0.0, 0.0, 0.9));

        let available_h = content_bottom - content_top;

        // Cuadro para imagen (placeholder)
        let img_h = available_h * 0.22;
        let img_w = sw * 0.5;
        let img_x = (sw - img_w) / 2.0;
        let img_y = content_top + available_h * 0.03;
        draw_rectangle(img_x, img_y, img_w, img_h, COLOR_BG_ALT);
        draw_rectangle_lines(img_x, img_y, img_w, img_h, 1.0, COLOR_BORDER);
        let ph = "[ imagen ]";
        let fs_ph = fs_pct(0.02);
        let phw = measure_text(ph, Some(&self.font), fs_ph, 1.0).width;
        draw_text_ex(ph, img_x + (img_w - phw) / 2.0, img_y + img_h * 0.55, TextParams {
            font: Some(&self.font), font_size: fs_ph, color: COLOR_DIM, ..Default::default()
        });

        // Nombre + inicial
        let name_y = img_y + img_h + available_h * 0.04;
        let fs_name = fs_adaptativo(&animal.nombre_comun, &self.font, fs_pct(0.04), sw * 0.8);
        let ntw = measure_text(&animal.nombre_comun, Some(&self.font), fs_name, 1.0).width;
        let th_name = text_height(&self.font, fs_name);

        // Inicial a la izquierda del nombre
        let inicial = animal.nombre_comun.chars().next().unwrap_or('?').to_uppercase().to_string();
        let ini_fs = fs_name;
        let ini_w = measure_text(&inicial, Some(&self.font), ini_fs, 1.0).width;
        let total_name_w = ini_w + 8.0 + ntw;
        let name_x = (sw - total_name_w) / 2.0;

        draw_text_ex(&inicial, name_x, name_y + th_name, TextParams {
            font: Some(&self.font), font_size: ini_fs, color: COLOR_WARM, ..Default::default()
        });
        draw_text_ex(&animal.nombre_comun, name_x + ini_w + 8.0, name_y + th_name, TextParams {
            font: Some(&self.font), font_size: fs_name, color: COLOR_ACCENT, ..Default::default()
        });

        // Cientifico
        let sci_y = name_y + th_name + 6.0;
        let fs_sci = fs_adaptativo(&animal.nombre_cientifico, &self.font, fs_pct(0.025), sw * 0.8);
        let stw = measure_text(&animal.nombre_cientifico, Some(&self.font), fs_sci, 1.0).width;
        let th_sci = text_height(&self.font, fs_sci);
        draw_text_ex(&animal.nombre_cientifico, (sw - stw) / 2.0, sci_y + th_sci, TextParams {
            font: Some(&self.font), font_size: fs_sci, color: COLOR_TEXT_DIM, ..Default::default()
        });

        // Iconos placeholder: [S] sonido  [I] icono
        let icon_y = sci_y + th_sci + 8.0;
        let fs_icon = fs_pct(0.022);
        let th_icon = text_height(&self.font, fs_icon);
        let s_text = "[S]";
        let i_text = "[I]";
        let s_w = measure_text(s_text, Some(&self.font), fs_icon, 1.0).width;
        let i_w = measure_text(i_text, Some(&self.font), fs_icon, 1.0).width;
        let icons_w = s_w + 20.0 + i_w;
        let icons_x = (sw - icons_w) / 2.0;
        draw_text_ex(s_text, icons_x, icon_y + th_icon, TextParams { font: Some(&self.font), font_size: fs_icon, color: COLOR_HIGHLIGHT, ..Default::default() });
        draw_text_ex(i_text, icons_x + s_w + 20.0, icon_y + th_icon, TextParams { font: Some(&self.font), font_size: fs_icon, color: COLOR_WARM, ..Default::default() });

        // Separador
        let sep_y = icon_y + th_icon + 8.0;
        draw_line(sw * 0.1, sep_y, sw * 0.9, sep_y, 1.0, COLOR_BORDER);

        // Descripcion
        let fs_desc = fs_pct(0.026);
        let desc_text: String = animal.descripcion.chars().take(texto_pos).collect();
        let desc_top = sep_y + 8.0;
        let desc_bottom = content_bottom - 30.0;
        self.render_texto_wrapped(&desc_text, sw * 0.08, desc_top, sw * 0.84, desc_bottom, fs_desc, COLOR_INFO_TEXT);

        // Hint
        let hint = if terminado { "Z: Sonido  X: Volver" } else { "Z: Completar  X: Volver" };
        let fs_h = fs_adaptativo(hint, &self.font, fs_pct(0.022), sw * 0.95);
        let htw = measure_text(hint, Some(&self.font), fs_h, 1.0).width;
        draw_text_ex(hint, (sw - htw) / 2.0, content_bottom - 8.0, TextParams { font: Some(&self.font), font_size: fs_h, color: COLOR_DIM, ..Default::default() });
    }

    fn render_foto(&self, animales: &[crate::db::Animal], indice: usize, celda: usize,
                    foto_tomada: bool, texto_pos: usize, terminado: bool,
                    ya_vistos: &std::collections::HashSet<usize>,
                    content_top: f32, content_bottom: f32) {
        let sw = screen_width();
        let animal = &animales[indice];

        if !foto_tomada {
            let mid_y = (content_top + content_bottom) / 2.0;
            let grid_size = ((content_bottom - content_top) * 0.4).min(sw * 0.35);
            let gx = (sw - grid_size * 2.0) / 2.0;
            let gy = mid_y - grid_size;

            for c in 0..4_usize {
                let cx = gx + (c % 2) as f32 * grid_size;
                let cy = gy + (c / 2) as f32 * grid_size;
                let color = if c == celda { COLOR_GREEN } else { COLOR_BG_ALT };
                draw_rectangle(cx, cy, grid_size - 4.0, grid_size - 4.0, color);
                draw_rectangle_lines(cx, cy, grid_size - 4.0, grid_size - 4.0, 2.0, COLOR_BORDER);
                if c == celda {
                    let fs_b = fs_pct(0.06);
                    let btw = measure_text("?", Some(&self.font), fs_b, 1.0).width;
                    draw_text_ex("?", cx + (grid_size - 4.0 - btw) / 2.0, cy + grid_size * 0.6, TextParams { font: Some(&self.font), font_size: fs_b, color: COLOR_TEXT, ..Default::default() });
                }
            }

            let count = format!("{}/{}", ya_vistos.len(), animales.len());
            let fs_c = fs_pct(0.022);
            draw_text_ex(&count, 15.0, content_top + 25.0, TextParams { font: Some(&self.font), font_size: fs_c, color: COLOR_TEXT_DIM, ..Default::default() });

            let hint = "Z: Fotografiar  X: Salir";
            let fs_h = fs_pct(0.022);
            let htw = measure_text(hint, Some(&self.font), fs_h, 1.0).width;
            draw_text_ex(hint, (sw - htw) / 2.0, content_bottom - 8.0, TextParams { font: Some(&self.font), font_size: fs_h, color: COLOR_DIM, ..Default::default() });
        } else {
            self.render_animal_info(animal, texto_pos, terminado, content_top, content_bottom);
        }
    }

    fn render_pesca(&self, estado: &Estado, content_top: f32, content_bottom: f32) {
        let sw = screen_width();
        let pesca = &estado.pesca;
        draw_rectangle(0.0, content_top, sw, content_bottom - content_top, Color::new(0.0, 0.05, 0.1, 0.9));

        let fs_p = fs_pct(0.035);
        let fs_h = fs_pct(0.022);
        let mid_y = (content_top + content_bottom) / 2.0;

        match pesca.fase {
            FasePesca::Esperando => {
                let t = "Esperando picada...";
                let fs = fs_adaptativo(t, &self.font, fs_p, sw * 0.9);
                let tw = measure_text(t, Some(&self.font), fs, 1.0).width;
                draw_text_ex(t, (sw - tw) / 2.0, mid_y, TextParams { font: Some(&self.font), font_size: fs, color: COLOR_TEXT, ..Default::default() });
                let wave = (pesca.timer * 2.0).sin() * 0.5 + 0.5;
                let dots = match (wave * 3.0) as usize { 0 => "~", 1 => "~~", _ => "~~~" };
                let dtw = measure_text(dots, Some(&self.font), fs_p, 1.0).width;
                draw_text_ex(dots, (sw - dtw) / 2.0, mid_y + 30.0, TextParams { font: Some(&self.font), font_size: fs_p, color: COLOR_HIGHLIGHT, ..Default::default() });
            }
            FasePesca::Picando => {
                let t = "!! PICA !! Z!";
                let fs = fs_adaptativo(t, &self.font, fs_p, sw * 0.9);
                let tw = measure_text(t, Some(&self.font), fs, 1.0).width;
                let blink = (get_time() * 6.0) as i32 % 2 == 0;
                let color = if blink { COLOR_ACCENT } else { COLOR_DANGER };
                draw_text_ex(t, (sw - tw) / 2.0, mid_y, TextParams { font: Some(&self.font), font_size: fs, color, ..Default::default() });
                let bar_w = sw * 0.6; let bar_h = 8.0; let bar_x = (sw - bar_w) / 2.0; let bar_y = mid_y + 25.0;
                let progress = 1.0 - (pesca.timer / pesca.tiempo_picada).min(1.0);
                draw_rectangle(bar_x, bar_y, bar_w, bar_h, COLOR_BAR_BG);
                draw_rectangle(bar_x, bar_y, bar_w * progress, bar_h, COLOR_DANGER);
            }
            FasePesca::Resultado => {
                let t = "Se escapo...";
                let fs = fs_adaptativo(t, &self.font, fs_p, sw * 0.9);
                let tw = measure_text(t, Some(&self.font), fs, 1.0).width;
                draw_text_ex(t, (sw - tw) / 2.0, mid_y, TextParams { font: Some(&self.font), font_size: fs, color: COLOR_DANGER, ..Default::default() });
                let hint = "Z: Siguiente";
                let htw = measure_text(hint, Some(&self.font), fs_h, 1.0).width;
                draw_text_ex(hint, (sw - htw) / 2.0, content_bottom - 8.0, TextParams { font: Some(&self.font), font_size: fs_h, color: COLOR_DIM, ..Default::default() });
            }
            FasePesca::InfoPez => {
                if let Some(ref pez) = pesca.pez_actual {
                    let y_s = content_top + 15.0;
                    let fs_n = fs_adaptativo(&pez.nombre, &self.font, fs_pct(0.04), sw * 0.85);
                    let th_n = text_height(&self.font, fs_n);
                    let ntw = measure_text(&pez.nombre, Some(&self.font), fs_n, 1.0).width;
                    draw_text_ex(&pez.nombre, (sw - ntw) / 2.0, y_s + th_n, TextParams { font: Some(&self.font), font_size: fs_n, color: COLOR_ACCENT, ..Default::default() });

                    let fs_sci = fs_pct(0.025);
                    let th_sci = text_height(&self.font, fs_sci);
                    let stw = measure_text(&pez.cientifico, Some(&self.font), fs_sci, 1.0).width;
                    draw_text_ex(&pez.cientifico, (sw - stw) / 2.0, y_s + th_n + 4.0 + th_sci, TextParams { font: Some(&self.font), font_size: fs_sci, color: COLOR_TEXT_DIM, ..Default::default() });

                    let peso = format!("{:.1} kg", pez.peso_kg);
                    let ptw = measure_text(&peso, Some(&self.font), fs_sci, 1.0).width;
                    draw_text_ex(&peso, (sw - ptw) / 2.0, y_s + th_n + 4.0 + th_sci * 2.0 + 4.0, TextParams { font: Some(&self.font), font_size: fs_sci, color: COLOR_WARM, ..Default::default() });

                    let sep_y = y_s + th_n + th_sci * 2.0 + 20.0;
                    draw_line(sw * 0.1, sep_y, sw * 0.9, sep_y, 1.0, COLOR_BORDER);

                    let desc: String = pez.descripcion.chars().take(pesca.texto_pos).collect();
                    let fs_desc = fs_pct(0.026);
                    self.render_texto_wrapped(&desc, sw * 0.08, sep_y + 8.0, sw * 0.84, content_bottom - 30.0, fs_desc, COLOR_INFO_TEXT);

                    let hint = if pesca.texto_terminado { "Z: Siguiente" } else { "Z: Completar" };
                    let htw = measure_text(hint, Some(&self.font), fs_h, 1.0).width;
                    draw_text_ex(hint, (sw - htw) / 2.0, content_bottom - 8.0, TextParams { font: Some(&self.font), font_size: fs_h, color: COLOR_DIM, ..Default::default() });
                }
            }
        }
    }

    fn render_museo(&self, estado: &Estado, content_top: f32, content_bottom: f32) {
        let sw = screen_width();
        let museo = &estado.museo;
        draw_rectangle(0.0, content_top, sw, content_bottom - content_top, Color::new(0.05, 0.03, 0.0, 0.92));
        let fs_h = fs_pct(0.022);

        match museo.fase {
            FaseMuseo::Entrada => {
                let mid_y = (content_top + content_bottom) / 2.0;
                let t = "Museo Paleontologico";
                let fs = fs_adaptativo(t, &self.font, fs_pct(0.04), sw * 0.9);
                let tw = measure_text(t, Some(&self.font), fs, 1.0).width;
                draw_text_ex(t, (sw - tw) / 2.0, mid_y - 15.0, TextParams { font: Some(&self.font), font_size: fs, color: COLOR_ACCENT, ..Default::default() });
                let hint = "Z: Entrar  X: Salir";
                let htw = measure_text(hint, Some(&self.font), fs_h, 1.0).width;
                draw_text_ex(hint, (sw - htw) / 2.0, content_bottom - 8.0, TextParams { font: Some(&self.font), font_size: fs_h, color: COLOR_DIM, ..Default::default() });
            }
            FaseMuseo::Explorando => {
                let fs_item = fs_adaptativo("Quiz Paleontologico", &self.font, fs_pct(0.03), sw * 0.75);
                let th = text_height(&self.font, fs_item);
                let item_h = th * 2.0;
                let y_start = content_top + 20.0;

                for (i, dino) in museo.exhibiciones.iter().enumerate() {
                    let sel = i == museo.indice;
                    let color = if sel { COLOR_ACCENT } else { COLOR_TEXT };
                    let prefix = if sel { "> " } else { "  " };
                    let texto = format!("{}{}", prefix, dino.nombre);
                    draw_text_ex(&texto, 20.0, y_start + i as f32 * item_h + th, TextParams { font: Some(&self.font), font_size: fs_item, color, ..Default::default() });
                }
                let exc_idx = museo.exhibiciones.len();
                let sel = museo.indice == exc_idx;
                let color = if sel { COLOR_WARM } else { COLOR_TEXT_DIM };
                let prefix = if sel { "> " } else { "  " };
                draw_text_ex(&format!("{}Excavar Fosil", prefix), 20.0, y_start + exc_idx as f32 * item_h + th, TextParams { font: Some(&self.font), font_size: fs_item, color, ..Default::default() });
                let quiz_idx = exc_idx + 1;
                let sel = museo.indice == quiz_idx;
                let color = if sel { COLOR_SPECIAL } else { COLOR_TEXT_DIM };
                let prefix = if sel { "> " } else { "  " };
                draw_text_ex(&format!("{}Quiz", prefix), 20.0, y_start + quiz_idx as f32 * item_h + th, TextParams { font: Some(&self.font), font_size: fs_item, color, ..Default::default() });

                let hint = "Z: Seleccionar  X: Salir";
                let htw = measure_text(hint, Some(&self.font), fs_h, 1.0).width;
                draw_text_ex(hint, (sw - htw) / 2.0, content_bottom - 8.0, TextParams { font: Some(&self.font), font_size: fs_h, color: COLOR_DIM, ..Default::default() });
            }
            FaseMuseo::Excavando => { self.render_excavacion(museo, content_top, content_bottom); }
            FaseMuseo::FosilRevelado => { self.render_fosil_revelado(museo, content_top, content_bottom); }
            FaseMuseo::ViendoExhibicion => {
                let dino = museo.dino_actual();
                let fs_n = fs_adaptativo(&dino.nombre, &self.font, fs_pct(0.04), sw * 0.85);
                let th_n = text_height(&self.font, fs_n);
                let ntw = measure_text(&dino.nombre, Some(&self.font), fs_n, 1.0).width;
                draw_text_ex(&dino.nombre, (sw - ntw) / 2.0, content_top + 20.0 + th_n, TextParams { font: Some(&self.font), font_size: fs_n, color: COLOR_ACCENT, ..Default::default() });

                let sep_y = content_top + 20.0 + th_n + 10.0;
                draw_line(sw * 0.1, sep_y, sw * 0.9, sep_y, 1.0, COLOR_BORDER);

                let desc: String = dino.descripcion.chars().take(museo.texto_pos).collect();
                let fs_desc = fs_pct(0.026);
                self.render_texto_wrapped(&desc, sw * 0.08, sep_y + 8.0, sw * 0.84, content_bottom - 30.0, fs_desc, COLOR_INFO_TEXT);

                let hint = if museo.terminado_texto { "Z: Volver" } else { "Z: Completar" };
                let htw = measure_text(hint, Some(&self.font), fs_h, 1.0).width;
                draw_text_ex(hint, (sw - htw) / 2.0, content_bottom - 8.0, TextParams { font: Some(&self.font), font_size: fs_h, color: COLOR_DIM, ..Default::default() });
            }
            FaseMuseo::Quiz => { self.render_quiz(museo, content_top, content_bottom); }
        }
    }

    fn render_excavacion(&self, museo: &MinijuegoMuseo, content_top: f32, content_bottom: f32) {
        let sw = screen_width();
        let fs_h = fs_pct(0.022);

        let golpes = format!("Golpes: {}/{}", museo.golpes_restantes, museo.max_golpes);
        let fs_g = fs_pct(0.022);
        let gtw = measure_text(&golpes, Some(&self.font), fs_g, 1.0).width;
        draw_text_ex(&golpes, (sw - gtw) / 2.0, content_top + 20.0, TextParams { font: Some(&self.font), font_size: fs_g, color: COLOR_TEXT_DIM, ..Default::default() });

        let grid_top = content_top + 30.0;
        let grid_bottom = content_bottom - 30.0;
        let available_h = grid_bottom - grid_top;
        let available_w = sw * 0.88;
        let cell_size = (available_h / museo.grilla_rows as f32).min(available_w / museo.grilla_cols as f32);
        let grid_w = cell_size * museo.grilla_cols as f32;
        let grid_h = cell_size * museo.grilla_rows as f32;
        let ox = (sw - grid_w) / 2.0;
        let oy = grid_top + (available_h - grid_h) / 2.0;

        for row in 0..museo.grilla_rows {
            for col in 0..museo.grilla_cols {
                let x = ox + col as f32 * cell_size;
                let y = oy + row as f32 * cell_size;
                let is_cursor = col == museo.cursor_x && row == museo.cursor_y;
                let (bg, bc) = match museo.grilla[row][col] {
                    CeldaExcavacion::Roca(3) => (P_BROWN, P_DARK3),
                    CeldaExcavacion::Roca(2) => (P_WARM_BROWN, P_MID_BROWN),
                    CeldaExcavacion::Roca(1) => (P_TAN, P_KHAKI),
                    CeldaExcavacion::Roca(_) => (P_BROWN, P_DARK3),
                    CeldaExcavacion::Fosil => (P_GOLD, P_AMBER),
                    CeldaExcavacion::Vacio => (P_DARK2, P_DARK3),
                };
                draw_rectangle(x + 1.0, y + 1.0, cell_size - 2.0, cell_size - 2.0, bg);
                if is_cursor { draw_rectangle_lines(x, y, cell_size, cell_size, 3.0, COLOR_ACCENT); }
                else { draw_rectangle_lines(x + 1.0, y + 1.0, cell_size - 2.0, cell_size - 2.0, 1.0, bc); }

                if museo.grilla[row][col] == CeldaExcavacion::Fosil {
                    let fs_i = (cell_size * 0.4) as u16;
                    let itw = measure_text("*", Some(&self.font), fs_i, 1.0).width;
                    draw_text_ex("*", x + (cell_size - itw) / 2.0, y + cell_size * 0.65, TextParams { font: Some(&self.font), font_size: fs_i, color: COLOR_BG_DARK, ..Default::default() });
                }
                if let CeldaExcavacion::Roca(c) = museo.grilla[row][col] {
                    let num = format!("{}", c);
                    let fs_n = (cell_size * 0.25) as u16;
                    let ntw = measure_text(&num, Some(&self.font), fs_n, 1.0).width;
                    draw_text_ex(&num, x + (cell_size - ntw) / 2.0, y + cell_size * 0.6, TextParams { font: Some(&self.font), font_size: fs_n, color: Color::new(1.0, 1.0, 1.0, 0.3), ..Default::default() });
                }
            }
        }

        let hint = "Flechas: mover  Z: golpear  X: salir";
        let fs_hi = fs_adaptativo(hint, &self.font, fs_h, sw * 0.95);
        let htw = measure_text(hint, Some(&self.font), fs_hi, 1.0).width;
        draw_text_ex(hint, (sw - htw) / 2.0, content_bottom - 8.0, TextParams { font: Some(&self.font), font_size: fs_hi, color: COLOR_DIM, ..Default::default() });
    }

    fn render_fosil_revelado(&self, museo: &MinijuegoMuseo, content_top: f32, content_bottom: f32) {
        let sw = screen_width();
        let fs_h = fs_pct(0.022);

        if let Some(ref dino) = museo.dino_excavado {
            let titulo = if museo.fosil_encontrado { "Fosil Completo!" } else { "Excavacion terminada" };
            let fs_ti = fs_adaptativo(titulo, &self.font, fs_pct(0.04), sw * 0.9);
            let th = text_height(&self.font, fs_ti);
            let tw = measure_text(titulo, Some(&self.font), fs_ti, 1.0).width;
            let color = if museo.fosil_encontrado { COLOR_ACCENT } else { COLOR_DANGER };
            draw_text_ex(titulo, (sw - tw) / 2.0, content_top + 20.0 + th, TextParams { font: Some(&self.font), font_size: fs_ti, color, ..Default::default() });

            if museo.fosil_encontrado {
                let desc: String = dino.descripcion.chars().take(museo.texto_pos).collect();
                let fs_desc = fs_pct(0.026);
                self.render_texto_wrapped(&desc, sw * 0.08, content_top + 20.0 + th + 20.0, sw * 0.84, content_bottom - 30.0, fs_desc, COLOR_INFO_TEXT);
            }

            let hint = "Z: Continuar";
            let htw = measure_text(hint, Some(&self.font), fs_h, 1.0).width;
            draw_text_ex(hint, (sw - htw) / 2.0, content_bottom - 8.0, TextParams { font: Some(&self.font), font_size: fs_h, color: COLOR_DIM, ..Default::default() });
        }
    }

    fn render_quiz(&self, museo: &MinijuegoMuseo, content_top: f32, content_bottom: f32) {
        let sw = screen_width();
        let fs_q = fs_pct(0.028);
        let fs_h = fs_pct(0.022);
        let th = text_height(&self.font, fs_q);

        self.render_texto_wrapped(&museo.quiz_pregunta, sw * 0.05, content_top + 20.0, sw * 0.9, content_top + 100.0, fs_q, COLOR_TEXT);

        let y_opts = content_top + 110.0;
        let opt_h = th * 2.0;
        for (i, opt) in museo.quiz_opciones.iter().enumerate() {
            let sel = i == museo.quiz_seleccion;
            let color = if museo.quiz_respondida {
                if i == museo.quiz_correcta { COLOR_GREEN }
                else if sel && !museo.quiz_correcta_resp { COLOR_DANGER }
                else { COLOR_TEXT_DIM }
            } else if sel { COLOR_ACCENT } else { COLOR_TEXT };
            let prefix = if sel { "> " } else { "  " };
            draw_text_ex(&format!("{}{}", prefix, opt), 30.0, y_opts + i as f32 * opt_h + th, TextParams { font: Some(&self.font), font_size: fs_q, color, ..Default::default() });
        }

        if museo.quiz_respondida {
            let msg = if museo.quiz_correcta_resp { "Correcto!" } else { "Incorrecto" };
            let color = if museo.quiz_correcta_resp { COLOR_GREEN } else { COLOR_DANGER };
            let fs_r = fs_pct(0.035);
            let mtw = measure_text(msg, Some(&self.font), fs_r, 1.0).width;
            draw_text_ex(msg, (sw - mtw) / 2.0, content_bottom - 40.0, TextParams { font: Some(&self.font), font_size: fs_r, color, ..Default::default() });
        }

        let hint = if museo.quiz_respondida { "Z: Siguiente" } else { "Z: Responder  X: Volver" };
        let htw = measure_text(hint, Some(&self.font), fs_h, 1.0).width;
        draw_text_ex(hint, (sw - htw) / 2.0, content_bottom - 8.0, TextParams { font: Some(&self.font), font_size: fs_h, color: COLOR_DIM, ..Default::default() });
    }

    fn render_dialogo(&self, estado: &Estado) {
        let sw = screen_width();
        let sh = screen_height();
        let sb = safe_bottom();

        let box_h = sh * 0.18;
        let box_y = sb - box_h - 8.0;
        let box_x = 8.0;
        let box_w = sw - 16.0;

        draw_rectangle(box_x, box_y, box_w, box_h, COLOR_DIALOG_BG);
        draw_rectangle_lines(box_x, box_y, box_w, box_h, 2.0, COLOR_BORDER);

        let padding = 10.0;
        let inner_w = box_w - padding * 2.0;

        // Personaje
        let personaje = estado.dialogo.personaje_actual();
        let fs_p = fs_adaptativo(personaje, &self.font, fs_pct(0.028), inner_w);
        let th_p = text_height(&self.font, fs_p);
        draw_text_ex(personaje, box_x + padding, box_y + padding + th_p, TextParams {
            font: Some(&self.font), font_size: fs_p, color: COLOR_ACCENT, ..Default::default()
        });

        // Texto con scroll
        let texto = estado.dialogo.texto_visible();
        let fs_d = fs_pct(0.024);
        let text_top = box_y + padding + th_p + 6.0;
        let text_bottom = box_y + box_h - padding - 15.0;
        let lines = self.word_wrap(&texto, fs_d, inner_w);
        let line_h = text_height(&self.font, fs_d) * 1.2;
        let max_visible_lines = ((text_bottom - text_top) / line_h) as usize;

        // Si hay mas lineas que las visibles, scrollear
        let start_line = if lines.len() > max_visible_lines { lines.len() - max_visible_lines } else { 0 };

        let mut cy = text_top;
        for line in lines.iter().skip(start_line) {
            if cy + line_h > text_bottom { break; }
            cy += line_h;
            draw_text_ex(line, box_x + padding, cy, TextParams {
                font: Some(&self.font), font_size: fs_d, color: COLOR_TEXT, ..Default::default()
            });
        }

        // Indicador continuar
        if estado.dialogo.terminado_linea {
            let fs_ind = fs_pct(0.02);
            draw_text_ex("v", box_x + box_w - 20.0, box_y + box_h - 8.0, TextParams {
                font: Some(&self.font), font_size: fs_ind, color: COLOR_ACCENT, ..Default::default()
            });
        }
    }

    fn render_evento(&self, evento: &crate::eventos::EventoAleatorio, mostrar_info: bool, content_bottom: f32) {
        let sw = screen_width();

        if mostrar_info {
            let box_h = content_bottom * 0.15;
            let box_y = content_bottom - box_h - 8.0;
            draw_rectangle(8.0, box_y, sw - 16.0, box_h, COLOR_DIALOG_BG);
            draw_rectangle_lines(8.0, box_y, sw - 16.0, box_h, 2.0, COLOR_BORDER);

            let fs_t = fs_adaptativo(&evento.texto, &self.font, fs_pct(0.028), sw - 40.0);
            let th = text_height(&self.font, fs_t);
            draw_text_ex(&evento.texto, 18.0, box_y + 8.0 + th, TextParams { font: Some(&self.font), font_size: fs_t, color: COLOR_ACCENT, ..Default::default() });

            let fs_d = fs_pct(0.022);
            self.render_texto_wrapped(&evento.detalle, 18.0, box_y + 8.0 + th + 8.0, sw - 36.0, box_y + box_h - 8.0, fs_d, COLOR_TEXT);
        } else {
            let banner_h = 28.0;
            let banner_y = content_bottom - banner_h - 8.0;
            draw_rectangle(8.0, banner_y, sw - 16.0, banner_h, COLOR_DIALOG_BG);
            draw_rectangle_lines(8.0, banner_y, sw - 16.0, banner_h, 2.0, COLOR_WARM);

            let fs_e = fs_adaptativo(&evento.texto, &self.font, fs_pct(0.022), sw * 0.4);
            draw_text_ex(&evento.texto, 15.0, banner_y + banner_h * 0.7, TextParams { font: Some(&self.font), font_size: fs_e, color: COLOR_WARM, ..Default::default() });

            let hint = "Z: Ver  X: Cerrar";
            let hint_fs = fs_adaptativo(hint, &self.font, fs_pct(0.02), sw * 0.4);
            let htw = measure_text(hint, Some(&self.font), hint_fs, 1.0).width;
            draw_text_ex(hint, sw - htw - 15.0, banner_y + banner_h * 0.7, TextParams { font: Some(&self.font), font_size: hint_fs, color: COLOR_DIM, ..Default::default() });
        }
    }

    fn render_minimapa(&self, estado: &Estado, content_bottom: f32) {
        let cell = mini_size();
        let gap = mini_gap();
        let total_w = MAPA_COLS as f32 * (cell + gap);
        let total_h = MAPA_ROWS as f32 * (cell + gap);
        let margin = 6.0;
        let ox = screen_width() - total_w - margin;
        let oy = content_bottom - total_h - margin;

        draw_rectangle(ox - 2.0, oy - 2.0, total_w + 4.0, total_h + 4.0, Color::new(0.0, 0.0, 0.0, 0.5));

        for escena in Escena::TODAS {
            let (c, r) = escena.pos_mapa();
            let x = ox + c as f32 * (cell + gap);
            let y = oy + r as f32 * (cell + gap);
            let color = if *escena == estado.escena { COLOR_ACCENT }
                else if estado.visitadas.contains(escena) { COLOR_DIM }
                else { Color::new(0.2, 0.2, 0.2, 0.4) };
            draw_rectangle(x, y, cell, cell, color);
        }
    }

    fn render_texto_wrapped(&self, texto: &str, x: f32, y: f32, max_w: f32, max_y: f32, fs: u16, color: Color) {
        let line_h = text_height(&self.font, fs) * 1.2;
        let mut cy = y;
        for line in self.word_wrap(texto, fs, max_w) {
            cy += line_h;
            if cy > max_y { break; }
            draw_text_ex(&line, x, cy, TextParams {
                font: Some(&self.font), font_size: fs, color, ..Default::default()
            });
        }
    }

    fn word_wrap(&self, texto: &str, fs: u16, max_w: f32) -> Vec<String> {
        let mut lines = Vec::new();
        let mut current_line = String::new();
        for word in texto.split_whitespace() {
            let test = if current_line.is_empty() { word.to_string() } else { format!("{} {}", current_line, word) };
            let w = measure_text(&test, Some(&self.font), fs, 1.0).width;
            if w > max_w && !current_line.is_empty() {
                lines.push(current_line);
                current_line = word.to_string();
            } else {
                current_line = test;
            }
        }
        if !current_line.is_empty() { lines.push(current_line); }
        lines
    }
}