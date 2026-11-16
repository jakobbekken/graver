use std::fmt;

#[derive(Debug)]
pub enum SvgError {
    Font(String),
    MissingGlyph(char),
}

impl fmt::Display for SvgError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SvgError::Font(e) => write!(f, "Could not read the embedded: {e}"),
            SvgError::MissingGlyph(c) => write!(f, "The font has no glyph for U+{:04X}", *c as u32),
        }
    }
}

impl std::error::Error for SvgError {}
