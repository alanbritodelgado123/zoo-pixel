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
use estado::Estado;
use ui::UiRenderer;

fn window_conf() -> Conf {
    Conf {
        window_title: "Zoológico Nacional".to_owned(),
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

    let mut audio = AudioManager::new();
    audio.set_fallback(include_bytes!("../assets/audio/extra/Seaside-ambiance-with-seagulls-and-boats.ogg")).await;
    audio.agregar_efecto("transicion", include_bytes!("../assets/audio/efectos/fx_transicion.wav")).await;
    audio.agregar_efecto("boton", include_bytes!("../assets/audio/efectos/fx_boton.wav")).await;

    audio.agregar_ambiente(escena::Escena::EntradaPrincipal, include_bytes!("../assets/audio/ambiente/amb_entrada.ogg")).await;
    audio.agregar_ambiente(escena::Escena::Acuario, include_bytes!("../assets/audio/ambiente/amb_acuario.ogg")).await;
    audio.agregar_ambiente(escena::Escena::Aviario, include_bytes!("../assets/audio/ambiente/amb_aves.ogg")).await;
    audio.agregar_ambiente(escena::Escena::A2, include_bytes!("../assets/audio/ambiente/amb_leones.ogg")).await;
    audio.agregar_ambiente(escena::Escena::B1, include_bytes!("../assets/audio/ambiente/amb_primates.ogg")).await;
    audio.agregar_ambiente(escena::Escena::A3, include_bytes!("../assets/audio/ambiente/amb_reptiles.ogg")).await;

    let mut estado = Estado::new();
    estado.duracion_transicion = audio.duracion_transicion().max(config::TRANSITION_MIN);

    let ui = UiRenderer::new(font);

    audio.iniciar_ambiente(estado.escena);

    let mut escena_anterior = estado.escena;

    loop {
        let dt = get_frame_time().min(0.1);

        // Input - usar la función del módulo
        for accion in input::leer_teclado() {
            estado.procesar_accion(accion, &db);
        }

        estado.update(dt);
        audio.update(dt);

        audio.set_volumen_musica(estado.save.config.volumen_musica);
        audio.set_volumen_efectos(estado.save.config.volumen_efectos);

        if estado.escena != escena_anterior {
            audio.transicionar_a(estado.escena);
            escena_anterior = estado.escena;
        }

        ui.render(&estado);

        if !cfg!(target_os = "android") {
            render_pc_overlay(&estado, &ui.font);
        }

        next_frame().await;
    }
}

fn render_pc_overlay(estado: &Estado, font: &Font) {
    let s = config::scale();
    let sw = screen_width();
    let sh = screen_height();

    let fs = config::fs_indicador();
    let margin = 8.0 * s;
    let y = sh - margin;
    let gap = 15.0 * s;

    let indicators: &[(&str, &str, f32)] = &[
        ("Z", "Acción", estado.indicador_z_pressed),
        ("X", "Atrás", estado.indicador_x_pressed),
        ("M", "Mapa", 0.0),
        ("L", "Libreta", 0.0),
    ];

    let total_w: f32 = indicators.iter().map(|(k, _, _)| {
        measure_text(&format!("[{}]", k), Some(font), fs, 1.0).width + gap
    }).sum::<f32>() - gap;

    let mut x = (sw - total_w) / 2.0;

    for (key, _label, pressed) in indicators {
        let texto = format!("[{}]", key);
        let tw = measure_text(&texto, Some(font), fs, 1.0).width;

        let alpha = if *pressed > 0.0 { 0.6 + pressed * 0.4 } else { 0.4 };
        let color = if *pressed > 0.0 { config::COLOR_ACCENT } else {
            Color::new(0.7, 0.7, 0.7, alpha)
        };

        draw_text_ex(&texto, x, y, TextParams {
            font: Some(font), font_size: fs, color, ..Default::default()
        });

        x += tw + gap;
    }
}