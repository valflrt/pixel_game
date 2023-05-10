use crate::{color::Color, mat_trait::Mat};

pub struct Grid {
    background_color: Color,
    mat: Vec<Color>,
    dims: (usize, usize),
}

impl Grid {
    pub fn new(dims: (usize, usize), background_color: Option<Color>) -> Self {
        let background_color = background_color.unwrap_or(Color::transparent());
        Grid {
            background_color,
            mat: vec![background_color; dims.0 * dims.1],
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
            self.mat.set(*index, self.background_color, self.dims);
        }
    }

    pub fn mat(&self) -> &Vec<Color> {
        &self.mat
    }
    // I could do a joke but I won't. Hint: France. Happy
    // googling.
    pub fn mat_mut(&mut self) -> &mut Vec<Color> {
        &mut self.mat
    }

    pub fn dims(&self) -> &(usize, usize) {
        &self.dims
    }
}
