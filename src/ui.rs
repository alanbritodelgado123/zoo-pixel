// src/ui.rs
use macroquad::prelude::*;
use crate::config::*;
use crate::estado::*;
use crate::escena::Escena;
use crate::minijuego::{FasePesca, FaseMuseo, CeldaExcavacion, MinijuegoMuseo};
use crate::ciclo_dia::ModoCiclo;

pub struct UiRenderer {
    pub font: Font,
}

impl UiRenderer {
    pub fn new(font: Font) -> Self {
        Self { font }
    }

    pub fn render(&self, estado: &Estado) {
        match estado.pantalla {
            Pantalla::Inicio => self.render_inicio(estado),
            Pantalla::Intro => self.render_intro(estado),
            Pantalla::Config => self.render_config(estado),
            Pantalla::MapaCompleto => self.render_mapa_completo(estado),
            Pantalla::LibretaCompleta => self.render_libreta_completa(estado),
            Pantalla::Juego => self.render_juego(estado),
        }
    }

    fn render_inicio(&self, estado: &Estado) {
        clear_background(COLOR_BG_DARK);
        let sw = screen_width();
        let sh = screen_height();
        let s = scale();

        let titulo = "Zoológico Nacional";
        let fs = fs_adaptativo(titulo, &self.font, fs_titulo(), sw * 0.9);
        let tw = measure_text(titulo, Some(&self.font), fs, 1.0).width;
        draw_text_ex(titulo, (sw - tw) / 2.0, sh * 0.25, TextParams {
            font: Some(&self.font), font_size: fs, color: COLOR_ACCENT, ..Default::default()
        });

        let sub = "Explora - Descubre - Aprende";
        let fs_sub = fs_subtitulo();
        let sw2 = measure_text(sub, Some(&self.font), fs_sub, 1.0).width;
        draw_text_ex(sub, (sw - sw2) / 2.0, sh * 0.25 + 40.0 * s, TextParams {
            font: Some(&self.font), font_size: fs_sub, color: COLOR_TEXT_DIM, ..Default::default()
        });

        let opciones = ["Explorar", "Modo Dia", "Modo Noche", "Configuracion"];
        let fs_m = fs_menu();
        let y_base = sh * 0.45;
        let gap = 45.0 * s;

        for (i, opt) in opciones.iter().enumerate() {
            let color = if i == estado.inicio_seleccion { COLOR_ACCENT } else { COLOR_TEXT };
            let prefix = if i == estado.inicio_seleccion { "> " } else { "  " };
            let texto = format!("{}{}", prefix, opt);
            let tw = measure_text(&texto, Some(&self.font), fs_m, 1.0).width;
            draw_text_ex(&texto, (sw - tw) / 2.0, y_base + i as f32 * gap, TextParams {
                font: Some(&self.font), font_size: fs_m, color, ..Default::default()
            });
        }

        let modo_texto = match estado.ciclo.modo() {
            ModoCiclo::Sistema => "Hora del sistema",
            ModoCiclo::DiaPermanente => "Dia permanente",
            ModoCiclo::NochePermanente => "Noche permanente",
        };
        let fs_small = fs_hint();
        let mtw = measure_text(modo_texto, Some(&self.font), fs_small, 1.0).width;
        draw_text_ex(modo_texto, (sw - mtw) / 2.0, sh * 0.88, TextParams {
            font: Some(&self.font), font_size: fs_small, color: COLOR_DIM, ..Default::default()
        });

        let hint = "Z: Seleccionar  Flechas: Navegar";
        let fs_hi = fs_adaptativo(hint, &self.font, fs_small, sw * 0.9);
        let htw = measure_text(hint, Some(&self.font), fs_hi, 1.0).width;
        draw_text_ex(hint, (sw - htw) / 2.0, sh * 0.94, TextParams {
            font: Some(&self.font), font_size: fs_hi, color: COLOR_DIM, ..Default::default()
        });
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
        let s = scale();
        let mc = &estado.menu_config;

        let titulo = "Configuracion";
        let fs_t = fs_sel_title();
        let tw = measure_text(titulo, Some(&self.font), fs_t, 1.0).width;
        draw_text_ex(titulo, (sw - tw) / 2.0, sh * 0.15, TextParams {
            font: Some(&self.font), font_size: fs_t, color: COLOR_ACCENT, ..Default::default()
        });

        let fs_c = fs_config();
        let y_base = sh * 0.30;
        let item_height = 70.0 * s;

        for (i, opt) in MenuConfig::OPCIONES.iter().enumerate() {
            let selected = i == mc.seleccion;
            let color = if selected { COLOR_ACCENT } else { COLOR_TEXT };
            let prefix = if selected { "> " } else { "  " };
            let texto = format!("{}{}", prefix, opt);
            let y = y_base + i as f32 * item_height;

            let fs_txt = fs_adaptativo(&texto, &self.font, fs_c, sw * 0.9);
            let ttw = measure_text(&texto, Some(&self.font), fs_txt, 1.0).width;
            draw_text_ex(&texto, (sw - ttw) / 2.0, y, TextParams {
                font: Some(&self.font), font_size: fs_txt, color, ..Default::default()
            });

            match i {
                0 => self.render_barra_volumen(sw, y + 25.0 * s, mc.volumen_musica, selected),
                1 => self.render_barra_volumen(sw, y + 25.0 * s, mc.volumen_efectos, selected),
                _ => {}
            }
        }

        let hint = "Flechas: Ajustar  Z/X: Volver";
        let fs_h = fs_hint();
        let htw = measure_text(hint, Some(&self.font), fs_h, 1.0).width;
        draw_text_ex(hint, (sw - htw) / 2.0, sh * 0.85, TextParams {
            font: Some(&self.font), font_size: fs_h, color: COLOR_DIM, ..Default::default()
        });
    }

    fn render_barra_volumen(&self, sw: f32, y: f32, valor: f32, selected: bool) {
        let s = scale();
        let bar_w = (sw * 0.5).min(300.0 * s);
        let bar_h = 12.0 * s;
        let x = (sw - bar_w) / 2.0;

        draw_rectangle(x, y, bar_w, bar_h, COLOR_BAR_BG);
        let fill_color = if selected { COLOR_HIGHLIGHT } else { COLOR_DIM };
        draw_rectangle(x, y, bar_w * valor, bar_h, fill_color);
        draw_rectangle_lines(x, y, bar_w, bar_h, 1.0, COLOR_BORDER);

        let pct = format!("{}%", (valor * 100.0) as i32);
        let fs_p = fs_config_val();
        let ptw = measure_text(&pct, Some(&self.font), fs_p, 1.0).width;
        draw_text_ex(&pct, (sw - ptw) / 2.0, y + bar_h + 18.0 * s, TextParams {
            font: Some(&self.font), font_size: fs_p, color: COLOR_TEXT_DIM, ..Default::default()
        });
    }

