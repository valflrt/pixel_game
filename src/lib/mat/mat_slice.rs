use std::ops::Index;

use super::{dims_product, enumerate::EnumerateMat, get_vec_index, Mat, SlicedMat};

pub trait MatSlice<T>: Index<(usize, usize), Output = T> {
    /// Create a new Vec representation of a Mat from the
    /// MatSlice.
    fn to_vec(&self) -> Vec<T>
    where
        T: Clone,
    {
        let mut new_vec = self.mat().vec().clone();
        new_vec.truncate(self.len());

        // TODO rework that
        for (x, y) in EnumerateMat::<true>::new(*self.slice_dims()) {
            new_vec[get_vec_index((x, y), self.slice_dims().0)] = self[(x, y)].to_owned();
        }
        new_vec
    }

    /// Return the 2d Vec representation of the Mat.
    fn to_2d_vec(&self) -> Vec<Vec<T>>
    where
        T: Clone,
    {
        self.to_vec()
            .chunks(self.slice_dims().1)
            .map(|e| e.to_vec())
            .collect()
    }

    /// Create a new Mat from the MatSlice.
    fn to_mat(&self) -> Mat<T>
    where
        T: Clone,
    {
        Mat::from_vec(self.to_vec(), *self.slice_dims())
    }

    /// Create a MatSlice starting at `index` with width and height
    /// `dims`. The selected area can be flipped row-wise with
    /// `flip.0` and/or column-wise with `flip.1`.
    fn slice<'a>(
        &'a self,
        slice_index: (usize, usize),
        slice_dims: (usize, usize),
        flip_slice: (bool, bool),
    ) -> SlicedMat<'a, T> {
        let dims = self.mat().dims();
        assert!(slice_index.0 < dims.0);
        assert!(slice_index.1 < dims.1);
        assert!(slice_index.0 + slice_dims.0 - 1 < dims.0);
        assert!(slice_index.1 + slice_dims.1 - 1 < dims.1);

        SlicedMat::new(self.mat(), slice_index, slice_dims, flip_slice)
    }

    /// Create a SlicedMat from the entire Mat that can be flipped.
    fn slice_flip(&self, flip_slice: (bool, bool)) -> SlicedMat<T> {
        self.slice((0, 0), *self.slice_dims(), flip_slice)
    }

    /// Create a SlicedMat from the entire Mat.
    fn as_slice(&self) -> SlicedMat<T> {
        self.slice_flip((false, false))
    }

    /// Enumerate Mat indexes row-wise.
    fn enumerate_r(&self) -> EnumerateMat<true> {
        EnumerateMat::new(*self.slice_dims())
    }

    /// Enumerate Mat indexes column-wise.
    fn enumerate_c(&self) -> EnumerateMat<false> {
        EnumerateMat::new(*self.slice_dims())
    }

    /// Get a reference to the Vec representation of the Mat
    /// referenced by this MatSlice.
    fn vec(&self) -> &Vec<T> {
        &self.mat().vec
    }
    /// Get the number of items in the MatSlice.
    fn len(&self) -> usize {
        dims_product(*self.slice_dims())
    }

    /// Get the start index of the MatSlice.
    fn slice_index(&self) -> &(usize, usize);
    /// Get the dimensions of the MatSlice.
    fn slice_dims(&self) -> &(usize, usize);
    /// Get the flip "state" of the MatSlice.
    fn flip(&self) -> &(bool, bool);
    /// Get a reference to the Mat referenced by this MatSlice.
    fn mat(&self) -> &Mat<T>;
}
