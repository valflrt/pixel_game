use crate::{
    color::Color,
    mat::Mat,
    resources::{import_sprites, import_spritesheet},
};

pub enum Drawable {
    UniqueFrame(UniqueFrame),
    Frames(Frames),
    Animation(Animation),
}

impl Drawable {
    pub fn dims(&self) -> &(usize, usize) {
        match self {
            Drawable::UniqueFrame(display) => &display.dims,
            Drawable::Frames(display) => &display.dims,
            Drawable::Animation(display) => &display.dims,
        }
    }
    pub fn dims_mut(&mut self) -> &mut (usize, usize) {
        match self {
            Drawable::UniqueFrame(display) => &mut display.dims,
            Drawable::Frames(display) => &mut display.dims,
            Drawable::Animation(display) => &mut display.dims,
        }
    }

    pub fn state(&self) -> &usize {
        match self {
            Drawable::UniqueFrame(_) => {
                panic!("State doesn't exist for UniqueFrame display type.")
            }
            Drawable::Frames(display) => &display.state,
            Drawable::Animation(display) => &display.state,
        }
    }
    pub fn state_mut(&mut self) -> &mut usize {
        match self {
            Drawable::UniqueFrame(_) => {
                panic!("State doesn't exist for UniqueFrame display type.")
            }
            Drawable::Frames(display) => &mut display.state,
            Drawable::Animation(display) => &mut display.state,
        }
    }
}

impl Iterator for Drawable {
    type Item = Mat<Color>;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Drawable::Animation(d) => {
                d.frame = (d.frame + 1) % d.states[d.state].len();
                Some(d.states[d.state][d.frame].to_owned())
            }
            Drawable::Frames(d) => {
                d.state = (d.state + 1) % d.states.len();
                Some(d.states[d.state].to_owned())
            }
            Drawable::UniqueFrame(d) => Some(d.state.to_owned()),
        }
    }
}

pub struct UniqueFrame {
    dims: (usize, usize),
    state: Mat<Color>,
}

impl UniqueFrame {
    pub fn from_file(path: &str, dims: (usize, usize)) -> Self {
        let image: image::ImageBuffer<image::Rgba<u8>, Vec<u8>> =
            image::open(path).unwrap().to_rgba8();

        UniqueFrame {
            dims,
            state: Mat::from_vec(
                image
                    .as_raw()
                    .chunks(4)
                    .map(|v| Color {
                        r: v[0],
                        g: v[1],
                        b: v[2],
                        a: v[3],
                    })
                    .collect::<Vec<_>>(),
                dims.into(),
            ),
        }
    }

    pub fn from_color(color: Color, dims: (usize, usize)) -> Self {
        let dims = dims.into();
        UniqueFrame {
            dims,
            state: Mat::filled_with(color, dims.into()),
        }
    }
}

impl Into<Drawable> for UniqueFrame {
    fn into(self) -> Drawable {
        Drawable::UniqueFrame(self)
    }
}

pub struct Frames {
    dims: (usize, usize),
    state: usize,
    states: Vec<Mat<Color>>,
}

impl Frames {
    pub fn from_spritesheet(
        path: &str,
        sprite_dims: (usize, usize),
        spritesheet_dims: (usize, usize),
        n_sprites: usize,
    ) -> Self {
        Frames {
            state: 0,
            states: import_spritesheet(path, sprite_dims, spritesheet_dims, n_sprites),
            dims: sprite_dims,
        }
    }

    pub fn from_files(paths: &[&str], dims: (usize, usize)) -> Self {
        Frames {
            state: 0,
            dims,
            states: import_sprites(paths, dims),
        }
    }
}

impl Into<Drawable> for Frames {
    fn into(self) -> Drawable {
        Drawable::Frames(self)
    }
}

pub struct Animation {
    dims: (usize, usize),
    state: usize,
    frame: usize,
    states: Vec<Vec<Mat<Color>>>,
}

impl Animation {
    pub fn from_files(paths: &[&[&str]], dims: (usize, usize)) -> Self {
        let mut states = Vec::new();
        for frames_paths in paths {
            states.push(import_sprites(frames_paths, dims));
        }
        Animation {
            state: 0,
            frame: 0,
            states,
            dims,
        }
    }
}

impl Into<Drawable> for Animation {
    fn into(self) -> Drawable {
        Drawable::Animation(self)
    }
}
