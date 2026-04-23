use macroquad::prelude::*;
use std::collections::HashMap;

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

    // ✅ FUENTE
    let font_bytes = include_bytes!("../assets/fonts/PressStart2P.ttf");
    let font = load_ttf_font_from_bytes(font_bytes)
        .expect("No se pudo cargar la fuente");

    // ✅ SPRITESHEET
    let spritesheet_bytes = include_bytes!("../assets/fondos/spritesheet_vertical.png");
    let fondos = Fondos::new(spritesheet_bytes, 640.0, 480.0);

    // ✅ ÍCONOS DE CATEGORÍA
    let mut iconos_categoria: HashMap<String, Texture2D> = HashMap::new();

    {
        let bytes = include_bytes!("../assets/categorias/anfibios_inspyrenet.png");
        let tex = Texture2D::from_file_with_format(&bytes[..], Some(ImageFormat::Png));
        tex.set_filter(FilterMode::Nearest);
        iconos_categoria.insert("anfibios".to_string(), tex);
    }
    {
        let bytes = include_bytes!("../assets/categorias/aves_inspyrenet.png");
        let tex = Texture2D::from_file_with_format(&bytes[..], Some(ImageFormat::Png));
        tex.set_filter(FilterMode::Nearest);
        iconos_categoria.insert("aves".to_string(), tex);
    }
    {
        let bytes = include_bytes!("../assets/categorias/fosiles_inspyrenet.png");
        let tex = Texture2D::from_file_with_format(&bytes[..], Some(ImageFormat::Png));
        tex.set_filter(FilterMode::Nearest);
        iconos_categoria.insert("fosiles".to_string(), tex);
    }
    {
        let bytes = include_bytes!("../assets/categorias/insectos_inspyrenet.png");
        let tex = Texture2D::from_file_with_format(&bytes[..], Some(ImageFormat::Png));
        tex.set_filter(FilterMode::Nearest);
        iconos_categoria.insert("insectos".to_string(), tex);
    }
    {
        let bytes = include_bytes!("../assets/categorias/mamiferos_inspyrenet.png");
        let tex = Texture2D::from_file_with_format(&bytes[..], Some(ImageFormat::Png));
        tex.set_filter(FilterMode::Nearest);
        iconos_categoria.insert("mamiferos".to_string(), tex);
    }
    {
        let bytes = include_bytes!("../assets/categorias/peces_inspyrenet.png");
        let tex = Texture2D::from_file_with_format(&bytes[..], Some(ImageFormat::Png));
        tex.set_filter(FilterMode::Nearest);
        iconos_categoria.insert("peces".to_string(), tex);
    }
    {
        let bytes = include_bytes!("../assets/categorias/primates_inspyrenet.png");
        let tex = Texture2D::from_file_with_format(&bytes[..], Some(ImageFormat::Png));
        tex.set_filter(FilterMode::Nearest);
        iconos_categoria.insert("primates".to_string(), tex);
    }
    {
        let bytes = include_bytes!("../assets/categorias/reptiles_inspyrenet.png");
        let tex = Texture2D::from_file_with_format(&bytes[..], Some(ImageFormat::Png));
        tex.set_filter(FilterMode::Nearest);
        iconos_categoria.insert("reptiles".to_string(), tex);
    }
    println!("✅ 8 íconos de categoría cargados");

    // ✅ AUDIO
    let mut audio = AudioManager::new();
    audio.set_fallback(
        include_bytes!("../assets/audio/ambiente/amb_entrada.ogg")
    ).await;
    audio.agregar_efecto(
        "transicion",
        include_bytes!("../assets/audio/efectos/fx_transicion.wav"),
    ).await;
    audio.agregar_efecto(
        "boton",
        include_bytes!("../assets/audio/efectos/fx_boton.wav"),
    ).await;

    // ✅ 8 AUDIOS DE CATEGORÍA
    audio.agregar_efecto(
        "grito_anfibios",
        include_bytes!("../assets/audio/categorias/grito_anfibios.ogg"),
    ).await;
    audio.agregar_efecto(
        "grito_aves",
        include_bytes!("../assets/audio/categorias/grito_aves.ogg"),
    ).await;
    audio.agregar_efecto(
        "grito_fosiles",
        include_bytes!("../assets/audio/categorias/grito_fosiles.ogg"),
    ).await;
    audio.agregar_efecto(
        "grito_insectos",
        include_bytes!("../assets/audio/categorias/grito_insectos.ogg"),
    ).await;
    audio.agregar_efecto(
        "grito_mamiferos",
        include_bytes!("../assets/audio/categorias/grito_mamiferos.ogg"),
    ).await;
    audio.agregar_efecto(
        "grito_peces",
        include_bytes!("../assets/audio/categorias/grito_peces.ogg"),
    ).await;
    audio.agregar_efecto(
        "grito_primates",
        include_bytes!("../assets/audio/categorias/grito_primates.ogg"),
    ).await;
    audio.agregar_efecto(
        "grito_reptiles",
        include_bytes!("../assets/audio/categorias/grito_reptiles.ogg"),
    ).await;
    println!("✅ 8 audios de categoría cargados");

    // ✅ AMBIENTES (uno por escena, usando fallback)
    for escena in escena::Escena::TODAS {
        audio.agregar_ambiente(
            *escena,
            include_bytes!("../assets/audio/ambiente/amb_entrada.ogg"),
        ).await;
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

    // ✅ INICIALIZAR UI Y ESTADO
    let ui = UiRenderer::new(font, textura_eli, textura_ani, iconos_categoria);
    let mut estado = Estado::new(&db);
    estado.duracion_transicion = (audio.duracion_transicion() + 0.2)
        .max(config::TRANSITION_MIN);
    audio.iniciar_ambiente(estado.escena);

    println!("🎮 Zoo Pixel v0.6.1 iniciado");
    println!("📍 Escena inicial: {:?}", estado.escena);
    println!("📐 Resolución: {}x{}", screen_width(), screen_height());

    // ✅ GAME LOOP
    loop {
        let dt = get_frame_time().min(0.1);

        // ── Input ────────────────────────────────────────────────────
        for accion in input::leer_teclado() {
            estado.procesar_accion(accion, &db);
        }
        for accion in input::leer_tactil(&estado) {
            estado.procesar_accion(accion, &db);
        }

        // ── Update ───────────────────────────────────────────────────
        estado.update(dt, &db);
        audio.update(dt);

        // ── Audio ────────────────────────────────────────────────────

        // Volumen desde config activa
        let (vol_m, vol_e) = if matches!(estado.pantalla, Pantalla::Config) {
            (estado.menu_config.volumen_musica, estado.menu_config.volumen_efectos)
        } else {
            (estado.save.config.volumen_musica, estado.save.config.volumen_efectos)
        };
        audio.set_volumen_musica(vol_m);
        audio.set_volumen_efectos(vol_e);

        // Transición de audio entre escenas
        if let Some(destino) = estado.necesita_transicion_audio.take() {
            audio.transicionar_a(destino);
        }

        // ✅ Gritos de categoría (FIX: resetear flag después de reproducir)
        if estado.necesita_sonido_animal {
            // Obtener categoría del animal actual según el modo de vista
            let categoria = match &estado.modo {
                estado::ModoVista::ViendoAnimal { animal, .. } => {
                    Some(&animal.categoria)
                }
                estado::ModoVista::Foto { animales, indice_actual, foto_tomada, .. } => {
                    if *foto_tomada {
                        Some(&animales[*indice_actual].categoria)
                    } else {
                        None
                    }
                }
                _ => None,
            };

            // Reproducir sonido si hay categoría válida
            if let Some(cat) = categoria {
                audio.reproducir_grito_categoria(cat);
            }

            // ✅ RESETEAR FLAG (esto evita el loop infinito)
            estado.necesita_sonido_animal = false;
        }

        // ── Render ───────────────────────────────────────────────────
        ui.render(&estado, &fondos, &audio);

        // Overlay PC (indicadores de botones)
        let mostrar_overlay_pc = estado.mostrar_overlay
            && !cfg!(target_os = "android")
            && !estado.en_pantalla_info()
            && !estado.dialogo.activo
            && !estado.en_transicion();
        if mostrar_overlay_pc {
            ui::render_pc_overlay(&estado, &ui.font);
        }

        // Filtro CRT
        let crt_activo = if matches!(estado.pantalla, Pantalla::Config) {
            estado.menu_config.crt
        } else {
            estado.save.config.crt
        };
        if crt_activo {
            ui::render_crt();
        }

        next_frame().await;
    }
}