pub struct Scene {
    pub width: f64,
    pub height: f64,
    pub items: Vec<Item>,
}

pub enum Item {
    Line(Line),
    Glyph(Glyph),
}

pub struct Line {
    pub x1: f64,
    pub y1: f64,
    pub x2: f64,
    pub y2: f64,
    pub thickness: f64,
}

pub struct Glyph {
    pub codepoint: char,
    pub x: f64,
    pub y: f64,
}
