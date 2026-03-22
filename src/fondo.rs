// src/fondo.rs
use std::collections::HashMap;
use macroquad::prelude::*;
use crate::escena::Escena;

pub struct Fondos {
    texturas: HashMap<Escena, Texture2D>,
    fallback: Texture2D,
}

impl Fondos {
    pub fn new(fallback_bytes: &[u8]) -> Self {
        let fallback = Texture2D::from_file_with_format(fallback_bytes, Some(ImageFormat::Png));
        fallback.set_filter(FilterMode::Nearest);
        Self { texturas: HashMap::new(), fallback }
    }

    pub fn agregar(&mut self, escena: Escena, png_bytes: &[u8]) {
        let tex = Texture2D::from_file_with_format(png_bytes, Some(ImageFormat::Png));
        tex.set_filter(FilterMode::Nearest);
        self.texturas.insert(escena, tex);
    }

    /// Dibuja el fondo respetando solo el top (barra superior)
    pub fn draw(&self, escena: &Escena, tint: Color, top: f32, _bottom: f32) {
        let tex = self.texturas.get(escena).unwrap_or(&self.fallback);
        let sw = screen_width();
        let sh = screen_height();
        let area_h = sh - top;
        let aspect = tex.width() / tex.height();
        let (dw, dh) = if sw / area_h > aspect {
            (area_h * aspect, area_h)
        } else {
            (sw, sw / aspect)
        };
        draw_texture_ex(
            tex,
            (sw - dw) / 2.0,
            top + (area_h - dh) / 2.0,
            tint,
            DrawTextureParams {
                dest_size: Some(vec2(dw, dh)),
                ..Default::default()
            },
        );
    }

    pub fn tiene_propio(&self, escena: &Escena) -> bool {
        self.texturas.contains_key(escena)
    }
}