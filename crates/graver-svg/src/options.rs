pub struct SvgOptions {
    pub pixels_per_space: f64,
    pub margin: f64,
}

impl Default for SvgOptions {
    fn default() -> Self {
        Self {
            pixels_per_space: 12.0,
            margin: 1.0,
        }
    }
}
