use macroquad::prelude::*;
use crate::estado::Estado;

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
    if is_key_pressed(KeyCode::Enter)  { acciones.push(Accion::BotonA); }
    if is_key_pressed(KeyCode::X)      { acciones.push(Accion::BotonB); }
    if is_key_pressed(KeyCode::Escape) { acciones.push(Accion::BotonB); }
    if is_key_pressed(KeyCode::M)      { acciones.push(Accion::Mapa); }
    if is_key_pressed(KeyCode::L)      { acciones.push(Accion::Libreta); }
    
    acciones
}

// ✅ NUEVO: Input táctil para Android (landscape)
pub fn leer_tactil(estado: &Estado) -> Vec<Accion> {
    let mut acciones = Vec::new();
    let touches = touches();
    
    if !estado.plataforma.es_movil() {
        return acciones;
    }
    
    let sw = screen_width();
    let sh = screen_height();
    
    // Zonas de botones en landscape (800x480)
    let btn_size = sh * 0.15;
    let margin = sh * 0.05;
    
    for touch in touches {
        let x = touch.position.x;
        let y = touch.position.y;
        
        // Botón A (abajo derecha)
        if x > sw - btn_size - margin && y > sh - btn_size - margin {
            acciones.push(Accion::BotonA);
        }
        
        // Botón B (abajo derecha, más adentro)
        if x > sw - btn_size * 2.5 - margin && y > sh - btn_size - margin
            && x < sw - btn_size - margin {
            acciones.push(Accion::BotonB);
        }
        
        // Stick virtual (abajo izquierda)
        if x < btn_size * 2.0 + margin && y > sh - btn_size * 2.0 - margin {
            let centro_x = margin + btn_size;
            let centro_y = sh - margin - btn_size;
            let dx = x - centro_x;
            let dy = y - centro_y;
            let distancia = (dx * dx + dy * dy).sqrt();
            
            if distancia > btn_size * 0.3 {
                if dx.abs() > dy.abs() {
                    if dx > 0.0 { acciones.push(Accion::Derecha); }
                    else { acciones.push(Accion::Izquierda); }
                } else {
                    if dy > 0.0 { acciones.push(Accion::Abajo); }
                    else { acciones.push(Accion::Arriba); }
                }
            }
        }
        
        // Botón Mapa (arriba derecha)
        if x > sw - btn_size - margin && y < margin + btn_size {
            acciones.push(Accion::Mapa);
        }
        
        // Botón Libreta (arriba derecha, más adentro)
        if x > sw - btn_size * 2.5 - margin && y < margin + btn_size
            && x < sw - btn_size - margin {
            acciones.push(Accion::Libreta);
        }
    }
    
    acciones
}