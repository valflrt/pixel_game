use std::path::Path;

use crate::color::Color;

pub enum Drawable {
    UniqueFrame(UniqueFrame),
    Frames(Frames),
    Animation(Animation),
}

impl Drawable {
    pub fn update(&mut self) {
        match self {
            Drawable::Animation(display) => {
                display.frame = (display.frame + 1) % display.states[display.state].len();
            }
            _ => {}
        }
    }

    pub fn current(&self) -> &Vec<Color> {
        match self {
            Drawable::UniqueFrame(display) => &display.state,
            Drawable::Frames(display) => &display.states[display.state],
            Drawable::Animation(display) => &display.states[display.state][display.frame],
        }
    }

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

    pub fn flip(&self) -> &(bool, bool) {
        match self {
            Drawable::UniqueFrame(display) => &display.flip,
            Drawable::Frames(display) => &display.flip,
            Drawable::Animation(display) => &display.flip,
        }
    }
    pub fn flip_mut(&mut self) -> &mut (bool, bool) {
        match self {
            Drawable::UniqueFrame(display) => &mut display.flip,
            Drawable::Frames(display) => &mut display.flip,
            Drawable::Animation(display) => &mut display.flip,
        }
    }
}

pub struct UniqueFrame {
    dims: (usize, usize),
    state: Vec<Color>,
    flip: (bool, bool),
}

impl UniqueFrame {
    pub fn from_files(path: &str, dims: (usize, usize)) -> Self {
        let image: image::ImageBuffer<image::Rgba<u8>, Vec<u8>> =
            image::open(path).unwrap().to_rgba8();

        UniqueFrame {
            dims,
            state: image
                .as_raw()
                .chunks(4)
                .map(|v| Color {
                    r: v[0],
                    g: v[1],
                    b: v[2],
                    a: v[3],
                })
                .collect(),
            flip: (false, false),
        }
    }

    pub fn from_color(color: Color, dims: (usize, usize)) -> Self {
        UniqueFrame {
            dims,
            state: vec![color; dims.0 * dims.1],
            flip: (false, false),
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
    states: Vec<Vec<Color>>,
    flip: (bool, bool),
}

impl Frames {
    pub fn from_files(paths: &[&str], dims: (usize, usize)) -> Self {
        let mut states = Vec::new();
        for path in paths {
            let image: image::ImageBuffer<image::Rgba<u8>, Vec<u8>> =
                image::open(path).unwrap().to_rgba8();
            states.push(
                image
                    .as_raw()
                    .chunks(4)
                    .map(|v| Color {
                        r: v[0],
                        g: v[1],
                        b: v[2],
                        a: v[3],
                    })
                    .collect(),
            );
        }
        Frames {
            state: 0,
            dims,
            states,
            flip: (false, false),
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
    states: Vec<Vec<Vec<Color>>>,
    flip: (bool, bool),
}

impl Animation {
    pub fn from_files<T>(paths: &[&[T]], dims: (usize, usize)) -> Self
    where
        T: AsRef<Path>,
    {
        let mut states = Vec::new();
        for frames_paths in paths {
            let mut frames = Vec::new();
            for path in *frames_paths {
                let image = image::open(path).unwrap().to_rgba8();
                let image_pixels: Vec<Color> = image
                    .as_raw()
                    .chunks(4)
                    .map(|v| Color {
                        r: v[0],
                        g: v[1],
                        b: v[2],
                        a: v[3],
                    })
                    .collect();
                frames.push(image_pixels);
            }
            states.push(frames)
        }
        Animation {
            state: 0,
            frame: 0,
            states,
            flip: (false, false),
            dims,
        }
    }
}

impl Into<Drawable> for Animation {
    fn into(self) -> Drawable {
        Drawable::Animation(self)
    }
}

// TODO Add TexturedAnimation
