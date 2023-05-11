use std::{
    fmt::{Debug, Display},
    ops::{Index, IndexMut},
};

fn dims_product(dims: (usize, usize)) -> usize {
    dims.0
        .checked_mul(dims.1)
        .expect("Mat dimensions are too big so their product is out of bounds")
        .try_into()
        .unwrap()
}

fn get_vec_index(index: (usize, usize), width: usize) -> usize {
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
            vec: vec![default_value; dims_product(dims)],
        }
    }

    /// Create Mat from a Vec and dimensions, the Vec length must
    /// equal the product of the dimensions.
    pub fn from_vec<V>(vec: V, dims: (usize, usize)) -> Self
    where
        T: Clone,
        V: Into<Vec<T>>,
    {
        let vec = vec.into();
        assert_eq!(vec.len(), dims_product(dims));
        Mat { dims, vec }
    }

    /// Fill the Mat with the given value.
    pub fn fill(&mut self, value: T)
    where
        T: Clone,
    {
        self.vec.fill(value);
    }

    /// Fill the Mat row-wise with the value returned by the
    /// provided closure.
    pub fn fill_with_r<F>(&mut self, mut f: F)
    where
        F: FnMut((usize, usize)) -> T,
    {
        for y in 0..self.dims.1 {
            for x in 0..self.dims.0 {
                self[(x, y)] = f((x, y))
            }
        }
    }

    /// Fill the Mat column-wise with the value returned by the
    /// provided closure.
    pub fn fill_with_c<F>(&mut self, mut f: F)
    where
        F: FnMut((usize, usize)) -> T,
    {
        for x in 0..self.dims.0 {
            for y in 0..self.dims.1 {
                self[(x, y)] = f((x, y))
            }
        }
    }

    /// Swap two items with indexes `a` and `b`.
    pub fn swap(&mut self, a: (usize, usize), b: (usize, usize)) {
        let a = self.get_vec_index(a);
        let b = self.get_vec_index(b);
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
                let index = self.get_vec_index((x, y));
                let swapped_index = get_vec_index((y, x), self.dims.1);
                self.vec[swapped_index] = old[index].to_owned();
            }
        }
        self.invert_dims();
    }

    /// Flip the Mat row-wise.
    fn flip_r(&mut self)
    where
        T: Clone,
    {
        *self = self.slice((0, 0), self.dims, (true, false)).to_mat();
    }

    /// Flip the Mat column-wise.
    fn flip_c(&mut self)
    where
        T: Clone,
    {
        *self = self.slice((0, 0), self.dims, (false, true)).to_mat();
    }

    /// Flip the Mat row-wise and column-wise.
    fn flip_both(&mut self)
    where
        T: Clone,
    {
        *self = self.slice((0, 0), self.dims, (true, true)).to_mat();
    }

    /// Inverts the dimensions of the Mat.
    pub fn invert_dims(&mut self) {
        self.dims = (self.dims.1, self.dims.0);
    }

    /// Create a MatSlice starting at `index` with width and height
    /// `dims`. Flip the selected area row-wise with `flip.0`
    /// and/or column-wise with `flip.1`.
    fn slice<'a>(
        &'a self,
        index: (usize, usize),
        dims: (usize, usize),
        flip: (bool, bool),
    ) -> MatSlice<'a, T>
    where
        T: Clone,
    {
        assert!(index.0 < self.dims.0);
        assert!(index.1 < self.dims.1);
        assert!(index.0 + dims.0 - 1 < self.dims.0);
        assert!(index.1 + dims.1 - 1 < self.dims.1);

        MatSlice {
            mat: &self,
            index,
            dims,
            flip,
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

    /// Return the Vec representation of the Mat.
    pub fn to_vec(&self) -> Vec<T>
    where
        T: Clone,
    {
        self.vec.to_owned()
    }

    /// Return the 2d Vec representation of the Mat.
    pub fn to_2d_vec(&self) -> Vec<&[T]> {
        self.vec.chunks(self.dims.1.try_into().unwrap()).collect()
    }

    pub fn as_slice(&self) -> MatSlice<T>
    where
        T: Clone,
    {
        self.slice((0, 0), self.dims, (false, false))
    }

    /// Return a reference to the Vec representation of the Mat.
    pub fn vec(&self) -> &Vec<T> {
        &self.vec
    }

    /// The dimensions of the Mat (x and y).
    pub fn dims(&self) -> &(usize, usize) {
        &self.dims
    }

    fn get_vec_index(&self, index: (usize, usize)) -> usize {
        get_vec_index(index, self.dims.0)
    }
}

impl<T, I> Index<I> for Mat<T>
where
    I: Into<(usize, usize)>,
{
    type Output = T;

    fn index(&self, index: I) -> &Self::Output {
        let index = self.get_vec_index(index.into());
        &self.vec[index]
    }
}

impl<T, I> IndexMut<I> for Mat<T>
where
    I: Into<(usize, usize)>,
{
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        let index = self.get_vec_index(index.into());
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

pub struct MatSlice<'a, T> {
    mat: &'a Mat<T>,
    index: (usize, usize),
    dims: (usize, usize),
    flip: (bool, bool),
}

impl<'a, T> MatSlice<'a, T> {
    pub fn to_mat(&self) -> Mat<T>
    where
        T: Clone,
    {
        let b = (
            self.index.0 + self.dims.0 - 1,
            self.index.1 + self.dims.1 - 1,
        );

        let mut new_vec = self.mat.vec.clone();
        new_vec.truncate(dims_product(self.dims));

        for (x, u) in (self.index.0..=b.0).enumerate() {
            for (y, v) in (self.index.1..=b.1).enumerate() {
                let index = (
                    if !self.flip.0 { x } else { self.dims.0 - 1 - x },
                    if !self.flip.1 { y } else { self.dims.1 - 1 - y },
                );
                new_vec[get_vec_index(index, self.dims.0)] = self[(u, v)].to_owned();
            }
        }
        Mat::from_vec(new_vec, self.dims)
    }

    pub fn dims(&self) -> &(usize, usize) {
        &self.dims
    }
}

impl<'a, T, I> Index<I> for MatSlice<'a, T>
where
    I: Into<(usize, usize)>,
{
    type Output = T;
    fn index(&self, index: I) -> &Self::Output {
        let index = index.into();
        let index = (
            if !self.flip.0 {
                self.index.0 + index.0
            } else {
                self.index.0 + self.dims.0 - 1 - index.0
            },
            if !self.flip.1 {
                self.index.1 + index.1
            } else {
                self.index.1 + self.dims.1 - 1 - index.1
            },
        );
        &self.mat[index]
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
        let mut mat: Mat<bool> = Mat::filled_with(false, (1, 1));

        assert_eq!(mat[(0, 0)], false);

        mat[(0, 0)] = true;

        assert_eq!(mat[(0, 0)], true);
    }

    #[test]
    fn fill() {
        let mut mat: Mat<bool> = Mat::filled_with(false, (2, 2));

        mat.fill(true);

        for x in 0..2 {
            for y in 0..2 {
                assert_eq!(mat[(x, y)], true);
            }
        }
    }

    #[test]
    fn fill_with() {
        let mut mat: Mat<u16> = Mat::filled_with(0, (2, 3));

        let mut n: u16 = 0;

        mat.fill_with_r(|_| {
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
        mat1.flip_c();
        assert_eq!(mat1, Mat::from_vec([4, 5, 6, 7, 0, 1, 2, 3], mat1.dims));

        let mut mat2 = mat.clone();
        mat2.flip_r();
        assert_eq!(mat2, Mat::from_vec([3, 2, 1, 0, 7, 6, 5, 4], mat2.dims));

        let mut mat3 = mat.clone();
        mat3.flip_r();
        mat3.flip_c();
        assert_eq!(mat3, Mat::from_vec([7, 6, 5, 4, 3, 2, 1, 0], mat3.dims));
    }

    #[test]
    fn slice() {
        let mat = Mat::from_vec((0..16).collect::<Vec<_>>(), (4, 4));

        assert_eq!(
            mat.slice((0, 0), (4, 4), (false, false)).to_mat(),
            Mat::from_vec(
                [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
                (4, 4)
            )
        );
        assert_eq!(
            mat.slice((2, 2), (2, 2), (false, false)).to_mat(),
            Mat::from_vec([10, 11, 14, 15], (2, 2))
        );
        assert_eq!(
            mat.slice((0, 0), (4, 4), (true, false)).to_mat(),
            Mat::from_vec(
                [3, 2, 1, 0, 7, 6, 5, 4, 11, 10, 9, 8, 15, 14, 13, 12],
                (4, 4)
            )
        );
        assert_eq!(
            mat.slice((0, 0), (4, 1), (true, false)).to_mat(),
            Mat::from_vec([3, 2, 1, 0], (4, 1))
        );
        assert_eq!(
            mat.slice((0, 1), (4, 1), (false, false)).to_mat(),
            Mat::from_vec([4, 5, 6, 7], (4, 1))
        );

        assert_eq!(
            mat.slice((0, 0), (4, 4), (false, true)).to_mat(),
            Mat::from_vec(
                [12, 13, 14, 15, 8, 9, 10, 11, 4, 5, 6, 7, 0, 1, 2, 3],
                (4, 4)
            )
        );
        assert_eq!(
            mat.slice((0, 0), (4, 4), (true, true)).to_mat(),
            Mat::from_vec(
                [15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0],
                (4, 4)
            )
        );
        assert_eq!(
            mat.slice((0, 0), (1, 4), (false, true)).to_mat(),
            Mat::from_vec([12, 8, 4, 0], (1, 4))
        );
    }

    #[test]
    #[should_panic]
    fn slice_fail() {
        let mat = Mat::from_vec((0..16).collect::<Vec<_>>(), (4, 4));
        mat.slice((1, 1), (4, 4), (false, false));
    }

    #[test]
    fn mat_slice_index() {
        let mat = Mat::from_vec((0..16).collect::<Vec<_>>(), (4, 4));

        assert_eq!(mat.slice((1, 1), (2, 2), (false, false))[(0, 0)], 5);
        assert_eq!(mat.slice((1, 1), (2, 2), (true, false))[(0, 0)], 6);
        assert_eq!(mat.slice((1, 1), (2, 2), (true, true))[(0, 0)], 10);
        assert_eq!(mat.slice((1, 1), (2, 2), (true, true))[(1, 1)], 5);
    }
}
