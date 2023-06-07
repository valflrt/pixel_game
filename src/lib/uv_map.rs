use std::ops::Index;

use crate::{
    color::Color,
    mat::{Mat, MatSlice, SlicedMat},
};

pub struct UvMap {
    texture: Mat<Color>,
}

impl UvMap {
    pub fn new(texture: Mat<Color>) -> Self {
        UvMap { texture }
    }

    pub fn render<'a>(&self, image: &impl MatSlice<Color>) -> Mat<Color> {
        let dims = *image.slice_dims();
        let mut output = Mat::filled_with(Color::TRANSPARENT, dims);
        for x in 0..dims.0 {
            for y in 0..dims.1 {
                let Color { r, g, a, .. } = image[(x, y)];
                if a == 255 {
                    output[(x, y)] = self.texture[(r as usize, g as usize)].to_owned();
                };
            }
        }
        output
    }
}

pub struct UvRendered<'a> {
    map_slice: SlicedMat<'a, Color>,
    slice: SlicedMat<'a, Color>,
}

impl<'a> MatSlice<Color> for UvRendered<'a> {
    fn slice_index(&self) -> &(usize, usize) {
        self.slice.slice_index()
    }
    fn slice_dims(&self) -> &(usize, usize) {
        self.slice.slice_dims()
    }
    fn flip(&self) -> &(bool, bool) {
        self.slice.flip()
    }
    fn mat(&self) -> &Mat<Color> {
        self.slice.mat()
    }
}

impl<'a> Index<(usize, usize)> for UvRendered<'a> {
    type Output = Color;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let slice_index = self.slice.slice_index();
        let slice_dims = self.slice.slice_dims();
        let index = (
            if self.slice.flip().0 {
                slice_index.0 + slice_dims.0 - 1 - index.0
            } else {
                slice_index.0 + index.0
            },
            if self.slice.flip().1 {
                slice_index.1 + slice_dims.1 - 1 - index.1
            } else {
                slice_index.1 + index.1
            },
        );
        let Color { r, g, .. } = self.slice.mat()[index];
        &self.map_slice[(r as usize, g as usize)]
    }
}
