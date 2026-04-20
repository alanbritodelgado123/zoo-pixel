use std::collections::HashMap;
use macroquad::prelude::*;
use crate::escena::Escena;

pub struct Fondos {
    spritesheet: Texture2D,
    frame_w: f32,
    frame_h: f32,
    indices: HashMap<Escena, usize>,
}

impl Fondos {
    pub fn new(spritesheet_bytes: &[u8], frame_w: f32, frame_h: f32) -> Self {
        let spritesheet = Texture2D::from_file_with_format(spritesheet_bytes, Some(ImageFormat::Png));
        spritesheet.set_filter(FilterMode::Nearest);
        
        // ORDEN ALFABÉTICO de montage (31 frames completos)
        let mut indices = HashMap::new();
        indices.insert(Escena::E, 0);
        indices.insert(Escena::P1, 1);
        indices.insert(Escena::P2, 2);
        indices.insert(Escena::P3, 3);
        indices.insert(Escena::P4, 4);
        indices.insert(Escena::P5, 5);
        indices.insert(Escena::Z1_1, 6);
        indices.insert(Escena::Z1_2, 7);
        indices.insert(Escena::Z1_3, 8);
        indices.insert(Escena::Z1_4, 9);
        indices.insert(Escena::Z1_5, 10);
        indices.insert(Escena::Z2_1, 11);
        indices.insert(Escena::Z2_2, 12);
        indices.insert(Escena::Z2_3, 13);
        indices.insert(Escena::Z2_4, 14);
        indices.insert(Escena::Z2_5, 15);
        indices.insert(Escena::Z3_1, 16);
        indices.insert(Escena::Z3_2, 17);
        indices.insert(Escena::Z3_3, 18);
        indices.insert(Escena::Z3_4, 19);
        indices.insert(Escena::Z3_5, 20);
        indices.insert(Escena::Z4_1, 21);
        indices.insert(Escena::Z4_2, 22);
        indices.insert(Escena::Z4_3, 23);
        indices.insert(Escena::Z4_4, 24);
        indices.insert(Escena::Z4_5, 25);
        indices.insert(Escena::Z5_1, 26);
        indices.insert(Escena::Z5_2, 27);
        indices.insert(Escena::Z5_3, 28);
        indices.insert(Escena::Z5_4, 29);
        indices.insert(Escena::Z5_5, 30);
        
        Self { spritesheet, frame_w, frame_h, indices }
    }

    pub fn draw(&self, escena: &Escena, tint: Color, top: f32, bottom: f32) {
        if let Some(&index) = self.indices.get(escena) {
            let max_index = (self.spritesheet.height() / self.frame_h) as usize;
            
            if index < max_index {
                let src = Rect::new(0.0, index as f32 * self.frame_h, self.frame_w, self.frame_h);
                
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
                return;
            }
        }
        
        // Fallback: color sólido para zonas sin imagen
        draw_rectangle(0.0, top, screen_width(), bottom - top, escena.color_fondo());
    }

    pub fn tiene_propio(&self, escena: &Escena) -> bool {
        self.indices.contains_key(escena)
    }
}