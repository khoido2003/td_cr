use macroquad::prelude::*;

#[derive(Debug, Default)]
pub struct Renderer {
    scale: f32,
    offset_x: f32,
    offset_y: f32,
}

pub const VIRTUAL_WIDTH: f32 = 480.0;
pub const VIRTUAL_HEIGHT: f32 = 854.0;

impl Renderer {
    pub fn new() -> Self {
        let screen_w = screen_width();
        let screen_h = screen_height();
        let aspect_ratio = VIRTUAL_WIDTH / VIRTUAL_HEIGHT;
        let screen_aspect = screen_w / screen_h;

        let scale = if screen_aspect > aspect_ratio {
            screen_h / VIRTUAL_HEIGHT
        } else {
            screen_w / VIRTUAL_WIDTH
        };

        let scaled_width = VIRTUAL_WIDTH * scale;
        let scaled_height = VIRTUAL_HEIGHT * scale;

        let offset_x = (screen_w - scaled_width) / 2.0;
        let offset_y = (screen_h - scaled_height) / 2.0;

        Renderer {
            scale,
            offset_y,
            offset_x,
        }
    }

    fn to_screen(&self, x: f32, y: f32) -> (f32, f32) {
        (
            self.offset_x + x * self.scale,
            self.offset_y + y * self.scale,
        )
    }

pub     fn to_virtual(&self, x: f32, y: f32) -> (f32, f32) {
        (
            (x - self.offset_x) / self.scale,
            (y - self.offset_y) / self.scale,
        )
    }

    pub fn draw_rectangle(&self, x: f32, y: f32, w: f32, h: f32, color: Color) {
        let (sx, sy) = self.to_screen(x * VIRTUAL_WIDTH, y * VIRTUAL_HEIGHT);
        let sw = w * VIRTUAL_WIDTH * self.scale;
        let sh = h * VIRTUAL_HEIGHT * self.scale;

        draw_rectangle(sx, sy, sw, sh, color);
    }

    pub fn draw_rectangle_lines(
        &self,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        thickness: f32,
        color: Color,
    ) {
        let (sx, sy) = self.to_screen(x * VIRTUAL_WIDTH, y * VIRTUAL_HEIGHT);
        let sw = w * VIRTUAL_WIDTH * self.scale;
        let sh = h * VIRTUAL_HEIGHT * self.scale;

        draw_rectangle_lines(sx, sy, sw, sh, thickness, color);
    }

    pub fn draw_circle(&self, x: f32, y: f32, radius: f32, color: Color) {
        let (sx, sy) = self.to_screen(x * VIRTUAL_WIDTH, y * VIRTUAL_HEIGHT);
        let sr = radius * self.scale;

        draw_circle(sx, sy, sr, color);
    }

    pub fn draw_circle_lines(&self, x: f32, y: f32, radius: f32, thickness: f32, color: Color) {
        let (sx, sy) = self.to_screen(x * VIRTUAL_WIDTH, y * VIRTUAL_HEIGHT);
        let sr = radius * self.scale;

        draw_circle_lines(sx, sy, sr, thickness, color);
    }

    pub fn draw_text(&self, text: &str, x: f32, y: f32, font_size: f32, color: Color) {
        let (sx, sy) = self.to_screen(x * VIRTUAL_WIDTH, y * VIRTUAL_HEIGHT);
        let scaled_font_size = font_size * self.scale;

        draw_text(text, sx, sy, scaled_font_size, color);
    }

    pub fn draw_line(&self, x1: f32, y1: f32, x2: f32, y2: f32, thickness: f32, color: Color) {
        let (sx1, sy1) = self.to_screen(x1 * VIRTUAL_WIDTH, y1 * VIRTUAL_HEIGHT);
        let (sx2, sy2) = self.to_screen(x2 * VIRTUAL_WIDTH, y2 * VIRTUAL_HEIGHT);

        draw_line(sx1, sy1, sx2, sy2, thickness, color);
    }
}
