mod audio;
mod config;
mod db;
mod escena;
mod estado;
mod fondo;
mod input;
mod ui;

use macroquad::prelude::*;

use audio::AudioManager;
use db::ZooDB;
use escena::Escena;
use estado::Estado;
use fondo::Fondos;

#[macroquad::main("Zoo Pixel")]
async fn main() {
    let font = load_ttf_font_from_bytes(include_bytes!("../assets/fonts/Ithaca-LVB75.ttf"))
        .expect("No se pudo cargar la fuente");

    let zoo_db = ZooDB::new();

    let fondos = Fondos::new(
        include_bytes!("../assets/fondos/spritesheet_zonas.png"),
        10,
    );

    let mut audio = AudioManager::new();

    audio.agregar_ambiente(Escena::Entrada,      include_bytes!("../assets/audio/ambiente/amb_entrada.ogg")).await;
    audio.agregar_ambiente(Escena::Sabana,       include_bytes!("../assets/audio/ambiente/amb_leones.ogg")).await;
    audio.agregar_ambiente(Escena::Aviario,      include_bytes!("../assets/audio/ambiente/amb_aves.ogg")).await;
    audio.agregar_ambiente(Escena::Reptiliario,  include_bytes!("../assets/audio/ambiente/amb_reptiles.ogg")).await;
    audio.agregar_ambiente(Escena::Laguna,       include_bytes!("../assets/audio/ambiente/amb_acuario.ogg")).await;
    audio.agregar_ambiente(Escena::Primates,     include_bytes!("../assets/audio/ambiente/amb_primates.ogg")).await;

    audio.agregar_efecto("transicion", include_bytes!("../assets/audio/efectos/fx_transicion.wav")).await;
    audio.agregar_efecto("boton",      include_bytes!("../assets/audio/efectos/fx_boton.wav")).await;

    let dur_transicion = {
        let dur = audio.duracion_efecto("transicion");
        if dur > config::TRANSITION_MIN { dur }
        else { config::TRANSITION_SECS_FALLBACK }
    };

    let mut estado = Estado::new();
    estado.duracion_transicion = dur_transicion;

    audio.iniciar_ambiente(Escena::Entrada);

    let es_android = cfg!(target_os = "android");

    loop {
        let dt = get_frame_time();
        clear_background(config::COLOR_BG_DARK);

        let en_transicion_antes = estado.en_transicion();
        estado.update(dt);

        if en_transicion_antes && !estado.en_transicion() {
            audio.iniciar_ambiente(estado.escena);
        }

        let area_h = screen_height() - config::bar_height();

        // === 1. FONDO ===
        if fondos.tiene(&estado.escena) {
            fondos.draw(&estado.escena, WHITE, area_h);
        } else {
            ui::dibujar_placeholder(&estado.escena, area_h, &font);
        }

        // === 2. MINIMAPA ===
        ui::dibujar_minimapa(&estado, &font);

        // === 3. SELECCIÓN ===
        ui::dibujar_seleccion(&estado, &font);

        // === 4. VISTA ANIMAL ===
        ui::dibujar_animal(&estado, &font);

        // === 5. MODO FOTO ===
        ui::dibujar_foto(&estado, &font);

        // === 6. TRANSICIÓN ===
        ui::dibujar_transicion(&estado);

        // === 7. BARRA ===
        ui::dibujar_barra(&estado, es_android, &font);

        // === 8. INPUT ===
        let mut acciones = input::leer_teclado();
        if es_android {
            let botones = ui::Botones::calcular();
            acciones.extend(ui::dibujar_controles(&estado, &botones, &font));
        }

        // === 9. PROCESAR ===
        let en_transicion_antes_input = estado.en_transicion();

        for accion in acciones {
            estado.procesar_accion(accion, &zoo_db);
        }

        if !en_transicion_antes_input && estado.en_transicion() {
            audio.parar_ambiente();
            audio.efecto("transicion");
        }

        next_frame().await;
    }
}