use std::{
    fmt::{Debug, Display},
    ops::{Index, IndexMut},
};

fn get_index(index: (usize, usize), width: usize) -> usize {
    TryInto::<usize>::try_into(index.1 * width + index.0).unwrap()
}

/// Mat (like Vec but in 2 dimensions).
#[derive(Debug, Clone)]
pub struct Mat<T> {
    vec: Vec<T>,
    dims: (usize, usize),
}

impl<T> Mat<T> {
    /// Create new Mat.
    pub fn filled_with(default_value: T, dims: (usize, usize)) -> Self
    where
        T: Clone,
    {
        Mat {
            dims,
            vec: vec![
                default_value;
                dims.0
                    .checked_mul(dims.1)
                    .expect("Mat dimensions are too big so their product is out of bounds")
                    .try_into()
                    .unwrap()
            ],
        }
    }

    /// Create Mat from a vector and dimensions, the vector length
    /// must equal the product of the dimensions.
    pub fn from_vec<V>(vec: V, dims: (usize, usize)) -> Self
    where
        T: Clone,
        V: Into<Vec<T>>,
    {
        let vec = vec.into();
        assert_eq!(
            vec.len(),
            dims.0
                .checked_mul(dims.1)
                .expect("Mat dimensions are too big so their product is out of bounds")
                .try_into()
                .unwrap()
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

    /// Create an Iterator that goes through all the values in
    /// the Mat.
    pub fn iter(&self) -> std::slice::Iter<T> {
        self.vec.iter()
    }

    /// Create a mutable Iterator that goes through all the values
    /// in the Mat.
    pub fn iter_mut(&mut self) -> std::slice::IterMut<T> {
        self.vec.iter_mut()
    }

    /// Return a reference to the vector representation of the
    /// Mat.
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

    /// Return the 2d vector representation of the Mat.
    pub fn to_2d_vec(&self) -> Vec<&[T]> {
        self.vec.chunks(self.dims.1.try_into().unwrap()).collect()
    }

    pub fn swap(&mut self, a: (usize, usize), b: (usize, usize)) {
        let a = self.get_index(a);
        let b = self.get_index(b);
        self.vec.swap(a, b);
    }

    /// Transpose the Mat.
    pub fn transpose(&mut self)
    where
        T: Clone,
    {
        let old = self.vec.to_owned();
        for x in 0..self.dims.0 {
            for y in 0..self.dims.1 {
                let index = self.get_index((x, y));
                let swapped_index = get_index((y, x), self.dims.1);
                self.vec[swapped_index] = old[index].to_owned();
            }
        }
        self.invert_dims();
    }

    /// Flip the Vec in 2d around the x axis.
    fn flip_h(&mut self)
    where
        T: Clone,
    {
        *self = self.slice((0, 0), self.dims, (true, false));
    }

    /// Flip the Vec in 2d around the y axis.
    fn flip_v(&mut self)
    where
        T: Clone,
    {
        *self = self.slice((0, 0), self.dims, (false, true));
    }

    /// Flip the Vec in 2d around the x and y axes.
    fn flip_both(&mut self)
    where
        T: Clone,
    {
        *self = self.slice((0, 0), self.dims, (true, true));
    }

    /// Inverts the dimensions of the Mat.
    pub fn invert_dims(&mut self) {
        self.dims = (self.dims.1, self.dims.0);
    }

    /// Create a new Mat starting at `index` with width and height
    /// `dims`, `flip` is used to flip the selected area around
    /// the x and/or y axis.
    fn slice(&self, index: (usize, usize), dims: (usize, usize), flip: (bool, bool)) -> Mat<T>
    where
        T: Clone,
    {
        let b = (index.0 + dims.0 - 1, index.1 + dims.1 - 1);

        assert!(index.0 < self.dims.0);
        assert!(index.1 < self.dims.1);
        assert!(b.0 < self.dims.0);
        assert!(b.1 < self.dims.1);

        let mut new_vec = self.vec.clone();
        new_vec.truncate(dims.0 * dims.1);

        for (x, u) in (index.0..=b.0).enumerate() {
            for (y, v) in (index.1..=b.1).enumerate() {
                let index = (
                    if !flip.0 { x } else { dims.0 - 1 - x },
                    if !flip.1 { y } else { dims.1 - 1 - y },
                );
                new_vec[get_index(index, dims.0)] = self[(u, v)].to_owned();
            }
        }
        Mat::from_vec(new_vec, dims)
    }

    /// The dimensions of the Mat (x and y).
    pub fn dims(&self) -> &(usize, usize) {
        &self.dims
    }

    fn get_index(&self, index: (usize, usize)) -> usize {
        get_index(index, self.dims.0)
    }
}

impl<T, D> Index<D> for Mat<T>
where
    D: Into<(usize, usize)>,
{
    type Output = T;

    fn index(&self, index: D) -> &Self::Output {
        let index = self.get_index(index.into());
        &self.vec[index]
    }
}

impl<T, D> IndexMut<D> for Mat<T>
where
    D: Into<(usize, usize)>,
{
    fn index_mut(&mut self, index: D) -> &mut Self::Output {
        let index = self.get_index(index.into());
        &mut self.vec[index]
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
        let mat = Mat::from_vec((0..6).collect::<Vec<_>>(), (3, 2));

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
        Mat::from_vec((0..10).collect::<Vec<_>>(), (2, 2));
    }

    #[test]
    fn index() {
        let mut vec: Mat<bool> = Mat::filled_with(false, (1, 1));

        assert_eq!(vec[(0, 0)], false);

        vec[(0, 0)] = true;

        assert_eq!(vec[(0, 0)], true);
    }

    #[test]
    fn fill() {
        let mut vec: Mat<bool> = Mat::filled_with(false, (2, 2));

        vec.fill(true);

        for x in 0..2 {
            for y in 0..2 {
                assert_eq!(vec[(x, y)], true);
            }
        }
    }

    #[test]
    fn fill_with() {
        let mut mat: Mat<u16> = Mat::filled_with(0, (2, 3));

        let mut n: u16 = 0;

        mat.fill_with(|| {
            n += 1;
            n - 1
        });

        assert_eq!(mat, Mat::from_vec([0, 1, 2, 3, 4, 5].to_vec(), (2, 3)));
    }

    #[test]
    fn swap() {
        let mat = Mat::from_vec((0..6).collect::<Vec<_>>(), (3, 2));

        let mut mat1 = mat.clone();
        let mut mat2 = mat.clone();
        let mut mat3 = mat.clone();

        mat1.swap((0, 0), (2, 1));

        mat2.swap((1, 0), (1, 0));

        mat3.swap((2, 0), (0, 1));
        mat3.swap((0, 1), (2, 1));

        assert_eq!(mat1, Mat::from_vec([5, 1, 2, 3, 4, 0].to_vec(), (3, 2)));
        assert_eq!(mat2, mat);
        assert_eq!(mat3, Mat::from_vec([0, 1, 3, 5, 4, 2].to_vec(), (3, 2)));
    }

    #[test]
    fn transpose() {
        let mut mat = Mat::from_vec((0..12).collect::<Vec<_>>(), (4, 3));
        mat.transpose();

        assert_eq!(
            mat,
            Mat::from_vec([0, 4, 8, 1, 5, 9, 2, 6, 10, 3, 7, 11].to_vec(), (3, 4))
        );
    }

    #[test]
    fn flip() {
        let mat = Mat::from_vec((0..8).collect::<Vec<_>>(), (4, 2));

        let mut mat1 = mat.clone();
        mat1.flip_v();
        assert_eq!(mat1, Mat::from_vec([4, 5, 6, 7, 0, 1, 2, 3], mat1.dims));

        let mut mat2 = mat.clone();
        mat2.flip_h();
        assert_eq!(mat2, Mat::from_vec([3, 2, 1, 0, 7, 6, 5, 4], mat2.dims));

        let mut mat3 = mat.clone();
        mat3.flip_h();
        mat3.flip_v();
        assert_eq!(mat3, Mat::from_vec([7, 6, 5, 4, 3, 2, 1, 0], mat3.dims));
    }

    #[test]
    fn slice() {
        let mat = Mat::from_vec((0..16).collect::<Vec<_>>(), (4, 4));

        assert_eq!(
            mat.slice((0, 0), (4, 4), (false, false)),
            Mat::from_vec(
                [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
                (4, 4)
            )
        );
        assert_eq!(
            mat.slice((2, 2), (2, 2), (false, false)),
            Mat::from_vec([10, 11, 14, 15], (2, 2))
        );
        assert_eq!(
            mat.slice((0, 0), (4, 4), (true, false)),
            Mat::from_vec(
                [3, 2, 1, 0, 7, 6, 5, 4, 11, 10, 9, 8, 15, 14, 13, 12],
                (4, 4)
            )
        );
        assert_eq!(
            mat.slice((0, 0), (4, 1), (true, false)),
            Mat::from_vec([3, 2, 1, 0], (4, 1))
        );
        assert_eq!(
            mat.slice((0, 1), (4, 1), (false, false)),
            Mat::from_vec([4, 5, 6, 7], (4, 1))
        );

        assert_eq!(
            mat.slice((0, 0), (4, 4), (false, true)),
            Mat::from_vec(
                [12, 13, 14, 15, 8, 9, 10, 11, 4, 5, 6, 7, 0, 1, 2, 3],
                (4, 4)
            )
        );
        assert_eq!(
            mat.slice((0, 0), (4, 4), (true, true)),
            Mat::from_vec(
                [15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0],
                (4, 4)
            )
        );
        assert_eq!(
            mat.slice((0, 0), (1, 4), (false, true)),
            Mat::from_vec([12, 8, 4, 0], (1, 4))
        );
    }

    #[test]
    fn slice_fail() {
        let mat = Mat::from_vec((0..16).collect::<Vec<_>>(), (4, 4));
        mat.slice((1, 1), (4, 4), (false, false));
    }
}
