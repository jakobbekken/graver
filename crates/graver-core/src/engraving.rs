pub struct EngravingDefaults {
    pub staff_line_thickness: f64,
    pub stem_thickness: f64,
    pub thin_barline_thickness: f64,
    pub leger_line_thickness: f64,
    pub leger_line_extension: f64,
    pub beam_thickness: f64,
    pub beam_spacing: f64,
}

impl EngravingDefaults {
    pub const BRAVURA: Self = Self {
        staff_line_thickness: 0.13,
        stem_thickness: 0.12,
        thin_barline_thickness: 0.16,
        leger_line_thickness: 0.16,
        leger_line_extension: 0.4,
        beam_thickness: 0.5,
        beam_spacing: 0.25,
    };
}

impl Default for EngravingDefaults {
    fn default() -> Self {
        Self::BRAVURA
    }
}
