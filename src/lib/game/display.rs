use std::path::Path;

use crate::{color::Color, mat::Mat};

pub enum Displayable {
    UniqueFrame(UniqueFrame),
    Frames(Frames),
    Animation(Animation),
}

impl Displayable {
    pub fn update(&mut self) {
        match self {
            Displayable::Animation(display) => {
                display.frame = (display.frame + 1) % display.states[display.state].len();
            }
            _ => {}
        }
    }

    pub fn current(&self) -> &Mat<Color> {
        match self {
            Displayable::UniqueFrame(display) => &display.state,
            Displayable::Frames(display) => &display.states[display.state],
            Displayable::Animation(display) => &display.states[display.state][display.frame],
        }
    }

    pub fn dims(&self) -> &(u32, u32) {
        match self {
            Displayable::UniqueFrame(display) => &display.dims,
            Displayable::Frames(display) => &display.dims,
            Displayable::Animation(display) => &display.dims,
        }
    }
    pub fn dims_mut(&mut self) -> &mut (u32, u32) {
        match self {
            Displayable::UniqueFrame(display) => &mut display.dims,
            Displayable::Frames(display) => &mut display.dims,
            Displayable::Animation(display) => &mut display.dims,
        }
    }

    pub fn state(&self) -> &usize {
        match self {
            Displayable::UniqueFrame(_) => {
                panic!("State doesn't exist for UniqueFrame display type.")
            }
            Displayable::Frames(display) => &display.state,
            Displayable::Animation(display) => &display.state,
        }
    }
    pub fn state_mut(&mut self) -> &mut usize {
        match self {
            Displayable::UniqueFrame(_) => {
                panic!("State doesn't exist for UniqueFrame display type.")
            }
            Displayable::Frames(display) => &mut display.state,
            Displayable::Animation(display) => &mut display.state,
        }
    }

    pub fn flip(&self) -> &(bool, bool) {
        match self {
            Displayable::UniqueFrame(display) => &display.flip,
            Displayable::Frames(display) => &display.flip,
            Displayable::Animation(display) => &display.flip,
        }
    }
    pub fn flip_mut(&mut self) -> &mut (bool, bool) {
        match self {
            Displayable::UniqueFrame(display) => &mut display.flip,
            Displayable::Frames(display) => &mut display.flip,
            Displayable::Animation(display) => &mut display.flip,
        }
    }
}

pub struct UniqueFrame {
    dims: (u32, u32),
    state: Mat<Color>,
    flip: (bool, bool),
}

impl UniqueFrame {
    pub fn from_files(path: &str, dims: (u32, u32)) -> Self {
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
                    .collect(),
                dims,
            ),
            flip: (false, false),
        }
    }

    pub fn from_color(color: Color, dims: (u32, u32)) -> Self {
        UniqueFrame {
            dims,
            state: Mat::new(color, dims),
            flip: (false, false),
        }
    }
}

impl Into<Displayable> for UniqueFrame {
    fn into(self) -> Displayable {
        Displayable::UniqueFrame(self)
    }
}

pub struct Frames {
    dims: (u32, u32),
    state: usize,
    states: Vec<Mat<Color>>,
    flip: (bool, bool),
}

impl Frames {
    pub fn from_files(paths: &[&str], dims: (u32, u32)) -> Self {
        let mut states = Vec::new();
        for path in paths {
            let image: image::ImageBuffer<image::Rgba<u8>, Vec<u8>> =
                image::open(path).unwrap().to_rgba8();
            states.push(Mat::from_vec(
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
                dims,
            ));
        }
        Frames {
            state: 0,
            dims,
            states,
            flip: (false, false),
        }
    }
}

impl Into<Displayable> for Frames {
    fn into(self) -> Displayable {
        Displayable::Frames(self)
    }
}

pub struct Animation {
    dims: (u32, u32),
    state: usize,
    frame: usize,
    states: Vec<Vec<Mat<Color>>>,
    flip: (bool, bool),
}

impl Animation {
    pub fn from_files<T>(paths: &[&[T]], dims: (u32, u32)) -> Self
    where
        T: AsRef<Path>,
    {
        let mut animations = Vec::new();
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
                frames.push(Mat::from_vec(image_pixels, dims));
            }
            animations.push(frames)
        }
        Animation {
            state: 0,
            frame: 0,
            states: animations,
            flip: (false, false),
            dims,
        }
    }
}

impl Into<Displayable> for Animation {
    fn into(self) -> Displayable {
        Displayable::Animation(self)
    }
}

// TODO Add TexturedAnimation
