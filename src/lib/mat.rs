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
    default_value: T,
}

impl<T> Mat<T> {
    /// Create new Mat.
    pub fn filled_with(default_value: T, dims: (usize, usize)) -> Self
    where
        T: Clone,
    {
        Mat {
            dims,
            default_value: default_value.to_owned(),
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

    /// Create Mat from a vector and dimensions, the vector
    /// length must equal the product of the dimensions.
    pub fn from_vec(vec: Vec<T>, dims: (usize, usize)) -> Self
    where
        T: Clone,
    {
        assert_eq!(
            vec.len(),
            dims.0
                .checked_mul(dims.1)
                .expect("Mat dimensions are too big so their product is out of bounds")
                .try_into()
                .unwrap()
        );
        let default_value = vec[0].to_owned();
        Mat {
            dims,
            vec,
            default_value,
        }
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

    /// Flip the Mat around the horizontal axis, the vertical
    /// axis, or both.
    pub fn flip(&mut self, h: bool, v: bool)
    where
        T: Clone,
    {
        if h && v {
            self.vec = self.sub_mat((self.dims.0 - 1, self.dims.1 - 1), (0, 0)).vec;
        } else if h {
            self.vec = self.sub_mat((self.dims.0 - 1, self.dims.1 - 1), (0, 0)).vec;
        } else if v {
            self.vec = self.sub_mat((self.dims.0 - 1, self.dims.1 - 1), (0, 0)).vec;
        }
    }

    /// Inverts the dimensions of the Mat.
    pub fn invert_dims(&mut self) {
        self.dims = (self.dims.1, self.dims.0);
    }

    /// Create a sub-Mat from index `a` to index `b`.
    pub fn sub_mat(&self, a: (usize, usize), dims: (isize, isize)) -> Mat<T>
    where
        T: Clone,
    {
        let b = (
            a.0.wrapping_add(dims.0 as usize),
            a.1.wrapping_add(dims.1 as usize),
        );

        println!("{b:?}");

        assert!(a.0 < self.dims.0);
        assert!(a.1 < self.dims.1);
        // assert!(a.0.abs_diff(b.0) < self.dims.0);
        // assert!(a.1.abs_diff(b.1) < self.dims.1);

        let dims = (a.0.abs_diff(b.0) + 1, a.1.abs_diff(b.1) + 1);
        let mut mat = Mat::filled_with(self.default_value.to_owned(), dims);

        for (x, u) in (a.0..b.0).enumerate() {
            for (y, v) in (a.1..b.1).enumerate() {
                mat[(x, y)] = self[(u, v)].to_owned();
            }
        }
        mat
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
        let mat = Mat::from_vec((0..6).collect(), (3, 2));

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
        let mut mat = Mat::from_vec((0..12).collect(), (4, 3));
        mat.transpose();

        assert_eq!(
            mat,
            Mat::from_vec([0, 4, 8, 1, 5, 9, 2, 6, 10, 3, 7, 11].to_vec(), (3, 4))
        );
    }

    #[test]
    fn flip() {
        let mat = Mat::from_vec((0..8).collect(), (2, 4));

        let mut mat1 = mat.clone();
        let mut mat2 = mat.clone();
        let mut mat3 = mat.clone();
        let mut mat4 = mat.clone();

        mat1.flip(false, false);
        assert_eq!(
            mat1,
            Mat::from_vec([0, 1, 2, 3, 4, 5, 6, 7].to_vec(), (2, 4))
        );
        mat2.flip(false, true);
        assert_eq!(
            mat2,
            Mat::from_vec([4, 5, 6, 7, 0, 1, 2, 3].to_vec(), (2, 4))
        );
        mat3.flip(true, false);
        assert_eq!(
            mat3,
            Mat::from_vec([3, 2, 1, 0, 7, 6, 5, 4].to_vec(), (2, 4))
        );
        mat4.flip(true, true);
        assert_eq!(
            mat4,
            Mat::from_vec([7, 6, 5, 4, 3, 2, 1, 0].to_vec(), (2, 4))
        );
    }

    #[test]
    fn sub_mat() {
        let mat = Mat::from_vec((0..16).collect(), (4, 4));
        let sub_mat1 = mat.sub_mat((0, 0), (4, 4));
        let sub_mat2 = mat.sub_mat((2, 2), (2, 2));
        let sub_mat3 = mat.sub_mat((0, 0), (4, -4));

        assert_eq!(
            sub_mat1,
            Mat::from_vec(
                [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15].to_vec(),
                (4, 4)
            )
        );
        assert_eq!(sub_mat2, Mat::from_vec([10, 11, 14, 15].to_vec(), (2, 2)));
        assert_eq!(
            sub_mat3,
            Mat::from_vec(
                [3, 2, 1, 0, 7, 6, 5, 4, 11, 10, 9, 8, 15, 14, 13, 12].to_vec(),
                (4, 4)
            )
        );
    }
}