    fn render_mapa_completo(&self, estado: &Estado) {
        clear_background(COLOR_BG_DARK);
        let sw = screen_width();
        let sh = screen_height();
        let s = scale();
        let st = safe_top();

        let titulo = "Mapa del Zoologico";
        let fs_t = fs_adaptativo(titulo, &self.font, fs_sel_title(), sw * 0.9);
        let tw = measure_text(titulo, Some(&self.font), fs_t, 1.0).width;
        draw_text_ex(titulo, (sw - tw) / 2.0, st + 30.0 * s, TextParams {
            font: Some(&self.font), font_size: fs_t, color: COLOR_ACCENT, ..Default::default()
        });

        let mapa_top = st + 55.0 * s;
        let mapa_bottom = sh - 60.0 * s;
        let mapa_h = mapa_bottom - mapa_top;
        let cell = (mapa_h / MAPA_ROWS as f32).min(sw * 0.85 / MAPA_COLS as f32);
        let mapa_w = cell * MAPA_COLS as f32;
        let ox = (sw - mapa_w) / 2.0;
        let oy = mapa_top;

        for escena in Escena::TODAS {
            let (c, r) = escena.pos_mapa();
            let cx = ox + c as f32 * cell + cell / 2.0;
            let cy = oy + r as f32 * cell + cell / 2.0;

            for conexion in escena.conexiones().iter().flatten() {
                let (c2, r2) = conexion.pos_mapa();
                let cx2 = ox + c2 as f32 * cell + cell / 2.0;
                let cy2 = oy + r2 as f32 * cell + cell / 2.0;
                let color = if estado.visitadas.contains(escena) && estado.visitadas.contains(conexion) {
                    COLOR_DIM
                } else {
                    Color::new(0.2, 0.2, 0.2, 0.5)
                };
                draw_line(cx, cy, cx2, cy2, 2.0, color);
            }
        }

        let fs_n = (cell * 0.25).max(10.0) as u16;
        for escena in Escena::TODAS {
            let (c, r) = escena.pos_mapa();
            let x = ox + c as f32 * cell;
            let y = oy + r as f32 * cell;
            let size = cell * 0.7;
            let nx = x + (cell - size) / 2.0;
            let ny = y + (cell - size) / 2.0;

            let (bg, border) = if *escena == estado.mapa_cursor {
                (COLOR_HIGHLIGHT, COLOR_ACCENT)
            } else if *escena == estado.escena {
                (COLOR_GREEN, COLOR_ACCENT)
            } else if estado.visitadas.contains(escena) {
                (COLOR_BG_ALT, COLOR_BORDER)
            } else {
                (Color::new(0.15, 0.15, 0.15, 0.8), Color::new(0.3, 0.3, 0.3, 0.5))
            };

            draw_rectangle(nx, ny, size, size, bg);
            draw_rectangle_lines(nx, ny, size, size, 2.0, border);

            let letra = escena.letra();
            let ltw = measure_text(letra, Some(&self.font), fs_n, 1.0).width;
            draw_text_ex(letra, nx + (size - ltw) / 2.0, ny + size * 0.6, TextParams {
                font: Some(&self.font), font_size: fs_n, color: COLOR_TEXT, ..Default::default()
            });
        }

        let cursor_name = estado.mapa_cursor.nombre();
        let fs_info = fs_adaptativo(cursor_name, &self.font, fs_sel_name(), sw * 0.9);
        let inf_tw = measure_text(cursor_name, Some(&self.font), fs_info, 1.0).width;
        draw_text_ex(cursor_name, (sw - inf_tw) / 2.0, sh - 35.0 * s, TextParams {
            font: Some(&self.font), font_size: fs_info, color: COLOR_TEXT, ..Default::default()
        });

        let hint = "Flechas: mover  Z: ir  X: cerrar";
        let fs_h = fs_adaptativo(hint, &self.font, fs_hint(), sw * 0.95);
        let htw = measure_text(hint, Some(&self.font), fs_h, 1.0).width;
        draw_text_ex(hint, (sw - htw) / 2.0, sh - 12.0 * s, TextParams {
            font: Some(&self.font), font_size: fs_h, color: COLOR_DIM, ..Default::default()
        });
    }

    fn render_libreta_completa(&self, estado: &Estado) {
        clear_background(COLOR_BG_DARK);
        let sw = screen_width();
        let sh = screen_height();
        let s = scale();
        let st = safe_top();
        let lib = &estado.libreta;

        let titulo = "Libreta de Campo";
        let fs_t = fs_adaptativo(titulo, &self.font, fs_sel_title(), sw * 0.9);
        let tw = measure_text(titulo, Some(&self.font), fs_t, 1.0).width;
        draw_text_ex(titulo, (sw - tw) / 2.0, st + 30.0 * s, TextParams {
            font: Some(&self.font), font_size: fs_t, color: COLOR_ACCENT, ..Default::default()
        });

        let total = lib.entradas.len();
        if total == 0 {
            let msg = "Aun no has descubierto animales.";
            let fs_m = fs_libreta();
            let mtw = measure_text(msg, Some(&self.font), fs_m, 1.0).width;
            draw_text_ex(msg, (sw - mtw) / 2.0, sh * 0.5, TextParams {
                font: Some(&self.font), font_size: fs_m, color: COLOR_TEXT_DIM, ..Default::default()
            });
        } else {
            let por_pagina = 6;
            let pagina = lib.pagina;
            let inicio = pagina * por_pagina;
            let fin = (inicio + por_pagina).min(total);

            let y_start = st + 60.0 * s;
            let item_h = (sh - y_start - 80.0 * s) / por_pagina as f32;
            let fs_name = fs_libreta();
            let fs_sci = fs_sel_sci();

            for (i, idx) in (inicio..fin).enumerate() {
                let entry = &lib.entradas[idx];
                let y = y_start + i as f32 * item_h;

                draw_text_ex("*", 20.0 * s, y + 5.0, TextParams {
                    font: Some(&self.font), font_size: fs_name, color: COLOR_GREEN, ..Default::default()
                });

                let name_fs = fs_adaptativo(&entry.nombre, &self.font, fs_name, sw - 60.0 * s);
                draw_text_ex(&entry.nombre, 45.0 * s, y + 5.0, TextParams {
                    font: Some(&self.font), font_size: name_fs, color: COLOR_TEXT, ..Default::default()
                });

                let sci_fs = fs_adaptativo(&entry.cientifico, &self.font, fs_sci, sw - 60.0 * s);
                draw_text_ex(&entry.cientifico, 45.0 * s, y + 25.0 * s, TextParams {
                    font: Some(&self.font), font_size: sci_fs, color: COLOR_TEXT_DIM, ..Default::default()
                });

                draw_line(20.0 * s, y + item_h - 5.0, sw - 20.0 * s, y + item_h - 5.0, 1.0, COLOR_DIM);
            }

            let total_paginas = (total + por_pagina - 1) / por_pagina;
            let pag_texto = format!("Pagina {} / {}", pagina + 1, total_paginas);
            let fs_p = fs_hint();
            let ptw = measure_text(&pag_texto, Some(&self.font), fs_p, 1.0).width;
            draw_text_ex(&pag_texto, (sw - ptw) / 2.0, sh - 40.0 * s, TextParams {
                font: Some(&self.font), font_size: fs_p, color: COLOR_TEXT_DIM, ..Default::default()
            });
        }

        let hint = "Flechas: Pagina  X: Cerrar";
        let fs_h = fs_hint();
        let htw = measure_text(hint, Some(&self.font), fs_h, 1.0).width;
        draw_text_ex(hint, (sw - htw) / 2.0, sh - 15.0 * s, TextParams {
            font: Some(&self.font), font_size: fs_h, color: COLOR_DIM, ..Default::default()
        });
    }

