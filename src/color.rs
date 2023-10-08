pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}
impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Self { r, g, b }
    }

    pub fn to_string(&self) -> String {
        format!["{} {} {}", self.r, self.g, self.b]
    }
}
