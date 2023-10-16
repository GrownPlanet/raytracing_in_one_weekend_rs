use rand::Rng;
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

    fn random(min: f64, max: f64) -> Self {
        let mut rng = rand::thread_rng();

        let random_x: f64 = rng.gen_range(min..max);
        let random_y: f64 = rng.gen_range(min..max);
        let random_z: f64 = rng.gen_range(min..max);

        Self {
            x: random_x,
            y: random_y,
            z: random_z,
        }
    }

    pub fn random_unit_vector() -> Self {
        Self::unit_vector(&Self::random(-1., 1.))
    }

    // pub fn random_on_hemisphere(normal: &Self) -> Self {
    //     let on_unit_sphere = Self::random_unit_vector();

    //     if Self::dot(&on_unit_sphere, normal) > 0. {
    //         return on_unit_sphere;
    //     } else {
    //         return on_unit_sphere * -1.;
    //     }
    // }
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