    fn render_juego(&self, estado: &Estado) {
        clear_background(estado.escena.color_fondo());

        let tinte = estado.ciclo.tinte();
        if tinte != WHITE {
            draw_rectangle(0.0, 0.0, screen_width(), screen_height(),
                Color::new(tinte.r * 0.3, tinte.g * 0.3, tinte.b * 0.3, 0.3));
        }

        let sw = screen_width();
        let sh = screen_height();
        let s = scale();
        let bar_h = bar_height();
        let st = safe_top();
        let sb = safe_bottom();

        self.render_barra_superior(estado, sw, bar_h, st);

        let content_top = st + bar_h;
        let content_bottom = sb;

        let nombre = estado.escena.nombre();
        let fs_p = fs_adaptativo(nombre, &self.font, fs_place(), sw * 0.9);
        let ntw = measure_text(nombre, Some(&self.font), fs_p, 1.0).width;
        draw_text_ex(nombre, (sw - ntw) / 2.0, content_top + 40.0 * s, TextParams {
            font: Some(&self.font), font_size: fs_p, color: COLOR_TEXT, ..Default::default()
        });

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

        if estado.pesca.activo {
            self.render_pesca(estado, content_top, content_bottom);
        }
        if estado.museo.activo {
            self.render_museo(estado, content_top, content_bottom);
        }

        if estado.dialogo.activo {
            self.render_dialogo(estado);
        }

        if let Some(ref evento) = estado.eventos.evento_actual {
            self.render_evento(evento, estado.eventos.mostrar_info, content_bottom);
        }

        let alpha = estado.ciclo.overlay_alpha();
        if alpha > 0.0 {
            draw_rectangle(0.0, 0.0, sw, sh, Color::new(0.0, 0.0, 0.15, alpha));
        }

        if estado.en_transicion() {
            let a = estado.alpha_transicion();
            draw_rectangle(0.0, 0.0, sw, sh, Color::new(0.0, 0.0, 0.0, a));
        }

        self.render_minimapa(estado, content_bottom);
        self.render_fase_dia(estado, content_bottom);
    }

    fn render_barra_superior(&self, estado: &Estado, sw: f32, bar_h: f32, st: f32) {
        draw_rectangle(0.0, st, sw, bar_h, COLOR_BAR_BG);
        let fs = fs_bar();
        let nombre = estado.escena.nombre();
        let name_fs = fs_adaptativo(nombre, &self.font, fs, sw * 0.6);
        draw_text_ex(nombre, 10.0, st + bar_h * 0.7, TextParams {
            font: Some(&self.font), font_size: name_fs, color: COLOR_TEXT, ..Default::default()
        });

        let visitadas = format!("{}/{}", estado.visitadas.len(), Escena::TODAS.len());
        let vtw = measure_text(&visitadas, Some(&self.font), fs, 1.0).width;
        draw_text_ex(&visitadas, sw - vtw - 10.0, st + bar_h * 0.7, TextParams {
            font: Some(&self.font), font_size: fs, color: COLOR_ACCENT, ..Default::default()
        });
    }

    fn render_normal(&self, estado: &Estado, content_top: f32, content_bottom: f32) {
        let sw = screen_width();
        let s = scale();
        let mid_y = (content_top + content_bottom) / 2.0;

        let conns = estado.escena.conexiones();
        let fs_arrow = fs_sel_title();
        let arrow_color = COLOR_HIGHLIGHT;

        if conns[0].is_some() {
            let atxt = "^";
            let atw = measure_text(atxt, Some(&self.font), fs_arrow, 1.0).width;
            draw_text_ex(atxt, (sw - atw) / 2.0, content_top + 70.0 * s, TextParams {
                font: Some(&self.font), font_size: fs_arrow, color: arrow_color, ..Default::default()
            });
        }
        if conns[1].is_some() {
            let atxt = "v";
            let atw = measure_text(atxt, Some(&self.font), fs_arrow, 1.0).width;
            draw_text_ex(atxt, (sw - atw) / 2.0, content_bottom - 30.0 * s, TextParams {
                font: Some(&self.font), font_size: fs_arrow, color: arrow_color, ..Default::default()
            });
        }
        if conns[2].is_some() {
            draw_text_ex("<", 15.0 * s, mid_y, TextParams {
                font: Some(&self.font), font_size: fs_arrow, color: arrow_color, ..Default::default()
            });
        }
        if conns[3].is_some() {
            let atxt = ">";
            let atw = measure_text(atxt, Some(&self.font), fs_arrow, 1.0).width;
            draw_text_ex(atxt, sw - atw - 15.0 * s, mid_y, TextParams {
                font: Some(&self.font), font_size: fs_arrow, color: arrow_color, ..Default::default()
            });
        }

        if !estado.escena.es_entrada() {
            let hint = "Z: Explorar";
            let fs_h = fs_hint();
            let htw = measure_text(hint, Some(&self.font), fs_h, 1.0).width;
            draw_text_ex(hint, (sw - htw) / 2.0, content_bottom - 10.0 * s, TextParams {
                font: Some(&self.font), font_size: fs_h, color: COLOR_DIM, ..Default::default()
            });
        }
    }

    fn render_seleccion(&self, animales: &[crate::db::Animal], indice: usize, content_top: f32, content_bottom: f32) {
        let sw = screen_width();
        let s = scale();
        let fs_name = fs_sel_name();
        let fs_sci = fs_sel_sci();

        let y_start = content_top + 60.0 * s;
        let available_h = content_bottom - y_start - 40.0 * s;
        let item_h = (available_h / animales.len().max(1) as f32).min(50.0 * s);

        for (i, animal) in animales.iter().enumerate() {
            let selected = i == indice;
            let color = if selected { COLOR_ACCENT } else { COLOR_TEXT };
            let prefix = if selected { "> " } else { "  " };
            let y = y_start + i as f32 * item_h;

            let texto = format!("{}{}", prefix, animal.nombre_comun);
            let txt_fs = fs_adaptativo(&texto, &self.font, fs_name, sw - 40.0 * s);
            draw_text_ex(&texto, 30.0 * s, y, TextParams {
                font: Some(&self.font), font_size: txt_fs, color, ..Default::default()
            });

            if selected {
                let sci_fs = fs_adaptativo(&animal.nombre_cientifico, &self.font, fs_sci, sw - 60.0 * s);
                draw_text_ex(&animal.nombre_cientifico, 50.0 * s, y + 22.0 * s, TextParams {
                    font: Some(&self.font), font_size: sci_fs, color: COLOR_TEXT_DIM, ..Default::default()
                });
            }
        }

        let hint = "Z: Ver  X: Volver";
        let fs_h = fs_hint();
        let htw = measure_text(hint, Some(&self.font), fs_h, 1.0).width;
        draw_text_ex(hint, (sw - htw) / 2.0, content_bottom - 10.0 * s, TextParams {
            font: Some(&self.font), font_size: fs_h, color: COLOR_DIM, ..Default::default()
        });
    }

