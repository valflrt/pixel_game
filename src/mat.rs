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

    pub fn to_2d_vec(&self) -> Vec<&[T]> {
        self.vec.chunks(self.dims.1).collect()
    }

    pub fn transpose(&mut self)
    where
        T: Clone,
    {
        let mut mat = self.clone();
        mat.invert_dims();
        for x in 0..mat.dims.0 {
            for y in 0..mat.dims.1 {
                self[(y, x)] = mat[(x, y)].clone();
            }
        }
        self.invert_dims();
    }

    pub fn reverse(&mut self, horizontally: bool, vertically: bool)
    where
        T: Clone,
    {
        if (!horizontally && vertically) || (horizontally && !vertically) {
            self.vec.chunks_mut(self.dims.1).for_each(|v| v.reverse());
        }
        if vertically {
            self.vec.reverse();
        }
    }

    pub fn invert_dims(&mut self) {
        self.dims = (self.dims.1, self.dims.0);
    }

    /// The dimensions of the matrix (x, y).
    pub fn dims(&self) -> &(usize, usize) {
        &self.dims
    }
}

impl<T, D> Index<D> for Mat<T>
where
    D: Into<(usize, usize)>,
{
    type Output = T;

    fn index(&self, index: D) -> &Self::Output {
        let (x, y) = index.into();
        &self.vec[y * self.dims.0 + x]
    }
}

impl<T, D> IndexMut<D> for Mat<T>
where
    D: Into<(usize, usize)>,
{
    fn index_mut(&mut self, index: D) -> &mut Self::Output {
        let (x, y) = index.into();
        &mut self.vec[y * self.dims.0 + x]
    }
}

impl<T> PartialEq<Mat<T>> for Mat<T>
where
    T: PartialEq<T>,
{
    fn eq(&self, other: &Mat<T>) -> bool {
        self.dims == other.dims && self.vec == other.vec
    }
}

impl<T> Display for Mat<T>
where
    T: Debug + Clone,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.to_2d_vec())
    }
}

#[cfg(test)]
mod test {
    use super::Mat;

    #[test]
    fn from_vec() {
        let mat = Mat::from_vec((0..6).collect(), (3, 2));

        println!("{}", mat);

        assert_eq!(mat[(0, 0)], 0);
        assert_eq!(mat[(1, 0)], 1);
        assert_eq!(mat[(2, 0)], 2);
        assert_eq!(mat[(0, 1)], 3);
        assert_eq!(mat[(1, 1)], 4);
        assert_eq!(mat[(2, 1)], 5);
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
        let mut mat: Mat<u16> = Mat::new(0, (2, 3));

        let mut n: u16 = 0;

        mat.fill_with(|| {
            n += 1;
            n - 1
        });

        assert_eq!(mat, Mat::from_vec([0, 1, 2, 3, 4, 5].to_vec(), (2, 3)));
    }

    #[test]
    fn transpose() {
        let mut mat = Mat::from_vec((0..8).collect(), (2, 4));
        mat.transpose();

        println!("{}", mat);

        assert_eq!(
            mat,
            Mat::from_vec([0, 4, 1, 5, 2, 6, 3, 7].to_vec(), (4, 2))
        );
    }

    #[test]
    fn reverse() {
        let mat = Mat::from_vec((0..8).collect(), (2, 4));

        let mut mat1 = mat.clone();
        let mut mat2 = mat.clone();
        let mut mat3 = mat.clone();
        let mut mat4 = mat.clone();

        mat1.reverse(false, false);
        assert_eq!(
            mat1,
            Mat::from_vec([0, 1, 2, 3, 4, 5, 6, 7].to_vec(), (2, 4))
        );
        mat2.reverse(false, true);
        assert_eq!(
            mat2,
            Mat::from_vec([4, 5, 6, 7, 0, 1, 2, 3].to_vec(), (2, 4))
        );
        mat3.reverse(true, false);
        assert_eq!(
            mat3,
            Mat::from_vec([3, 2, 1, 0, 7, 6, 5, 4].to_vec(), (2, 4))
        );
        mat4.reverse(true, true);
        assert_eq!(
            mat4,
            Mat::from_vec([7, 6, 5, 4, 3, 2, 1, 0].to_vec(), (2, 4))
        );
    }
}
