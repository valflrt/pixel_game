use std::path::Path;

use crate::{color::Color, mat::Mat, vec2::Vec2};

pub enum Drawable {
    UniqueFrame(UniqueFrame),
    Frames(Frames),
    Animation(Animation),
}

impl Drawable {
    pub fn dims(&self) -> &Vec2<i32> {
        match self {
            Drawable::UniqueFrame(display) => &display.dims,
            Drawable::Frames(display) => &display.dims,
            Drawable::Animation(display) => &display.dims,
        }
    }
    pub fn dims_mut(&mut self) -> &mut Vec2<i32> {
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
    dims: Vec2<i32>,
    state: Mat<Color>,
}

impl UniqueFrame {
    pub fn from_files(path: &str, dims: Vec2<i32>) -> Self {
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
                dims,
            ),
        }
    }

    pub fn from_color<D>(color: Color, dims: D) -> Self
    where
        D: Into<Vec2<i32>>,
    {
        let dims = dims.into();
        UniqueFrame {
            dims,
            state: Mat::filled_with(color, dims),
        }
    }
}

impl Into<Drawable> for UniqueFrame {
    fn into(self) -> Drawable {
        Drawable::UniqueFrame(self)
    }
}

pub struct Frames {
    dims: Vec2<i32>,
    state: usize,
    states: Vec<Mat<Color>>,
}

impl Frames {
    pub fn from_spritesheet<T>(
        path: T,
        sprite_dims: Vec2<i32>,
        spritesheet_dims: Vec2<i32>,
        n_sprites: usize,
    ) -> Self
    where
        T: AsRef<Path>,
    {
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
        let image_dims = (
            sprite_dims.0 * spritesheet_dims.0,
            sprite_dims.1 * spritesheet_dims.1,
        );
        let image_mat = Mat::from_vec(image_pixels, image_dims);

        let mut states = Vec::new();

        let mut n: usize = 0;
        for v in 0..image_dims.1 {
            for u in 0..image_dims.0 {
                if n == n_sprites {
                    break;
                } else {
                    n += 1;
                }
                states.push(
                    image_mat
                        .slice((u, v), sprite_dims, (false, false))
                        .to_mat(),
                );
            }
            if n == n_sprites {
                break;
            }
        }

        Frames {
            state: 0,
            states,
            dims: sprite_dims,
        }
    }

    pub fn from_files(paths: &[&str], dims: Vec2<i32>) -> Self {
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
                    .collect::<Vec<_>>(),
                dims,
            ));
        }
        Frames {
            state: 0,
            dims,
            states,
        }
    }
}

impl Into<Drawable> for Frames {
    fn into(self) -> Drawable {
        Drawable::Frames(self)
    }
}

pub struct Animation {
    dims: Vec2<i32>,
    state: usize,
    frame: usize,
    states: Vec<Vec<Mat<Color>>>,
}

impl Animation {
    pub fn from_files<T>(paths: &[&[T]], dims: Vec2<i32>) -> Self
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
                frames.push(Mat::from_vec(image_pixels, dims));
            }
            states.push(frames)
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

// TODO Add TexturedAnimation