    fn render_animal_info(&self, animal: &crate::db::Animal, texto_pos: usize, terminado: bool, content_top: f32, content_bottom: f32) {
        let sw = screen_width();
        let s = scale();

        draw_rectangle(0.0, content_top, sw, content_bottom - content_top,
            Color::new(0.0, 0.0, 0.0, 0.85));

        let y_start = content_top + 20.0 * s;

        let fs_name = fs_adaptativo(&animal.nombre_comun, &self.font, fs_anim_name(), sw * 0.85);
        let ntw = measure_text(&animal.nombre_comun, Some(&self.font), fs_name, 1.0).width;
        draw_text_ex(&animal.nombre_comun, (sw - ntw) / 2.0, y_start + 30.0 * s, TextParams {
            font: Some(&self.font), font_size: fs_name, color: COLOR_ACCENT, ..Default::default()
        });

        let fs_sci = fs_adaptativo(&animal.nombre_cientifico, &self.font, fs_anim_sci(), sw * 0.85);
        let stw = measure_text(&animal.nombre_cientifico, Some(&self.font), fs_sci, 1.0).width;
        draw_text_ex(&animal.nombre_cientifico, (sw - stw) / 2.0, y_start + 55.0 * s, TextParams {
            font: Some(&self.font), font_size: fs_sci, color: COLOR_TEXT_DIM, ..Default::default()
        });

        let sep_y = y_start + 70.0 * s;
        draw_line(sw * 0.15, sep_y, sw * 0.85, sep_y, 1.0, COLOR_BORDER);

        let fs_desc = fs_anim_desc();
        let desc_text: String = animal.descripcion.chars().take(texto_pos).collect();
        let max_w = sw * 0.8;
        let desc_top = sep_y + 15.0 * s;
        let desc_bottom = content_bottom - 40.0 * s;
        self.render_texto_wrapped(&desc_text, sw * 0.1, desc_top, max_w, desc_bottom, fs_desc, COLOR_TEXT);

        let hint = if terminado { "Z: Volver" } else { "Z: Completar" };
        let fs_h = fs_hint();
        let htw = measure_text(hint, Some(&self.font), fs_h, 1.0).width;
        draw_text_ex(hint, (sw - htw) / 2.0, content_bottom - 10.0 * s, TextParams {
            font: Some(&self.font), font_size: fs_h, color: COLOR_DIM, ..Default::default()
        });
    }

    fn render_foto(&self, animales: &[crate::db::Animal], indice: usize, celda: usize,
                    foto_tomada: bool, texto_pos: usize, terminado: bool,
                    ya_vistos: &std::collections::HashSet<usize>,
                    content_top: f32, content_bottom: f32) {
        let sw = screen_width();
        let s = scale();
        let animal = &animales[indice];

        if !foto_tomada {
            let mid_y = (content_top + content_bottom) / 2.0;
            let grid_size = ((content_bottom - content_top) * 0.5).min(sw * 0.4);
            let gx = (sw - grid_size * 2.0) / 2.0;
            let gy = mid_y - grid_size;

            for c in 0..4_usize {
                let cx = gx + (c % 2) as f32 * grid_size;
                let cy = gy + (c / 2) as f32 * grid_size;
                let color = if c == celda { COLOR_GREEN } else { COLOR_BG_ALT };
                draw_rectangle(cx, cy, grid_size - 4.0, grid_size - 4.0, color);
                draw_rectangle_lines(cx, cy, grid_size - 4.0, grid_size - 4.0, 2.0, COLOR_BORDER);

                if c == celda {
                    let bird = "?";
                    let fs_b = fs_foto_bird();
                    let btw = measure_text(bird, Some(&self.font), fs_b, 1.0).width;
                    draw_text_ex(bird, cx + (grid_size - 4.0 - btw) / 2.0, cy + grid_size * 0.6, TextParams {
                        font: Some(&self.font), font_size: fs_b, color: COLOR_TEXT, ..Default::default()
                    });
                }
            }

            let count = format!("{}/{}", ya_vistos.len(), animales.len());
            let fs_c = fs_foto_count();
            draw_text_ex(&count, 20.0 * s, content_top + 30.0 * s, TextParams {
                font: Some(&self.font), font_size: fs_c, color: COLOR_TEXT_DIM, ..Default::default()
            });

            let hint = "Z: Fotografiar  X: Salir";
            let fs_h = fs_hint();
            let htw = measure_text(hint, Some(&self.font), fs_h, 1.0).width;
            draw_text_ex(hint, (sw - htw) / 2.0, content_bottom - 10.0 * s, TextParams {
                font: Some(&self.font), font_size: fs_h, color: COLOR_DIM, ..Default::default()
            });
        } else {
            self.render_animal_info(animal, texto_pos, terminado, content_top, content_bottom);
        }
    }

