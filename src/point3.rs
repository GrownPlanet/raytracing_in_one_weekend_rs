use std::ops;

#[derive(Clone, Default)]
pub struct Point3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn dot(p1: &Self, p2: &Self) -> f64 {
        p1.x * p2.x + p1.y * p2.y + p1.z * p2.z
    }

    // pub fn cross(p1: &Self, p2: &Self) -> Self {
    //     Self {
    //         x: p1.y * p2.z - p1.z * p2.y,
    //         y: p1.z * p2.x - p1.x * p2.z,
    //         z: p1.x * p2.y - p1.y * p2.x,
    //     }
    // }

    pub fn len(&self) -> f64 {
        self.len_squared().sqrt()
    }

    pub fn len_squared(&self) -> f64 {
        self.x.powi(2) + self.y.powi(2) + self.z.powi(2)
    }

    // change vector coordinates to a value between `-1` and `1`
    pub fn unit_vector(&self) -> Self {
        self.clone() / self.len()
    }
}

// opperator overloading
impl ops::Add for Point3 {
    type Output = Point3;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl ops::Sub for Point3 {
    type Output = Point3;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl ops::Mul for Point3 {
    type Output = Point3;

    fn mul(self, other: Self) -> Self {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl ops::Mul<f64> for Point3 {
    type Output = Point3;

    fn mul(self, other: f64) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl ops::Div<f64> for Point3 {
    type Output = Point3;

    fn div(self, other: f64) -> Self {
        Self {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}
