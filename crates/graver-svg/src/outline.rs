use ttf_parser::OutlineBuilder;

use crate::number::num;

pub fn glyph_path() {}

struct PathBuilder {
    path: String,
    scale: f64,
}

impl PathBuilder {
    fn pt(&self, x: f32, y: f32) -> String {
        format!(
            "{} {}",
            num(f64::from(x) * self.scale),
            num(-f64::from(y) * self.scale)
        )
    }
}

impl OutlineBuilder for PathBuilder {
    fn move_to(&mut self, x: f32, y: f32) {}
    fn line_to(&mut self, x: f32, y: f32) {}
    fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {}
    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {}
    fn close(&mut self) {}
}