    fn render_pesca(&self, estado: &Estado, content_top: f32, content_bottom: f32) {
        let sw = screen_width();
        let s = scale();
        let pesca = &estado.pesca;

        draw_rectangle(0.0, content_top, sw, content_bottom - content_top,
            Color::new(0.0, 0.05, 0.1, 0.9));

        let fs_p = fs_pesca();
        let fs_h = fs_hint();
        let mid_y = (content_top + content_bottom) / 2.0;

        match pesca.fase {
            FasePesca::Esperando => {
                let texto = "Esperando picada...";
                let fs_t = fs_adaptativo(texto, &self.font, fs_p, sw * 0.9);
                let tw = measure_text(texto, Some(&self.font), fs_t, 1.0).width;
                draw_text_ex(texto, (sw - tw) / 2.0, mid_y, TextParams {
                    font: Some(&self.font), font_size: fs_t, color: COLOR_TEXT, ..Default::default()
                });

                let wave = (pesca.timer * 2.0).sin() * 0.5 + 0.5;
                let dots = match (wave * 3.0) as usize {
                    0 => "~", 1 => "~~", _ => "~~~",
                };
                let dtw = measure_text(dots, Some(&self.font), fs_p, 1.0).width;
                draw_text_ex(dots, (sw - dtw) / 2.0, mid_y + 35.0 * s, TextParams {
                    font: Some(&self.font), font_size: fs_p, color: COLOR_HIGHLIGHT, ..Default::default()
                });

                let intentos = format!("Intento {}/{}", pesca.intentos + 1, pesca.max_intentos);
                let itw = measure_text(&intentos, Some(&self.font), fs_h, 1.0).width;
                draw_text_ex(&intentos, (sw - itw) / 2.0, content_top + 30.0 * s, TextParams {
                    font: Some(&self.font), font_size: fs_h, color: COLOR_TEXT_DIM, ..Default::default()
                });
            }
            FasePesca::Picando => {
                let texto = "!! PICA !! Presiona Z!";
                let fs_t = fs_adaptativo(texto, &self.font, fs_p, sw * 0.9);
                let tw = measure_text(texto, Some(&self.font), fs_t, 1.0).width;

                let blink = (get_time() * 6.0) as i32 % 2 == 0;
                let color = if blink { COLOR_ACCENT } else { COLOR_DANGER };

                draw_text_ex(texto, (sw - tw) / 2.0, mid_y, TextParams {
                    font: Some(&self.font), font_size: fs_t, color, ..Default::default()
                });

                let bar_w = sw * 0.6;
                let bar_h = 10.0 * s;
                let bar_x = (sw - bar_w) / 2.0;
                let bar_y = mid_y + 30.0 * s;
                let progress = 1.0 - (pesca.timer / pesca.tiempo_picada).min(1.0);
                draw_rectangle(bar_x, bar_y, bar_w, bar_h, COLOR_BAR_BG);
                draw_rectangle(bar_x, bar_y, bar_w * progress, bar_h, COLOR_DANGER);
            }
            FasePesca::Resultado => {
                let texto = "Se escapo...";
                let fs_t = fs_adaptativo(texto, &self.font, fs_p, sw * 0.9);
                let tw = measure_text(texto, Some(&self.font), fs_t, 1.0).width;
                draw_text_ex(texto, (sw - tw) / 2.0, mid_y, TextParams {
                    font: Some(&self.font), font_size: fs_t, color: COLOR_DANGER, ..Default::default()
                });

                let hint = "Z: Siguiente";
                let htw = measure_text(hint, Some(&self.font), fs_h, 1.0).width;
                draw_text_ex(hint, (sw - htw) / 2.0, content_bottom - 10.0 * s, TextParams {
                    font: Some(&self.font), font_size: fs_h, color: COLOR_DIM, ..Default::default()
                });
            }
            FasePesca::InfoPez => {
                if let Some(ref pez) = pesca.pez_actual {
                    let y_start = content_top + 20.0 * s;

                    let fs_name = fs_adaptativo(&pez.nombre, &self.font, fs_anim_name(), sw * 0.85);
                    let ntw = measure_text(&pez.nombre, Some(&self.font), fs_name, 1.0).width;
                    draw_text_ex(&pez.nombre, (sw - ntw) / 2.0, y_start + 20.0 * s, TextParams {
                        font: Some(&self.font), font_size: fs_name, color: COLOR_ACCENT, ..Default::default()
                    });

                    let fs_sci = fs_anim_sci();
                    let stw = measure_text(&pez.cientifico, Some(&self.font), fs_sci, 1.0).width;
                    draw_text_ex(&pez.cientifico, (sw - stw) / 2.0, y_start + 45.0 * s, TextParams {
                        font: Some(&self.font), font_size: fs_sci, color: COLOR_TEXT_DIM, ..Default::default()
                    });

                    let peso = format!("{:.1} kg", pez.peso_kg);
                    let ptw = measure_text(&peso, Some(&self.font), fs_sci, 1.0).width;
                    draw_text_ex(&peso, (sw - ptw) / 2.0, y_start + 65.0 * s, TextParams {
                        font: Some(&self.font), font_size: fs_sci, color: COLOR_WARM, ..Default::default()
                    });

                    let sep_y = y_start + 80.0 * s;
                    draw_line(sw * 0.15, sep_y, sw * 0.85, sep_y, 1.0, COLOR_BORDER);

                    let desc: String = pez.descripcion.chars().take(pesca.texto_pos).collect();
                    let fs_desc = fs_anim_desc();
                    self.render_texto_wrapped(&desc, sw * 0.1, sep_y + 15.0 * s, sw * 0.8, content_bottom - 40.0 * s, fs_desc, COLOR_TEXT);

                    let hint = if pesca.texto_terminado { "Z: Siguiente" } else { "Z: Completar" };
                    let htw = measure_text(hint, Some(&self.font), fs_h, 1.0).width;
                    draw_text_ex(hint, (sw - htw) / 2.0, content_bottom - 10.0 * s, TextParams {
                        font: Some(&self.font), font_size: fs_h, color: COLOR_DIM, ..Default::default()
                    });
                }
            }
        }
    }

