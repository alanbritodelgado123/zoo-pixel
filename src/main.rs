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
use escena::Escena;
use macroquad::prelude::*;
use std::collections::HashMap;

fn window_conf() -> Conf {
    Conf {
        window_title: "Zoo Pixel".to_owned(),
        window_width: 800,
        window_height: 480,
        window_resizable: !cfg!(target_os = "android"),
        high_dpi: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let db = ZooDB::new();
    
    // ✅ FUENTE: include_bytes!
    let font_bytes = include_bytes!("../assets/fonts/PressStart2P.ttf");
    let font = load_ttf_font_from_bytes(font_bytes).expect("No se pudo cargar la fuente");
    
    // ✅ SPRITESHEET: include_bytes!
    let spritesheet_bytes = include_bytes!("../assets/fondos/spritesheet_vertical.png");
    let fondos = Fondos::new(spritesheet_bytes, 640.0, 480.0);
    
    // ✅ ÍCONOS DE CATEGORÍA: Cargar CADA UNO individualmente
    // NO usar array fijo porque cada PNG tiene tamaño diferente
    let mut iconos_categoria: HashMap<String, Texture2D> = HashMap::new();
    
    // ✅ Cargar cada ícono por separado (sin array de tuples)
    let bytes_anfibios = include_bytes!("../assets/categorias/anfibios_inspyrenet.png");
    let tex = Texture2D::from_file_with_format(&bytes_anfibios[..], Some(ImageFormat::Png));
    tex.set_filter(FilterMode::Nearest);
    iconos_categoria.insert("anfibios".to_string(), tex);
    println!("✅ Icono cargado: anfibios");
    
    let bytes_aves = include_bytes!("../assets/categorias/aves_inspyrenet.png");
    let tex = Texture2D::from_file_with_format(&bytes_aves[..], Some(ImageFormat::Png));
    tex.set_filter(FilterMode::Nearest);
    iconos_categoria.insert("aves".to_string(), tex);
    println!("✅ Icono cargado: aves");
    
    let bytes_fosiles = include_bytes!("../assets/categorias/fosiles_inspyrenet.png");
    let tex = Texture2D::from_file_with_format(&bytes_fosiles[..], Some(ImageFormat::Png));
    tex.set_filter(FilterMode::Nearest);
    iconos_categoria.insert("fosiles".to_string(), tex);
    println!("✅ Icono cargado: fosiles");
    
    let bytes_insectos = include_bytes!("../assets/categorias/insectos_inspyrenet.png");
    let tex = Texture2D::from_file_with_format(&bytes_insectos[..], Some(ImageFormat::Png));
    tex.set_filter(FilterMode::Nearest);
    iconos_categoria.insert("insectos".to_string(), tex);
    println!("✅ Icono cargado: insectos");
    
    let bytes_mamiferos = include_bytes!("../assets/categorias/mamiferos_inspyrenet.png");
    let tex = Texture2D::from_file_with_format(&bytes_mamiferos[..], Some(ImageFormat::Png));
    tex.set_filter(FilterMode::Nearest);
    iconos_categoria.insert("mamiferos".to_string(), tex);
    println!("✅ Icono cargado: mamiferos");
    
    let bytes_peces = include_bytes!("../assets/categorias/peces_inspyrenet.png");
    let tex = Texture2D::from_file_with_format(&bytes_peces[..], Some(ImageFormat::Png));
    tex.set_filter(FilterMode::Nearest);
    iconos_categoria.insert("peces".to_string(), tex);
    println!("✅ Icono cargado: peces");
    
    let bytes_primates = include_bytes!("../assets/categorias/primates_inspyrenet.png");
    let tex = Texture2D::from_file_with_format(&bytes_primates[..], Some(ImageFormat::Png));
    tex.set_filter(FilterMode::Nearest);
    iconos_categoria.insert("primates".to_string(), tex);
    println!("✅ Icono cargado: primates");
    
    let bytes_reptiles = include_bytes!("../assets/categorias/reptiles_inspyrenet.png");
    let tex = Texture2D::from_file_with_format(&bytes_reptiles[..], Some(ImageFormat::Png));
    tex.set_filter(FilterMode::Nearest);
    iconos_categoria.insert("reptiles".to_string(), tex);
    println!("✅ Icono cargado: reptiles");
    
    // ✅ AUDIO: include_bytes!
    let mut audio = AudioManager::new();
    
    audio.set_fallback(include_bytes!("../assets/audio/ambiente/amb_entrada.ogg")).await;
    audio.agregar_efecto("transicion", include_bytes!("../assets/audio/efectos/fx_transicion.wav")).await;
    audio.agregar_efecto("boton", include_bytes!("../assets/audio/efectos/fx_boton.wav")).await;
    
    for escena in Escena::TODAS {
        audio.agregar_ambiente(*escena, include_bytes!("../assets/audio/ambiente/amb_entrada.ogg")).await;
    }
    
    // ✅ TEXTURAS DE GUÍAS
    let textura_eli = {
        let bytes = include_bytes!("../assets/guias/guiaEli.png");
        let tex = Texture2D::from_file_with_format(&bytes[..], Some(ImageFormat::Png));
        tex.set_filter(FilterMode::Nearest);
        println!("✅ Guía Eli cargada");
        Some(tex)
    };
    
    let textura_ani = {
        let bytes = include_bytes!("../assets/guias/guiaAni.png");
        let tex = Texture2D::from_file_with_format(&bytes[..], Some(ImageFormat::Png));
        tex.set_filter(FilterMode::Nearest);
        println!("✅ Guía Ani cargada");
        Some(tex)
    };
    
    let ui = UiRenderer::new(font, textura_eli, textura_ani, iconos_categoria);
    let mut estado = Estado::new(&db);
    
    estado.duracion_transicion = (audio.duracion_transicion() + 0.2)
        .max(config::TRANSITION_MIN);
    
    audio.iniciar_ambiente(estado.escena);
    
    println!("🎮 Zoo Pixel v0.5.1 iniciado correctamente");
    println!("📍 Escena inicial: {:?}", estado.escena);
    println!("📐 Resolución: {}x{}", screen_width(), screen_height());
    
    loop {
        let dt = get_frame_time().min(0.1);
        
        // Input PC
        for accion in input::leer_teclado() {
            estado.procesar_accion(accion, &db);
        }
        
        // Input Android (táctil)
        for accion in input::leer_tactil(&estado) {
            estado.procesar_accion(accion, &db);
        }
        
        estado.update(dt);
        audio.update(dt);
        
        let (vol_m, vol_e) = if matches!(estado.pantalla, Pantalla::Config) {
            (estado.menu_config.volumen_musica, estado.menu_config.volumen_efectos)
        } else {
            (estado.save.config.volumen_musica, estado.save.config.volumen_efectos)
        };
        
        audio.set_volumen_musica(vol_m);
        audio.set_volumen_efectos(vol_e);
        
        if let Some(destino) = estado.necesita_transicion_audio.take() {
            audio.transicionar_a(destino);
        }
        
        if estado.necesita_sonido_animal {
            estado.necesita_sonido_animal = false;
            audio.efecto_unico("boton");
        }
        
        ui.render(&estado, &fondos);
        
        // Overlay de controles en PC
        let mostrar_overlay_pc = estado.mostrar_overlay
            && !cfg!(target_os = "android")
            && !estado.en_pantalla_info()
            && !estado.dialogo.activo
            && !estado.eventos.hay_evento()
            && !estado.en_transicion();
        
        if mostrar_overlay_pc {
            render_pc_overlay(&estado, &ui.font);
        }
        
        // Filtro CRT
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
    
    let total_w: f32 = indicators.iter().map(|(k, _)| {
        measure_text(&format!("[{}]", k), Some(font), fs, 1.0).width + gap
    }).sum::<f32>() - gap;
    
    let th = config::text_height(font, fs);
    let group_x = (sw - total_w) / 2.0;
    let text_y = sh - margin;
    
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
        
        draw_text_ex(
            &texto,
            x + shadow_offset,
            text_y + shadow_offset,
            TextParams {
                font: Some(font),
                font_size: fs,
                color: Color::new(0.0, 0.0, 0.0, 0.7),
                ..Default::default()
            },
        );
        
        draw_text_ex(
            &texto,
            x,
            text_y,
            TextParams {
                font: Some(font),
                font_size: fs,
                color,
                ..Default::default()
            },
        );
        
        x += tw + gap;
    }
}

fn render_crt() {
    let sw = screen_width();
    let sh = screen_height();
    let gap = (sh / 200.0).max(2.0).min(4.0);
    let mut y = 0.0;
    while y < sh {
        draw_rectangle(0.0, y, sw, 1.0, Color::new(0.0, 0.0, 0.0, 0.15));
        y += gap;
    }
}