// src/fondo.rs
use std::collections::HashMap;
use macroquad::prelude::*;
use crate::escena::Escena;

// =====================================================================
//  SISTEMA DE FONDOS (spritesheet vertical)
// =====================================================================

pub struct Fondos {
    spritesheet: Texture2D,
    frame_w: f32,
    frame_h: f32,
    indices: HashMap<Escena, usize>,
}

impl Fondos {
    pub fn new(spritesheet_bytes: &[u8], frame_w: f32, frame_h: f32) -> Self {
        let spritesheet = Texture2D::from_file_with_format(
            spritesheet_bytes,
            Some(ImageFormat::Png),
        );
        spritesheet.set_filter(FilterMode::Nearest);

        // Índices en orden de montaje del spritesheet (31 frames)
        let mut indices = HashMap::new();
        let escenas_ordenadas = [
            Escena::E,
            Escena::P1, Escena::P2, Escena::P3, Escena::P4, Escena::P5,
            Escena::Z1_1, Escena::Z1_2, Escena::Z1_3, Escena::Z1_4, Escena::Z1_5,
            Escena::Z2_1, Escena::Z2_2, Escena::Z2_3, Escena::Z2_4, Escena::Z2_5,
            Escena::Z3_1, Escena::Z3_2, Escena::Z3_3, Escena::Z3_4, Escena::Z3_5,
            Escena::Z4_1, Escena::Z4_2, Escena::Z4_3, Escena::Z4_4, Escena::Z4_5,
            Escena::Z5_1, Escena::Z5_2, Escena::Z5_3, Escena::Z5_4, Escena::Z5_5,
        ];
        for (i, escena) in escenas_ordenadas.iter().enumerate() {
            indices.insert(*escena, i);
        }

        Self {
            spritesheet,
            frame_w,
            frame_h,
            indices,
        }
    }

    /// Dibuja el fondo de la escena en el área [top, bottom]
    pub fn draw(&self, escena: &Escena, tint: Color, top: f32, bottom: f32) {
        if let Some(&index) = self.indices.get(escena) {
            let max_index = (self.spritesheet.height() / self.frame_h) as usize;
            if index < max_index {
                self.draw_frame(index, tint, top, bottom);
                return;
            }
        }
        // Fallback: color sólido
        draw_rectangle(
            0.0,
            top,
            screen_width(),
            bottom - top,
            escena.color_fondo(),
        );
    }

    fn draw_frame(&self, index: usize, tint: Color, top: f32, bottom: f32) {
        let src = Rect::new(
            0.0,
            index as f32 * self.frame_h,
            self.frame_w,
            self.frame_h,
        );
        let sw = screen_width();
        let area_h = bottom - top;
        let aspect = self.frame_w / self.frame_h;

        let (dw, dh) = if sw / area_h > aspect {
            (area_h * aspect, area_h)
        } else {
            (sw, sw / aspect)
        };

        draw_texture_ex(
            &self.spritesheet,
            (sw - dw) / 2.0,
            top + (area_h - dh) / 2.0,
            tint,
            DrawTextureParams {
                dest_size: Some(vec2(dw, dh)),
                source: Some(src),
                ..Default::default()
            },
        );
    }

    pub fn tiene_propio(&self, escena: &Escena) -> bool {
        self.indices.contains_key(escena)
    }

    pub fn total_frames(&self) -> usize {
        self.indices.len()
    }
}