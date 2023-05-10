fn get_index(index: (usize, usize), width: usize) -> usize {
    index.1 * width + index.0
}

/// Mat.
pub trait Mat<T> {
    fn to_2d_vec(&self, dims: (usize, usize)) -> Vec<&[T]>;

    fn swap_2d(&mut self, a: (usize, usize), b: (usize, usize), dims: (usize, usize));

    fn transpose(&mut self, dims: (usize, usize))
    where
        T: Clone;

    fn flip_h(&mut self, dims: (usize, usize))
    where
        T: Clone;

    fn flip_v(&mut self, dims: (usize, usize))
    where
        T: Clone;

    fn flip_both(&mut self, dims: (usize, usize))
    where
        T: Clone;

    fn slice(
        &self,
        index: (usize, usize),
        selection_dims: (usize, usize),
        revert: (bool, bool),
        dims: (usize, usize),
    ) -> Vec<T>
    where
        T: Clone;

    fn get(&self, index: (usize, usize), dims: (usize, usize)) -> &T;
    fn set(&mut self, index: (usize, usize), value: T, dims: (usize, usize));
}

impl<T> Mat<T> for Vec<T> {
    /// Return the 2d representation of the Vec.
    fn to_2d_vec(&self, dims: (usize, usize)) -> Vec<&[T]> {
        self.chunks(dims.1.try_into().unwrap()).collect()
    }

    /// Swap two elements of a 2d representation of the Vec.
    fn swap_2d(&mut self, a: (usize, usize), b: (usize, usize), dims: (usize, usize)) {
        let a = get_index(a, dims.0);
        let b = get_index(b, dims.0);
        self.swap(a, b);
    }

    /// Transpose the Vec.
    fn transpose(&mut self, dims: (usize, usize))
    where
        T: Clone,
    {
        let old = self.to_owned();
        for x in 0..dims.0 {
            for y in 0..dims.1 {
                let index = get_index((x, y), dims.0);
                let swapped_index = get_index((y, x), dims.1);
                self[swapped_index] = old[index].to_owned();
            }
        }
    }

    /// Flip the Vec in 2d around the horizontal axis.
    fn flip_h(&mut self, dims: (usize, usize))
    where
        T: Clone,
    {
        *self = self.slice((0, 0), dims, (true, false), dims);
    }

    /// Flip the Vec in 2d around the vertical axis.
    fn flip_v(&mut self, dims: (usize, usize))
    where
        T: Clone,
    {
        *self = self.slice((0, 0), dims, (false, true), dims);
    }

    /// Flip the Vec in 2d around the horizontal and vertical
    /// axes.
    fn flip_both(&mut self, dims: (usize, usize))
    where
        T: Clone,
    {
        *self = self.slice((0, 0), dims, (true, true), dims);
    }

    /// Create a 2d slice of the Vec starting at `index` and
    /// with dimensions `dims`.
    fn slice(
        &self,
        index: (usize, usize),
        select_dims: (usize, usize),
        revert: (bool, bool),
        dims: (usize, usize),
    ) -> Vec<T>
    where
        T: Clone,
    {
        let b = (index.0 + select_dims.0 - 1, index.1 + select_dims.1 - 1);

        assert!(index.0 < dims.0);
        assert!(index.1 < dims.1);
        assert!(b.0 < dims.0);
        assert!(b.1 < dims.1);

        let mut new_vec = self.clone();
        new_vec.truncate(select_dims.0 * select_dims.1);

        for (x, u) in (index.0..=b.0).enumerate() {
            for (y, v) in (index.1..=b.1).enumerate() {
                let index = (
                    if !revert.0 { x } else { select_dims.0 - 1 - x },
                    if !revert.1 { y } else { select_dims.1 - 1 - y },
                );
                new_vec[get_index(index, select_dims.0)] =
                    self[get_index((u, v), dims.0)].to_owned();
            }
        }
        new_vec
    }

    fn get(&self, index: (usize, usize), dims: (usize, usize)) -> &T {
        &self[get_index(index, dims.0)]
    }

    fn set(&mut self, index: (usize, usize), value: T, dims: (usize, usize)) {
        self[get_index(index, dims.0)] = value
    }
}

#[cfg(test)]
mod test {
    use super::Mat;

    #[test]
    fn swap() {
        let vec: Vec<_> = (0..6).collect();
        let dims = (3, 2);

        let mut vec1 = vec.clone();
        let mut vec2 = vec.clone();
        let mut vec3 = vec.clone();

        vec1.swap_2d((0, 0), (2, 1), dims);
        assert_eq!(vec1, [5, 1, 2, 3, 4, 0]);

        vec2.swap_2d((1, 0), (1, 0), dims);
        assert_eq!(vec2, vec);

        vec3.swap_2d((2, 0), (0, 1), dims);
        vec3.swap_2d((0, 1), (2, 1), dims);
        assert_eq!(vec3, [0, 1, 3, 5, 4, 2]);
    }

    #[test]
    fn transpose() {
        let mut vec: Vec<_> = (0..12).collect();
        let dims = (4, 3);

        vec.transpose(dims);

        assert_eq!(vec, [0, 4, 8, 1, 5, 9, 2, 6, 10, 3, 7, 11]);
    }

    #[test]
    fn flip() {
        let vec: Vec<_> = (0..8).collect();
        let dims = (4, 2);

        let mut vec1 = vec.clone();
        vec1.flip_v(dims);
        assert_eq!(vec1, [4, 5, 6, 7, 0, 1, 2, 3]);

        let mut vec2 = vec.clone();
        vec2.flip_h(dims);
        assert_eq!(vec2, [3, 2, 1, 0, 7, 6, 5, 4]);

        let mut vec3 = vec.clone();
        vec3.flip_h(dims);
        vec3.flip_v(dims);
        assert_eq!(vec3, [7, 6, 5, 4, 3, 2, 1, 0]);
    }

    #[test]
    fn sub_mat() {
        let vec: Vec<_> = (0..16).collect();
        let dims = (4, 4);

        println!("{:?}", vec.to_2d_vec(dims));

        assert_eq!(
            vec.slice((0, 0), (4, 4), (false, false), dims),
            [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]
        );
        assert_eq!(
            vec.slice((2, 2), (2, 2), (false, false), dims),
            [10, 11, 14, 15]
        );
        assert_eq!(
            vec.slice((0, 0), (4, 4), (true, false), dims),
            [3, 2, 1, 0, 7, 6, 5, 4, 11, 10, 9, 8, 15, 14, 13, 12]
        );
        assert_eq!(vec.slice((0, 0), (4, 1), (true, false), dims), [3, 2, 1, 0]);
        assert_eq!(
            vec.slice((0, 1), (4, 1), (false, false), dims),
            [4, 5, 6, 7]
        );

        assert_eq!(
            vec.slice((0, 0), (4, 4), (false, true), dims),
            [12, 13, 14, 15, 8, 9, 10, 11, 4, 5, 6, 7, 0, 1, 2, 3]
        );
        assert_eq!(
            vec.slice((0, 0), (4, 4), (true, true), dims),
            [15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0]
        );
        assert_eq!(
            vec.slice((0, 0), (1, 4), (false, true), dims),
            [12, 8, 4, 0]
        );
    }
}
