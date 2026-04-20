use macroquad::prelude::*;

pub struct Animacion {
    texture: Texture2D,
    total_frames: usize,
    fps: f32,
    timer: f32,
    frame_actual: usize,
    frame_w: f32,
    frame_h: f32,
}

impl Animacion {
    pub fn new(png_bytes: &[u8], total_frames: usize, fps: f32) -> Self {
        let texture = Texture2D::from_file_with_format(png_bytes, Some(ImageFormat::Png));
        texture.set_filter(FilterMode::Nearest);
        let w = texture.width();
        let h = texture.height();
        Self {
            texture, total_frames, fps, timer: 0.0, frame_actual: 0,
            frame_w: w, frame_h: h / total_frames as f32,
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.timer += dt;
        if self.timer >= 1.0 / self.fps {
            self.timer = 0.0;
            self.frame_actual = (self.frame_actual + 1) % self.total_frames;
        }
    }

    #[allow(dead_code)]
    pub fn draw_at(&self, x: f32, y: f32, w: f32, h: f32, tint: Color) {
        let src = Rect::new(0.0, self.frame_actual as f32 * self.frame_h,
                            self.frame_w, self.frame_h);
        draw_texture_ex(&self.texture, x, y, tint,
            DrawTextureParams {
                dest_size: Some(vec2(w, h)),
                source: Some(src),
                ..Default::default()
            });
    }

    pub fn draw_fullscreen(&self, tint: Color) {
        let src = Rect::new(0.0, self.frame_actual as f32 * self.frame_h,
                            self.frame_w, self.frame_h);
        let sw = screen_width();
        let sh = screen_height();
        let aspect = self.frame_w / self.frame_h;
        let (dw, dh) = if sw / sh > aspect {
            (sh * aspect, sh)
        } else {
            (sw, sw / aspect)
        };
        draw_texture_ex(&self.texture, (sw - dw) / 2.0, (sh - dh) / 2.0, tint,
            DrawTextureParams {
                dest_size: Some(vec2(dw, dh)),
                source: Some(src),
                ..Default::default()
            });
    }

    pub fn frame(&self) -> usize { self.frame_actual }
}