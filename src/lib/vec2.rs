use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

fn f64_to_usize(value: f64) -> usize {
    let value = value.floor();
    if value.is_sign_positive() && value == (value as usize) as f64 {
        value as usize
    } else {
        panic!("Failed to convert f64 to usize.")
    }
}
const fn usize_to_f64(value: usize) -> f64 {
    if value == (value as f64) as usize {
        value as f64
    } else {
        panic!("Failed to convert usize to f64.")
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec2(pub f64, pub f64);

impl Vec2 {
    pub const ZERO: Vec2 = Vec2(0., 0.);

    pub const fn from_usize(x: usize, y: usize) -> Vec2 {
        Vec2(usize_to_f64(x), usize_to_f64(y))
    }

    pub const fn filled_with(value: f64) -> Vec2 {
        Vec2(value, value)
    }

    pub fn to_usize(&self) -> (usize, usize) {
        (f64_to_usize(self.0), f64_to_usize(self.1))
    }

    /// Return a vector of the same length but orthogonal to itself.
    pub fn orthogonal(&self) -> Vec2 {
        return Vec2(self.1, -self.0);
    }

    /// Return the dot (or scalar) product of the Vec2 with another
    /// one.
    pub fn dot_product(&self, other: Vec2) -> f64 {
        return self.0 * other.0 + self.1 * other.1;
    }

    // fn project(poly: &[Vector], axis: Vector) -> Vector {
    //     // Returns a vector showing how much of the poly lies along the axis
    //     let mut min: Option<f32> = None;
    //     let mut max: Option<f32> = None;

    //     for point in poly.iter() {
    //         let dot = dot_product(*point, axis);

    //         match min {
    //             Some(val) if val < dot => (),
    //             _ => min = Some(dot),
    //         }
    //         match max {
    //             Some(val) if val > dot => (),
    //             _ => max = Some(dot),
    //         }
    //     }

    //     return Vector(min.unwrap(), max.unwrap());
    // }

    pub fn abs(&self) -> Vec2 {
        Vec2(self.0.abs(), self.1.abs())
    }

    pub fn length(&self) -> f64 {
        (self.0.powi(2) + self.1.powi(2)).sqrt()
    }
}

impl Add for Vec2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vec2(self.0 + rhs.0, self.1 + rhs.1)
    }
}
impl Sub for Vec2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec2(self.0 - rhs.0, self.1 - rhs.1)
    }
}
impl Mul for Vec2 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Vec2(self.0 * rhs.0, self.1 * rhs.1)
    }
}
impl Neg for Vec2 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vec2(-self.0, -self.1)
    }
}

impl Mul<f64> for Vec2 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec2(self.0 * rhs, self.1 * rhs)
    }
}
impl Div<f64> for Vec2 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Vec2(self.0 / rhs, self.1 / rhs)
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}
impl SubAssign for Vec2 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs
    }
}
impl MulAssign for Vec2 {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs
    }
}
impl MulAssign<f64> for Vec2 {
    fn mul_assign(&mut self, rhs: f64) {
        *self = *self * rhs
    }
}
impl DivAssign<f64> for Vec2 {
    fn div_assign(&mut self, rhs: f64) {
        *self = *self / rhs
    }
}
