use crate::{color::Color, mat::Mat};

pub struct Grid {
    background_color: Color,
    mat: Mat<Color>,
    dims: (usize, usize),
}

impl Grid {
    pub fn new(dims: (usize, usize), background_color: Option<Color>) -> Self {
        let background_color = background_color.unwrap_or(Color::transparent());
        Grid {
            background_color,
            mat: Mat::filled_with(background_color, dims),
            dims,
        }
    }

    pub fn draw(&self, pixels: &mut [u8]) {
        for (c, pix) in self.mat.iter().zip(pixels.chunks_exact_mut(4)) {
            pix.copy_from_slice(&c.to_bytes());
        }
    }

    pub fn clear_pixels(&mut self, last_pixels: &Vec<(usize, usize)>) {
        for index in last_pixels {
            self.mat[*index] = self.background_color;
        }
    }

    pub fn mat(&self) -> &Mat<Color> {
        &self.mat
    }
    // I could do a joke but I won't. Hint: France. Happy
    // googling.
    pub fn mat_mut(&mut self) -> &mut Mat<Color> {
        &mut self.mat
    }

    pub fn dims(&self) -> &(usize, usize) {
        &self.dims
    }
}
