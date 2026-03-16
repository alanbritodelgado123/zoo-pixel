use std::collections::HashMap;
use macroquad::prelude::*;
use crate::escena::Escena;

pub struct Fondos {
    texture: Texture2D,
    frame_w: f32,
    frame_h: f32,
    indices: HashMap<Escena, usize>,
}

impl Fondos {
    pub fn new(png_bytes: &[u8], total_zonas: usize) -> Self {
        let texture = Texture2D::from_file_with_format(png_bytes, Some(ImageFormat::Png));
        texture.set_filter(FilterMode::Nearest);
        let w = texture.width();
        let h = texture.height();

        let mut indices = HashMap::new();
        indices.insert(Escena::Entrada,      0);
        indices.insert(Escena::Sabana,       1);
        indices.insert(Escena::Laguna,       2);
        indices.insert(Escena::Aviario,      3);
        indices.insert(Escena::Felinos,      4);
        indices.insert(Escena::Reptiliario,  5);
        indices.insert(Escena::Primates,     6);
        indices.insert(Escena::Montana,      7);
        indices.insert(Escena::Humedal,      8);
        indices.insert(Escena::Nocturario,   9);

        Self {
            texture,
            frame_w: w,
            frame_h: h / total_zonas as f32,
            indices,
        }
    }

    /// Dibuja el fondo en el área disponible (encima de la barra)
    pub fn draw(&self, escena: &Escena, tint: Color, area_h: f32) {
        let idx = match self.indices.get(escena) {
            Some(i) => *i,
            None => return,
        };

        let src = Rect::new(
            0.0,
            idx as f32 * self.frame_h,
            self.frame_w,
            self.frame_h,
        );

        let sw = screen_width();
        let aspect = self.frame_w / self.frame_h;
        let (dw, dh) = if sw / area_h > aspect {
            (area_h * aspect, area_h)
        } else {
            (sw, sw / aspect)
        };

        draw_texture_ex(
            &self.texture,
            (sw - dw) / 2.0,
            (area_h - dh) / 2.0,
            tint,
            DrawTextureParams {
                dest_size: Some(vec2(dw, dh)),
                source: Some(src),
                ..Default::default()
            },
        );
    }

    pub fn tiene(&self, escena: &Escena) -> bool {
        self.indices.contains_key(escena)
    }
}