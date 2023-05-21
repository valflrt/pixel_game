use std::ops::{Add, AddAssign, Div, Mul, Sub};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec2<T>(pub T, pub T);

impl<T> Add for Vec2<T>
where
    T: Add<Output = T>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vec2(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl<T> AddAssign for Vec2<T>
where
    Vec2<T>: Add<Output = Vec2<T>> + Copy,
{
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}

impl<T> Sub for Vec2<T>
where
    T: Sub<Output = T>,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec2(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl<T> Mul for Vec2<T>
where
    T: Mul<Output = T>,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Vec2(self.0 * rhs.0, self.1 * rhs.1)
    }
}

impl<T> Mul<T> for Vec2<T>
where
    T: Mul<Output = T> + Copy,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Vec2(self.0 * rhs, self.1 * rhs)
    }
}

impl<T> Div<T> for Vec2<T>
where
    T: Div<Output = T> + Copy,
{
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Vec2(self.0 / rhs, self.1 / rhs)
    }
}

impl<T: Default> Default for Vec2<T> {
    fn default() -> Self {
        Vec2(T::default(), T::default())
    }
}

impl<T> Into<Vec2<T>> for (T, T) {
    fn into(self) -> Vec2<T> {
        Vec2(self.0, self.1)
    }
}
impl<T> Into<(T, T)> for Vec2<T> {
    fn into(self) -> (T, T) {
        (self.0, self.1)
    }
}

impl Into<Vec2<i32>> for Vec2<f64> {
    fn into(self) -> Vec2<i32> {
        Vec2(self.0.floor() as i32, self.1.floor() as i32)
    }
}
impl Into<Vec2<i32>> for (f64, f64) {
    fn into(self) -> Vec2<i32> {
        Vec2(self.0.floor() as i32, self.1.floor() as i32)
    }
}
impl Into<Vec2<f64>> for Vec2<i32> {
    fn into(self) -> Vec2<f64> {
        Vec2(self.0 as f64, self.1 as f64)
    }
}
impl Into<Vec2<f64>> for (i32, i32) {
    fn into(self) -> Vec2<f64> {
        Vec2(self.0 as f64, self.1 as f64)
    }
}

impl Into<Vec2<usize>> for Vec2<i32> {
    fn into(self) -> Vec2<usize> {
        Vec2(
            self.0.try_into().expect("Failed to convert i32 to usize."),
            self.1.try_into().expect("Failed to convert i32 to usize."),
        )
    }
}
impl Into<Vec2<usize>> for (i32, i32) {
    fn into(self) -> Vec2<usize> {
        Vec2(
            self.0.try_into().expect("Failed to convert i32 to usize."),
            self.1.try_into().expect("Failed to convert i32 to usize."),
        )
    }
}
impl Into<Vec2<i32>> for Vec2<usize> {
    fn into(self) -> Vec2<i32> {
        Vec2(
            self.0.try_into().expect("Failed to convert usize to i32."),
            self.1.try_into().expect("Failed to convert usize to i32."),
        )
    }
}
impl Into<Vec2<i32>> for (usize, usize) {
    fn into(self) -> Vec2<i32> {
        Vec2(
            self.0.try_into().expect("Failed to convert usize to i32."),
            self.1.try_into().expect("Failed to convert usize to i32."),
        )
    }
}