    fn render_museo(&self, estado: &Estado, content_top: f32, content_bottom: f32) {
        let sw = screen_width();
        let s = scale();
        let museo = &estado.museo;

        draw_rectangle(0.0, content_top, sw, content_bottom - content_top,
            Color::new(0.05, 0.03, 0.0, 0.92));

        let fs_t = fs_sel_title();
        let fs_h = fs_hint();

        match museo.fase {
            FaseMuseo::Entrada => {
                let titulo = "Museo Paleontologico";
                let fs_ti = fs_adaptativo(titulo, &self.font, fs_t, sw * 0.9);
                let tw = measure_text(titulo, Some(&self.font), fs_ti, 1.0).width;
                let mid_y = (content_top + content_bottom) / 2.0;
                draw_text_ex(titulo, (sw - tw) / 2.0, mid_y - 20.0 * s, TextParams {
                    font: Some(&self.font), font_size: fs_ti, color: COLOR_ACCENT, ..Default::default()
                });

                let sub = "Descubre criaturas del pasado";
                let fs_s = fs_subtitulo();
                let stw = measure_text(sub, Some(&self.font), fs_s, 1.0).width;
                draw_text_ex(sub, (sw - stw) / 2.0, mid_y + 15.0 * s, TextParams {
                    font: Some(&self.font), font_size: fs_s, color: COLOR_TEXT_DIM, ..Default::default()
                });

                let hint = "Z: Entrar  X: Salir";
                let htw = measure_text(hint, Some(&self.font), fs_h, 1.0).width;
                draw_text_ex(hint, (sw - htw) / 2.0, content_bottom - 10.0 * s, TextParams {
                    font: Some(&self.font), font_size: fs_h, color: COLOR_DIM, ..Default::default()
                });
            }
            FaseMuseo::Explorando => {
                let titulo = "Exhibiciones";
                let tw = measure_text(titulo, Some(&self.font), fs_t, 1.0).width;
                draw_text_ex(titulo, (sw - tw) / 2.0, content_top + 30.0 * s, TextParams {
                    font: Some(&self.font), font_size: fs_t, color: COLOR_ACCENT, ..Default::default()
                });

                let fs_item = fs_menu();
                let y_start = content_top + 65.0 * s;
                let item_h = 35.0 * s;

                for (i, dino) in museo.exhibiciones.iter().enumerate() {
                    let selected = i == museo.indice;
                    let color = if selected { COLOR_ACCENT } else { COLOR_TEXT };
                    let prefix = if selected { "> " } else { "  " };
                    let texto = format!("{}{}", prefix, dino.nombre);
                    let txt_fs = fs_adaptativo(&texto, &self.font, fs_item, sw - 40.0 * s);
                    draw_text_ex(&texto, 30.0 * s, y_start + i as f32 * item_h, TextParams {
                        font: Some(&self.font), font_size: txt_fs, color, ..Default::default()
                    });
                }

                let exc_idx = museo.exhibiciones.len();
                let selected = museo.indice == exc_idx;
                let color = if selected { COLOR_WARM } else { COLOR_TEXT_DIM };
                let prefix = if selected { "> " } else { "  " };
                let exc_text = format!("{}Excavar Fosil", prefix);
                let exc_fs = fs_adaptativo(&exc_text, &self.font, fs_item, sw - 40.0 * s);
                draw_text_ex(&exc_text, 30.0 * s, y_start + exc_idx as f32 * item_h, TextParams {
                    font: Some(&self.font), font_size: exc_fs, color, ..Default::default()
                });

                let quiz_idx = exc_idx + 1;
                let selected = museo.indice == quiz_idx;
                let color = if selected { COLOR_SPECIAL } else { COLOR_TEXT_DIM };
                let prefix = if selected { "> " } else { "  " };
                let quiz_text = format!("{}Quiz Paleontologico", prefix);
                let quiz_fs = fs_adaptativo(&quiz_text, &self.font, fs_item, sw - 40.0 * s);
                draw_text_ex(&quiz_text, 30.0 * s, y_start + quiz_idx as f32 * item_h, TextParams {
                    font: Some(&self.font), font_size: quiz_fs, color, ..Default::default()
                });

                let hint = "Z: Seleccionar  X: Salir";
                let htw = measure_text(hint, Some(&self.font), fs_h, 1.0).width;
                draw_text_ex(hint, (sw - htw) / 2.0, content_bottom - 10.0 * s, TextParams {
                    font: Some(&self.font), font_size: fs_h, color: COLOR_DIM, ..Default::default()
                });
            }
            FaseMuseo::Excavando => {
                self.render_excavacion(museo, content_top, content_bottom);
            }
            FaseMuseo::FosilRevelado => {
                self.render_fosil_revelado(museo, content_top, content_bottom);
            }
            FaseMuseo::ViendoExhibicion => {
                let dino = museo.dino_actual();
                let y_start = content_top + 20.0 * s;

                let fs_name = fs_adaptativo(&dino.nombre, &self.font, fs_anim_name(), sw * 0.85);
                let ntw = measure_text(&dino.nombre, Some(&self.font), fs_name, 1.0).width;
                draw_text_ex(&dino.nombre, (sw - ntw) / 2.0, y_start + 25.0 * s, TextParams {
                    font: Some(&self.font), font_size: fs_name, color: COLOR_ACCENT, ..Default::default()
                });

                let fs_sci = fs_adaptativo(&dino.cientifico, &self.font, fs_anim_sci(), sw * 0.85);
                let stw = measure_text(&dino.cientifico, Some(&self.font), fs_sci, 1.0).width;
                draw_text_ex(&dino.cientifico, (sw - stw) / 2.0, y_start + 48.0 * s, TextParams {
                    font: Some(&self.font), font_size: fs_sci, color: COLOR_TEXT_DIM, ..Default::default()
                });

                let era_t = format!("Era: {}", dino.era);
                let era_fs = fs_adaptativo(&era_t, &self.font, fs_anim_sci(), sw * 0.85);
                let etw = measure_text(&era_t, Some(&self.font), era_fs, 1.0).width;
                draw_text_ex(&era_t, (sw - etw) / 2.0, y_start + 68.0 * s, TextParams {
                    font: Some(&self.font), font_size: era_fs, color: COLOR_WARM, ..Default::default()
                });

                let sep_y = y_start + 80.0 * s;
                draw_line(sw * 0.15, sep_y, sw * 0.85, sep_y, 1.0, COLOR_BORDER);

                let desc: String = dino.descripcion.chars().take(museo.texto_pos).collect();
                let fs_desc = fs_anim_desc();
                self.render_texto_wrapped(&desc, sw * 0.1, sep_y + 15.0 * s, sw * 0.8, content_bottom - 40.0 * s, fs_desc, COLOR_TEXT);

                let hint = if museo.terminado_texto { "Z: Volver" } else { "Z: Completar" };
                let htw = measure_text(hint, Some(&self.font), fs_h, 1.0).width;
                draw_text_ex(hint, (sw - htw) / 2.0, content_bottom - 10.0 * s, TextParams {
                    font: Some(&self.font), font_size: fs_h, color: COLOR_DIM, ..Default::default()
                });
            }
            FaseMuseo::Quiz => {
                self.render_quiz(museo, content_top, content_bottom);
            }
        }
    }

