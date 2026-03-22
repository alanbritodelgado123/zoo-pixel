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

    // Fondos - usar entrada para todas las zonas
    let entrada_bytes = include_bytes!("../assets/fondos/pixel/px_entrada.png");
    let fondos = Fondos::new(entrada_bytes);
    // Todas las zonas usan el fallback (entrada) por ahora

    let mut audio = AudioManager::new();
    audio.set_fallback(include_bytes!("../assets/audio/ambiente/amb_entrada.ogg")).await;
    audio.agregar_efecto("transicion", include_bytes!("../assets/audio/efectos/fx_transicion.wav")).await;
    audio.agregar_efecto("boton", include_bytes!("../assets/audio/efectos/fx_boton.wav")).await;

    // Todas las zonas usan amb_entrada por ahora
    for escena in escena::Escena::TODAS {
        audio.agregar_ambiente(*escena, include_bytes!("../assets/audio/ambiente/amb_entrada.ogg")).await;
    }

    let mut estado = Estado::new();
    estado.duracion_transicion = audio.duracion_transicion().max(config::TRANSITION_MIN);

    let ui = UiRenderer::new(font);

    audio.iniciar_ambiente(estado.escena);
    let mut escena_anterior = estado.escena;

    loop {
        let dt = get_frame_time().min(0.1);

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

        // Sonido animal
        if estado.necesita_sonido_animal {
            estado.necesita_sonido_animal = false;
            audio.reproducir_animal();
        }

        // Al salir de ViendoAnimal, parar sonido animal
        if !matches!(estado.modo, estado::ModoVista::ViendoAnimal { .. }) && audio.animal_sonando() {
            audio.parar_animal();
        }

        ui.render(&estado, &fondos);

        if estado.mostrar_overlay && !cfg!(target_os = "android") {
            render_pc_overlay(&estado, &ui.font);
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
    let y = sh - margin;
    let gap = 12.0 * s;

    let indicators: &[(&str, f32)] = &[
        ("Z", estado.indicador_z_pressed),
        ("X", estado.indicador_x_pressed),
        ("M", 0.0),
        ("L", 0.0),
    ];

    let total_w: f32 = indicators.iter().map(|(k, _)| {
        measure_text(&format!("[{}]", k), Some(font), fs, 1.0).width + gap
    }).sum::<f32>() - gap;

    let mut x = (sw - total_w) / 2.0;

    for (key, pressed) in indicators {
        let texto = format!("[{}]", key);
        let tw = measure_text(&texto, Some(font), fs, 1.0).width;
        let color = if *pressed > 0.0 { config::COLOR_ACCENT } else { Color::new(0.7, 0.7, 0.7, 0.4) };
        draw_text_ex(&texto, x, y, TextParams {
            font: Some(font), font_size: fs, color, ..Default::default()
        });
        x += tw + gap;
    }
}