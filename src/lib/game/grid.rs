use crate::{color::Color, mat::Mat, vec2::Vec2};

pub struct Grid {
    background_color: Color,
    mat: Mat<Color>,
    dims: Vec2,
}

impl Grid {
    pub fn new(dims: Vec2, background_color: Option<Color>) -> Self {
        let background_color = background_color.unwrap_or(Color::TRANSPARENT);
        Grid {
            background_color,
            mat: Mat::filled_with(background_color, dims.to_usize()),
            dims,
        }
    }

    pub fn clear_pixels(&mut self, pixels: &Vec<Vec2>) {
        for index in pixels {
            self.mat[(*index).to_usize()] = self.background_color;
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

    pub fn dims(&self) -> &Vec2 {
        &self.dims
    }
}
