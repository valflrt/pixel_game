pub struct EnumerateMat<const ROW_WISE: bool> {
    current: (usize, usize),
    dims: (usize, usize),
}

impl<const R: bool> EnumerateMat<R> {
    pub fn new(dims: (usize, usize)) -> Self {
        EnumerateMat {
            current: (0, 0),
            dims,
        }
    }
}

impl Iterator for EnumerateMat<true> {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.current.1 < self.dims.1 {
            let result = Some(self.current);

            self.current.0 += 1;
            if self.current.0 >= self.dims.0 {
                self.current.0 = 0;
                self.current.1 += 1;
            };

            result
        } else {
            None
        }
    }
}

impl Iterator for EnumerateMat<false> {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.current.0 < self.dims.0 {
            let result = Some(self.current);

            self.current.1 += 1;
            if self.current.1 >= self.dims.1 {
                self.current.1 = 0;
                self.current.0 += 1;
            };

            result
        } else {
            None
        }
    }
}
