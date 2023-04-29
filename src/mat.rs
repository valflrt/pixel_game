use std::{
    fmt::{Debug, Display},
    ops::{Index, IndexMut},
};

#[derive(Debug, Clone)]
pub struct Mat<T> {
    vec: Vec<T>,
    dims: (usize, usize),
}

impl<T> Mat<T> {
    pub fn new(default_value: T, dims: (usize, usize)) -> Self
    where
        T: Clone,
    {
        Mat {
            dims,
            vec: vec![
                default_value;
                dims.0
                    .checked_mul(dims.1)
                    .expect("matrix dimensions are too big so their product is out of bounds")
            ],
        }
    }

    pub fn from_vec(vec: Vec<T>, dims: (usize, usize)) -> Self {
        assert_eq!(
            vec.len(),
            dims.0
                .checked_mul(dims.1)
                .expect("matrix dimensions are too big so their product is out of bounds")
        );
        Mat { dims, vec }
    }

    pub fn fill(&mut self, value: T)
    where
        T: Clone,
    {
        self.vec.fill(value);
    }

    pub fn fill_with<F>(&mut self, mut f: F)
    where
        F: FnMut() -> T,
    {
        for el in self.vec.iter_mut() {
            *el = f();
        }
    }

    pub fn iter(&self) -> std::slice::Iter<T> {
        self.vec.iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<T> {
        self.vec.iter_mut()
    }
}

impl<T, D> Index<D> for Mat<T>
where
    D: Into<(usize, usize)>,
{
    type Output = T;

    fn index(&self, index: D) -> &Self::Output
    where
        D: Into<(usize, usize)>,
    {
        let index = index.into();
        &self.vec[index.0 * self.dims.1 + index.1]
    }
}

impl<T, D> IndexMut<D> for Mat<T>
where
    D: Into<(usize, usize)>,
{
    fn index_mut(&mut self, index: D) -> &mut Self::Output {
        let index = index.into();
        &mut self.vec[index.0 * self.dims.1 + index.1]
    }
}

impl<T> Display for Mat<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?}",
            self.vec.chunks(self.dims.1).collect::<Vec<&[T]>>()
        )
    }
}

#[cfg(test)]
mod test {
    use super::Mat;

    #[test]
    fn from_vec() {
        let vec = Mat::from_vec((0..6).collect(), (3, 2));

        assert_eq!(vec[(0, 0)], 0);
        assert_eq!(vec[(0, 1)], 1);
        assert_eq!(vec[(1, 0)], 2);
        assert_eq!(vec[(1, 1)], 3);
        assert_eq!(vec[(2, 0)], 4);
        assert_eq!(vec[(2, 1)], 5);
    }

    #[test]
    #[should_panic]
    fn from_vec_fail() {
        Mat::from_vec((0..10).collect(), (2, 2));
    }

    #[test]
    fn index() {
        let mut vec: Mat<bool> = Mat::new(false, (1, 1));

        assert_eq!(vec[(0, 0)], false);

        vec[(0, 0)] = true;

        assert_eq!(vec[(0, 0)], true);
    }

    #[test]
    fn fill() {
        let mut vec: Mat<bool> = Mat::new(false, (2, 2));

        vec.fill(true);

        for x in 0..2 {
            for y in 0..2 {
                assert_eq!(vec[(x, y)], true);
            }
        }
    }

    #[test]
    fn fill_with() {
        let mut vec: Mat<u16> = Mat::new(0, (2, 3));

        let mut n: u16 = 0;

        vec.fill_with(|| {
            n += 1;
            n - 1
        });

        assert_eq!(vec[(0, 0)], 0);
        assert_eq!(vec[(0, 1)], 1);
        assert_eq!(vec[(0, 2)], 2);
        assert_eq!(vec[(1, 0)], 3);
        assert_eq!(vec[(1, 1)], 4);
        assert_eq!(vec[(1, 2)], 5);
    }
}
