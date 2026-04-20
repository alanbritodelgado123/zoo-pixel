use std::collections::HashMap;
use macroquad::prelude::*;
use macroquad::window::PlatformSettings;
use macroquad::window::ActivityOrientation;

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

fn window_conf() -> Conf {
    Conf {
        window_title: "Zoo Pixel".to_owned(),
        window_width: 800,   // ✅ Landscape: ancho > alto
        window_height: 480,
        window_resizable: !cfg!(target_os = "android"),  // ✅ PC: sí, Android: no
        high_dpi: true,
        platform: PlatformSettings {
            android_keep_screen_on: true,
            android_orientation: ActivityOrientation::Landscape,  // ✅ Landscape
            ..Default::default()
        },
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let db = ZooDB::new();
    
    // ✅ Rutas desde la raíz del proyecto (donde está Cargo.toml)
    let font_bytes = include_bytes!("../assets/fonts/PressStart2P.ttf");
    let font = load_ttf_font_from_bytes(font_bytes).expect("No se pudo cargar la fuente");
    
    let spritesheet_bytes = include_bytes!("../assets/fondos/spritesheet_vertical.png");
    let fondos = Fondos::new(spritesheet_bytes, 640.0, 480.0);
    
    // ✅ Cargar íconos de categoría
    let mut iconos_categoria: HashMap<String, Texture2D> = HashMap::new();
    let categorias = [
        ("anfibios", "anfibios_inspyrenet.png"),
        ("aves", "aves_inspyrenet.png"),
        ("fosiles", "fosiles_inspyrenet.png"),  // ✅ SIN TILDE (renombrar archivo)
        ("insectos", "insectos_inspyrenet.png"),
        ("mamiferos", "mamiferos_inspyrenet.png"),
        ("peces", "peces_inspyrenet.png"),
        ("primates", "primates_inspyrenet.png"),
        ("reptiles", "reptiles_inspyrenet.png"),
    ];
    
    for (key, filename) in &categorias {
        let path = format!("assets/categorias/{}", filename);
        if let Ok(bytes) = std::fs::read(&path) {
            let tex = Texture2D::from_file_with_format(&bytes, Some(ImageFormat::Png));
            tex.set_filter(FilterMode::Nearest);
            iconos_categoria.insert(key.to_string(), tex);
            println!("✅ Icono cargado: {}", key);
        } else {
            println!("⚠️ No se encontró: {}", path);
        }
    }
    
    // ✅ Cargar audio
    let mut audio = AudioManager::new();
    
    match std::fs::read("assets/audio/ambiente/amb_entrada.ogg") {
        Ok(bytes) => audio.set_fallback(&bytes).await,
        Err(e) => println!("⚠️ Error fallback audio: {:?}", e),
    }
    
    match std::fs::read("assets/audio/efectos/fx_transicion.wav") {
        Ok(bytes) => audio.agregar_efecto("transicion", &bytes).await,
        Err(e) => println!("⚠️ Error efecto transicion: {:?}", e),
    }
    
    match std::fs::read("assets/audio/efectos/fx_boton.wav") {
        Ok(bytes) => audio.agregar_efecto("boton", &bytes).await,
        Err(e) => println!("⚠️ Error efecto boton: {:?}", e),
    }
    
    for escena in Escena::TODAS {
        match std::fs::read("assets/audio/ambiente/amb_entrada.ogg") {
            Ok(bytes) => audio.agregar_ambiente(*escena, &bytes).await,
            Err(e) => println!("⚠️ Error ambiente {:?}: {:?}", escena, e),
        }
    }
    
    // ✅ Cargar texturas de guías
    let textura_eli = std::fs::read("assets/guias/guiaEli.png")
        .ok()
        .map(|bytes| {
            let tex = Texture2D::from_file_with_format(&bytes, Some(ImageFormat::Png));
            tex.set_filter(FilterMode::Nearest);
            println!("✅ Guía Eli cargada");
            tex
        });
    
    let textura_ani = std::fs::read("assets/guias/guiaAni.png")
        .ok()
        .map(|bytes| {
            let tex = Texture2D::from_file_with_format(&bytes, Some(ImageFormat::Png));
            tex.set_filter(FilterMode::Nearest);
            println!("✅ Guía Ani cargada");
            tex
        });
    
    let ui = UiRenderer::new(font, textura_eli, textura_ani, iconos_categoria);
    
    let mut estado = Estado::new(&db);
    estado.duracion_transicion = (audio.duracion_transicion() + 0.2)
        .max(config::TRANSITION_MIN);
    
    audio.iniciar_ambiente(estado.escena);
    
    println!("🎮 Zoo Pixel iniciado correctamente");
    println!("📍 Escena inicial: {:?}", estado.escena);
    
    loop {
        let dt = get_frame_time().min(0.1);
        
        for accion in input::leer_teclado() {
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
        
        let mostrar_overlay_pc = estado.mostrar_overlay
            && !cfg!(target_os = "android")
            && !estado.en_pantalla_info()
            && !estado.dialogo.activo
            && !estado.eventos.hay_evento()
            && !estado.en_transicion();
        
        if mostrar_overlay_pc {
            render_pc_overlay(&estado, &ui.font);
        }
        
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