    fn render_excavacion(&self, museo: &MinijuegoMuseo, content_top: f32, content_bottom: f32) {
        let sw = screen_width();
        let s = scale();

        let titulo = "Excavacion";
        let fs_t = fs_sel_name();
        let tw = measure_text(titulo, Some(&self.font), fs_t, 1.0).width;
        draw_text_ex(titulo, (sw - tw) / 2.0, content_top + 25.0 * s, TextParams {
            font: Some(&self.font), font_size: fs_t, color: COLOR_WARM, ..Default::default()
        });

        let golpes = format!("Golpes: {}/{}", museo.golpes_restantes, museo.max_golpes);
        let fs_g = fs_hint();
        let gtw = measure_text(&golpes, Some(&self.font), fs_g, 1.0).width;
        draw_text_ex(&golpes, (sw - gtw) / 2.0, content_top + 45.0 * s, TextParams {
            font: Some(&self.font), font_size: fs_g, color: COLOR_TEXT_DIM, ..Default::default()
        });

        let grid_top = content_top + 55.0 * s;
        let grid_bottom = content_bottom - 35.0 * s;
        let available_h = grid_bottom - grid_top;
        let available_w = sw * 0.85;
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

                let (bg, border_color) = match museo.grilla[row][col] {
                    CeldaExcavacion::Roca(3) => (P_BROWN, P_DARK3),
                    CeldaExcavacion::Roca(2) => (P_WARM_BROWN, P_MID_BROWN),
                    CeldaExcavacion::Roca(1) => (P_TAN, P_KHAKI),
                    CeldaExcavacion::Roca(_) => (P_BROWN, P_DARK3),
                    CeldaExcavacion::Fosil => (P_GOLD, P_AMBER),
                    CeldaExcavacion::Vacio => (P_DARK2, P_DARK3),
                };

                draw_rectangle(x + 1.0, y + 1.0, cell_size - 2.0, cell_size - 2.0, bg);

                if is_cursor {
                    draw_rectangle_lines(x, y, cell_size, cell_size, 3.0, COLOR_ACCENT);
                } else {
                    draw_rectangle_lines(x + 1.0, y + 1.0, cell_size - 2.0, cell_size - 2.0, 1.0, border_color);
                }

                if museo.grilla[row][col] == CeldaExcavacion::Fosil {
                    let icon = "*";
                    let fs_i = (cell_size * 0.5) as u16;
                    let itw = measure_text(icon, Some(&self.font), fs_i, 1.0).width;
                    draw_text_ex(icon, x + (cell_size - itw) / 2.0, y + cell_size * 0.7, TextParams {
                        font: Some(&self.font), font_size: fs_i, color: COLOR_BG_DARK, ..Default::default()
                    });
                }

                if let CeldaExcavacion::Roca(c) = museo.grilla[row][col] {
                    let num = format!("{}", c);
                    let fs_n = (cell_size * 0.3) as u16;
                    let ntw = measure_text(&num, Some(&self.font), fs_n, 1.0).width;
                    draw_text_ex(&num, x + (cell_size - ntw) / 2.0, y + cell_size * 0.65, TextParams {
                        font: Some(&self.font), font_size: fs_n, color: Color::new(1.0, 1.0, 1.0, 0.4), ..Default::default()
                    });
                }
            }
        }

        let hint = "Flechas: mover  Z: golpear  X: salir";
        let fs_h = fs_adaptativo(hint, &self.font, fs_hint(), sw * 0.95);
        let htw = measure_text(hint, Some(&self.font), fs_h, 1.0).width;
        draw_text_ex(hint, (sw - htw) / 2.0, content_bottom - 10.0 * s, TextParams {
            font: Some(&self.font), font_size: fs_h, color: COLOR_DIM, ..Default::default()
        });
    }

    fn render_fosil_revelado(&self, museo: &MinijuegoMuseo, content_top: f32, content_bottom: f32) {
        let sw = screen_width();
        let s = scale();

        if let Some(ref dino) = museo.dino_excavado {
            let titulo = if museo.fosil_encontrado {
                "Fosil Completo!"
            } else {
                "Excavacion terminada"
            };
            let fs_ti = fs_adaptativo(titulo, &self.font, fs_sel_title(), sw * 0.9);
            let tw = measure_text(titulo, Some(&self.font), fs_ti, 1.0).width;
            let color = if museo.fosil_encontrado { COLOR_ACCENT } else { COLOR_DANGER };
            draw_text_ex(titulo, (sw - tw) / 2.0, content_top + 30.0 * s, TextParams {
                font: Some(&self.font), font_size: fs_ti, color, ..Default::default()
            });

            if museo.fosil_encontrado {
                let fs_name = fs_adaptativo(&dino.nombre, &self.font, fs_anim_name(), sw * 0.85);
                let ntw = measure_text(&dino.nombre, Some(&self.font), fs_name, 1.0).width;
                draw_text_ex(&dino.nombre, (sw - ntw) / 2.0, content_top + 65.0 * s, TextParams {
                    font: Some(&self.font), font_size: fs_name, color: COLOR_WARM, ..Default::default()
                });

                let era_t = format!("Era: {}", dino.era);
                let fs_sci = fs_anim_sci();
                let etw = measure_text(&era_t, Some(&self.font), fs_sci, 1.0).width;
                draw_text_ex(&era_t, (sw - etw) / 2.0, content_top + 90.0 * s, TextParams {
                    font: Some(&self.font), font_size: fs_sci, color: COLOR_TEXT_DIM, ..Default::default()
                });

                let sep_y = content_top + 105.0 * s;
                draw_line(sw * 0.15, sep_y, sw * 0.85, sep_y, 1.0, COLOR_BORDER);

                let desc: String = dino.descripcion.chars().take(museo.texto_pos).collect();
                let fs_desc = fs_anim_desc();
                self.render_texto_wrapped(&desc, sw * 0.1, sep_y + 15.0 * s, sw * 0.8, content_bottom - 40.0 * s, fs_desc, COLOR_TEXT);
            } else {
                let msg = format!("Revelaste {}/{} celdas", museo.fosil_reveladas, museo.fosil_celdas.len());
                let fs_m = fs_adaptativo(&msg, &self.font, fs_subtitulo(), sw * 0.9);
                let mtw = measure_text(&msg, Some(&self.font), fs_m, 1.0).width;
                draw_text_ex(&msg, (sw - mtw) / 2.0, (content_top + content_bottom) / 2.0, TextParams {
                    font: Some(&self.font), font_size: fs_m, color: COLOR_TEXT_DIM, ..Default::default()
                });
            }

            let hint = "Z: Continuar";
            let fs_h = fs_hint();
            let htw = measure_text(hint, Some(&self.font), fs_h, 1.0).width;
            draw_text_ex(hint, (sw - htw) / 2.0, content_bottom - 10.0 * s, TextParams {
                font: Some(&self.font), font_size: fs_h, color: COLOR_DIM, ..Default::default()
            });
        }
    }

    fn render_quiz(&self, museo: &MinijuegoMuseo, content_top: f32, content_bottom: f32) {
        let sw = screen_width();
        let s = scale();

        let titulo = "Quiz Paleontologico";
        let fs_ti = fs_adaptativo(titulo, &self.font, fs_sel_title(), sw * 0.9);
        let tw = measure_text(titulo, Some(&self.font), fs_ti, 1.0).width;
        draw_text_ex(titulo, (sw - tw) / 2.0, content_top + 25.0 * s, TextParams {
            font: Some(&self.font), font_size: fs_ti, color: COLOR_SPECIAL, ..Default::default()
        });

        let fs_q = fs_quiz();
        let max_w = sw * 0.85;
        self.render_texto_wrapped(&museo.quiz_pregunta, sw * 0.075, content_top + 60.0 * s, max_w, content_top + 120.0 * s, fs_q, COLOR_TEXT);

        let y_opts = content_top + 130.0 * s;
        let opt_h = 35.0 * s;
        for (i, opt) in museo.quiz_opciones.iter().enumerate() {
            let selected = i == museo.quiz_seleccion;
            let color = if museo.quiz_respondida {
                if i == museo.quiz_correcta {
                    COLOR_GREEN
                } else if selected && !museo.quiz_correcta_resp {
                    COLOR_DANGER
                } else {
                    COLOR_TEXT_DIM
                }
            } else if selected {
                COLOR_ACCENT
            } else {
                COLOR_TEXT
            };
            let prefix = if selected { "> " } else { "  " };
            let opt_text = format!("{}{}", prefix, opt);
            let opt_fs = fs_adaptativo(&opt_text, &self.font, fs_q, sw - 50.0 * s);
            draw_text_ex(&opt_text, 40.0 * s, y_opts + i as f32 * opt_h, TextParams {
                font: Some(&self.font), font_size: opt_fs, color, ..Default::default()
            });
        }

        if museo.quiz_respondida {
            let msg = if museo.quiz_correcta_resp { "Correcto!" } else { "Incorrecto" };
            let color = if museo.quiz_correcta_resp { COLOR_GREEN } else { COLOR_DANGER };
            let fs_r = fs_pesca();
            let mtw = measure_text(msg, Some(&self.font), fs_r, 1.0).width;
            draw_text_ex(msg, (sw - mtw) / 2.0, content_bottom - 50.0 * s, TextParams {
                font: Some(&self.font), font_size: fs_r, color, ..Default::default()
            });
        }

        let hint = if museo.quiz_respondida { "Z: Siguiente" } else { "Z: Responder  X: Volver" };
        let fs_h = fs_adaptativo(hint, &self.font, fs_hint(), sw * 0.95);
        let htw = measure_text(hint, Some(&self.font), fs_h, 1.0).width;
        draw_text_ex(hint, (sw - htw) / 2.0, content_bottom - 10.0 * s, TextParams {
            font: Some(&self.font), font_size: fs_h, color: COLOR_DIM, ..Default::default()
        });
    }

    fn render_dialogo(&self, estado: &Estado) {
        let sw = screen_width();
        let s = scale();
        let sb = safe_bottom();

        let box_h = 120.0 * s;
        let box_y = sb - box_h - 10.0 * s;
        let box_x = 15.0 * s;
        let box_w = sw - 30.0 * s;

        draw_rectangle(box_x, box_y, box_w, box_h, COLOR_DIALOG_BG);
        draw_rectangle_lines(box_x, box_y, box_w, box_h, 2.0, COLOR_BORDER);

        let personaje = estado.dialogo.personaje_actual();
        let fs_p = fs_adaptativo(personaje, &self.font, fs_sel_name(), box_w - 30.0 * s);
        draw_text_ex(personaje, box_x + 15.0 * s, box_y + 25.0 * s, TextParams {
            font: Some(&self.font), font_size: fs_p, color: COLOR_ACCENT, ..Default::default()
        });

        let texto = estado.dialogo.texto_visible();
        let fs_d = fs_dialogo();
        let max_w = box_w - 30.0 * s;
        self.render_texto_wrapped(&texto, box_x + 15.0 * s, box_y + 50.0 * s, max_w, box_y + box_h - 15.0 * s, fs_d, COLOR_TEXT);

        if estado.dialogo.terminado_linea {
            let indicator = "v";
            let itw = measure_text(indicator, Some(&self.font), fs_d, 1.0).width;
            draw_text_ex(indicator, box_x + box_w - itw - 15.0 * s, box_y + box_h - 12.0 * s, TextParams {
                font: Some(&self.font), font_size: fs_d, color: COLOR_ACCENT, ..Default::default()
            });
        }
    }

    fn render_evento(&self, evento: &crate::eventos::EventoAleatorio, mostrar_info: bool, content_bottom: f32) {
        let sw = screen_width();
        let s = scale();

        if mostrar_info {
            let box_h = 100.0 * s;
            let box_y = content_bottom - box_h - 10.0 * s;
            draw_rectangle(10.0, box_y, sw - 20.0, box_h, COLOR_DIALOG_BG);
            draw_rectangle_lines(10.0, box_y, sw - 20.0, box_h, 2.0, COLOR_BORDER);

            let fs_t = fs_adaptativo(&evento.texto, &self.font, fs_sel_name(), sw - 50.0 * s);
            draw_text_ex(&evento.texto, 25.0 * s, box_y + 25.0 * s, TextParams {
                font: Some(&self.font), font_size: fs_t, color: COLOR_ACCENT, ..Default::default()
            });

            let fs_d = fs_evento();
            let max_w = sw - 50.0 * s;
            self.render_texto_wrapped(&evento.detalle, 25.0 * s, box_y + 45.0 * s, max_w, box_y + box_h - 10.0, fs_d, COLOR_TEXT);

            let hint = "Z/X: Cerrar";
            let fs_h = fs_hint();
            let htw = measure_text(hint, Some(&self.font), fs_h, 1.0).width;
            draw_text_ex(hint, sw - htw - 20.0, box_y - 5.0, TextParams {
                font: Some(&self.font), font_size: fs_h, color: COLOR_DIM, ..Default::default()
            });
        } else {
            let banner_h = 35.0 * s;
            let banner_y = content_bottom - banner_h - 10.0 * s;
            draw_rectangle(10.0, banner_y, sw - 20.0, banner_h, COLOR_DIALOG_BG);
            draw_rectangle_lines(10.0, banner_y, sw - 20.0, banner_h, 2.0, COLOR_WARM);

            let fs_e = fs_adaptativo(&evento.texto, &self.font, fs_evento(), sw * 0.4);
            draw_text_ex(&evento.texto, 20.0 * s, banner_y + banner_h * 0.65, TextParams {
                font: Some(&self.font), font_size: fs_e, color: COLOR_WARM, ..Default::default()
            });

            let hint = "Z: Ver  X: Cerrar";
            let hint_fs = fs_adaptativo(hint, &self.font, fs_evento(), sw * 0.4);
            let htw = measure_text(hint, Some(&self.font), hint_fs, 1.0).width;
            draw_text_ex(hint, sw - htw - 20.0, banner_y + banner_h * 0.65, TextParams {
                font: Some(&self.font), font_size: hint_fs, color: COLOR_DIM, ..Default::default()
            });
        }
    }

    fn render_minimapa(&self, estado: &Estado, content_bottom: f32) {
        let s = scale();
        let cell = mini_size();
        let gap = mini_gap();
        let total_w = MAPA_COLS as f32 * (cell + gap);
        let total_h = MAPA_ROWS as f32 * (cell + gap);

        let margin = 8.0 * s;
        let ox = screen_width() - total_w - margin;
        let oy = content_bottom - total_h - margin;

        draw_rectangle(ox - 3.0, oy - 3.0, total_w + 6.0, total_h + 6.0,
            Color::new(0.0, 0.0, 0.0, 0.6));

        for escena in Escena::TODAS {
            let (c, r) = escena.pos_mapa();
            let x = ox + c as f32 * (cell + gap);
            let y = oy + r as f32 * (cell + gap);

            let color = if *escena == estado.escena {
                COLOR_ACCENT
            } else if estado.visitadas.contains(escena) {
                COLOR_DIM
            } else {
                Color::new(0.2, 0.2, 0.2, 0.5)
            };

            draw_rectangle(x, y, cell, cell, color);
        }
    }

    fn render_fase_dia(&self, estado: &Estado, content_bottom: f32) {
        let s = scale();
        let margin = 8.0 * s;
        let fs = fs_fase_dia();
        let texto = estado.ciclo.nombre_fase();

        let x = margin;
        let y = content_bottom - margin;

        draw_text_ex(texto, x, y, TextParams {
            font: Some(&self.font), font_size: fs, color: COLOR_DIM, ..Default::default()
        });
    }

    fn render_texto_wrapped(&self, texto: &str, x: f32, y: f32, max_w: f32, max_y: f32, fs: u16, color: Color) {
        let mut cy = y;
        let line_h = fs as f32 * 1.3;

        for line in self.word_wrap(texto, fs, max_w) {
            if cy + line_h > max_y { break; }
            draw_text_ex(&line, x, cy, TextParams {
                font: Some(&self.font), font_size: fs, color, ..Default::default()
            });
            cy += line_h;
        }
    }

    fn word_wrap(&self, texto: &str, fs: u16, max_w: f32) -> Vec<String> {
        let mut lines = Vec::new();
        let mut current_line = String::new();

        for word in texto.split_whitespace() {
            let test = if current_line.is_empty() {
                word.to_string()
            } else {
                format!("{} {}", current_line, word)
            };

            let w = measure_text(&test, Some(&self.font), fs, 1.0).width;
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
        lines
    }
}