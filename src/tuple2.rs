use std::ops::Mul;

#[derive(Debug, Clone, Copy)]
pub struct Tuple2<T>(pub T, pub T)
where
    T: Mul<Output = T> + Copy;

impl<T> Tuple2<T>
where
    T: Mul<Output = T> + Copy,
{
    pub fn product(&self) -> T {
        self.0 * self.1
    }
}

impl<T> Into<Tuple2<T>> for (T, T)
where
    T: Mul<Output = T> + Copy,
{
    fn into(self) -> Tuple2<T> {
        Tuple2(self.0, self.1)
    }
}
