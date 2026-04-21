// src/animacion.rs
use macroquad::prelude::*;

/// Sistema de animación para spritesheets verticales
/// Útil para futuros GIFs de animales en pantalla de info
pub struct Animacion {
    texture: Texture2D,
    total_frames: usize,
    fps: f32,
    timer: f32,
    frame_actual: usize,
    frame_w: f32,
    frame_h: f32,
    pub loop_animation: bool,
}

impl Animacion {
    /// Crea una animación desde bytes PNG (spritesheet vertical)
    pub fn new(png_bytes: &[u8], total_frames: usize, fps: f32) -> Self {
        let texture = Texture2D::from_file_with_format(png_bytes, Some(ImageFormat::Png));
        texture.set_filter(FilterMode::Nearest);
        let w = texture.width();
        let h = texture.height();
        Self {
            texture,
            total_frames,
            fps,
            timer: 0.0,
            frame_actual: 0,
            frame_w: w,
            frame_h: h / total_frames as f32,
            loop_animation: true,
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.timer += dt;
        if self.timer >= 1.0 / self.fps {
            self.timer = 0.0;
            if self.loop_animation {
                self.frame_actual = (self.frame_actual + 1) % self.total_frames;
            } else if self.frame_actual + 1 < self.total_frames {
                self.frame_actual += 1;
            }
        }
    }

    pub fn reset(&mut self) {
        self.frame_actual = 0;
        self.timer = 0.0;
    }

    /// Dibuja el frame actual en posición y tamaño específicos
    pub fn draw_at(&self, x: f32, y: f32, w: f32, h: f32, tint: Color) {
        let src = Rect::new(
            0.0,
            self.frame_actual as f32 * self.frame_h,
            self.frame_w,
            self.frame_h,
        );
        draw_texture_ex(
            &self.texture,
            x,
            y,
            tint,
            DrawTextureParams {
                dest_size: Some(vec2(w, h)),
                source: Some(src),
                ..Default::default()
            },
        );
    }

    /// Dibuja el frame actual en pantalla completa (aspect ratio preservado)
    pub fn draw_fullscreen(&self, tint: Color) {
        let src = Rect::new(
            0.0,
            self.frame_actual as f32 * self.frame_h,
            self.frame_w,
            self.frame_h,
        );
        let sw = screen_width();
        let sh = screen_height();
        let aspect = self.frame_w / self.frame_h;
        let (dw, dh) = if sw / sh > aspect {
            (sh * aspect, sh)
        } else {
            (sw, sw / aspect)
        };
        draw_texture_ex(
            &self.texture,
            (sw - dw) / 2.0,
            (sh - dh) / 2.0,
            tint,
            DrawTextureParams {
                dest_size: Some(vec2(dw, dh)),
                source: Some(src),
                ..Default::default()
            },
        );
    }

    pub fn frame(&self) -> usize {
        self.frame_actual
    }

    pub fn is_finished(&self) -> bool {
        !self.loop_animation && self.frame_actual + 1 >= self.total_frames
    }
}