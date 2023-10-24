use std::ops;

#[derive(Default, Clone)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}
impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Self { r, g, b }
    }
}

impl ops::Mul<f64> for Color {
    type Output = Self;

    fn mul(self, other: f64) -> Self {
        Self {
            r: self.r * other,
            g: self.g * other,
            b: self.b * other,
        }
    }
}

impl ops::Add for Color {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            r: self.r + other.r,
            g: self.g + other.g,
            b: self.b + other.b,
        }
    }
}

impl ops::Mul for Color {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            r: self.r * other.r,
            g: self.g * other.g,
            b: self.b * other.b,
        }
    }
}
