use std::ops::Index;

use super::{dims_product, Mat, MatSlice};

#[derive(Debug, Clone, Copy)]
pub struct SlicedMat<'a, T> {
    mat: &'a Mat<T>,
    slice_index: (usize, usize),
    slice_dims: (usize, usize),
    flip_slice: (bool, bool),
}

impl<'a, T> SlicedMat<'a, T> {
    pub fn new(
        mat: &'a Mat<T>,
        slice_index: (usize, usize),
        slice_dims: (usize, usize),
        flip_slice: (bool, bool),
    ) -> Self {
        SlicedMat {
            mat,
            slice_index,
            slice_dims,
            flip_slice,
        }
    }
}

impl<'a, T> MatSlice<T> for SlicedMat<'a, T> {
    fn slice_index(&self) -> &(usize, usize) {
        &self.slice_index
    }
    fn slice_dims(&self) -> &(usize, usize) {
        &self.slice_dims
    }
    fn flip(&self) -> &(bool, bool) {
        &self.flip_slice
    }
    fn len(&self) -> usize {
        dims_product(self.slice_dims)
    }
    fn mat(&self) -> &Mat<T> {
        self.mat
    }
}

impl<'a, T> Index<(usize, usize)> for SlicedMat<'a, T> {
    type Output = T;
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let index = (
            if self.flip_slice.0 {
                self.slice_index.0 + self.slice_dims.0 - 1 - index.0
            } else {
                self.slice_index.0 + index.0
            },
            if self.flip_slice.1 {
                self.slice_index.1 + self.slice_dims.1 - 1 - index.1
            } else {
                self.slice_index.1 + index.1
            },
        );
        &self.mat[index]
    }
}
