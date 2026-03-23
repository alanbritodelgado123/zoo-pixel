// src/main.rs
use macroquad::prelude::*;

mod animacion;
mod audio;
mod ciclo_dia;
mod config;
mod db;
mod escena;
mod estado;
mod eventos;
mod fondo;
mod guia;
mod input;
mod libreta;
mod minijuego;
mod plataforma;
mod save;
mod ui;

use audio::AudioManager;
use db::ZooDB;
use estado::{Estado, Pantalla};
use fondo::Fondos;
use ui::UiRenderer;

fn window_conf() -> Conf {
    Conf {
        window_title: "Zoo Pixel".to_owned(),
        window_width: 480,
        window_height: 800,
        window_resizable: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let db = ZooDB::new();

    let font_bytes = include_bytes!("../assets/fonts/PressStart2P.ttf");
    let font = load_ttf_font_from_bytes(font_bytes).expect("No se pudo cargar la fuente");

    let entrada_bytes = include_bytes!("../assets/fondos/pixel/px_entrada.png");
    let fondos = Fondos::new(entrada_bytes);

    let mut audio = AudioManager::new();
    audio.set_fallback(include_bytes!("../assets/audio/ambiente/amb_entrada.ogg")).await;
    audio.agregar_efecto("transicion",
        include_bytes!("../assets/audio/efectos/fx_transicion.wav")).await;
    audio.agregar_efecto("boton",
        include_bytes!("../assets/audio/efectos/fx_boton.wav")).await;

    for escena in escena::Escena::TODAS {
        audio.agregar_ambiente(*escena,
            include_bytes!("../assets/audio/ambiente/amb_entrada.ogg")).await;
    }

    let mut estado = Estado::new(&db);
    estado.duracion_transicion = (audio.duracion_transicion() + 0.2)
        .max(config::TRANSITION_MIN);

    let ui = UiRenderer::new(font);

    audio.iniciar_ambiente(estado.escena);

    loop {
        let dt = get_frame_time().min(0.1);

        for accion in input::leer_teclado() {
            estado.procesar_accion(accion, &db);
        }

        estado.update(dt);
        audio.update(dt);

        // Volumen: usar menu_config durante edición para preview en vivo
        let (vol_m, vol_e) = if matches!(estado.pantalla, Pantalla::Config) {
            (estado.menu_config.volumen_musica, estado.menu_config.volumen_efectos)
        } else {
            (estado.save.config.volumen_musica, estado.save.config.volumen_efectos)
        };
        audio.set_volumen_musica(vol_m);
        audio.set_volumen_efectos(vol_e);

        // Transición de audio
        if let Some(destino) = estado.necesita_transicion_audio.take() {
            audio.transicionar_a(destino);
        }

        // Sonido animal sin solapamiento
        if estado.necesita_sonido_animal {
            estado.necesita_sonido_animal = false;
            audio.efecto_unico("boton");
        }

        // Render principal
        ui.render(&estado, &fondos);

        // Overlay PC con sombra
        let mostrar_overlay_pc = estado.mostrar_overlay
            && !cfg!(target_os = "android")
            && !estado.en_pantalla_info()
            && !estado.dialogo.activo
            && !estado.eventos.hay_evento()
            && !estado.en_transicion();

        if mostrar_overlay_pc {
            render_pc_overlay(&estado, &ui.font);
        }

        // CRT: usar menu_config durante edición para preview en vivo
        let crt_activo = if matches!(estado.pantalla, Pantalla::Config) {
            estado.menu_config.crt
        } else {
            estado.save.config.crt
        };
        if crt_activo {
            render_crt();
        }

        next_frame().await;
    }
}

fn render_pc_overlay(estado: &Estado, font: &Font) {
    let s = config::scale();
    let sw = screen_width();
    let sh = screen_height();
    let fs = config::fs_pct(0.028);
    let margin = 8.0 * s;
    let gap = 12.0 * s;
    let shadow_offset = (1.5 * s).max(1.0);
    let pad_x = 10.0 * s;
    let pad_y = 5.0 * s;

    let indicators: &[(&str, f32)] = &[
        ("Z", estado.indicador_z_pressed),
        ("X", estado.indicador_x_pressed),
        ("M", 0.0),
        ("L", 0.0),
    ];

    // Calcular ancho total
    let total_w: f32 = indicators.iter().map(|(k, _)| {
        measure_text(&format!("[{}]", k), Some(font), fs, 1.0).width + gap
    }).sum::<f32>() - gap;

    let th = config::text_height(font, fs);
    let group_x = (sw - total_w) / 2.0;
    let text_y = sh - margin;

    // Fondo gris semitransparente detrás del grupo
    draw_rectangle(
        group_x - pad_x,
        text_y - th - pad_y,
        total_w + pad_x * 2.0,
        th + pad_y * 2.0,
        Color::new(0.15, 0.15, 0.15, 0.6),
    );

    let mut x = group_x;

    for (key, pressed) in indicators {
        let texto = format!("[{}]", key);
        let tw = measure_text(&texto, Some(font), fs, 1.0).width;
        let color = if *pressed > 0.0 { config::COLOR_ACCENT } else { WHITE };

        // Sombra
        draw_text_ex(&texto, x + shadow_offset, text_y + shadow_offset,
            TextParams {
                font: Some(font), font_size: fs,
                color: Color::new(0.0, 0.0, 0.0, 0.7),
                ..Default::default()
            });
        // Texto
        draw_text_ex(&texto, x, text_y,
            TextParams { font: Some(font), font_size: fs, color,
                          ..Default::default() });

        x += tw + gap;
    }
}

fn render_crt() {
    let sw = screen_width();
    let sh = screen_height();
    // Gap escalado para aspecto consistente en cualquier resolución
    let gap = (sh / 200.0).max(2.0).min(4.0);
    let mut y = 0.0;
    while y < sh {
        draw_rectangle(0.0, y, sw, 1.0, Color::new(0.0, 0.0, 0.0, 0.15));
        y += gap;
    }
}