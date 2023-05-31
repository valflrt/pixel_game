use crate::{color::Color, mat::Mat};

pub struct LookupTexture {
    texture: Mat<Color>,
}

impl LookupTexture {
    pub fn new(texture: Mat<Color>) -> Self {
        LookupTexture { texture }
    }

    pub fn render(&self, image: Mat<Color>) -> Mat<Color> {
        let dims = *image.dims();
        let output = Mat::filled_with(Color::TRANSPARENT, dims);
        for x in 0..dims.0 {
            for y in 0..dims.1 {
                let Color { r, g, a, .. } = image[(x, y)];
                if a == 255 {
                    self.texture[(r as usize, g as usize)];
                };
            }
        }
        output
    }
}
