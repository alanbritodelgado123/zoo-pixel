// src/input.rs
use macroquad::prelude::*;

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

pub fn leer_teclado() -> Vec<Accion> {
    let mut acciones = Vec::new();
    if is_key_pressed(KeyCode::Up)     { acciones.push(Accion::Arriba); }
    if is_key_pressed(KeyCode::Down)   { acciones.push(Accion::Abajo); }
    if is_key_pressed(KeyCode::Left)   { acciones.push(Accion::Izquierda); }
    if is_key_pressed(KeyCode::Right)  { acciones.push(Accion::Derecha); }
    if is_key_pressed(KeyCode::Z)      { acciones.push(Accion::BotonA); }
    if is_key_pressed(KeyCode::X)      { acciones.push(Accion::BotonB); }
    if is_key_pressed(KeyCode::M)      { acciones.push(Accion::Mapa); }
    if is_key_pressed(KeyCode::Escape) { acciones.push(Accion::Menu); }
    if is_key_pressed(KeyCode::L)      { acciones.push(Accion::Libreta); }
    acciones
}