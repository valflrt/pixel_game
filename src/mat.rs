use std::{
    fmt::{Debug, Display},
    ops::{Index, IndexMut},
};

/// Matrix (vector in 2 dimensions).
#[derive(Debug, Clone)]
pub struct Mat<T> {
    vec: Vec<T>,
    dims: (usize, usize),
}

impl<T> Mat<T> {
    /// Create new Mat.
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

    /// Create Mat from a vector and dimensions, the vector
    /// length must equal the product of the dimensions.
    pub fn from_vec(vec: Vec<T>, dims: (usize, usize)) -> Self {
        assert_eq!(
            vec.len(),
            dims.0
                .checked_mul(dims.1)
                .expect("matrix dimensions are too big so their product is out of bounds")
        );
        Mat { dims, vec }
    }

    /// Fill the Mat with the given value.
    pub fn fill(&mut self, value: T)
    where
        T: Clone,
    {
        self.vec.fill(value);
    }

    /// Fill the Mat with the value returned by the provided
    /// closure.
    pub fn fill_with<F>(&mut self, mut f: F)
    where
        F: FnMut() -> T,
    {
        for el in self.vec.iter_mut() {
            *el = f();
        }
    }

    /// Create an Iterator that goes through all the values
    /// in the Mat.
    pub fn iter(&self) -> std::slice::Iter<T> {
        self.vec.iter()
    }

    /// Create a mutable Iterator that goes through all the
    /// values in the Mat.
    pub fn iter_mut(&mut self) -> std::slice::IterMut<T> {
        self.vec.iter_mut()
    }

    /// Return a reference to the vector representation of
    /// the Mat.
    pub fn vec(&self) -> &Vec<T> {
        &self.vec
    }

    /// Return the vector representation of the Mat.
    pub fn to_vec(&self) -> Vec<T>
    where
        T: Clone,
    {
        self.vec.to_owned()
    }

    pub fn transpose(&mut self)
    where
        T: Clone,
    {
        let mat = self.clone();
        for x in 0..self.dims.0 {
            for y in 0..self.dims.1 {
                self[(y, x)] = mat[(x, y)].clone();
            }
        }
        self.dims = (self.dims.1, self.dims.0)
    }

    pub fn flip(horizontally: bool, vertically: bool) {}

    /// The dimensions of the matrix (x, y).
    pub fn dims(&self) -> (usize, usize) {
        self.dims
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
        &self.vec[index.1 * self.dims.0 + index.0]
    }
}

impl<T, D> IndexMut<D> for Mat<T>
where
    D: Into<(usize, usize)>,
{
    fn index_mut(&mut self, index: D) -> &mut Self::Output {
        let index = index.into();
        &mut self.vec[index.1 * self.dims.0 + index.0]
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
