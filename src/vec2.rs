use std::ops::{Index, IndexMut};

pub type Dims = (usize, usize);
pub type Pos = Dims;

#[derive(Debug, Clone)]
pub struct Vec2D<T> {
    dims: Dims,
    vec: Vec<T>,
}

impl<T> Vec2D<T> {
    pub fn new(dims: Dims) -> Self {
        println!("{}", dims.0 * dims.1);
        Vec2D {
            dims,
            vec: Vec::new(),
        }
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

    pub fn clear(&mut self) {
        self.vec.clear();
    }
}

impl<T> Index<Pos> for Vec2D<T> {
    type Output = T;

    fn index(&self, index: Pos) -> &Self::Output {
        &self.vec[index.0 * self.dims.1 + index.1]
    }
}

impl<T> IndexMut<Pos> for Vec2D<T> {
    fn index_mut(&mut self, index: Pos) -> &mut Self::Output {
        &mut self.vec[index.0 * self.dims.1 + index.1]
    }
}

/* #[cfg(test)]
mod test {
    use super::Vec2D;

    #[test]
    fn index() {
        let mut vec: Vec2D<bool> = Vec2D::new((2, 2));

        assert_eq!(vec[(0, 0)], false);

        vec[(0, 0)] = true;

        assert_eq!(vec[(0, 0)], true);
    }

    #[test]
    fn fill() {
        let mut vec: Vec2D<bool> = Vec2D::new((2, 2));

        vec.fill(true);

        for x in 0..2 {
            for y in 0..2 {
                assert_eq!(vec[(x, y)], true);
            }
        }
    }

    #[test]
    fn fill_with() {
        let mut vec: Vec2D<u16> = Vec2D::new((2, 3));

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
} */
