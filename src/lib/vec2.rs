use std::ops::{Add, AddAssign, Div, Mul, Sub};

fn f64_to_usize(value: f64) -> usize {
    let value = value.floor();
    if value.is_sign_positive() && value == (value as usize) as f64 {
        value as usize
    } else {
        panic!("Failed to convert f64 to usize.")
    }
}
fn usize_to_f64(value: usize) -> f64 {
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

    pub fn from_usize(x: usize, y: usize) -> Vec2 {
        Vec2(usize_to_f64(x), usize_to_f64(y))
    }

    pub fn filled_with(value: f64) -> Vec2 {
        Vec2(value.clone(), value)
    }

    pub fn abs(&self) -> Vec2 {
        Vec2(self.0.abs(), self.1.abs())
    }

    pub fn to_usize(&self) -> (usize, usize) {
        (f64_to_usize(self.0), f64_to_usize(self.1))
    }
}

impl Add for Vec2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vec2(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
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
