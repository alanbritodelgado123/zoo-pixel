// src/input.rs
use macroquad::prelude::*;
use crate::estado::Estado;

// =====================================================================
//  ACCIONES
// =====================================================================

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Accion {
    Arriba,
    Abajo,
    Izquierda,
    Derecha,
    BotonA,
    BotonB,
    Mapa,
    Menu,
    Libreta,
}

// =====================================================================
//  INPUT DE TECLADO (PC)
// =====================================================================

pub fn leer_teclado() -> Vec<Accion> {
    let mut acciones = Vec::new();

    if is_key_pressed(KeyCode::Up)     { acciones.push(Accion::Arriba); }
    if is_key_pressed(KeyCode::Down)   { acciones.push(Accion::Abajo); }
    if is_key_pressed(KeyCode::Left)   { acciones.push(Accion::Izquierda); }
    if is_key_pressed(KeyCode::Right)  { acciones.push(Accion::Derecha); }
    if is_key_pressed(KeyCode::Z)      { acciones.push(Accion::BotonA); }
    if is_key_pressed(KeyCode::Enter)  { acciones.push(Accion::BotonA); }
    if is_key_pressed(KeyCode::X)      { acciones.push(Accion::BotonB); }
    if is_key_pressed(KeyCode::Escape) { acciones.push(Accion::BotonB); }
    if is_key_pressed(KeyCode::M)      { acciones.push(Accion::Mapa); }
    if is_key_pressed(KeyCode::L)      { acciones.push(Accion::Libreta); }

    acciones
}

// =====================================================================
//  INPUT TÁCTIL (Android landscape)
// =====================================================================

pub fn leer_tactil(estado: &Estado) -> Vec<Accion> {
    let mut acciones = Vec::new();

    if !estado.plataforma.es_movil() {
        return acciones;
    }

    let touches_list = touches();
    let sw = screen_width();
    let sh = screen_height();

    // Zonas de la pantalla (landscape 800x480)
    let btn_size  = sh * 0.14;
    let margin    = sh * 0.05;
    let dpad_size = sh * 0.35;

    for touch in &touches_list {
        if touch.phase != TouchPhase::Started {
            continue;
        }

        let x = touch.position.x;
        let y = touch.position.y;

        // ── D-pad (izquierda abajo) ──────────────────────────────────
        let dpad_cx = margin + dpad_size / 2.0;
        let dpad_cy = sh - margin - dpad_size / 2.0;

        if (x - dpad_cx).abs() < dpad_size / 2.0
            && (y - dpad_cy).abs() < dpad_size / 2.0
        {
            let dx = x - dpad_cx;
            let dy = y - dpad_cy;
            if dx.abs() > dy.abs() {
                if dx > 0.0 {
                    acciones.push(Accion::Derecha);
                } else {
                    acciones.push(Accion::Izquierda);
                }
            } else if dy.abs() > dpad_size * 0.15 {
                if dy > 0.0 {
                    acciones.push(Accion::Abajo);
                } else {
                    acciones.push(Accion::Arriba);
                }
            }
            continue;
        }

        // ── Botón A (derecha abajo, más a la derecha) ────────────────
        let btn_a_x = sw - margin - btn_size * 0.5;
        let btn_a_y = sh - margin - btn_size * 0.5;
        if (x - btn_a_x).abs() < btn_size * 0.7
            && (y - btn_a_y).abs() < btn_size * 0.7
        {
            acciones.push(Accion::BotonA);
            continue;
        }

        // ── Botón B (derecha abajo, más a la izquierda) ──────────────
        let btn_b_x = sw - margin - btn_size * 1.8;
        let btn_b_y = sh - margin - btn_size * 0.5;
        if (x - btn_b_x).abs() < btn_size * 0.7
            && (y - btn_b_y).abs() < btn_size * 0.7
        {
            acciones.push(Accion::BotonB);
            continue;
        }

        // ── Botón Mapa (arriba derecha) ──────────────────────────────
        let btn_m_x = sw - margin - btn_size * 0.5;
        let btn_m_y = margin + btn_size * 0.5;
        if (x - btn_m_x).abs() < btn_size * 0.7
            && (y - btn_m_y).abs() < btn_size * 0.7
        {
            acciones.push(Accion::Mapa);
            continue;
        }

        // ── Botón Libreta (arriba derecha, más adentro) ──────────────
        let btn_l_x = sw - margin - btn_size * 1.8;
        let btn_l_y = margin + btn_size * 0.5;
        if (x - btn_l_x).abs() < btn_size * 0.7
            && (y - btn_l_y).abs() < btn_size * 0.7
        {
            acciones.push(Accion::Libreta);
            continue;
        }
    }

    // Deduplicar acciones (si hay múltiples touches simultáneos)
    acciones.dedup();
    acciones
}