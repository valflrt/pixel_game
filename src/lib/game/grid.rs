use crate::{color::Color, mat::Mat};

pub struct Grid {
    background_color: Color,
    mat: Mat<Color>,
}

impl Grid {
    pub fn new(dims: (u32, u32), background_color: Option<Color>) -> Self {
        let background_color = background_color.unwrap_or(Color::transparent());
        Grid {
            background_color,
            mat: Mat::new(background_color, dims),
        }
    }

    pub fn draw(&self, pixels: &mut [u8]) {
        for (c, pix) in self.mat.iter().zip(pixels.chunks_exact_mut(4)) {
            pix.copy_from_slice(&c.to_bytes());
        }
    }

    pub fn clear_pixels(&mut self, last_pixels: &Vec<(u32, u32)>) {
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
}
